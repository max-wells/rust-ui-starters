use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};

use crate::features::auth::auth_services::get_user;
use crate::features::auth::auth_services::{Login, Logout, Signup};
use crate::features::auth::components::{login::Login, logout::Logout, signup::Signup};
use crate::features::todos::todos_component::TodosComponent;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/session_auth_axum.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let login = ServerAction::<Login>::new();
    let logout = ServerAction::<Logout>::new();
    let signup = ServerAction::<Signup>::new();

    let user = Resource::new(
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_user(),
    );
    provide_meta_context();

    view! {
        <Router>
            <header>
                <A href="/">
                    <h1>"My Tasks"</h1>
                </A>
                <Transition fallback=move || {
                    view! { <span>"Loading..."</span> }
                }>
                    {move || {
                        user.get()
                            .map(|user| match user {
                                Err(e) => {
                                    view! {
                                        <A href="/signup">"Signup"</A>
                                        ", "
                                        <A href="/login">"Login"</A>
                                        ", "
                                        <span>{format!("Login error: {}", e)}</span>
                                    }
                                        .into_any()
                                }
                                Ok(None) => {
                                    view! {
                                        <A href="/signup">"Signup"</A>
                                        ", "
                                        <A href="/login">"Login"</A>
                                        ", "
                                        <span>"Logged out."</span>
                                    }
                                        .into_any()
                                }
                                Ok(Some(user)) => {
                                    view! {
                                        <A href="/settings">"Settings"</A>
                                        ", "
                                        <span>
                                            {format!("Logged in as: {} ({})", user.username, user.id)}
                                        </span>
                                    }
                                        .into_any()
                                }
                            })
                    }}
                </Transition>
            </header>
            <hr/>
            <main>
                <FlatRoutes fallback=|| "Not found.">
                    <Route path=path!("") view=TodosComponent/>
                    <Route path=path!("signup") view=move || view! { <Signup action=signup/> }/>
                    <Route path=path!("login") view=move || view! { <Login action=login/> }/>
                    <ProtectedRoute
                        path=path!("settings")
                        condition=move || user.get().map(|r| r.ok().flatten().is_some())
                        redirect_path=|| "/"
                        view=move || {
                            view! {
                                <h1>"Settings"</h1>
                                <Logout action=logout/>
                            }
                        }
                    />
                </FlatRoutes>
            </main>
        </Router>
    }
}
