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
pub struct MyBlog {
    pub id: BlogId,
    pub title: String,
    pub content: String,
    pub author: String,
    pub image_url: String,
    pub tags: Vec<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewBlog {
    pub title: String,
    pub content: String,
    pub author: String,
    pub image_url: String,
    pub tags: Vec<u32>,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ ACTIONS  ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// Read.
pub type BlogResponse = Result<Option<MyBlog>, ServerFnError>;

//
//
/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                    ✨ LEPTOS QUERY ✨                      */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllBlogsQKey;
