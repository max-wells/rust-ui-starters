use leptos::*;

use crate::registry::primitives::p_avatar::{
    PrimitiveAvatarFallback, PrimitiveAvatarImage, PrimitiveAvatarRoot,
};

#[component]
pub fn DemoAvatar() -> impl IntoView {
    view! {
        <div class="flex gap-5">
            <PrimitiveAvatarRoot attr:class=CLASS_AVATAR_ROOT>
                <PrimitiveAvatarImage
                    attr:class=CLASS_AVATAR_IMAGE
                    attr:src="https://images.unsplash.com/photo-1492633423870-43d1cd2775eb?&w=128&h=128&dpr=2&q=80"
                    attr:alt="Colm Tuite"
                />
                <PrimitiveAvatarFallback attr:class=CLASS_AVATAR_FALLBACK delay_ms=DELAY_MS>
                    CT
                </PrimitiveAvatarFallback>
            </PrimitiveAvatarRoot>

            <PrimitiveAvatarRoot attr:class=CLASS_AVATAR_ROOT>
                <PrimitiveAvatarFallback attr:class=CLASS_AVATAR_FALLBACK delay_ms=DELAY_MS>
                    PD
                </PrimitiveAvatarFallback>
            </PrimitiveAvatarRoot>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ CONSTANTS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const CLASS_AVATAR_ROOT: &str = "inline-flex size-10 select-none items-center justify-center overflow-hidden rounded-full align-middle";

const CLASS_AVATAR_IMAGE: &str = "size-full rounded-[inherit] object-cover";

const CLASS_AVATAR_FALLBACK: &str =
    "leading-1 flex size-full items-center justify-center bg-muted text-[15px] font-medium";

const DELAY_MS: f64 = 600.0;
