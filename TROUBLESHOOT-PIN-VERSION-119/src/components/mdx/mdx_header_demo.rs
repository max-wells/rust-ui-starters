use leptos::*;

use crate::registry::ui::badge::BadgeVariant;
use crate::registry::ui::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use crate::registry::ui::{badge::Badge, headings::H1};

#[component]
pub fn MdxHeaderDemo(
    name: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">

            <div>
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem>
                            <span>"Home"</span>
                        </BreadcrumbItem>

                        <BreadcrumbSeparator />

                        <BreadcrumbItem>
                            <span>"Components"</span>
                        </BreadcrumbItem>

                        <BreadcrumbSeparator />

                        <BreadcrumbItem>
                            <BreadcrumbPage>{name}</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>

                <H1 class="mt-4">{name}</H1>
                <p class="text-muted-foreground">{description}</p>
                <div class="flex gap-2 items-center mt-2 mb-8">
                    {tags
                        .iter()
                        .map(|tag| {
                            view! { <Badge variant=BadgeVariant::Secondary>{*tag}</Badge> }
                        })
                        .collect_view()}
                </div>
            </div>

        </div>
    }
}
