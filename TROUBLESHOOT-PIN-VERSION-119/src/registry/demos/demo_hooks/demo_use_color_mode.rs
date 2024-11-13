use leptos::html::html;
use leptos::*;
use leptos_use::docs::Note;
use leptos_use::{
    use_color_mode_with_options, use_cycle_list_with_options, ColorMode, UseColorModeOptions,
    UseColorModeReturn, UseCycleListOptions, UseCycleListReturn,
};

#[component]
pub fn DemoUseColorMode() -> impl IntoView {
    let UseColorModeReturn {
        mode,
        set_mode,
        ..
    } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .custom_modes(vec![
                "rust".into(),
                "coal".into(),
                "navy".into(),
                "ayu".into(),
            ])
            .initial_value(ColorMode::from(html().class_name())),
    );

    let UseCycleListReturn {
        state,
        next,
        ..
    } = use_cycle_list_with_options(
        vec![
            ColorMode::Light,
            ColorMode::Custom("rust".into()),
            ColorMode::Custom("coal".into()),
            ColorMode::Custom("navy".into()),
            ColorMode::Custom("ayu".into()),
        ],
        UseCycleListOptions::default().initial_value(Some((mode, set_mode).into())),
    );

    view! {
        <button on:click=move |_| next()>{move || format!("{}", state.get())}</button>
        <Note>"Click to change the color mode"</Note>
    }
}
