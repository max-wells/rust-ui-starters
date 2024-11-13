use leptos::*;

use crate::registry::ui::{
    bento_grid::{BentoCell, BentoGrid, BentoRow},
    headings::H4,
};

#[component]
pub fn DemoBentoGrid5() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 py-4 w-full max-w-[800px]">
            <Variant1 />
            <Variant2 />
            <Variant3 />
            <Variant4 />
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn Variant1() -> impl IntoView {
    view! {
        <div>
            <H4>Variant 1</H4>

            <BentoGrid>
                <BentoRow class="md:col-start-1">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-start-4">
                    <BentoCell>3</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2">
                    <BentoCell>4</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2">
                    <BentoCell>5</BentoCell>
                </BentoRow>
            </BentoGrid>
        </div>
    }
}

#[component]
pub fn Variant2() -> impl IntoView {
    view! {
        <div>
            <H4>Variant 2</H4>

            <BentoGrid>
                <BentoRow class="md:col-span-2">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-4">
                    <BentoCell>3</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2">
                    <BentoCell>4</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2">
                    <BentoCell>5</BentoCell>
                </BentoRow>
            </BentoGrid>
        </div>
    }
}

#[component]
pub fn Variant3() -> impl IntoView {
    view! {
        <div>
            <H4>Variant 3</H4>

            <div class="grid gap-2 sm:grid-cols-2 md:grid-cols-9">
                <BentoRow class="md:col-span-5">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-4">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-3">
                    <BentoCell>3</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-3">
                    <BentoCell>4</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-3">
                    <BentoCell>5</BentoCell>
                </BentoRow>
            </div>

        </div>
    }
}

#[component]
pub fn Variant4() -> impl IntoView {
    view! {
        <div>
            <H4>Variant 4</H4>

            <BentoGrid>
                <BentoRow class="">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2 md:row-span-2 md:h-full">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>3</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>4</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-start-4">
                    <BentoCell>5</BentoCell>
                </BentoRow>
            </BentoGrid>

        </div>
    }
}
