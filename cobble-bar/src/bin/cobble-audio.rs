use glib::{prelude::*, translate::IntoGlib, value::FromValue, Value, Variant, VariantTy};
use serde::Serialize;
use std::{borrow::Cow, collections::HashMap, future::Future, sync::Arc};
use wireplumber::traits::ObjectExt as _;
use wireplumber::{
    ConstraintType, ConstraintVerb, Core, Node, ObjectInterest, ObjectManager, Plugin,
    PluginFeatures,
};

/// Widget state, as reported by wireplumber signals.
#[derive(Debug, Serialize)]
struct State {
    /// The current volume of the default audio sink.
    volume: f64,
}

impl StaticVariantType for State {
    fn static_variant_type() -> Cow<'static, glib::VariantTy> {
        Cow::Borrowed(VariantTy::VARDICT)
    }
}

impl FromVariant for State {
    fn from_variant(variant: &Variant) -> Option<Self> {
        let map: HashMap<String, Variant> = HashMap::from_variant(variant)?;
        let property = map.get("volume")?;
        let volume = f64::from_variant(&property)?;
        Some(Self { volume })
    }
}

/// Wrap a function that takes `&T` into a function that takes `&[Value]`. This is handy for
/// converting GLib signal handlers between the two different interfaces that may be used.
fn signal_handler<F, T>(inner: F) -> impl Fn(&[Value]) -> Option<Value>
where
    F: Fn(&T),
    for<'a> T: FromValue<'a>,
{
    move |value: &[Value]| {
        inner(&value[0].get::<T>().expect("Type casting failed!"));
        None
    }
}

/// An error that may occur while attempting to load a Wireplumber plugin.
#[derive(Debug)]
struct PluginLoadError(String);
impl std::error::Error for PluginLoadError {}
impl std::fmt::Display for PluginLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Couldn't load plugin: {}", self.0)
    }
}

/// Convenience for ergonomically loading plugins.
trait LoadPlugin {
    fn load_plugin(
        &self,
        name: &str,
        provides: &str,
    ) -> impl Future<Output = anyhow::Result<Plugin>>;
}

impl LoadPlugin for Core {
    async fn load_plugin(&self, name: &str, provides: &str) -> anyhow::Result<Plugin> {
        self.load_component_future(Some(name), "module", None, Some(provides))
            .await?;
        let plugin =
            Plugin::find(self, provides).ok_or_else(|| PluginLoadError(name.to_string()))?;
        plugin
            .upcast_ref::<glib::Object>()
            .downcast_ref::<wireplumber::Object>()
            .unwrap()
            .activate_future(PluginFeatures::ENABLED.into_glib())
            .await?;
        Ok(plugin)
    }
}

/// Prints state for the audio widget.
fn print_state(default_nodes_api: &Plugin, mixer_api: &Plugin) {
    let node_id: u32 = default_nodes_api.emit_by_name("get-default-node", &[&"Audio/Sink"]);
    let result: Variant = mixer_api.emit_by_name("get-volume", &[&node_id]);
    let Some(state) = State::from_variant(&result) else {
        return;
    };
    let Ok(message) = serde_json::to_string(&state) else {
        return;
    };
    println!("{}", message);
}

/// Sets up the application by registering signal handlers and initializing objects.
async fn application(core: &Core, object_manager: &ObjectManager) -> anyhow::Result<()> {
    let interest = ObjectInterest::new_type(Node::static_type());
    interest.add_constraint(
        ConstraintType::PwProperty,
        "media.class",
        ConstraintVerb::Equals,
        Some(&"Audio/Sink".to_variant()),
    );
    object_manager.add_interest_full(interest);

    let default_nodes_api = core
        .load_plugin(
            "libwireplumber-module-default-nodes-api",
            "default-nodes-api",
        )
        .await?;
    let mixer_api = core
        .load_plugin("libwireplumber-module-mixer-api", "mixer-api")
        .await?;

    let printer = Arc::new(signal_handler(move |mixer_api: &Plugin| {
        print_state(&default_nodes_api, mixer_api);
    }));
    object_manager.connect_installed(move |_| {
        let printer = Arc::clone(&printer);
        (*printer)(&[mixer_api.to_value()]);
        mixer_api.connect("changed", true, move |value| (*printer)(value));
    });
    core.install_object_manager(&object_manager);
    Ok(())
}

fn main() {
    unsafe { ffi::wp_init(ffi::WP_INIT_PIPEWIRE) };

    let context = glib::MainContext::default();

    let core = Core::new(Some(&context), None, None);
    core.connect();

    // NOTE: If the Core/ObjectManager are allowed to Drop, e.g. by moving them into the Future
    // created below, the signal handlers will never run.
    let object_manager = ObjectManager::new();

    context
        .block_on(application(&core, &object_manager))
        .unwrap();

    let main_loop = glib::MainLoop::new(Some(&context), false);
    main_loop.run();
}
