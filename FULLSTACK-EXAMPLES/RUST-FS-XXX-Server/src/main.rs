#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::post, Router};
    use http::{HeaderMap, HeaderName, HeaderValue};
    use leptos::logging::log;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_default_headers::DefaultHeadersLayer;
    use sqlx::PgPool;
    use dotenv::dotenv;
    // Using my own crate:
    use start_axum::app::*;
    use start_axum::fileserv::file_and_error_handler;

    simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

    // Load .env file
    dotenv().ok();
    
    // Set up database connection
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(move || {
        provide_context(pool.clone());
        view! { <App /> }
    });

    let mut default_headers = HeaderMap::new();
    let color_header = HeaderValue::from_static("Sec-CH-Prefers-Color-Scheme");
    default_headers.insert(HeaderName::from_static("accept-ch"), color_header.clone());
    default_headers.insert(HeaderName::from_static("vary"), color_header.clone());
    default_headers.insert(HeaderName::from_static("critical-ch"), color_header);

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, || view! { <App /> })
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .layer(DefaultHeadersLayer::new(default_headers));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
