use std::{
    collections::HashMap,
    env::{self},
    pin::Pin,
};

use futures::{
    channel::mpsc::UnboundedReceiver, pin_mut, Future, FutureExt, StreamExt, TryStreamExt,
};
use genetlink::{message::RawGenlMessage, GenetlinkHandle};
use netlink_packet_core::{
    NetlinkHeader, NetlinkMessage, NetlinkPayload, NLM_F_DUMP, NLM_F_REQUEST,
};
use netlink_packet_generic::{
    ctrl::{
        nlas::{GenlCtrlAttrs, McastGrpAttrs},
        GenlCtrl, GenlCtrlCmd,
    },
    GenlMessage,
};
use netlink_packet_route::{
    link::LinkAttribute,
    route::{RouteAttribute, RouteMessage},
    RouteNetlinkMessage,
};
use netlink_proto::Connection;
use netlink_sys::{AsyncSocket, SocketAddr};
use rtnetlink::{
    constants::{
        RTMGRP_IPV4_IFADDR, RTMGRP_IPV4_ROUTE, RTMGRP_IPV6_IFADDR, RTMGRP_IPV6_ROUTE, RTMGRP_LINK,
    },
    IpVersion,
};
use wl_nl80211::{Nl80211Attr, Nl80211Handle, Nl80211Message};

/// This netlink code uses a lot of enums. This convenient macro emits a match expression that
/// evaluates to Some($value) if $name matches $pattern, and None otherwise.
macro_rules! some_if_matches {
    ($name: ident, $pattern: pat, $value: expr) => {
        match $name {
            $pattern => Some($value),
            _ => None,
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NetworkState {
    /// Reached when low level transport is down (no carrier, no associated SSID, etc.).
    Disconnected,

    /// Reached when low level transport is up, but there is no default route.
    Connecting,

    /// Reached when a default route is present and the low level transport is up.
    Connected,
}

/// Represents the connection state of a wireless interface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WirelessInterface {
    interface_index: u32,
    is_associated: bool,
    default_route_present: bool,
}

// TODO: Implement EthernetState. Document the decision to prioritize Wifi over Ethernet and the
// cases where this breaks down (e.g. on a Router, where the wireless interface is not WAN-facing).

pub async fn get_interface_index(
    rtnetlink: &rtnetlink::Handle,
    interface_name: &str,
) -> anyhow::Result<u32> {
    let mut links = rtnetlink.link().get().execute();
    while let Some(link) = links.try_next().await? {
        let name = link
            .attributes
            .iter()
            .find_map(|attr| some_if_matches!(attr, LinkAttribute::IfName(name), name))
            .ok_or(MissingAttributeError("IfName".to_string()))?;
        if name == interface_name {
            return Ok(link.header.index);
        }
    }

    panic!("No interface matching name {}", interface_name);
}

impl WirelessInterface {
    pub async fn new(
        interface_index: u32,
        rtnetlink: &rtnetlink::Handle,
        nl80211: &Nl80211Handle,
    ) -> anyhow::Result<Self> {
        let mut default_route_present = false;
        let mut routes = rtnetlink.route().get(IpVersion::V4).execute();
        while let Some(route) = routes.try_next().await? {
            let route = Route::from(route);
            let oif = route
                .oif()
                .ok_or(MissingAttributeError("Oif".to_string()))?;
            if oif == interface_index && route.is_default_route() {
                default_route_present = true;
            }
        }

        let mut is_associated = false;
        let mut interfaces = nl80211.interface().get().execute().await;
        while let Some(interface) = interfaces.try_next().await? {
            let wiphy = Wiphy::from(interface);
            let index = wiphy
                .id()
                .ok_or(MissingAttributeError("IfIndex".to_string()))?;
            if index == interface_index && wiphy.is_associated_with_ssid() {
                is_associated = true;
            }
        }

        Ok(WirelessInterface {
            interface_index,
            default_route_present,
            is_associated,
        })
    }

    pub fn handle_event<T>(self, event: Option<(NetlinkMessage<T>, SocketAddr)>) -> Self
    where
        WirelessInterface: Transition<T>,
    {
        let Some((event, _)) = event else {
            return self;
        };
        let NetlinkPayload::InnerMessage(event) = event.payload else {
            return self;
        };

        self.transition(event)
    }

    // TODO: Should probably refactor this method (and handle_event?) out into a monad.
    pub fn replace_if_changed(&mut self, next_state: WirelessInterface) -> Option<&Self> {
        if self != &next_state {
            *self = next_state;
            Some(self)
        } else {
            None
        }
    }

    // TODO: Should maybe be a trait?
    pub fn network_state(&self) -> NetworkState {
        if self.is_associated && self.default_route_present {
            NetworkState::Connected
        } else if self.is_associated {
            NetworkState::Connecting
        } else {
            NetworkState::Disconnected
        }
    }
}

const NL80211_CMD_CONNECT: u8 = 46;
const NL80211_CMD_DISCONNECT: u8 = 48;

/// A state may transition on multiple kinds of events.
trait Transition<Event> {
    fn transition(self, event: Event) -> Self
    where
        Self: Sized;
}

impl Transition<RawGenlMessage> for WirelessInterface {
    fn transition(mut self, event: RawGenlMessage) -> Self
    where
        Self: Sized,
    {
        let (header, _) = event.into_parts();
        // TODO: Need to parse the message here and check the interface this event was triggered
        // on.
        if header.cmd == NL80211_CMD_CONNECT {
            self.is_associated = true;
        } else if header.cmd == NL80211_CMD_DISCONNECT {
            self.is_associated = false;
        }

        self
    }
}

impl Transition<RouteNetlinkMessage> for WirelessInterface {
    fn transition(mut self, event: RouteNetlinkMessage) -> Self {
        let (route, present) = match event {
            RouteNetlinkMessage::NewRoute(route) => (route, true),
            RouteNetlinkMessage::DelRoute(route) => (route, false),
            _ => return self,
        };

        let route = Route::from(route);
        let Some(oif) = route.oif() else {
            return self;
        };
        if oif == self.interface_index && route.is_default_route() {
            self.default_route_present = present;
        }

        self
    }
}

#[derive(Debug)]
struct MissingAttributeError(String);
impl std::error::Error for MissingAttributeError {}
impl std::fmt::Display for MissingAttributeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Missing attribute \"{}\"", self.0)
    }
}

