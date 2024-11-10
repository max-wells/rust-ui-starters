use leptos_query::*;
use std::time::Duration;

use crate::{
    api::api_books::{get_all_books, get_book_by_id},
    models::model_books::{
        AllBooksTag,
        MyBook,
        BookId,
        BookResponse,
    },
};

// TODO. Create a [#hook] for this

const STALE_TIME_SECS: u64 = 300;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ HELPERS ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// TODO. this should receive a book_id I guess ? 🤔
#[allow(non_snake_case)]
pub fn useBookById() -> QueryScope<BookId, BookResponse> {
    create_query(
        get_book_by_id,
        QueryOptions {
            stale_time: Some(Duration::from_secs(STALE_TIME_SECS)),
            ..Default::default()
        },
    )
}


#[allow(non_snake_case)]
pub fn useAllBooks() -> QueryScope<AllBooksTag, Vec<MyBook>> {
    create_query(
        |_| async move { get_all_books().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(STALE_TIME_SECS)),
            ..Default::default()
        },
    )
}
