use leptos::ServerFnError;
use serde::*;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct UserIdJson(pub u32);

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserJson {
    pub id: UserIdJson,
    pub userId: UserIdJson,
    pub title: String,
    pub body: String,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ ACTIONS  ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllUsersTagJson;

// Read.
pub type UserResponseJson = Result<Option<UserJson>, ServerFnError>;
