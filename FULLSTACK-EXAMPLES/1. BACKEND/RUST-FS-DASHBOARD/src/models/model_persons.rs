use chrono::NaiveDate;
use leptos::ServerFnError;
use serde::*;
use std::fmt;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PersonId(pub u32);

// * 💁 Check delete_server_person_from_client, it's to avoid having id.0 since it's a tuple
impl fmt::Display for PersonId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyPerson {
    pub id: PersonId,
    pub name: String,
    pub title: String,
    pub level: String,
    pub compensation: i32,
    pub joined_date: NaiveDate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewPerson {
    pub name: String,
    pub title: String,
    pub level: String,
    pub compensation: i32,
    pub joined_date: NaiveDate,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ ACTIONS  ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// Read.
pub type PersonResponse = Result<Option<MyPerson>, ServerFnError>;

//
//
/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                    ✨ LEPTOS QUERY ✨                      */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllPersonsTag;
