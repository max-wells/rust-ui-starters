use leptos::*;

use crate::registry::ui::status::{Status, StatusIndactorVariant};

#[component]
pub fn DemoStatusVariants() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            <Status variant=StatusIndactorVariant::Normal>
                <DemoContainer />
            </Status>
            <Status variant=StatusIndactorVariant::Active>
                <DemoContainer />
            </Status>
            <Status variant=StatusIndactorVariant::Inactive>
                <DemoContainer />
            </Status>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
fn DemoContainer() -> impl IntoView {
    view! { <div class="rounded-md size-16 bg-neutral-500" /> }
}
