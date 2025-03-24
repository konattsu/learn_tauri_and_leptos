#[derive(serde::Serialize)]
struct GreetArgs {
    name: String,
}

#[leptos::component]
pub fn greet() -> impl leptos::IntoView {
    use leptos::prelude::*;

    let name = RwSignal::new("".to_string());
    let greet_msg = RwSignal::new("".to_string());

    let on_click = move |_| {
        leptos::task::spawn_local(async move {
            let name = name.get();
            leptos::logging::log!("name: {}", name);
            let args = GreetArgs { name };
            let msg: String = tauri_sys::core::invoke("greet", args).await;
            greet_msg.set(msg);
        });
    };

    view! {
        <div>
            <Input placeholder="Enter a name" value=name />
            <Button appearance=ButtonAppearance::Primary on_click>
                "greet"
            </Button>
        </div>
    }
}
