use leptos::html::Div;
use leptos::*;
use leptos_use::{use_css_var_with_options, UseCssVarOptions};

#[component]
pub fn DemoUseCssVar() -> impl IntoView {
    let el = create_node_ref::<Div>();
    let (color, set_color) =
        use_css_var_with_options("--color", UseCssVarOptions::default().target(el));
    let switch_color = move |_| {
        if color.get() == "#df8543" {
            set_color.set("#7fa998".to_string());
        } else {
            set_color.set("#df8543".to_string());
        }
    };

    let elv = create_node_ref::<Div>();
    let (key, set_key) = create_signal("--color".to_string());
    let (color_val, _) = use_css_var_with_options(key, UseCssVarOptions::default().target(elv));
    let change_var = move |_| {
        if key.get() == "--color" {
            set_key.set("--color-one".to_string());
        } else {
            set_key.set("--color".to_string());
        }
    };
    let style = move || {
        format!(
            "--color: #7fa998; --color-one: #df8543; color: {}",
            color_val.get()
        )
    };

    view! {
        <div>
            <div node_ref=el style="--color: #7fa998; color: var(--color)">
                "Sample text, "
                {color}
            </div>
            <button on:click=switch_color>"Change color value"</button>
        </div>

        <div>
            <div node_ref=elv style=style class="mt-4">
                "Sample text, "
                {key}
                ": "
                {color_val}
            </div>
            <button on:click=change_var>"Change color variable"</button>
        </div>
    }
}
