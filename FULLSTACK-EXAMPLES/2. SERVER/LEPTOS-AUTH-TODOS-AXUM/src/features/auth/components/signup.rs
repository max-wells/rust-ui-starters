use leptos::html::*;
use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::features::auth::auth_services::Signup;

#[component]
pub fn Signup(action: ServerAction<Signup>) -> impl IntoView {
    view! {
        <ActionForm action=action>
            <h1>"Sign Up"</h1>
            <label>
                "User ID:" <Input r#type="text" placeholder="User ID" name="username" />/>
            </label>
            <br />
            <label>
                "Password:" <Input r#type="password" placeholder="Password" name="password" />
            </label>
            <br />
            <label>
                "Confirm Password:"
                <Input
                    r#type="password"
                    placeholder="Password again"
                    name="password_confirmation"
                />/>
            </label>
            <br />
            <label>"Remember me?" <Input r#type="checkbox" name="remember" /></label>
            <br />
            <Button r#type="submit">"Sign Up"</Button>
        </ActionForm>
    }
}
