use std::rc::Rc;

use dioxus::prelude::*;

fn main() {
    launch_desktop(app);
}

fn app() -> Element {
    let mut elements = use_signal(Vec::<Rc<MountedData>>::new);
    let mut running = use_signal(|| true);

    use_future(move || async move {
        let mut focused = 0;

        loop {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;

            if !running() {
                continue;
            }

            if let Some(element) = elements.with(|f| f.get(focused).cloned()) {
                _ = element.set_focus(true).await;
            } else {
                focused = 0;
            }

            focused += 1;
        }
    });

    rsx! {
        div {
            h1 { "Input Roulette" }
            for i in 0..100 {
                input {
                    value: "{i}",
                    onmounted: move |cx| {
                        elements.write().push(cx.data());
                    },
                    oninput: move |_| {
                        running.set(false);
                    }
                }
            }
        }
    }
}
