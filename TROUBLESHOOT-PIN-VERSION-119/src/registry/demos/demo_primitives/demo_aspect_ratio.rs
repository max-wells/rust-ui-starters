use leptos::*;

use crate::registry::primitives::p_aspect_ratio::PrimitiveAspectRatioRoot;

#[component]
pub fn DemoAspectRatio() -> impl IntoView {
    view! {
        <div class="overflow-hidden rounded-md w-[300px]">
            <PrimitiveAspectRatioRoot ratio=RATIO>
                <img
                    attr:class=CLASS_IMG
                    src="https://images.unsplash.com/photo-1535025183041-0991a977e25b?w=300&dpr=2&q=80"
                    alt="Landscape photograph by Tobias Tullius"
                />
            </PrimitiveAspectRatioRoot>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ CONSTANTS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const CLASS_IMG: &str = "object-cover w-full h-full";

const RATIO: f64 = 16.0 / 9.0;
