use leptos::*;

use crate::components::mdx::mdx_skeleton_demo::MdxSkeletonDemo;
use crate::{components::mdx::my_mdx::MyMdx, utils::mdx::api_read_mdx::read_mdx_file};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      🦀 ENTRYPOINT 🦀                      */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// TODO 🐛 2024-10-08: Noticed a Borrowing error when refreshing the page :/ (but no problem in production !)

#[component]
pub fn SharedDemoMdx(mdx_path: &'static str) -> impl IntoView {
    let mdx_content = create_resource(
        || (),
        move |_| async move {
            read_mdx_file(mdx_path.to_string())
                .await
                .unwrap_or_else(|e| e.to_string())
        },
    );

    view! {
        <Suspense fallback=move || {
            view! { <MdxSkeletonDemo /> }
        }>
            {move || {
                mdx_content
                    .get()
                    .map(|content| {
                        view! { <MyMdx source=content /> }
                    })
            }}
        </Suspense>
    }
}


