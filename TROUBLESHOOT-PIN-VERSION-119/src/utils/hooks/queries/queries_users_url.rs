use leptos_query::*;
use std::time::Duration;

use crate::{
    api::api_users_url::{get_all_users_url, get_user_url, get_all_users_url_from_client},
    models::model_users_url::{
        AllUsersTagUrl, AllUsersTagUrlFromClient, UserIdUrl, UserResponseUrl, UserUrl,
    },
};

// TODO. Create a [#hook] for this

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ HELPERS ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[allow(non_snake_case)]
pub fn useUsersQueryUrl() -> QueryScope<UserIdUrl, UserResponseUrl> {
    create_query(
        get_user_url,
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}

#[allow(non_snake_case)]
pub fn useAllUsersQueryUrl() -> QueryScope<AllUsersTagUrl, Vec<UserUrl>> {
    create_query(
        |_| async move { get_all_users_url().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}


#[allow(non_snake_case)]
pub fn useAllUsersQueryUrlFromClient() -> QueryScope<AllUsersTagUrlFromClient, Vec<UserUrl>> {
    create_query(
        |_| async move { get_all_users_url_from_client().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}
