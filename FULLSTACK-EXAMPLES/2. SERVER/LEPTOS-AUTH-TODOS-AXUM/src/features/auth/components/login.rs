use leptos::html::*;
use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::features::auth::auth_services::Login;

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

            <Button r#type="submit">
                "Log In"
            </Button>
        </ActionForm>
    }
}
