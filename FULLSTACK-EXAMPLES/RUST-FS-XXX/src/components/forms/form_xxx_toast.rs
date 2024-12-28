use leptos::*;
use ev::SubmitEvent;
use html::Input;
use std::rc::Rc;

use crate::api::api_xxxs::add_xxx;
use crate::components::toaster_custom::toast::ToastVariant;
use crate::components::toaster_custom::types::Toasts;
use crate::components::ui::button::Button;
use crate::components::ui::headings::H2;
use crate::components::ui::input::Input;
use crate::models::model_xxxs::{ TagAllXxxs, NewXxx};
use crate::utils::hooks::queries::queries_xxxs::useAllXxxs;
use crate::utils::toast_utils::{handle_error_toast, show_toast};



#[component]
pub fn FormXxxToast() -> impl IntoView {
    let all_xxxs_query = Rc::new(useAllXxxs());
    let toast_context = expect_context::<Toasts>();
    
    let (field_one, set_field_one) = create_signal("field_one".to_string());
    let (field_two, set_field_two) = create_signal("field_two".to_string());

    let input_field_one: NodeRef<Input> = create_node_ref();
    let input_field_two: NodeRef<Input> = create_node_ref();

    // TODO. Improve this with a function
    let handle_success = {
        let all_xxxs_query = Rc::clone(&all_xxxs_query);
        move || {
            show_toast(toast_context, ToastVariant::Success, "✅ Xxx added");

            let _ = all_xxxs_query.invalidate_query(TagAllXxxs);
        }
    };

    let on_submit = {
        // let all_server_xxxs_query = Rc::clone(&all_server_xxxs_query);
        move |ev: SubmitEvent| {
            ev.prevent_default();

            let value_input_field_one = input_field_one().expect("<input> should be mounted").value();
            let value_input_field_two = input_field_two().expect("<input> should be mounted").value();

            set_field_one(value_input_field_one.clone());
            set_field_two(value_input_field_two.clone());

            let new_xxx = NewXxx {
                field_one: value_input_field_one.clone(),
                field_two: value_input_field_two.clone(),
            };

            // Call the async function to add the xxx to the server
            wasm_bindgen_futures::spawn_local({
                let handle_success = handle_success.clone();
                // let toast_context = toast_context;
                async move {
                    match add_xxx(new_xxx).await {
                        Ok(_server_xxx) => handle_success(),
                        Err(_err) => handle_error_toast(toast_context),
                    }
                }
            });
        }
    };

    view! {
        <form on:submit=on_submit class="flex flex-col gap-2 p-1 border max-w-[320px]">
            <H2>"Simple (Toast)"</H2>

            <Input value=Some(field_one) node_ref=input_field_one />
            <Input value=Some(field_two) node_ref=input_field_two />
            <Button r#type="submit">"Submit"</Button>
        </form>
    }
}

