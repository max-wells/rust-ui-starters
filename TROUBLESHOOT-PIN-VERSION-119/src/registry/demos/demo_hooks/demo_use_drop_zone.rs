use leptos::*;

#[cfg(feature = "hydrate")]
use leptos::html::Div;
#[cfg(feature = "hydrate")]
use leptos_use::docs::BooleanDisplay;
#[cfg(feature = "hydrate")]
use leptos_use::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn};

#[component]
pub fn DemoUseDropZone() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let drop_zone_component = {
        let (dropped, set_dropped) = create_signal(false);

        let drop_zone_el = create_node_ref::<Div>();

        let UseDropZoneReturn {
            is_over_drop_zone,
            files,
        } = use_drop_zone_with_options(
            drop_zone_el,
            UseDropZoneOptions::default()
                .on_drop(move |_| set_dropped(true))
                .on_enter(move |_| set_dropped(false)),
        );

        view! {
            <div class="flex">
                <div class="relative w-full h-auto">
                    <p>Drop files into dropZone</p>
                    <img
                        width="64"
                        src="use_drop_zone/demo/img/leptos-use-logo.svg"
                        alt="Drop me"
                    />
                    <div
                        node_ref=drop_zone_el
                        class="flex flex-col justify-center items-center pt-6 w-full h-auto min-h-[200px] bg-gray-400/10"
                    >
                        <div>is_over_drop_zone: <BooleanDisplay value=is_over_drop_zone /></div>
                        <div>dropped: <BooleanDisplay value=dropped /></div>
                        <div class="flex flex-wrap justify-center items-center">
                            <For each=files key=|f| f.name() let:file>
                                <div class="w-200px bg-black-200/10 ma-2 pa-6">
                                    <p>Name: {file.name()}</p>
                                    <p>Size: {file.size()}</p>
                                    <p>Type: {file.type_()}</p>
                                    <p>Last modified: {file.last_modified()}</p>
                                </div>
                            </For>
                        </div>
                    </div>
                </div>
            </div>
        }
    };

    #[cfg(not(feature = "hydrate"))]
    let drop_zone_component = view! {
        <p>
            "Drop zone functionality is not available on the server. This component requires client-side hydration."
        </p>
    };

    view! { <div>{drop_zone_component}</div> }
}
