use leptos::html::*;
use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::features::auth::auth_services::Signup;

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
            <Button r#type="submit">
                "Sign Up"
            </Button>
        </ActionForm>
    }
}