#[derive(Debug)]
struct MissingFamilyError;
impl std::error::Error for MissingFamilyError {}
impl std::fmt::Display for MissingFamilyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Missing attribute")
    }
}

fn family_name_filter<'a>(
    name: &'a str,
) -> impl Fn(
    GenlMessage<GenlCtrl>,
) -> Pin<Box<dyn Future<Output = Option<GenlMessage<GenlCtrl>>> + 'a>>
       + 'a {
    move |message| {
        let future = async move {
            let matches = message
                .payload
                .nlas
                .iter()
                .any(|attr| matches!(attr, GenlCtrlAttrs::FamilyName(attr) if attr == name));
            if matches {
                Some(message)
            } else {
                None
            }
        };
        Box::pin(future)
    }
}

async fn get_family_by_name(
    handle: &mut GenetlinkHandle,
    name: &str,
) -> anyhow::Result<GenlMessage<GenlCtrl>> {
    let mut nl_hdr = NetlinkHeader::default();
    nl_hdr.flags = NLM_F_REQUEST | NLM_F_DUMP;
    let nlmsg = NetlinkMessage::new(
        nl_hdr,
        GenlMessage::from_payload(GenlCtrl {
            cmd: GenlCtrlCmd::GetFamily,
            nlas: vec![],
        })
        .into(),
    );

    let responses = handle
        .request(nlmsg)
        .await?
        .filter_map(|value| async { value.ok() })
        .filter_map(|NetlinkMessage { payload, .. }| async {
            some_if_matches!(payload, NetlinkPayload::InnerMessage(m), m)
        })
        .filter_map(family_name_filter(name));
    pin_mut!(responses);

    responses
        .next()
        .await
        .ok_or_else(|| MissingFamilyError.into())
}

async fn get_mcast_groups(family: GenlMessage<GenlCtrl>) -> anyhow::Result<HashMap<String, u32>> {
    // Each family message is an iovec of attributes, where each attribute is a GenlCtrlAttrs
    // object.
    let mcast_groups = family
        .payload
        .nlas
        .into_iter()
        .find_map(|attr| {
            if let GenlCtrlAttrs::McastGroups(groups) = attr {
                Some(groups)
            } else {
                None
            }
        })
        .ok_or(MissingAttributeError("McastGroups".to_string()))?
        .into_iter()
        .map(|attrs| {
            let id = attrs
                .iter()
                .find_map(|v| some_if_matches!(v, McastGrpAttrs::Id(i), *i))
                .unwrap();
            let name = attrs
                .into_iter()
                .find_map(|v| some_if_matches!(v, McastGrpAttrs::Name(n), n))
                .unwrap();
            (name, id)
        })
        .collect::<HashMap<String, u32>>();
    Ok(mcast_groups)
}

struct Route(RouteMessage);
impl From<RouteMessage> for Route {
    fn from(value: RouteMessage) -> Self {
        Self(value)
    }
}

