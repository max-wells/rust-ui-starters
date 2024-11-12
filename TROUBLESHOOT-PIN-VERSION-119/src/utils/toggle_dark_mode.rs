use leptos::*;


#[server]
pub async fn toggle_dark_mode(enabled: bool) -> Result<(), ServerFnError> {
    use axum::{http::header, http::header::HeaderValue};
use axum_extra::extract::cookie::{Cookie, SameSite};
use leptos_axum::ResponseOptions;
use time::{Duration, OffsetDateTime};

    let cookie = if enabled {
        Cookie::build(("dark", "true"))
            .max_age(Duration::hours(500 * 365 * 24))
            .expires(OffsetDateTime::now_utc() + Duration::hours(500 * 365 * 24))
            .http_only(true)
            .same_site(SameSite::Lax)
            .path("/")
            .build()
    } else {
        Cookie::build(("dark", ""))
            .max_age(Duration::seconds(0))
            .expires(OffsetDateTime::now_utc() - Duration::seconds(1))
            .http_only(true)
            .same_site(SameSite::Lax)
            .path("/")
            .build()
    };

    if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
        expect_context::<ResponseOptions>().insert_header(header::SET_COOKIE, cookie);
    }

    Ok(())
}
