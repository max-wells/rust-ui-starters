use leptos::*;

#[cfg(feature = "hydrate")]
use crate::registry::primitives::p_scroll_area::{
    PrimitiveScrollAreaCorner, PrimitiveScrollAreaRoot, PrimitiveScrollAreaScrollbar,
    PrimitiveScrollAreaThumb, PrimitiveScrollAreaViewport,
};
#[cfg(feature = "hydrate")]
use crate::registry::primitives::Orientation;

// #[cfg(feature = "hydrate")]
// use crate::registry::primitives::scroll_area::CustomScrollAreaScrollbar;

#[component]
pub fn DemoScrollArea() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let scroll_area_component = {
        let tags = StoredValue::new(
            (1..=50)
                .rev()
                .map(|num| format!("v1.2.0-beta.{num}"))
                .collect::<Vec<_>>(),
        );

        view! {
            <PrimitiveScrollAreaRoot attr:class="w-[200px] h-[225px] rounded overflow-hidden border">
                <PrimitiveScrollAreaViewport attr:class="w-full h-full rounded">
                    <div class="px-5 py-[15px]">
                        <div class="font-medium">"Tags"</div>
                        <For each=move || tags.get_value() key=|n| n.clone() let:data>
                            <div class="pt-2.5 mt-2.5 border-t">{data}</div>
                        </For>
                    </div>
                </PrimitiveScrollAreaViewport>

                // TODO. Not working properly... Keeps in Loading MDX component... state.
                // <CustomPrimitiveScrollAreaScrollbar>
                <PrimitiveScrollAreaScrollbar
                    attr:class="flex select-none touch-none p-0.5 bg-[#dddddd] dark:bg-[#222] transition-colors duration-[160ms] ease-out hover:bg-[#dddddd] dark:hover:bg-[#222] data-[orientation=vertical]:w-2.5 data-[orientation=horizontal]:flex-col data-[orientation=horizontal]:h-2.5"
                    orientation=Orientation::Vertical
                >
                    <PrimitiveScrollAreaThumb attr:class="flex-1 bg-[#b9b9b9] dark:bg-[#555]  rounded-[10px] relative before:content-[''] before:absolute before:top-1/2 before:left-1/2 before:-translate-x-1/2 before:-translate-y-1/2 before:w-full before:h-full before:min-w-[44px] before:min-h-[44px]" />
                </PrimitiveScrollAreaScrollbar>
                // <CustomPrimitiveScrollAreaScrollbar>

                <PrimitiveScrollAreaScrollbar
                    attr:class="flex select-none touch-none p-0.5 bg-[#dddddd] dark:bg-[#222] transition-colors duration-[160ms] ease-out hover:bg-[#dddddd] dark:hover:bg-[#222] data-[orientation=vertical]:w-2.5 data-[orientation=horizontal]:flex-col data-[orientation=horizontal]:h-2.5"
                    orientation=Orientation::Horizontal
                >
                    <PrimitiveScrollAreaThumb attr:class="flex-1 bg-[#b9b9b9] dark:bg-[#555]  rounded-[10px] relative before:content-[''] before:absolute before:top-1/2 before:left-1/2 before:-translate-x-1/2 before:-translate-y-1/2 before:w-full before:h-full before:min-w-[44px] before:min-h-[44px]" />
                </PrimitiveScrollAreaScrollbar>
                <PrimitiveScrollAreaCorner attr:class="bg-[#dddddd] dark:bg-[#222]" />
            </PrimitiveScrollAreaRoot>
        }
    };

    #[cfg(not(feature = "hydrate"))]
    let scroll_area_component = view! {
        <p>
            "Scroll area functionality is not available on the server. This component requires client-side hydration."
        </p>
    };

    view! { <div>{scroll_area_component}</div> }
}
