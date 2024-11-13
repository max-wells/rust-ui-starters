use leptos::*;

use crate::registry::{
    icons::others::check::Check,
    primitives::p_checkbox::{CheckedState, PrimitiveCheckboxIndicator, PrimitiveCheckboxRoot},
};

#[component]
pub fn DemoPrimitiveCheckbox() -> impl IntoView {
    view! {
        <form>
            <div class="flex items-center">
                <PrimitiveCheckboxRoot
                    default_checked=CheckedState::Checked(true)
                    attr:class="shadow-blackA4 hover:bg-violet3 flex h-[25px] w-[25px] appearance-none items-center justify-center rounded-[4px] bg-white shadow-[0_2px_10px] outline-none focus:shadow-[0_0_0_2px_black]"
                    attr:id="c1"
                >
                    <PrimitiveCheckboxIndicator attr:class="text-violet11">
                        <Check class="size-4" />
                    </PrimitiveCheckboxIndicator>
                </PrimitiveCheckboxRoot>

                <label class="leading-none pl-[15px] text-[15px]" for="c1">
                    <span class="select-none">"Accept terms and conditions."</span>
                </label>
            </div>
        </form>
    }
}
