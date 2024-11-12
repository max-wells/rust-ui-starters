use leptos::*;

use crate::registry::ui::carousel_parallax::{CarouselParallax, SlideImage, SlideItem, Slides};
use crate::registry::ui::headings::H3;

#[component]
pub fn DemoCarouselParallax() -> impl IntoView {
    view! {
        <CarouselParallax class="min-h-[650px]">
            <H3 class="py-8 text-center md:text-4xl">Tailwind CSS Parallax Carousel</H3>

            <Slides>
                {IMAGE_URLS
                    .iter()
                    .map(|&src| {
                        view! {
                            <SlideItem>
                                <SlideImage src=src alt="Carousel Parallax image demo" />
                            </SlideItem>
                        }
                    })
                    .collect::<Vec<_>>()}
            </Slides>
        </CarouselParallax>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ CONSTANTS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const IMAGE_URLS: &[&str] = &[
    "https://images.pexels.com/photos/687501/pexels-photo-687501.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1",
    "https://images.pexels.com/photos/1459347/pexels-photo-1459347.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1",
    "https://images.pexels.com/photos/1022166/pexels-photo-1022166.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1",
    "https://images.pexels.com/photos/3162822/pexels-photo-3162822.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1",
    "https://images.pexels.com/photos/2016703/pexels-photo-2016703.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1",
    "https://images.pexels.com/photos/5818810/pexels-photo-5818810.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1",
    "https://images.pexels.com/photos/5896359/pexels-photo-5896359.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1",
    "https://images.pexels.com/photos/5430078/pexels-photo-5430078.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1",
    "https://images.pexels.com/photos/1431114/pexels-photo-1431114.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1",
    "https://images.pexels.com/photos/1722179/pexels-photo-1722179.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1",
];
