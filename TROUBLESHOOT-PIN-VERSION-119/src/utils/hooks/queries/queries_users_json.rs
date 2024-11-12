use leptos_query::*;
use std::time::Duration;

use crate::{
    api::api_users_json::{get_all_users_json, get_user_json},
    models::model_users_json::{AllUsersTagJson, UserIdJson, UserResponseJson, UserJson},
};

// TODO. Create a [#hook] for this

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ HELPERS ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[allow(non_snake_case)]
pub fn useUsersQueryJson() -> QueryScope<UserIdJson, UserResponseJson> {
    create_query(
        get_user_json,
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}

#[allow(non_snake_case)]
pub fn useAllUsersQueryJson() -> QueryScope<AllUsersTagJson, Vec<UserJson>> {
    create_query(
        |_| async move { get_all_users_json().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}
