use leptos::html::*;
use leptos::prelude::*;

use crate::features::auth::auth_services::Logout;

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
