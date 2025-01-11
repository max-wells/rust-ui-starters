use leptos::ServerFnError;
use serde::*;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct UserIdUrl(pub u32);

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyTodo {
    pub id: UserIdUrl,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ ACTIONS  ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllTodosTagUrl;

// Read.
pub type TodoResponseUrl = Result<Option<MyTodo>, ServerFnError>;

//
//
/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FROM CLIENT ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllTodosTagUrlFromClient;
