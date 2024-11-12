use convert_case::{Case, Casing};
use floating_ui_leptos::{MiddlewareVec, Offset, OffsetOptions, Placement};
use leptos::*;
use tailwind_fuse::tw_merge;

use crate::registry::floating_ui::{
    chrome::Chrome, floating::Floating, grid_item::GridItem, reference::Reference,
};

struct PlacementData {
    placement: Placement,
    top: Option<String>,
    right: Option<String>,
    bottom: Option<String>,
    left: Option<String>,
}

#[component]
pub fn DemoUseFloatingPlacement() -> impl IntoView {
    let (placement, set_placement) = create_signal(Placement::Top);

    view! {
        <GridItem
            title="Placement"
            description="Places your floating element relative to another element."
            chrome=move || {
                view! {
                    <Chrome label="Click the dots" center=true shadow=false>
                        <For
                            each=|| [
                                PlacementData {
                                    placement: Placement::Top,
                                    top: Some("0px".into()),
                                    right: None,
                                    bottom: None,
                                    left: Some("calc(50% - 10px - 1rem)".into()),
                                },
                                PlacementData {
                                    placement: Placement::TopStart,
                                    top: Some("0px".into()),
                                    right: None,
                                    bottom: None,
                                    left: Some("calc(50% - 70px - 1rem)".into()),
                                },
                                PlacementData {
                                    placement: Placement::TopEnd,
                                    top: Some("0px".into()),
                                    right: None,
                                    bottom: None,
                                    left: Some("calc(50% + 50px - 1rem)".into()),
                                },
                                PlacementData {
                                    placement: Placement::Bottom,
                                    top: None,
                                    right: None,
                                    bottom: Some("0px".into()),
                                    left: Some("calc(50% - 10px - 1rem)".into()),
                                },
                                PlacementData {
                                    placement: Placement::BottomStart,
                                    top: None,
                                    right: None,
                                    bottom: Some("0px".into()),
                                    left: Some("calc(50% - 70px - 1rem)".into()),
                                },
                                PlacementData {
                                    placement: Placement::BottomEnd,
                                    top: None,
                                    right: None,
                                    bottom: Some("0px".into()),
                                    left: Some("calc(50% + 50px - 1rem)".into()),
                                },
                                PlacementData {
                                    placement: Placement::Right,
                                    top: Some("calc(50% - 10px - 1rem)".into()),
                                    right: Some("min(50px, 5%)".into()),
                                    bottom: None,
                                    left: None,
                                },
                                PlacementData {
                                    placement: Placement::RightStart,
                                    top: Some("calc(50% - 70px - 1rem)".into()),
                                    right: Some("min(50px, 5%)".into()),
                                    bottom: None,
                                    left: None,
                                },
                                PlacementData {
                                    placement: Placement::RightEnd,
                                    top: Some("calc(50% + 50px - 1rem)".into()),
                                    right: Some("min(50px, 5%)".into()),
                                    bottom: None,
                                    left: None,
                                },
                                PlacementData {
                                    placement: Placement::Left,
                                    top: Some("calc(50% - 10px - 1rem)".into()),
                                    right: None,
                                    bottom: None,
                                    left: Some("min(50px, 5%)".into()),
                                },
                                PlacementData {
                                    placement: Placement::LeftStart,
                                    top: Some("calc(50% - 70px - 1rem)".into()),
                                    right: None,
                                    bottom: None,
                                    left: Some("min(50px, 5%)".into()),
                                },
                                PlacementData {
                                    placement: Placement::LeftEnd,
                                    top: Some("calc(50% + 50px - 1rem)".into()),
                                    right: None,
                                    bottom: None,
                                    left: Some("min(50px, 5%)".into()),
                                },
                            ]
                            key=|data| format!("{:?}", data.placement).to_case(Case::Kebab)
                            children=move |data| {
                                view! {
                                    <button
                                        class="absolute p-4 transition hover:scale-125"
                                        aria-label=format!("{:?}", data.placement)
                                            .to_case(Case::Kebab)
                                        style:top=data.top
                                        style:right=data.right
                                        style:bottom=data.bottom
                                        style:left=data.left
                                        on:click=move |_| set_placement.set(data.placement)
                                    >
                                        <div class=tw_merge!(
                                            "h-5 w-5 rounded-full border-2 border-solid",
                                        match placement.get() == data.placement {
                                            true => "border-gray-800 bg-gray-800",
                                            false => "border-gray-900"
                                        }
                                        ) />
                                    </button>
                                }
                            }
                        />
                        <Floating
                            placement=placement
                            middleware={
                                let middleware: MiddlewareVec = vec![
                                    Box::new(Offset::new(OffsetOptions::Value(5.0))),
                                ];
                                middleware
                            }
                            content=move || {
                                view! {
                                    <div
                                        class="text-sm font-bold text-center"
                                        style:min-width=move || {
                                            matches!(
                                                placement.get(),
                                                Placement::TopStart
                                                | Placement::TopEnd
                                                | Placement::BottomStart
                                                | Placement::BottomEnd
                                            )
                                                .then_some("8rem")
                                        }
                                    >
                                        {move || {
                                            format!("{:?}", placement.get()).to_case(Case::Kebab)
                                        }}
                                    </div>
                                }
                            }
                            reference=move |node_ref| view! { <Reference node_ref=node_ref /> }
                        />
                    </Chrome>
                }
            }
        />
    }
}
