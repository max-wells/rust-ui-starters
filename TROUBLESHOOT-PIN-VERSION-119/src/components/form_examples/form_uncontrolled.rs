use leptos::*;

use crate::registry::ui::{button::Button, headings::H1};

#[component]
pub fn FormUncontrolled() -> impl IntoView {
    let (title, set_title) = create_signal("TITLE".to_string());
    let (author, set_author) = create_signal("AUTHOR".to_string());

    let input_title: NodeRef<html::Input> = create_node_ref();
    let input_author: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let value_input_title = input_title().expect("<input> should be mounted").value();
        let value_input_author = input_author().expect("<input> should be mounted").value();

        set_title(value_input_title);
        set_author(value_input_author);
    };

    view! {
        <div class="p-4 border border-sky-500 max-w-[600px]">

            <H1>"Form Uncontrolled"</H1>

            <form on:submit=on_submit>
                <input type="text" value=title node_ref=input_title />
                <input type="text" value=author node_ref=input_author />
                <Button r#type="submit">"Submit"</Button>
            </form>

            <div>
                <p>"Title is: " {title}</p>
                <p>"Author is: " {author}</p>
            </div>
        </div>
    }
}
