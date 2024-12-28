use leptos_query::*;
use std::time::Duration;

use crate::{
    api::api_xxxs::{get_all_xxxs, get_xxx_by_id},
    models::model_xxxs::{
        TagAllXxxs,
        MyXxx,
        XxxId,
        ResponseXxx,
    },
};

// TODO. Create a [#hook] for this

const STALE_TIME_SECS: u64 = 300;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ HELPERS ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// TODO. this should receive a xxx_id I guess ? 🤔
#[allow(non_snake_case)]
pub fn useXxxById() -> QueryScope<XxxId, ResponseXxx> {
    create_query(
        get_xxx_by_id,
        QueryOptions {
            stale_time: Some(Duration::from_secs(STALE_TIME_SECS)),
            ..Default::default()
        },
    )
}


#[allow(non_snake_case)]
pub fn useAllXxxs() -> QueryScope<TagAllXxxs, Vec<MyXxx>> {
    create_query(
        |_| async move { get_all_xxxs().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(STALE_TIME_SECS)),
            ..Default::default()
        },
    )
}
