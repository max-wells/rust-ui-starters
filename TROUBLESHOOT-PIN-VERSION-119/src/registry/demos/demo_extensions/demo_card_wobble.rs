use leptos::*;
use leptos_meta::Stylesheet;

use crate::registry::ui::{
    button::Button,
    card_wobble::{
        provide_card_wobble_context, CardWobble, CardWobbleContent, CardWobbleImage,
        CardWobbleImageWrapper,
    },
};

#[component]
pub fn DemoCardWobble() -> impl IntoView {
    // provide_card_wobble_context();
    // TODO. └──>  Not working.. (I've added debug logs and we do get the mouse position, though))

    view! {
        <Stylesheet id="card-wobble" href="/components/card-wobble.css" />
        <script src="/components/card-wobble.js" />

        <div class="overflow-x-hidden p-10 w-full h-full border border-sky-500">
            <CardWobble>
                <CardWobbleImageWrapper>
                    <CardWobbleImage
                        src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/225363/forest.jpg"
                        alt="Woods landscape"
                    />
                </CardWobbleImageWrapper>

                <CardWobbleContent class="flex absolute top-0 left-0 z-10 flex-col justify-center items-center p-4 w-full h-full">
                    <h2 class="relative text-6xl font-bold text-white">"In The Wilderness"</h2>
                    <Button class="relative z-10 slide__action">Book Travel</Button>
                </CardWobbleContent>
            </CardWobble>
        </div>
    }
}
