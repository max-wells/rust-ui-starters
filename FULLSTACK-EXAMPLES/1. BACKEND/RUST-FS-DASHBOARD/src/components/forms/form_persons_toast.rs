use chrono::NaiveDate;
use leptos::*;
use ev::SubmitEvent;
use html::Input;
use std::rc::Rc;

use crate::api::api_persons::add_person;
use crate::components::toaster_custom::toast::ToastVariant;
use crate::components::toaster_custom::types::Toasts;
use crate::components::ui::button::Button;
use crate::components::ui::headings::H2;
use crate::components::ui::input::Input;
use crate::models::model_persons::{ AllPersonsTag, NewPerson};
use crate::utils::hooks::queries::queries_persons::useAllPersons;
use crate::utils::toast_utils::{handle_error_toast, show_toast};



#[component]
pub fn FormPersonsToast() -> impl IntoView {
    let all_persons_query = Rc::new(useAllPersons());
    let toast_context = expect_context::<Toasts>();
    
    let (title, set_title) = create_signal("TITLE".to_string());
    let (name, set_name) = create_signal("NAME".to_string());
    let (level, set_level) = create_signal("LEVEL".to_string());
    let (compensation, set_compensation) = create_signal(0);
    let (joined_date, set_joined_date) = create_signal(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());

    let (compensation_str, set_compensation_str) = create_signal(compensation().to_string());
    let (joined_date_str, set_joined_date_str) = create_signal(joined_date().format("%Y-%m-%d").to_string());

    // Update the string signals whenever the original signals change
    create_effect(move |_| {
        set_compensation_str(compensation().to_string());
        set_joined_date_str(joined_date().format("%Y-%m-%d").to_string());
    });

    let input_title: NodeRef<Input> = create_node_ref();
    let input_name: NodeRef<Input> = create_node_ref();
    let input_level: NodeRef<Input> = create_node_ref();
    let input_compensation: NodeRef<Input> = create_node_ref();
    let input_joined_date: NodeRef<Input> = create_node_ref();

    // TODO. Improve this with a function
    let handle_success = {
        let all_persons_query = Rc::clone(&all_persons_query);
        move || {
            show_toast(toast_context, ToastVariant::Success, "✅ Person added");

            let _ = all_persons_query.invalidate_query(AllPersonsTag);
        }
    };

    let on_submit = {
        // let all_server_persons_query = Rc::clone(&all_server_persons_query);
        move |ev: SubmitEvent| {
            ev.prevent_default();

            let value_input_title = input_title().expect("<input> should be mounted").value();
            let value_input_name = input_name().expect("<input> should be mounted").value();
            let value_input_level = input_level().expect("<input> should be mounted").value();
            let value_input_compensation = input_compensation().expect("<input> should be mounted").value();
            let value_input_joined_date = input_joined_date().expect("<input> should be mounted").value();

            set_title(value_input_title.clone());
            set_name(value_input_name.clone());
            set_level(value_input_level.clone());
            set_compensation(value_input_compensation.parse::<i32>().unwrap());
            set_joined_date(value_input_joined_date.parse::<NaiveDate>().unwrap());

            let new_person = NewPerson {
                title: value_input_title.clone(),
                name: value_input_name.clone(),
                level: value_input_level.clone(),
                compensation: value_input_compensation.parse::<i32>().unwrap(),
                joined_date: value_input_joined_date.parse::<NaiveDate>().unwrap(),
            };

            // Call the async function to add the person to the server
            wasm_bindgen_futures::spawn_local({
                let handle_success = handle_success.clone();
                // let toast_context = toast_context;
                async move {
                    match add_person(new_person).await {
                        Ok(_server_person) => handle_success(),
                        Err(_err) => handle_error_toast(toast_context),
                    }
                }
            });
        }
    };

    view! {
        <form on:submit=on_submit class="flex flex-col gap-2 p-1 border max-w-[320px]">
            <H2>"Simple (Toast)"</H2>

            <Input value=Some(title) node_ref=input_title />
            <Input value=Some(name) node_ref=input_name />
            <Input value=Some(level) node_ref=input_level />
            <Input value=Some(compensation_str) node_ref=input_compensation />
            <Input value=Some(joined_date_str) node_ref=input_joined_date />

            <Button r#type="submit">"Submit"</Button>
        </form>
    }
}

