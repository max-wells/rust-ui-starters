use leptos::*;
use tailwind_fuse::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn Table(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("w-full max-w-7xl text-sm caption-bottom", class()));

    view! {
        <table {..attributes} class=class>
            {children()}
        </table>
    }
}

#[component]
pub fn TableCaption(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("mt-4 text-sm text-muted-foreground", class()));

    view! {
        <caption {..attributes} class=class>
            {children()}
        </caption>
    }
}

#[component]
pub fn TableHeader(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("[&amp;_tr]:border-b", class()));

    view! {
        <thead {..attributes} class=class>
            {children()}
        </thead>
    }
}

#[component]
pub fn TableRow(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "border border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50",
            class()
        )
    });

    view! {
        <tr {..attributes} class=class>
            {children()}
        </tr>
    }
}

#[component]
pub fn TableHead(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "px-4 h-12 font-medium text-left align-middle text-muted-foreground [&amp;:has([role=checkbox])]:pr-0",
            class()
        )
    });

    view! {
        <th {..attributes} class=class>
            {children()}
        </th>
    }
}

#[component]
pub fn TableBody(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <tbody {..attributes} class=class>
            {children()}
        </tbody>
    }
}

#[component]
pub fn TableCell(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "p-4 align-middle [&amp;:has([role=checkbox])]:pr-0",
            class()
        )
    });

    view! {
        <td {..attributes} class=class>
            {children()}
        </td>
    }
}

#[component]
pub fn TableFooter(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "font-medium border border-t bg-muted/50 [&amp;&gt;tr]:last:border-b-0",
            class()
        )
    });

    view! {
        <tfoot {..attributes} class=class>
            {children()}
        </tfoot>
    }
}
