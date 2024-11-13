use leptos::*;
use leptos_meta::Stylesheet;
use tailwind_fuse::*;

#[component]
pub fn DemoGalleryZoom() -> impl IntoView {
    view! {
        <Stylesheet id="gallery-zoom" href="/components/gallery-zoom.css" />

        <GalleryGrid class="p-2 border border-sky-500">
            <GalleryList class="grid fixed p-0 m-0 galleryList top-[50%] left-[50%]">
                {ALL_ITEMS
                    .iter()
                    .enumerate()
                    .map(|(index, style)| {
                        view! {
                            <GalleryItem class="galleryItem" style=*style>
                                <GalleryItemImage index=index />
                            </GalleryItem>
                        }
                    })
                    .collect::<Vec<_>>()}
            </GalleryList>
        </GalleryGrid>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ CONSTANTS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const ALL_ITEMS: &[&str] = &[
    "--x1: 2; --x2: 6; --y1: 1; --y2: 4;",
    "--x1: 6; --x2: 8; --y1: 2; --y2: 4;",
    "--x1: 1; --x2: 4; --y1: 4; --y2: 7;",
    "--x1: 4; --x2: 7; --y1: 4; --y2: 7;",
    "--x1: 7; --x2: 9; --y1: 4; --y2: 6;",
    "--x1: 2; --x2: 4; --y1: 7; --y2: 9;",
    "--x1: 4; --x2: 7; --y1: 7; --y2: 10;",
    "--x1: 7; --x2: 10; --y1: 6; --y2: 9;",
];

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn GalleryGrid(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "grid overflow-hidden overflow-x-hidden place-items-center w-full min-h-screen",
            class()
        )
    });

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn GalleryList(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <ul {..attributes} class=class>
            {children()}
        </ul>
    }
}

#[component]
pub fn GalleryItem(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
    #[prop(into)] style: MaybeSignal<String>,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <li {..attributes} class=class style=style>
            {children()}
        </li>
    }
}

#[component]
pub fn GalleryItemImage(index: usize) -> impl IntoView {
    view! {
        <img
            class="object-cover absolute galleryGridImage top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] min-w-[200%] h-[200%]"
            src=format!("https://picsum.photos/600/600?random={}", index + 1)
            alt=""
        />
    }
}
