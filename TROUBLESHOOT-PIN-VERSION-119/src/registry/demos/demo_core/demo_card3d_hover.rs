use leptos::*;

use crate::registry::ui::card3d_hover::{
    Card3DHover, Card3DHoverImage, Card3DHoverImageOnHover, Card3DHoverImageTitle,
};

#[component]
pub fn DemoCard3dHover() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center w-full max-w-5xl bg-gray-900 h-[600px]">
            <CardDarkRider />
            <CardForceMage />
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn CardDarkRider() -> impl IntoView {
    view! {
        <Card3DHover>
            <Card3DHoverImage src="/images/3d-card/dark-rider-cover.png" />
            <Card3DHoverImageOnHover src="/images/3d-card/dark-rider-character.png" />

            // * NB: This one is not strictly necessary, but nice to have 😄
            <Card3DHoverImageTitle src="/images/3d-card/dark-rider-title.png" />
        </Card3DHover>
    }
}

#[component]
pub fn CardForceMage() -> impl IntoView {
    view! {
        <Card3DHover>
            <Card3DHoverImage src="/images/3d-card/force-mage-cover.png" />
            <Card3DHoverImageOnHover src="/images/3d-card/force-mage-character.png" />

            // * NB: This one is not strictly necessary, but nice to have 😄
            <Card3DHoverImageTitle src="/images/3d-card/force-mage-title.png" />
        </Card3DHover>
    }
}
