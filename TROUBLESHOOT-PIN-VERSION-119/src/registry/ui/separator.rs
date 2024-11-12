use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Separator(
    #[prop(into, optional)] orientation: MaybeSignal<SeparatorOrientation>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    // children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        let orientation = orientation.get();
        let separator = SeparatorClass {
            orientation,
        };
        separator.with_class(class.get())
    });

    view! { <div {..attributes} class=class role="separator" /> }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🧬 STRUCT 🧬                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(TwClass, Default)]
#[tw(class = "shrink-0 bg-border")]
pub struct SeparatorClass {
    orientation: SeparatorOrientation,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(TwVariant)]
pub enum SeparatorOrientation {
    #[tw(default, class = "w-full h-[1px]")]
    Default,
    #[tw(class = "h-full w-[1px]")]
    Vertical,
}
