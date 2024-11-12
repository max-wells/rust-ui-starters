use leptos::*;

#[component]
pub fn DemoTooltip() -> impl IntoView {
    view! {
        <div>
            <span
                class="overflow-hidden relative cursor-pointer hover:overflow-visible focus-visible:outline-none group"
                aria-describedby="tooltip-01"
            >
                top
                <span
                    role="tooltip"
                    id="tooltip-01"
                    class="absolute left-1/2 bottom-full invisible z-10 p-4 mb-2 w-48 text-sm text-white rounded opacity-0 transition-all -translate-x-1/2 group-hover:block group-hover:visible group-hover:opacity-100 bg-slate-700 before:invisible before:absolute before:left-1/2 before:top-full before:z-10 before:mb-2 before:-ml-2 before:border-x-8 before:border-t-8 before:border-x-transparent before:border-t-slate-700 before:opacity-0 before:transition-all before:content-[''] group-hover:before:visible group-hover:before:opacity-100"
                >
                    "Thanks for hovering! Im a tooltip"
                </span>
            </span>
        </div>
    }
}
