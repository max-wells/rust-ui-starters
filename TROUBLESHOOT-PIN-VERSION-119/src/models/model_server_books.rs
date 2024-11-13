use leptos::ServerFnError;
use serde::*;
use std::fmt;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ServerBookId(pub u32);

// * 💁 Check delete_server_book_from_client, it's to avoid having id.0 since it's a tuple
impl fmt::Display for ServerBookId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerBook {
    pub id: ServerBookId,
    pub title: String,
    pub author: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewServerBook {
    pub title: String,
    pub author: String,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ ACTIONS  ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllServerBooksTag;

// Read.
pub type ServerBookResponse = Result<Option<ServerBook>, ServerFnError>;

//
//
/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FROM CLIENT ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllServerBooksTagFromClient;
