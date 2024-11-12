use codee::string::FromToStringCodec;
use leptos::*;
use leptos_use::{use_cookie_with_options, UseCookieOptions};

/// Renders the home page of your application.
#[component]
pub fn DemoUseCookie() -> impl IntoView {
    let (test_cookie, _) = use_cookie_with_options::<String, FromToStringCodec>(
        "test-cookie",
        UseCookieOptions::<String, _, _>::default()
            .max_age(3000)
            .default_value(Some("Bogus string".to_owned())),
    );

    view! { <p>Test cookie: {move || test_cookie().unwrap_or("<Expired>".to_string())}</p> }
}
