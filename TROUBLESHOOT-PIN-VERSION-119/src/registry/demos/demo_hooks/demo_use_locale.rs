use leptos::*;
use leptos_use::use_locale;
use unic_langid::langid_slice;

#[component]
pub fn DemoUseLocale() -> impl IntoView {
    let locale = use_locale(langid_slice!["en", "de", "fr"]);

    view! { <p>Locale: <code class="font-bold">{move || locale.get().to_string()}</code></p> }
}
