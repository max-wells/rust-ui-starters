use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Status(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(into, optional)] variant: StatusIndactorVariant,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("relative", class()));

    view! {
        <div {..attributes} class=class>
            {children()}
            <StatusIndactor variant=variant class="animate-ping" />
            <StatusIndactor variant=variant />
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(TwClass, Clone, Copy)]
#[tw(class = "absolute top-0 right-0 -mt-1 -mr-1 rounded-full size-4")]
pub struct StatusIndactorClass {
    pub variant: StatusIndactorVariant,
}

#[derive(TwVariant)]
pub enum StatusIndactorVariant {
    #[tw(default, class = "bg-neutral-300")]
    Default,
    #[tw(class = "bg-green-300 ")]
    Active,
    #[tw(class = "bg-orange-300 ")]
    Inactive,
    #[tw(class = "bg-sky-300 ")]
    Normal,
}

#[component]
pub fn StatusIndactor(
    #[prop(into, optional)] variant: MaybeSignal<StatusIndactorVariant>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = create_memo(move |_| {
        let status_indicator = StatusIndactorClass {
            variant: variant.get(),
        };
        status_indicator.with_class(class.get())
    });

    view! { <div {..attributes} class=class /> }
}
