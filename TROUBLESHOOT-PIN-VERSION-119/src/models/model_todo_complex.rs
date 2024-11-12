use leptos::ServerFnError;
use serde::*;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TodoIdComplex(pub u32);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoComplex {
    pub id: TodoIdComplex,
    pub content: String,
    pub description: String,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ ACTIONS  ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllTodosTagComplex;

// Read.
pub type TodoResponseComplex = Result<Option<TodoComplex>, ServerFnError>;
