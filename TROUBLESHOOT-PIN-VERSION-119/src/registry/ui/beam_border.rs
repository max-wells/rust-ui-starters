use leptos::*;
use tailwind_fuse::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn BeamBorder(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("relative rounded-xl w-fit", class()));

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn BeamBorderImage(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(into)] src: MaybeSignal<String>,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!("object-contain border shadow-lg rounded-[inherit]", class())
    });

    view! { <img {..attributes} class=class src=src /> }
}

//
//
#[component]
pub fn BeamBorderLine(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "absolute inset-[0] rounded-[inherit]", 
            "[border:calc(var(--border-width)*1px)_solid_transparent] ![mask-clip:padding-box,border-box] ![mask-composite:intersect] [mask:linear-gradient(transparent,transparent),linear-gradient(white,white)]",
            "after:animate-beamBorder after:absolute after:aspect-square after:w-[calc(var(--size)*1px)] after:[animation-delay:var(--delay)] after:[background:linear-gradient(to_left,var(--color-from),var(--color-to),transparent)] after:[offset-anchor:calc(var(--anchor)*1%)_50%] after:[offset-path:rect(0_auto_auto_0_round_calc(var(--size)*1px))]",
            class()
        )
    });

    view! {
        <div
            {..attributes}
            class=class
            style="--size:250;--duration:12;--anchor:90;--border-width:1.5;--color-from:#ffaa40;--color-to:#9c40ff;--delay:-9s"
        />
    }
}
