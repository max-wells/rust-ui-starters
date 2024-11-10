use leptos_query::*;
use std::time::Duration;

use crate::{
    api::api_blogs::{get_all_blogs, get_blog_by_id},
    models::model_blogs::{
        AllBlogsQKey,
        MyBlog,
        BlogId,
        BlogResponse,
    },
};

// TODO. Create a [#hook] for this

const STALE_TIME_SECS: u64 = 300;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ HELPERS ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// TODO. this should receive a blog_id I guess ? 🤔
#[allow(non_snake_case)]
pub fn useBlogById() -> QueryScope<BlogId, BlogResponse> {
    create_query(
        get_blog_by_id,
        QueryOptions {
            stale_time: Some(Duration::from_secs(STALE_TIME_SECS)),
            ..Default::default()
        },
    )
}


#[allow(non_snake_case)]
pub fn useAllBlogs() -> QueryScope<AllBlogsQKey, Vec<MyBlog>> {
    create_query(
        |_| async move { get_all_blogs().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(STALE_TIME_SECS)),
            ..Default::default()
        },
    )
}
