use floating_ui_leptos::{
    use_floating, Arrow, ArrowData, ArrowOptions, DetectOverflowOptions, Flip, FlipOptions,
    IntoReference, MiddlewareVec, Offset, OffsetOptions, Padding, Placement, Shift, ShiftOptions,
    Side, UseFloatingOptions, UseFloatingReturn, ARROW_NAME,
};
use leptos::{
    html::{Button, Div},
    *,
};

#[component]
pub fn DemoUseFloatingTooltip() -> impl IntoView {
    let reference_ref = create_node_ref::<Button>();
    let floating_ref = create_node_ref::<Div>();
    let arrow_ref = create_node_ref::<Div>();

    let (open, set_open) = create_signal(false);

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(6.0))),
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(Shift::new(ShiftOptions::default().detect_overflow(
            DetectOverflowOptions::default().padding(Padding::All(5.0)),
        ))),
        Box::new(Arrow::new(ArrowOptions::new(arrow_ref))),
    ];

    let UseFloatingReturn {
        placement,
        floating_styles,
        middleware_data,
        ..
    } = use_floating(
        reference_ref.into_reference(),
        floating_ref,
        UseFloatingOptions::default()
            .open(open.into())
            .placement(Placement::Top.into())
            .middleware(middleware.into())
            .while_elements_mounted_auto_update(),
    );

    let static_side = move || placement().side().opposite();
    let arrow_data = move || -> Option<ArrowData> { middleware_data().get_as(ARROW_NAME) };
    let arrow_x =
        move || arrow_data().and_then(|arrow_data| arrow_data.x.map(|x| format!("{}px", x)));
    let arrow_y =
        move || arrow_data().and_then(|arrow_data| arrow_data.y.map(|y| format!("{}px", y)));

    view! {
        <button
            _ref=reference_ref
            id="button"
            class="p-2 text-white bg-blue-500 rounded-md"
            aria-describedby="tooltip"
            on:mouseenter=move |_| set_open(true)
            on:mouseleave=move |_| set_open(false)
            on:focus=move |_| set_open(true)
            on:blur=move |_| set_open(false)
        >
            My button
        </button>

        <div
            _ref=floating_ref
            class="p-2 text-white bg-blue-500 rounded-md"
            id="tooltip"
            role="tooltip"
            style:display=move || match open() {
                true => "block",
                false => "none",
            }
            style:position=move || floating_styles().style_position()
            style:top=move || floating_styles().style_top()
            style:left=move || floating_styles().style_left()
            style:transform=move || floating_styles().style_transform()
            style:will-change=move || floating_styles().style_will_change()
        >
            "My tooltip with more content"
            <div
                _ref=arrow_ref
                id="arrow"
                style:left=move || match static_side() {
                    Side::Left => Some("-4px".into()),
                    _ => arrow_x(),
                }
                style:top=move || match static_side() {
                    Side::Top => Some("-4px".into()),
                    _ => arrow_y(),
                }
                style:right=move || match static_side() {
                    Side::Right => Some("-4px"),
                    _ => None,
                }
                style:bottom=move || match static_side() {
                    Side::Bottom => Some("-4px"),
                    _ => None,
                }
            />
        </div>
    }
}
