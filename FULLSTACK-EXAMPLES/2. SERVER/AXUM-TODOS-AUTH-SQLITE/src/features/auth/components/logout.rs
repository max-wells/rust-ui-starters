use leptos::html::*;
use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::features::auth::auth_services::Logout;

#[component]
pub fn Logout(action: ServerAction<Logout>) -> impl IntoView {
    view! {
        <div id="loginbox">
            <ActionForm action=action>
                <Button r#type="submit">
                    "Log Out"
                </Button>
            </ActionForm>
        </div>
    }
}
