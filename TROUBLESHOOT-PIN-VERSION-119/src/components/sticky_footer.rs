use leptos::*;

use crate::registry::ui::headings::H3;

#[component]
pub fn StickyFooter() -> impl IntoView {
    view! {
        <div
            class="relative mt-2 h-[300px]"
            style="clip-path: polygon(0% 0px, 100% 0%, 100% 100%, 0px 100%);"
        >
            <footer class="relative -top-[100vh] h-[calc(100vh+300px)]">
                <div class="sticky h-[300px] top-[calc(100vh-300px)]">
                    <div class="w-full h-full px-12 py-8 bg-secondary">
                        <div class="flex justify-between items-center w-full h-[200px]">
                            <div>
                                <H3>"Want to see more components?"</H3>
                                <p class="text-muted-foreground">
                                    "Please check out our newsletter to get the latest updates!"
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </footer>
        </div>
    }
}
