use leptos::*;

use crate::registry::ui::{
    bento_grid::{BentoCell, BentoGrid, BentoRow},
    headings::H4,
};

#[component]
pub fn DemoBentoGrid4() -> impl IntoView {
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
                <BentoRow class="md:col-span-3">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-start-1">
                    <BentoCell>3</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-4 md:col-start-2">
                    <BentoCell>4</BentoCell>
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
                <BentoRow class="md:col-span-3">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2">
                    <BentoCell>3</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2">
                    <BentoCell>4</BentoCell>
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
                <BentoRow class="md:col-span-4">
                    <BentoCell>4</BentoCell>
                </BentoRow>
            </BentoGrid>

        </div>
    }
}

#[component]
pub fn Variant4() -> impl IntoView {
    view! {
        <div>
            <H4>Variant 4</H4>

            <BentoGrid>
                <BentoRow class="md:col-span-3 md:row-span-4 md:h-full">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-start-4">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-start-4">
                    <BentoCell>3</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-start-4">
                    <BentoCell>4</BentoCell>
                </BentoRow>
            </BentoGrid>

        </div>
    }
}
