use leptos::*;
use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(
    class = "absolute inset-0 w-full h-full bg-gradient-to-r  rounded-full transform blur-3xl scale-[0.80]"
)]
pub struct GradientClass {
    pub variant: GradientVariant,
}

#[derive(TwVariant)]
pub enum GradientVariant {
    #[tw(default, class = "")]
    Default,
    #[tw(class = "from-blue-500 to-teal-500")]
    BlueTeal,
}

#[component]
pub fn Gradient(
    #[prop(into, optional)] variant: MaybeSignal<GradientVariant>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = create_memo(move |_| {
        let gradient = GradientClass {
            variant: variant.get(),
        };
        gradient.with_class(class.get())
    });

    view! { <div {..attributes} class=class /> }
}
