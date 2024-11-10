use leptos_query::*;
use std::time::Duration;

use crate::{
    api::api_persons::{get_all_persons, get_person_by_id},
    models::model_persons::{
        AllPersonsTag,
        MyPerson,
        PersonId,
        PersonResponse,
    },
};

// TODO. Create a [#hook] for this

const STALE_TIME_SECS: u64 = 300;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ HELPERS ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// TODO. this should receive a person_id I guess ? 🤔
#[allow(non_snake_case)]
pub fn usePersonById() -> QueryScope<PersonId, PersonResponse> {
    create_query(
        get_person_by_id,
        QueryOptions {
            stale_time: Some(Duration::from_secs(STALE_TIME_SECS)),
            ..Default::default()
        },
    )
}


#[allow(non_snake_case)]
pub fn useAllPersons() -> QueryScope<AllPersonsTag, Vec<MyPerson>> {
    create_query(
        |_| async move { get_all_persons().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(STALE_TIME_SECS)),
            ..Default::default()
        },
    )
}
