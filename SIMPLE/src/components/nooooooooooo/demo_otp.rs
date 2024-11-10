use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoOtp() -> impl IntoView {
    view! {
        <Stylesheet id="otp" href="/components-nooo/otp.css" />

        <div class="mainDiv">
            <input type="text" maxlength="6" />
        </div>
    }
}
