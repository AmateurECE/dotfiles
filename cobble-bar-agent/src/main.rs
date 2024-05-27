use std::{
    ffi::{c_double, CStr, CString},
    mem, ptr,
};

use ffi::{
    wp_core_connect, wp_core_install_object_manager, wp_core_load_component,
    wp_core_load_component_finish, wp_core_new, wp_init, wp_node_get_type, wp_object_activate,
    wp_object_activate_finish, wp_object_manager_add_interest, wp_object_manager_new,
    wp_plugin_find, wp_plugin_get_name, WpCore, WpObject, WpObjectManager, WpPlugin,
    WP_CONSTRAINT_TYPE_PW_PROPERTY, WP_INIT_PIPEWIRE, WP_PLUGIN_FEATURE_ENABLED,
};
use futures::try_join;
use gio_sys::GAsyncResult;
use glib::{
    clone,
    ffi::{g_clear_pointer, g_variant_lookup, g_variant_unref, GVariant},
    gobject_ffi::{g_signal_connect_data, g_signal_emit_by_name, G_CONNECT_SWAPPED},
};
use gobject_sys::GObject;
use libc::c_void;
use sink::PrintSink;
use widgets::{clock::Clock, Widget};

mod sink;
mod widgets;

#[derive(Clone, Copy, Debug)]
struct PluginContext {
    core: *mut WpCore,
    object_manager: *mut WpObjectManager,
    pending_plugins: usize,
}

unsafe extern "C" fn print_volume(context: *mut PluginContext) {
    let PluginContext { core, .. } = *context;
    let mixer_api = CString::new("mixer-api").unwrap();
    let mixer_api = wp_plugin_find(core, mixer_api.as_ptr() as *mut _);

    let default_nodes_api = CString::new("default-nodes-api").unwrap();
    let default_nodes_api = wp_plugin_find(core, default_nodes_api.as_ptr() as *mut _);

    let mut node_id: u32 = 0;
    g_signal_emit_by_name(
        default_nodes_api as *mut _,
        b"get-default-node\0".as_ptr(),
        b"Audio/Sink\0".as_ptr(),
        &mut node_id as *mut u32,
    );

    // TODO: Check if node is valid

    let mut variant: *mut GVariant = ptr::null_mut();
    let signal = CString::new("get-volume").unwrap();
    g_signal_emit_by_name(
        mixer_api as *mut _,
        signal.as_ptr() as *mut _,
        node_id,
        &mut variant as *mut *mut _,
    );

    let double_type = CString::new("d").unwrap();
    let property = CString::new("volume").unwrap();
    let mut volume: c_double = 0.0;
    g_variant_lookup(
        variant,
        property.as_ptr() as *const _,
        double_type.as_ptr() as *const _,
        &mut volume as *mut c_double,
    );
    // TODO: let min_step: c_double = 0.0;
    // TODO: let muted = false;

    let callback: unsafe extern "C" fn(*mut c_void) =
        mem::transmute(g_variant_unref as *const c_void);
    g_clear_pointer(&mut variant as *mut *mut _ as *mut *mut _, Some(callback));
    println!("Volume: {}", volume);
}

unsafe extern "C" fn on_object_manager_installed(context: *mut PluginContext) {
    print_volume(context);

    let PluginContext { core, .. } = *context;
    let mixer_api = CString::new("mixer-api").unwrap();
    let mixer_api = wp_plugin_find(core, mixer_api.as_ptr() as *mut _);

    let callback: unsafe extern "C" fn() = mem::transmute(print_volume as *const c_void);
    let signal = CString::new("changed").unwrap();
    g_signal_connect_data(
        mixer_api as *mut _,
        signal.as_ptr() as *const _,
        Some(callback),
        context as *mut c_void,
        None,
        G_CONNECT_SWAPPED,
    );
}

unsafe extern "C" fn on_plugin_activated(
    plugin: *mut GObject,
    result: *mut GAsyncResult,
    data: *mut c_void,
) {
    let name = wp_plugin_get_name(plugin as *mut WpPlugin);
    let name = CStr::from_ptr(name).to_str().unwrap();
    let mut error = ptr::null_mut();
    let success = wp_object_activate_finish(plugin as *mut WpObject, result, &mut error);
    if success == 0 {
        if !error.is_null() {
            let message = CString::from_raw((*error).message).into_string().unwrap();
            panic!("Error activating plugin {}: {}", name, message);
        } else {
            panic!("Error activating plugin {}!", name);
        }
    }

    let PluginContext {
        ref mut pending_plugins,
        core,
        object_manager,
    } = *(data as *mut PluginContext);
    *pending_plugins -= 1;
    println!("Pending Plugins: {}", *pending_plugins);
    if *pending_plugins == 0 {
        wp_core_install_object_manager(core, object_manager);
    }
}

