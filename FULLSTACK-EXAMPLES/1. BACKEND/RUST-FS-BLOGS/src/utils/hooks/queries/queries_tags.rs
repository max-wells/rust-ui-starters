use leptos_query::*;
use std::time::Duration;

use crate::{
    api::api_tags::get_all_tags,
    models::model_tags::{
        AllTagsQKey,
        MyTag,
  
    },
};

// TODO. Create a [#hook] for this

const STALE_TIME_SECS: u64 = 300;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ HELPERS ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/



#[allow(non_snake_case)]
pub fn useAllTags() -> QueryScope<AllTagsQKey, Vec<MyTag>> {
    create_query(
        |_| async move { get_all_tags().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(STALE_TIME_SECS)),
            ..Default::default()
        },
    )
}
