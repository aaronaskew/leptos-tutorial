use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

/// App
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let double_count = move || count() * 2;

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }

            class=("red", move || count() % 2 == 1)
        >
            "Click me: "
            {count}
        </button>
        <br/>
        <progress
            max="50"
            // we use it once here
            value=count
        ></progress>
        <br/>
        <progress
            max="50"
            // we use it once here
            value=double_count
        ></progress>
        <p>"Count: " {count}</p>
        <p>"Double Count: " {double_count}</p>
    }
}
