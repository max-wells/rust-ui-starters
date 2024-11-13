use leptos::*;

use crate::registry::ui::beam_border::{BeamBorder, BeamBorderImage, BeamBorderLine};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn DemoBeamBorder() -> impl IntoView {
    const IMG_DARK: &str = "/images/dashboard-dark.webp";
    const IMG_LIGHT: &str = "/images/dashboard-light.webp";

    view! {
        <BeamBorder>
            <BeamBorderImage src=IMG_DARK class="hidden dark:block w-[700px]" />
            <BeamBorderImage src=IMG_LIGHT class="block dark:hidden w-[700px]" />

            <BeamBorderLine />
        </BeamBorder>
    }
}
