use dioxus::prelude::*;

fn main() {
    launch_desktop(app);
}

fn app() -> Element {
    let mut level = use_signal(|| 1.0);

    rsx! {
        input {
            r#type: "number",
            value: "{level}",
            oninput: move |e| {
                if let Ok(new_zoom) = e.value().parse::<f64>() {
                    level.set(new_zoom);
                    dioxus::desktop::window().webview.zoom(new_zoom);
                }
            }
        }
    }
}
