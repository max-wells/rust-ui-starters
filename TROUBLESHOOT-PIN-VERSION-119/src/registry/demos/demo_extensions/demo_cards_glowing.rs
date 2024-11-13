use leptos::*;
use leptos_meta::Stylesheet;

use crate::registry::ui::cards_glowing::{CardGlowingContent, CardGlowingItem, CardsGlowing};

#[component]
pub fn DemoCardsGlowing() -> impl IntoView {
    view! {
        <Stylesheet id="cards-glowing" href="/components/cards-glowing.css" />

        <div class="flex overflow-hidden justify-center items-center p-0 m-0">
            <CardsGlowing>
                {GLOWING_CARDS_TITLES
                    .iter()
                    .map(|title| {
                        view! {
                            <CardGlowingItem>
                                <CardGlowingContent class="card-content">
                                    <h3 class="text-neutral-300">{*title}</h3>
                                </CardGlowingContent>
                            </CardGlowingItem>
                        }
                    })
                    .collect::<Vec<_>>()}
            </CardsGlowing>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ CONSTANTS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const GLOWING_CARDS_TITLES: &[&str] = &[
    "Fake Title 1",
    "Fake Title 2",
    "Fake Title 3",
    "Fake Title 4",
    "Fake Title 5",
    "Fake Title 6",
];
