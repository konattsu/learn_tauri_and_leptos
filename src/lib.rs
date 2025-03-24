mod app;
mod components;

pub fn run() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App)
}
