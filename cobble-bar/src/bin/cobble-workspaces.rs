use std::{
    collections::VecDeque,
    env,
    path::PathBuf,
    pin::Pin,
    str::FromStr,
    task::{Context, Poll},
};

use futures::{pin_mut, stream, Stream, StreamExt};
use regex::Regex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixSocket,
};

lazy_static::lazy_static! {
    static ref WORKSPACE: Regex = Regex::new("^workspace ID ([0-9])").unwrap();
    static ref EVENT: Regex = Regex::new("^([a-z0-9]*)>>(.*)$").unwrap();
}

#[derive(Debug)]
enum HyprlandError {
    DoesNotExist,
    InvalidEvent,
}
impl std::error::Error for HyprlandError {}
impl std::fmt::Display for HyprlandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoesNotExist => {
                write!(f, "The Hyprland socket directory doesn't exist")
            }
            HyprlandError::InvalidEvent => {
                write!(f, "Received garbage on the Hyprland event socket")
            }
        }
    }
}

#[derive(Debug)]
enum HyprlandEvent {
    Workspace(u32),
    CreateWorkspace(u32),
    DestroyWorkspace(u32),
}

fn parse_workspace(text: &str) -> Option<(u32, String)> {
    let mut items = text.split(",");
    let id: u32 = items.next()?.parse().ok()?;
    let name = items.next()?.to_string();
    Some((id, name))
}

fn parse_event(text: &str) -> Option<HyprlandEvent> {
    let captures = EVENT.captures(text)?;
    let event_name = captures.get(1)?;
    let event_content = captures.get(2)?;
    let (id, _) = parse_workspace(event_content.as_str())?;
    let event = match event_name.as_str() {
        "createworkspacev2" => HyprlandEvent::CreateWorkspace(id),
        "workspacev2" => HyprlandEvent::Workspace(id),
        "destroyworkspacev2" => HyprlandEvent::DestroyWorkspace(id),
        _ => return None,
    };
    Some(event)
}

impl FromStr for HyprlandEvent {
    type Err = HyprlandError;

    fn from_str(event: &str) -> Result<Self, Self::Err> {
        parse_event(event).ok_or(HyprlandError::InvalidEvent)
    }
}

#[derive(Clone, Copy, Debug)]
enum WorkspaceState {
    Nonexistent,
    Inactive,
    Active,
}

#[derive(Debug)]
struct Workspaces {
    workspaces: Vec<u32>,
    active_workspace: u32,
}

impl Workspaces {
    /// Transition the state of the workspaces on an event.
    pub fn transition(self, event: HyprlandEvent) -> Self {
        match event {
            HyprlandEvent::Workspace(id) => self.transition_active_workspace(id),
            HyprlandEvent::CreateWorkspace(id) => self.create_workspace(id),
            HyprlandEvent::DestroyWorkspace(id) => self.destroy_workspace(id),
        }
    }

    fn transition_active_workspace(mut self, id: u32) -> Self {
        self.active_workspace = id;
        self
    }

    fn create_workspace(mut self, id: u32) -> Self {
        self.workspaces.push(id);
        self
    }

    fn destroy_workspace(mut self, id: u32) -> Self {
        let index = self
            .workspaces
            .iter()
            .enumerate()
            .find_map(|(index, workspace_id)| {
                if id == *workspace_id {
                    Some(index)
                } else {
                    None
                }
            })
            .unwrap();
        self.workspaces.remove(index);
        self
    }
}

/// Return Some if the directory at the provided path exists.
fn directory_if_exists(path: String) -> Option<PathBuf> {
    let directory = PathBuf::from(path);
    match directory.exists() {
        true => Some(directory),
        false => None,
    }
}

/// Extract the workspace id from the response of a hyprctl-style query on the compositor.
fn workspace_id(line: &str) -> Option<u32> {
    let captures = WORKSPACE.captures(line)?;
    let id = captures.get(1)?;
    let id: u32 = id.as_str().parse().ok()?;
    Some(id)
}