impl Route {
    pub fn is_in_table(&self, table: u32) -> bool {
        self.0
            .attributes
            .iter()
            .any(|attr| matches!(attr, RouteAttribute::Table(id) if *id == table))
    }

    pub fn has_destination(&self) -> bool {
        self.0
            .attributes
            .iter()
            .any(|attr| matches!(attr, RouteAttribute::Destination(_)))
    }

    pub fn destination_prefix_length(&self) -> u8 {
        self.0.header.destination_prefix_length
    }

    /// The default route exists in the main routing table (Always table 254, see:
    /// http://linux-ip.net/html/routing-tables.html#idm140337857356016), has no destination, and a
    /// destination prefix length of 0
    /// (https://github.com/iproute2/iproute2/blob/main/ip/iproute.c#L810)
    pub fn is_default_route(&self) -> bool {
        self.is_in_table(254) && !self.has_destination() && 0 == self.destination_prefix_length()
    }

    pub fn oif(&self) -> Option<u32> {
        self.0
            .attributes
            .iter()
            .find_map(|attr| some_if_matches!(attr, RouteAttribute::Oif(id), *id))
    }
}

struct Wiphy(GenlMessage<Nl80211Message>);
impl From<GenlMessage<Nl80211Message>> for Wiphy {
    fn from(value: GenlMessage<Nl80211Message>) -> Self {
        Self(value)
    }
}

impl Wiphy {
    pub fn is_associated_with_ssid(&self) -> bool {
        self.0
            .payload
            .nlas
            .iter()
            .any(|attr| matches!(attr, Nl80211Attr::Ssid(_)))
    }

    pub fn id(&self) -> Option<u32> {
        self.0
            .payload
            .nlas
            .iter()
            .find_map(|attr| some_if_matches!(attr, Nl80211Attr::IfIndex(id), *id))
    }
}

pub async fn new_genetlink_connection() -> anyhow::Result<(
    Connection<RawGenlMessage>,
    Nl80211Handle,
    UnboundedReceiver<(NetlinkMessage<RawGenlMessage>, SocketAddr)>,
)> {
    let mcast_groups = {
        let (conn, mut handle, _) = genetlink::new_connection()?;
        tokio::spawn(conn);
        get_mcast_groups(get_family_by_name(&mut handle, "nl80211").await?)
            .await?
            .into_values()
    };

    // TODO: Only spin up an nl80211 connection if our interface is a wiphy.
    let (mut connection, nlhandle, nl80211) = wl_nl80211::new_connection()?;
    let addr = SocketAddr::new(0, 0);
    let socket = connection.socket_mut().socket_mut();
    socket.bind(&addr)?;
    for group in mcast_groups {
        socket.add_membership(group)?;
    }
    Ok((connection, nlhandle, nl80211))
}

async fn new_rtnetlink_connection() -> anyhow::Result<(
    Connection<RouteNetlinkMessage>,
    rtnetlink::Handle,
    UnboundedReceiver<(NetlinkMessage<RouteNetlinkMessage>, SocketAddr)>,
)> {
    let (mut connection, rthandle, rtnetlink) = rtnetlink::new_connection()?;
    let mgroup_flags = RTMGRP_LINK
        | RTMGRP_IPV4_IFADDR
        | RTMGRP_IPV4_ROUTE
        | RTMGRP_IPV6_IFADDR
        | RTMGRP_IPV6_ROUTE;
    let addr = SocketAddr::new(0, mgroup_flags);
    connection.socket_mut().socket_mut().bind(&addr)?;
    Ok((connection, rthandle, rtnetlink))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let (connection, nlhandle, mut nl80211) = new_genetlink_connection().await?;
    tokio::spawn(connection);
    let (connection, rthandle, mut rtnetlink) = new_rtnetlink_connection().await?;
    tokio::spawn(connection);

    // TODO: This will suffice for now, but in the future we should enumerate all links and
    // intelligently track the state of each.
    let mut args = env::args();
    args.next();
    let interface_name = args.next().unwrap();

    let interface_index = get_interface_index(&rthandle, &interface_name).await?;
    let mut wifi = WirelessInterface::new(interface_index, &rthandle, &nlhandle).await?;
    let state = wifi.network_state();
    dbg!(state);
    loop {
        let mut nl80211 = nl80211.next().fuse();
        let mut rtnetlink = rtnetlink.next().fuse();

        let next_state = futures::select! {
            event = nl80211 => wifi.handle_event(event),
            event = rtnetlink => wifi.handle_event(event),
        };

        if wifi.replace_if_changed(next_state).is_some() {
            let state = wifi.network_state();
            dbg!(state);
        }
    }
}
