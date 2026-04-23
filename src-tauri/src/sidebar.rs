#[cfg(not(dev))]
use tauri::Manager;
#[cfg(not(dev))]
use tauri_plugin_shell::process::CommandChild;

#[cfg(not(dev))]
struct SidecarProcess(std::sync::Mutex<Option<CommandChild>>);

#[allow(unused_variables)]
pub(crate) fn setup(app_handle: tauri::AppHandle) {
    #[cfg(not(dev))]
    {
        use tauri_plugin_shell::ShellExt;
        let next = app_handle
            .shell()
            .sidecar("next")
            .unwrap()
            .env("PORT", "1420");
        let (rx, child) = next.spawn().expect("Failed to spawn next sidecar");
        app_handle.manage(SidecarProcess(std::sync::Mutex::new(Some(child))));
        return;
    }
    #[cfg(debug_assertions)]
    println!("Running in dev mode, skipping sidecar...");
}

#[allow(unused_variables)]
pub(crate) fn cleanup(window: &tauri::Window) {
    #[cfg(not(dev))]
    {
        let state = window.state::<SidecarProcess>();
        if let Some(child) = state.0.lock().unwrap().take() {
            child.kill().unwrap();
        };
    }
}
