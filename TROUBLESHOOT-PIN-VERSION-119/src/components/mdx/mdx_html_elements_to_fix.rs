use leptos::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[allow(non_camel_case_types)]
#[component]
pub fn h1(children: Children) -> impl IntoView {
    view! { <h1 class="mt-2 text-4xl font-bold tracking-tight scroll-m-2">{children()}</h1> }
}

#[allow(non_camel_case_types)]
#[component]
pub fn h2(children: Children) -> impl IntoView {
    view! { <h2 class="mt-12 text-3xl font-bold tracking-tight scroll-m-2">{children()}</h2> }
}

#[allow(non_camel_case_types)]
#[component]
pub fn h3(children: Children) -> impl IntoView {
    view! { <h3 class="mt-8 text-2xl font-bold tracking-tight scroll-m-2">{children()}</h3> }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[allow(non_camel_case_types)]
#[component]
pub fn a1(children: Children) -> impl IntoView {
    view! { <a class="font-medium no-underline hover:underline underline-offset-4">{children()}</a> }
}

#[allow(non_camel_case_types)]
#[component]
pub fn p1(children: Children) -> impl IntoView {
    view! { <p class="leading-7 [&:not(:first-child)]:mt-6">{children()}</p> }
}

#[allow(non_camel_case_types)]
#[component]
pub fn img1(src: &'static str) -> impl IntoView {
    view! { <img src=src class="rounded-md border" /> }
}

#[allow(non_camel_case_types)]
#[component]
pub fn blockquote1(children: Children) -> impl IntoView {
    view! {
        <blockquote class="pl-6 mt-6 italic border-l-2 [&>*]:text-muted-foreground">
            {children()}
        </blockquote>
    }
}

#[allow(non_camel_case_types)]
#[component]
pub fn pre1(children: Children) -> impl IntoView {
    view! {
        <pre class="overflow-x-auto py-4 mt-6 mb-4 rounded-lg border max-h-[650px] dark:text-neutral-100">
            {children()}
        </pre>
    }
}

#[allow(non_camel_case_types)]
#[component]
pub fn code1(children: Children) -> impl IntoView {
    view! {
        <code class="relative font-mono rounded px-[0.3rem] py-[0.2rem] text-[0.75rem] sm:text-[0.8125rem]">
            {children()}
        </code>
    }
}

#[allow(non_camel_case_types)]
#[component]
pub fn span1(children: Children) -> impl IntoView {
    // TODO 🚑 : I needed to override with dark:!text-neutral-100
    view! { <span class="dark:!text-neutral-100">{children()}</span> }
}
