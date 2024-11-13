use leptos::*;

use crate::registry::ui::{
    bento_grid::{BentoCell, BentoGrid6, BentoRow},
    headings::H4,
};

#[component]
pub fn DemoBentoGrid6() -> impl IntoView {
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
            <BentoGrid6>
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
                <BentoRow class="md:col-span-4">
                    <BentoCell>6</BentoCell>
                </BentoRow>
            </BentoGrid6>
        </div>
    }
}

#[component]
pub fn Variant2() -> impl IntoView {
    view! {
        <div>

            <H4>Variant 2</H4>
            <BentoGrid6>
                <div class="row-span-2 p-1 h-full rounded-lg">
                    <BentoCell>1</BentoCell>
                </div>
                <BentoRow class="">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <div class="row-span-2 p-1 h-full rounded-lg">
                    <BentoCell>3</BentoCell>
                </div>
                <BentoRow class="">
                    <BentoCell>4</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>5</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>6</BentoCell>
                </BentoRow>
            </BentoGrid6>

        </div>
    }
}

#[component]
pub fn Variant3() -> impl IntoView {
    view! {
        <div>

            <H4>Variant 3</H4>
            <BentoGrid6>
                <BentoRow class="">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <div class="col-span-2 p-1 h-32 rounded-lg">
                    <BentoCell>3</BentoCell>
                </div>
                <div class="col-span-2 p-1 h-32 rounded-lg">
                    <BentoCell>4</BentoCell>
                </div>
                <BentoRow class="">
                    <BentoCell>5</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>6</BentoCell>
                </BentoRow>
            </BentoGrid6>

        </div>
    }
}

#[component]
pub fn Variant4() -> impl IntoView {
    view! {
        <div>
            <H4>Variant 4</H4>
            <div class="grid gap-2 sm:grid-cols-2 md:grid-cols-6">
                <BentoRow class="md:col-span-3">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-3">
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
                <BentoRow class="md:col-span-4">
                    <BentoCell>6</BentoCell>
                </BentoRow>
            </div>
        </div>
    }
}
