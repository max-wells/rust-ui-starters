use leptos::ServerFnError;
use serde::*;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TodoIdJson(pub u32);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoJson {
    pub id: TodoIdJson,
    pub content: String,
    pub description: String,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ ACTIONS  ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllTodosTagJson;

// Read.
pub type TodoResponseJson = Result<Option<TodoJson>, ServerFnError>;
