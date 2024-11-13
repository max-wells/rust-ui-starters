use leptos::*;

use crate::registry::ui::{
    command::{
        Command, CommandContext, CommandGroup, CommandInput, CommandItem, CommandList,
        CommandProvider,
    },
    separator::Separator,
};

#[component]
pub fn MyCommandBar() -> impl IntoView {
    // List of demo items with name and href
    let items_components = vec![
        ("Button", "/demos-core/button"),
        ("Checkbox", "/demos-core/checkbox"),
        ("Dialog", "/demos-core/dialog"),
        ("Alert Dialog", "/demos-core/alert-dialog"),
    ];

    let items_hooks = vec![
        ("Use Hover", "/demos-hooks/use-hover"),
        ("Use Cycle List", "/demos-hooks/use-cycle-list"),
    ];

    view! {
        <div class="p-4">
            <CommandProvider>
                <Command class="rounded-lg border shadow-md w-[450px]">
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
                </Command>
            </CommandProvider>
        </div>
    }
}
