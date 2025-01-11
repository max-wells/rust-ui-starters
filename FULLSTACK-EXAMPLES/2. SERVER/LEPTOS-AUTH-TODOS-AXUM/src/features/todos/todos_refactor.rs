use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};

use crate::{auth::*, features::todos::todos_component::TodosComponent};

#[cfg(feature = "ssr")]
pub mod ssr {
    use crate::{
        auth::{ssr::AuthSession, User},
        features::todos::todos_models::Todo,
    };
    use leptos::prelude::*;
    use sqlx::SqlitePool;

    pub fn pool() -> Result<SqlitePool, ServerFnError> {
        use_context::<SqlitePool>()
            .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
    }

    pub fn auth() -> Result<AuthSession, ServerFnError> {
        use_context::<AuthSession>()
            .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlTodo {
        id: u32,
        user_id: i64,
        title: String,
        created_at: String,
        completed: bool,
    }

    impl SqlTodo {
        pub async fn into_todo(self, pool: &SqlitePool) -> Todo {
            Todo {
                id: self.id,
                user: User::get(self.user_id, pool).await,
                title: self.title,
                created_at: self.created_at,
                completed: self.completed,
            }
        }
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

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
                <TodoApp/>
            </body>
        </html>
    }
}

#[component]
pub fn TodoApp() -> impl IntoView {
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
                    // Route
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

#[component]
pub fn Login(action: ServerAction<Login>) -> impl IntoView {
    view! {
        <ActionForm action=action>
            <h1>"Log In"</h1>
            <label>
                "User ID:"
                <input
                    type="text"
                    placeholder="User ID"
                    maxlength="32"
                    name="username"
                    class="auth-input"
                />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" placeholder="Password" name="password" class="auth-input"/>
            </label>
            <br/>
            <label>
                <input type="checkbox" name="remember" class="auth-input"/>
                "Remember me?"
            </label>
            <br/>
            <button type="submit" class="button">
                "Log In"
            </button>
        </ActionForm>
    }
}

#[component]
pub fn Signup(action: ServerAction<Signup>) -> impl IntoView {
    view! {
        <ActionForm action=action>
            <h1>"Sign Up"</h1>
            <label>
                "User ID:"
                <input
                    type="text"
                    placeholder="User ID"
                    maxlength="32"
                    name="username"
                    class="auth-input"
                />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" placeholder="Password" name="password" class="auth-input"/>
            </label>
            <br/>
            <label>
                "Confirm Password:"
                <input
                    type="password"
                    placeholder="Password again"
                    name="password_confirmation"
                    class="auth-input"
                />
            </label>
            <br/>
            <label>
                "Remember me?" <input type="checkbox" name="remember" class="auth-input"/>
            </label>

            <br/>
            <button type="submit" class="button">
                "Sign Up"
            </button>
        </ActionForm>
    }
}

#[component]
pub fn Logout(action: ServerAction<Logout>) -> impl IntoView {
    view! {
        <div id="loginbox">
            <ActionForm action=action>
                <button type="submit" class="button">
                    "Log Out"
                </button>
            </ActionForm>
        </div>
    }
}