/// A stream that flattens collections of received events. If we have a stream that produces a
/// collection, wrap it in this stream to produce a stream that yields a single event at a time.
#[pin_project::pin_project]
struct FlatteningStream<I, S> {
    pending: VecDeque<I>,
    #[pin]
    inner: S,
}
impl<I, S> Stream for FlatteningStream<I, S>
where
    S: Stream<Item = VecDeque<I>>,
    I: std::fmt::Debug,
{
    type Item = I;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        if let Some(event) = this.pending.pop_front() {
            return Poll::Ready(Some(event));
        }

        let Poll::Ready(events) = this.inner.poll_next(context) else {
            return Poll::Pending;
        };

        let Some(mut events) = events else {
            return Poll::Ready(None);
        };

        this.pending.append(&mut events);
        Poll::Ready(this.pending.pop_front())
    }
}

/// Convenience function for wrapping a stream in a [FlatteningStream].
fn flatten<I, S>(inner: S) -> impl Stream<Item = I>
where
    S: Stream<Item = VecDeque<I>>,
    I: std::fmt::Debug,
{
    FlatteningStream {
        pending: VecDeque::new(),
        inner,
    }
}

/// Proxy class for interacting with the Hyprland server.
struct Hyprland {
    socket_directory: PathBuf,
}

impl Hyprland {
    /// Create a connection to the Hyprland server.
    pub async fn new_connection() -> anyhow::Result<Self> {
        let instance_signature = env::var("HYPRLAND_INSTANCE_SIGNATURE")?;
        let xdg_runtime_dir = env::var("XDG_RUNTIME_DIR")?;

        // NOTE: In a recent version of hyprland, sockets were moved from /tmp/hypr to $XDG_RUNTIME_DIR/hypr
        let recent_socket_path = format!("/{}/hypr/{}", xdg_runtime_dir, instance_signature);
        let legacy_socket_path = format!("/tmp/hypr/{}", instance_signature);
        let socket_directory = directory_if_exists(recent_socket_path)
            .or_else(|| directory_if_exists(legacy_socket_path))
            .ok_or(HyprlandError::DoesNotExist)?;

        Ok(Hyprland { socket_directory })
    }

    async fn request(&self, path: &[u8]) -> anyhow::Result<String> {
        let socket_path = self.socket_directory.join(".socket.sock");
        let socket = UnixSocket::new_stream()?;
        let mut control = socket.connect(socket_path).await?;

        control.write(path).await?;
        let mut response = String::new();
        control.read_to_string(&mut response).await?;

        Ok(response)
    }

    /// Query the state of the existing workspaces.
    pub async fn workspaces(&mut self) -> anyhow::Result<Workspaces> {
        let active_workspace = self.active_workspace().await?;
        let response = self.request(b"/workspaces\0".as_ref()).await?;
        let workspaces = response
            .lines()
            .filter_map(workspace_id)
            .collect::<Vec<u32>>();
        Ok(Workspaces {
            workspaces,
            active_workspace,
        })
    }

    async fn active_workspace(&mut self) -> anyhow::Result<u32> {
        let response = self.request(b"/activeworkspace\0".as_ref()).await?;
        Ok(response.lines().find_map(workspace_id).unwrap())
    }

    /// Consume the Hyprland proxy object and produce a stream that emits Hyprland events.
    pub async fn into_event_stream(self) -> anyhow::Result<impl Stream<Item = HyprlandEvent>> {
        let socket_path = self.socket_directory.join(".socket2.sock");
        let socket = UnixSocket::new_stream()?;
        let stream = socket.connect(socket_path).await?;

        Ok(flatten(stream::unfold(stream, |mut stream| async move {
            let mut events = VecDeque::new();
            while events.is_empty() {
                let mut buffer = [0u8; 128];
                stream.read(&mut buffer).await.ok()?;
                let event = String::from_utf8_lossy(&buffer);
                events = event
                    .trim_matches(char::from(0))
                    .lines()
                    .filter_map(|line| line.parse().ok())
                    .collect::<VecDeque<HyprlandEvent>>();
            }
            Some((events, stream))
        })))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let mut hyprland = Hyprland::new_connection().await?;
    let mut state = hyprland.workspaces().await?;
    dbg!(&state);

    let events = hyprland.into_event_stream().await?;
    pin_mut!(events);

    while let Some(event) = events.next().await {
        state = state.transition(event);
        dbg!(&state);
    }
    Ok(())
}