unsafe extern "C" fn on_mixer_api_loaded(
    _object: *mut GObject,
    result: *mut GAsyncResult,
    data: *mut c_void,
) {
    let context = data as *mut PluginContext;
    let mut error = ptr::null_mut();
    let PluginContext { core, .. } = *context;
    let success = wp_core_load_component_finish(core, result, &mut error);
    if success == 0 {
        panic!("Couldn't load mixer api!");
    }

    let default_nodes_api = CString::new("default-nodes-api").unwrap();
    let default_nodes_api = wp_plugin_find(core, default_nodes_api.as_ptr() as *mut _);
    wp_object_activate(
        default_nodes_api as *mut WpObject,
        WP_PLUGIN_FEATURE_ENABLED,
        ptr::null_mut(),
        Some(on_plugin_activated),
        data,
    );

    let mixer_api = CString::new("mixer-api").unwrap();
    let mixer_api = wp_plugin_find(core, mixer_api.as_ptr() as *mut _);
    wp_object_activate(
        mixer_api as *mut WpObject,
        WP_PLUGIN_FEATURE_ENABLED,
        ptr::null_mut(),
        Some(on_plugin_activated),
        data,
    );
}

unsafe extern "C" fn on_default_nodes_api_loaded(
    _object: *mut GObject,
    result: *mut GAsyncResult,
    data: *mut c_void,
) {
    let context = data as *mut PluginContext;
    let PluginContext { core, .. } = *context;
    let mut error = ptr::null_mut();
    let success = wp_core_load_component_finish(core, result, &mut error);
    if success == 0 {
        panic!("Couldn't load default nodes api!");
    }

    let component = CString::new("libwireplumber-module-mixer-api").unwrap();
    let component_type = CString::new("module").unwrap();
    let provides = CString::new("mixer-api").unwrap();
    wp_core_load_component(
        core,
        component.as_ptr() as *mut _,
        component_type.as_ptr() as *mut _,
        ptr::null_mut(),
        provides.as_ptr() as *mut _,
        ptr::null_mut(),
        Some(on_mixer_api_loaded),
        data,
    );
}

unsafe fn make_plugin_context() -> PluginContext {
    let core = wp_core_new(ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    let object_manager = wp_object_manager_new();
    PluginContext {
        core,
        object_manager,
        pending_plugins: 2,
    }
}

unsafe fn get_volume(context: &mut PluginContext) {
    let PluginContext {
        core,
        object_manager,
        ..
    } = *context;
    let class = CString::new("media.class").unwrap();
    let second = CString::new("=s").unwrap();
    let third = CString::new("Audio/Sink").unwrap();
    wp_object_manager_add_interest(
        object_manager,
        wp_node_get_type(),
        WP_CONSTRAINT_TYPE_PW_PROPERTY,
        class.as_ptr() as *const u8,
        second.as_ptr() as *const u8,
        third.as_ptr() as *const u8,
        ptr::null_mut() as *mut c_void,
    );

    if wp_core_connect(core) == 0 {
        panic!("Couldn't connect to PipeWire!");
    }

    let signal = CString::new("installed").unwrap();
    let callback: unsafe extern "C" fn() =
        mem::transmute::<_, _>(on_object_manager_installed as *const c_void);
    g_signal_connect_data(
        object_manager as *mut _,
        signal.as_ptr() as *const _,
        Some(callback),
        context as *mut _ as *mut _,
        None,
        G_CONNECT_SWAPPED,
    );

    let component = CString::new("libwireplumber-module-default-nodes-api").unwrap();
    let component_type = CString::new("module").unwrap();
    let provides = CString::new("default-nodes-api").unwrap();
    wp_core_load_component(
        core,
        component.as_ptr() as *mut _,
        component_type.as_ptr() as *mut _,
        ptr::null_mut(),
        provides.as_ptr() as *mut _,
        ptr::null_mut(),
        Some(on_default_nodes_api_loaded),
        context as *mut _ as *mut _,
    )
}

fn main() -> Result<(), anyhow::Error> {
    // TODO: Lazy once?
    unsafe { wp_init(WP_INIT_PIPEWIRE) };
    let mut context = unsafe { make_plugin_context() };

    let main_loop = glib::MainLoop::new(None, false);
    unsafe { get_volume(&mut context) };

    let context = glib::MainContext::default();
    context.spawn_local(clone!(@strong main_loop => async move {
        let mut clock = Clock;
        try_join!(
            clock.run(PrintSink),
        ).unwrap();
    }));

    main_loop.run();
    Ok(())
}
