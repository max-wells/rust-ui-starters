use leptos::*;
use leptos_use::use_device_pixel_ratio;

#[component]
pub fn DemoUseDevicePixelRatio() -> impl IntoView {
    let pixel_ratio = use_device_pixel_ratio();

    view! {
        <pre>{move || format!("pixelRatio: {}", pixel_ratio())}</pre>
        <p>
            "Zoom in and out (or move the window to a screen with a different scaling factor) to see the value changes."
        </p>
    }
}
