use leptos::*;

use crate::registry::ui::{
    animate::{Animate, AnimateHoverVariant},
    card::{Card, CardDescription},
    containers::Grid3,
    headings::H3,
};

#[component]
pub fn DemoAnimate() -> impl IntoView {
    view! {
        <div class="my-4 space-y-6">

            <H3 class="text-center">{format!("Hover animations ({})", HOVER_ANIMATIONS.len())}</H3>

            <Grid3>
                {HOVER_ANIMATIONS
                    .iter()
                    .map(|(variant, description)| {
                        view! {
                            <Card class="flex flex-col gap-4 items-center">
                                <Animate hover_variant=*variant>
                                    <AnimatedChildren />
                                </Animate>
                                <CardDescription>{*description}</CardDescription>
                            </Card>
                        }
                    })
                    .collect::<Vec<_>>()}
            </Grid3>

        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// * 💁 Could be litterally anything you want to animate :)
#[component]
fn AnimatedChildren() -> impl IntoView {
    view! {
        <div
            class="p-4 rounded-lg bg-primary/30 size-20"
            style="animation-iteration-count: infinite;"
        />
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ CONSTANTS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const HOVER_ANIMATIONS: &[(AnimateHoverVariant, &str)] = &[
    (AnimateHoverVariant::Blink, "Blink"),
    (AnimateHoverVariant::BlurredFadeIn, "BlurredFadeIn"),
    (AnimateHoverVariant::BounceFadeIn, "BounceFadeIn"),
    (AnimateHoverVariant::BounceHorizontal, "BounceHorizontal"),
    (AnimateHoverVariant::BounceVertical, "BounceVertical"),
    (AnimateHoverVariant::BounceCustom, "BounceCustom"),
    (
        AnimateHoverVariant::ContractHorizontally,
        "ContractHorizontally",
    ),
    (
        AnimateHoverVariant::ContractVertically,
        "ContractVertically",
    ),
    (
        AnimateHoverVariant::ExpandHorizontally,
        "ExpandHorizontally",
    ),
    (AnimateHoverVariant::ExpandVertically, "ExpandVertically"),
    (AnimateHoverVariant::FadeIn, "FadeIn"),
    (AnimateHoverVariant::FadeInDown, "FadeInDown"),
    (AnimateHoverVariant::FadeInLeft, "FadeInLeft"),
    (AnimateHoverVariant::FadeInRight, "FadeInRight"),
    (AnimateHoverVariant::FadeInUp, "FadeInUp"),
    (AnimateHoverVariant::FadeOut, "FadeOut"),
    (AnimateHoverVariant::FadeOutUp, "FadeOutUp"),
    (AnimateHoverVariant::FadeOutDownV2, "FadeOutDownV2"),
    (AnimateHoverVariant::FadeOutLeft, "FadeOutLeft"),
    (AnimateHoverVariant::FadeOutRight, "FadeOutRight"),
    (AnimateHoverVariant::Flash, "Flash"),
    (AnimateHoverVariant::FlashV0, "FlashV0"),
    (AnimateHoverVariant::FlipHorizontal, "FlipHorizontal"),
    (AnimateHoverVariant::FlipVertical, "FlipVertical"),
    (AnimateHoverVariant::FlipX, "FlipX"),
    (AnimateHoverVariant::FlipY, "FlipY"),
    (AnimateHoverVariant::FlipInY, "FlipInY"),
    (AnimateHoverVariant::FlipInX, "FlipInX"),
    (AnimateHoverVariant::FlipOutY, "FlipOutY"),
    (AnimateHoverVariant::FlipOutX, "FlipOutX"),
    (AnimateHoverVariant::Float, "Float"),
    (AnimateHoverVariant::Hang, "Hang"),
    (AnimateHoverVariant::Heartbeat, "Heartbeat"),
    (
        AnimateHoverVariant::HorizontalVibration,
        "HorizontalVibration",
    ),
    (AnimateHoverVariant::Jiggle, "Jiggle"),
    (AnimateHoverVariant::JiggleV0, "JiggleV0"),
    (AnimateHoverVariant::Jump, "Jump"),
    (AnimateHoverVariant::Pop, "Pop"),
    (AnimateHoverVariant::PulseCustom, "PulseCustom"),
    (AnimateHoverVariant::PulseFadeIn, "PulseFadeIn"),
    (AnimateHoverVariant::Rise, "Rise"),
    (AnimateHoverVariant::RollIn, "RollIn"),
    (AnimateHoverVariant::RollOut, "RollOut"),
    (AnimateHoverVariant::Rotate180, "Rotate180"),
    (AnimateHoverVariant::Rotate360, "Rotate360"),
    (AnimateHoverVariant::Rotate90, "Rotate90"),
    (AnimateHoverVariant::RotateIn, "RotateIn"),
    (AnimateHoverVariant::RotateOut, "RotateOut"),
    (AnimateHoverVariant::RotationalWave, "RotationalWave"),
    (AnimateHoverVariant::RubberBand, "RubberBand"),
    (AnimateHoverVariant::RubberBandV0, "RubberBandV0"),
    (AnimateHoverVariant::Scale, "Scale"),
    (AnimateHoverVariant::Shake, "Shake"),
    (AnimateHoverVariant::ShakeV0, "ShakeV0"),
    (AnimateHoverVariant::Sink, "Sink"),
    (AnimateHoverVariant::Skew, "Skew"),
    (AnimateHoverVariant::SlideDown, "SlideDown"),
    (AnimateHoverVariant::SlideDownAndFade, "SlideDownAndFade"),
    (AnimateHoverVariant::SlideInBottom, "SlideInBottom"),
    (AnimateHoverVariant::SlideInLeft, "SlideInLeft"),
    (AnimateHoverVariant::SlideInRight, "SlideInRight"),
    (AnimateHoverVariant::SlideInTop, "SlideInTop"),
    (AnimateHoverVariant::SlideLeft, "SlideLeft"),
    (AnimateHoverVariant::SlideLeftAndFade, "SlideLeftAndFade"),
    (AnimateHoverVariant::SlideOutBottom, "SlideOutBottom"),
    (AnimateHoverVariant::SlideOutLeft, "SlideOutLeft"),
    (AnimateHoverVariant::SlideOutTop, "SlideOutTop"),
    (AnimateHoverVariant::SlideRight, "SlideRight"),
    (AnimateHoverVariant::SlideRightAndFade, "SlideRightAndFade"),
    (AnimateHoverVariant::SlideRotateIn, "SlideRotateIn"),
    (AnimateHoverVariant::SlideRotateOut, "SlideRotateOut"),
    (AnimateHoverVariant::SlideUp, "SlideUp"),
    (AnimateHoverVariant::SlideUpAndFade, "SlideUpAndFade"),
    (AnimateHoverVariant::SlideUpFade, "SlideUpFade"),
    (AnimateHoverVariant::SpinClockwise, "SpinClockwise"),
    (
        AnimateHoverVariant::SpinCounterClockwise,
        "SpinCounterClockwise",
    ),
    (AnimateHoverVariant::Sway, "Sway"),
    (AnimateHoverVariant::Swing, "Swing"),
    (AnimateHoverVariant::SwingDropIn, "SwingDropIn"),
    (AnimateHoverVariant::SwingV0, "SwingV0"),
    (AnimateHoverVariant::Squeeze, "Squeeze"),
    (AnimateHoverVariant::Tada, "Tada"),
    (AnimateHoverVariant::TiltHorizontal, "TiltHorizontal"),
    (AnimateHoverVariant::Vibrate, "Vibrate"),
    (AnimateHoverVariant::Wobble, "Wobble"),
    (AnimateHoverVariant::ZoomIn, "ZoomIn"),
    (AnimateHoverVariant::ZoomOut, "ZoomOut"),
];
