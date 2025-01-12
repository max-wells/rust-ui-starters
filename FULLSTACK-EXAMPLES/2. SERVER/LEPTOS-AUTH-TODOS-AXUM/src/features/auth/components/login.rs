use leptos::html::*;
use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::features::auth::auth_services::Login;

#[component]
pub fn Login(action: ServerAction<Login>) -> impl IntoView {
    view! {
        <ActionForm action=action>
            <h1>"Log In"</h1>
            <label>"User ID:" <Input r#type="text" name="username" /></label>
            <br />
            <label>
                "Password:" <Input r#type="password" placeholder="Password" name="password" />
            </label>
            <br />
            <label>
                <Input r#type="checkbox" name="remember" />
                "Remember me?"
            </label>
            <br />

            <Button r#type="submit">"Log In"</Button>
        </ActionForm>
    }
}
