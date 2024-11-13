use leptos::*;

use crate::registry::primitives::Orientation;

#[component]
pub fn CustomScrollAreaScrollbar(
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    children: Children,
) -> impl IntoView {
    let base_class = "flex select-none touch-none p-0.5 bg-gray-200 transition-colors duration-[160ms] ease-out hover:bg-gray-300";
    let orientation_class = match orientation {
        Orientation::Vertical => "w-2.5",
        Orientation::Horizontal => "flex-col h-2.5",
    };

    view! {
        <div
            class=format!("{} {}", base_class, orientation_class)
            data-orientation=orientation.to_string().to_lowercase()
        >
            {children()}
        </div>
    }
}
