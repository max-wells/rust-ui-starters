use leptos::ServerFnError;
use serde::*;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TodoId(pub u32);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: TodoId,
    pub content: String,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ ACTIONS  ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllTodosTag;

// Read.
pub type TodoResponse = Result<Option<Todo>, ServerFnError>;
