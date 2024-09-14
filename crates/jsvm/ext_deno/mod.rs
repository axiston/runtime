//! `deno_core::`[`extension`]s bundled with `Deno`.
//!

mod ext_canvas;
mod ext_console;
mod ext_crypto;
mod ext_fetch;
mod ext_websocket;
mod ext_fs;
mod ext_url;
mod ext_webgpu;
mod ext_net;
mod ext_io;
mod ext_web;
mod ext_webidl;
mod permission;


// extension!(
//     axiston_permission,
//     options = { allow_net_access: bool, filter_net_access: Vec<NetDescriptor> },
//     state = |state, options| {
//         state.put::<MyPermission>(MyPermission::new(
//             options.allow_net_access, options.filter_net_access
//         ) );
//     }
// );
//
// extension!(
//     axiston_init_fetch,
//     deps = [rustyscript],
//     esm_entry_point = "ext:ext_boot/init_fetch.js",
//     esm = [ dir "ext_boot", "init_fetch.js" ],
// );
//
// extension!(
//     axiston_init_net,
//     deps = [rustyscript],
//     esm_entry_point = "ext:ext_boot/init_net.js",
//     esm = [ dir "ext_boot", "init_net.js" ],
// );
//
// extension!(
//     axiston_init_web,
//     deps = [rustyscript],
//     esm_entry_point = "ext:ext_boot/init_web.js",
//     esm = [ dir "ext_boot", "init_web.js" ],
// );
