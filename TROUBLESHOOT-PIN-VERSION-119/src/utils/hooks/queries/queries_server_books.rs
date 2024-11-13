use leptos_query::*;
use std::time::Duration;

use crate::{
    api::api_server_books::{
        get_all_server_books,
        get_all_server_books_from_client,
        get_server_book,
    },
    models::model_server_books::{
        AllServerBooksTag,
        AllServerBooksTagFromClient,
        ServerBook,
        ServerBookId,
        ServerBookResponse,
    },
};

// TODO. Create a [#hook] for this

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ HELPERS ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[allow(non_snake_case)]
pub fn useServerBookQuery() -> QueryScope<ServerBookId, ServerBookResponse> {
    create_query(
        get_server_book,
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}

#[allow(non_snake_case)]
pub fn useAllServerBooksQuery() -> QueryScope<AllServerBooksTag, Vec<ServerBook>> {
    create_query(
        |_| async move { get_all_server_books().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}


#[allow(non_snake_case)]
pub fn useAllServerBooksQueryFromClient() -> QueryScope<AllServerBooksTagFromClient, Vec<ServerBook>> {
    create_query(
        |_| async move { get_all_server_books_from_client().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}
