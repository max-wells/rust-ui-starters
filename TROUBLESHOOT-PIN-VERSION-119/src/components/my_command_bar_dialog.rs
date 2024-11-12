use leptos::*;

use crate::registry::icons::others::search::SearchIcon;
use crate::registry::ui::command::{
    CommandContext, CommandDialog, CommandGroup, CommandInput, CommandItem, CommandList,
    CommandTrigger,
};
use crate::registry::ui::dialog::DialogProvider;
use crate::registry::ui::separator::Separator;

// TODO 🐛. Bug with nested in layouts (Navbar)
// TODO. └──> ✅ Working fine DemoCommandDialog (content-mdx) + page_test.rs.
// TODO. └──> 🔸 Not working in Navbar.

// TODO ---> Maybe because the Component is still in the context of the page? And given in ✅ it changes completely of pages, maybe that works because of that?

#[component]
pub fn MyCommandBarDialog() -> impl IntoView {
    // List of demo items with name and href
    let items_components = vec![
        ("Button", "/demos-core/button"),
        ("Checkbox", "/demos-core/checkbox"),
        ("Input", "/demos-core/input"),
        ("Textarea", "/demos-core/textarea"),
        ("Dialog", "/demos-core/dialog"),
        ("Alert Dialog", "/demos-core/alert-dialog"),
    ];

    let items_hooks = vec![
        ("Use Hover", "/demos-hooks/use-hover"),
        ("Use Cycle List", "/demos-hooks/use-cycle-list"),
    ];

    view! {
        <div class="p-2">
            <DialogProvider>
                <CommandTrigger>
                    <SearchIcon class="mr-2 size-4" />
                    "Command Bar"
                </CommandTrigger>
                <CommandDialog>
                    <CommandInput placeholder="Search Components & Hooks..." autofocus=true />
                    <CommandList>
                        <CommandGroup heading="Components">
                            {move || {
                                let context = use_context::<CommandContext>()
                                    .expect("CommandContext not found");
                                let query = (context.search_query)().to_lowercase();
                                items_components
                                    .iter()
                                    .filter(|(name, _)| name.to_lowercase().contains(&query))
                                    .map(|&(name, href)| {
                                        view! { <CommandItem href=href>{name}</CommandItem> }
                                    })
                                    .collect::<Vec<_>>()
                            }}
                        </CommandGroup>

                        <Separator class="my-1" />

                        <CommandGroup heading="Hooks">
                            {move || {
                                let context = use_context::<CommandContext>()
                                    .expect("CommandContext not found");
                                let query = (context.search_query)().to_lowercase();
                                items_hooks
                                    .iter()
                                    .filter(|(name, _)| name.to_lowercase().contains(&query))
                                    .map(|&(name, href)| {
                                        view! { <CommandItem href=href>{name}</CommandItem> }
                                    })
                                    .collect::<Vec<_>>()
                            }}
                        </CommandGroup>
                    </CommandList>
                </CommandDialog>
            </DialogProvider>
        </div>
    }
}
