use leptos::*;

use crate::registry::primitives::p_slider::{
    PrimitiveSliderRange, PrimitiveSliderRoot, PrimitiveSliderThumb, SliderTrack,
};

#[component]
pub fn DemoSlider() -> impl IntoView {
    view! {
        <PrimitiveSliderRoot
            attr:class="relative flex items-center select-none touch-none w-[300px]"
            default_value=vec![5.0f64]
            max=20.0
            step=1.0
        >
            <SliderTrack attr:class="relative h-1.5 w-full grow overflow-hidden rounded-full bg-primary/20">
                <PrimitiveSliderRange attr:class="absolute h-full bg-primary">
                    {().into_view()}
                </PrimitiveSliderRange>
            </SliderTrack>
            <PrimitiveSliderThumb
                attr:class="block transition-colors border rounded-full shadow disabled:pointer-events-none disabled:opacity-50 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring size-4 border-primary/50 bg-background "
                attr:aria-label="Volume"
            >
                {().into_view()}
            </PrimitiveSliderThumb>
        </PrimitiveSliderRoot>
    }
}
