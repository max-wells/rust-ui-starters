use leptos::*;

use crate::components::layouts::nav_desktop::NavDesktop;
use crate::components::layouts::nav_mobile::NavMobile;
use crate::components::{
    my_command_bar_dialog::MyCommandBarDialog, reactive_indicator::ReactiveIndicator,
    test_toggle_temporary::TestToggleTemporary, theme_toggle::ThemeToggle,
};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <header class="sticky top-0 z-50 w-full bg-transparent border-b border-border/40 backdrop-blur supports-[backdrop-filter]:bg-background/60">
            <div class="container flex justify-between items-center px-4 h-14">
                <nav class="flex flex-1 justify-between items-center">
                    <div class="flex items-center space-x-4">
                        <NavMobile />
                        <NavDesktop />
                    </div>

                    <div class="flex items-center space-x-2">
                        <MyCommandBarDialog />
                        <ReactiveIndicator />
                        <ThemeToggle />
                        <TestToggleTemporary />
                    </div>
                </nav>
            </div>
        </header>
    }
}
