use leptos::*;
use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = "flex items-center justify-center w-full")]
pub struct Animate {
    pub variant: AnimateVariant,
    pub hover_variant: AnimateHoverVariant,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn Animate(
    #[prop(into, optional)] variant: MaybeSignal<AnimateVariant>,
    #[prop(into, optional)] hover_variant: MaybeSignal<AnimateHoverVariant>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into, optional)] style: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        let variant = variant.get();
        let hover_variant = hover_variant.get();
        let animate = Animate {
            variant,
            hover_variant,
        };
        animate.with_class(class.get())
    });

    view! {
        <div {..attributes} class=class style=style>
            {children()}
        </div>
    }
}

#[component]
pub fn AnimateGroup(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("w-full", class()));

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn AnimateGroupItem(
    #[prop(into, optional)] variant: MaybeSignal<AnimateVariant>,
    #[prop(into, optional)] hover_variant: MaybeSignal<AnimateHoverVariant>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into)] delay_ms: MaybeSignal<u32>,
    #[prop(into)] key: MaybeSignal<String>,
    #[prop(default = "forwards")] fill_mode: &'static str,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        let variant = variant.get();
        let hover_variant = hover_variant.get();
        let animate = Animate {
            variant,
            hover_variant,
        };
        animate.with_class(class.get())
    });

    let style = create_memo(move |_| {
        let delay = delay_ms.get();
        format!(
            "animation-delay: {}ms; animation-fill-mode: {};",
            delay, fill_mode
        )
    });

    view! {
        <div key=key.get() {..attributes} class=class style=style>
            {children()}
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ VARIANTS ✨                        */
/*.:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(TwVariant)]
pub enum AnimateVariant {
    #[tw(default, class = "")]
    Default,
    #[tw(class = "opacity-0 animate-fadeUp")]
    FadeUp,
    #[tw(
        class = "animate-fadeOutDown   [animation-range:0px_300px] [animation-timeline:scroll()] supports-no-scroll-driven-animations:animate-none"
    )]
    AnimateScrollFadeOut,
    #[tw(
        class = "animate-makeItBigger   [animation-range:0%_60%] [animation-timeline:--quote] [view-timeline-name:--quote] supports-no-scroll-driven-animations:animate-none"
    )]
    AnimateScrollBigger,
}

#[derive(TwVariant)]
pub enum AnimateHoverVariant {
    #[tw(default, class = "")]
    Default,
    #[tw(class = "hover:animate-blink")]
    Blink,
    #[tw(class = "hover:animate-blurredFadeIn")]
    BlurredFadeIn,
    #[tw(class = "hover:animate-bounceFadeIn")]
    BounceFadeIn,
    #[tw(class = "hover:animate-bounceHorizontal")]
    BounceHorizontal,
    #[tw(class = "hover:animate-bounceVertical")]
    BounceVertical,
    #[tw(class = "hover:animate-bounceCustom")] // TODO: check
    BounceCustom,
    #[tw(class = "hover:animate-contractHorizontally")]
    ContractHorizontally,
    #[tw(class = "hover:animate-contractVertically")]
    ContractVertically,
    #[tw(class = "hover:animate-expandHorizontally")]
    ExpandHorizontally,
    #[tw(class = "hover:animate-expandVertically")]
    ExpandVertically,
    #[tw(class = "hover:animate-fadeIn")]
    FadeIn,
    #[tw(class = "hover:animate-fadeInDown")]
    FadeInDown,
    #[tw(class = "hover:animate-fadeInLeft")]
    FadeInLeft,
    #[tw(class = "hover:animate-fadeInRight")]
    FadeInRight,
    #[tw(class = "hover:animate-fadeInUp")]
    FadeInUp,
    #[tw(class = "hover:animate-fadeOut")]
    FadeOut,
    #[tw(class = "hover:animate-fadeOutUp")]
    FadeOutUp,
    #[tw(class = "hover:animate-fadeOutDownV2")] // TODO: V2
    FadeOutDownV2,
    #[tw(class = "hover:animate-fadeOutLeft")]
    FadeOutLeft,
    #[tw(class = "hover:animate-fadeOutRight")]
    FadeOutRight,
    #[tw(class = "hover:animate-flash")]
    Flash,
    #[tw(class = "hover:animate-flashV0")] // TODO
    FlashV0,
    #[tw(class = "hover:animate-flipHorizontal")]
    FlipHorizontal,
    #[tw(class = "hover:animate-flipVertical")]
    FlipVertical,
    #[tw(class = "hover:animate-flipX")]
    FlipX,
    #[tw(class = "hover:animate-flipY")]
    FlipY,
    #[tw(class = "hover:animate-flipInY")]
    FlipInY,
    #[tw(class = "hover:animate-flipInX")]
    FlipInX,
    #[tw(class = "hover:animate-flipOutY")]
    FlipOutY,
    #[tw(class = "hover:animate-flipOutX")]
    FlipOutX,
    #[tw(class = "hover:animate-float")]
    Float,
    #[tw(class = "hover:animate-hang")]
    Hang,
    #[tw(class = "hover:animate-heartbeat")]
    Heartbeat,
    #[tw(class = "hover:animate-horizontalVibration")]
    HorizontalVibration,
    #[tw(class = "hover:animate-jiggle")]
    Jiggle,
    #[tw(class = "hover:animate-jiggleV0")] // TODO
    JiggleV0,
    #[tw(class = "hover:animate-jump")]
    Jump,
    #[tw(class = "hover:animate-pop")]
    Pop,
    #[tw(class = "hover:animate-pulseCustom")] // TODO: custom
    PulseCustom,
    #[tw(class = "hover:animate-pulseFadeIn")]
    PulseFadeIn,
    #[tw(class = "hover:animate-rise")]
    Rise,
    #[tw(class = "hover:animate-rollIn")]
    RollIn,
    #[tw(class = "hover:animate-rollOut")]
    RollOut,
    #[tw(class = "hover:animate-rotate180")]
    Rotate180,
    #[tw(class = "hover:animate-rotate360")]
    Rotate360,
    #[tw(class = "hover:animate-rotate90")]
    Rotate90,
    #[tw(class = "hover:animate-rotateIn")]
    RotateIn,
    #[tw(class = "hover:animate-rotateOut")]
    RotateOut,
    #[tw(class = "hover:animate-rotationalWave")]
    RotationalWave,
    #[tw(class = "hover:animate-rubberBand")]
    RubberBand,
    #[tw(class = "hover:animate-rubberBand-v0")] // TODO
    RubberBandV0,
    #[tw(class = "hover:animate-scale")] // TODO
    Scale,
    #[tw(class = "hover:animate-shake")]
    Shake,
    #[tw(class = "hover:animate-shakeV0")] // TODO
    ShakeV0,
    #[tw(class = "hover:animate-sink")]
    Sink,
    #[tw(class = "hover:animate-skew")]
    Skew,
    #[tw(class = "hover:animate-slideDown")]
    SlideDown,
    #[tw(class = "hover:animate-slideDownAndFade")]
    SlideDownAndFade,
    #[tw(class = "hover:animate-slideInBottom")]
    SlideInBottom,
    #[tw(class = "hover:animate-slideInLeft")]
    SlideInLeft,
    #[tw(class = "hover:animate-slideInRight")]
    SlideInRight,
    #[tw(class = "hover:animate-slideInTop")]
    SlideInTop,
    #[tw(class = "hover:animate-slideLeft")]
    SlideLeft,
    #[tw(class = "hover:animate-slideLeftAndFade")]
    SlideLeftAndFade,
    #[tw(class = "hover:animate-slideOutBottom")]
    SlideOutBottom,
    #[tw(class = "hover:animate-slideOurLeft")]
    SlideOurLeft,
    #[tw(class = "hover:animate-slideOutLeft")]
    SlideOutLeft,
    #[tw(class = "hover:animate-slideOutTop")]
    SlideOutTop,
    #[tw(class = "hover:animate-slideRight")]
    SlideRight,
    #[tw(class = "hover:animate-slideRightAndFade")]
    SlideRightAndFade,
    #[tw(class = "hover:animate-slideRotateIn")]
    SlideRotateIn,
    #[tw(class = "hover:animate-slideRotateOut")]
    SlideRotateOut,
    #[tw(class = "hover:animate-slideUp")]
    SlideUp,
    #[tw(class = "hover:animate-slideUpAndFade")]
    SlideUpAndFade,
    #[tw(class = "hover:animate-slideUpFade")]
    SlideUpFade,
    #[tw(class = "hover:animate-spinClockwise")]
    SpinClockwise,
    #[tw(class = "hover:animate-spinCounterClockwise")]
    SpinCounterClockwise,
    #[tw(class = "hover:animate-sway")]
    Sway,
    #[tw(class = "hover:animate-swing")]
    Swing,
    #[tw(class = "hover:animate-swingDropIn")]
    SwingDropIn,
    #[tw(class = "hover:animate-swingV0")] // TODO
    SwingV0,
    #[tw(class = "hover:animate-squeeze")]
    Squeeze,
    #[tw(class = "hover:animate-tada")]
    Tada,
    #[tw(class = "hover:animate-tiltHorizontal")]
    TiltHorizontal,
    #[tw(class = "hover:animate-vibrate")]
    Vibrate,
    #[tw(class = "hover:animate-wobble")]
    Wobble,
    #[tw(class = "hover:animate-zoomIn")]
    ZoomIn,
    #[tw(class = "hover:animate-zoomOut")]
    ZoomOut,
}
