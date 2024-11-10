use leptos::ServerFnError;
use serde::*;
use std::fmt;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct BlogId(pub u32);

// * 💁 Check delete_server_blog_from_client, it's to avoid having id.0 since it's a tuple
impl fmt::Display for BlogId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyTag {
    pub id: BlogId,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewTag {
    pub name: String,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ ACTIONS  ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// Read.
pub type TagResponse = Result<Option<MyTag>, ServerFnError>;

//
//
/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                    ✨ LEPTOS QUERY ✨                      */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllTagsQKey;
