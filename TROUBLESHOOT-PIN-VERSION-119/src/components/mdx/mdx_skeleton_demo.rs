use leptos::*;

use crate::registry::ui::skeleton::Skeleton;

#[component]
pub fn MdxSkeletonDemo() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-14">
            <div class="flex flex-col gap-2">
                <Skeleton class="h-10 w-[180px]" />
                <Skeleton class="w-full rounded-xl h-[350px]" />
            </div>
            <div class="flex flex-col gap-2">
                <Skeleton class="h-10 w-[180px]" />
                <Skeleton class="w-full rounded-xl h-[350px]" />
            </div>
        </div>
    }
}
