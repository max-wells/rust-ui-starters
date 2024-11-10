use leptos::*;
use ev::SubmitEvent;
use html::Input;
use std::rc::Rc;

use crate::api::api_blogs::add_blog;
use crate::components::toaster_custom::toast::ToastVariant;
use crate::components::toaster_custom::types::Toasts;
use crate::components::ui::button::Button;
use crate::components::ui::headings::H2;
use crate::components::ui::input::Input;
use crate::models::model_blogs::{ AllBlogsQKey, NewBlog};
use crate::utils::hooks::queries::queries_blogs::useAllBlogs;
use crate::utils::toast_utils::{handle_error_toast, show_toast};
use web_sys::console;

// curl -X POST http://localhost:8000/blogs -H "Content-Type: application/json" -d '{"title":"TITLE","content":"CONTENT","author":"AUTHOR","image_url":"https://proedu.com/cdn/shop/articles/What-Is-Grayscale-Photshop-Skills-blog.jpg?v=1702337949&width=1400","tags":[1,2]}'

#[component]
pub fn FormBlogsToast() -> impl IntoView {
    let all_blogs_query = Rc::new(useAllBlogs());
    let toast_context = expect_context::<Toasts>();

    let (title, set_title) = create_signal("title".to_string());
    let (content, set_content) = create_signal("content".to_string());
    let (image_url, set_image_url) = create_signal("https://proedu.com/cdn/shop/articles/What-Is-Grayscale-Photshop-Skills-blog.jpg?v=1702337949&width=1400".to_string());
    let (author, set_author) = create_signal("author".to_string());
    let (tags, set_tags) = create_signal(vec![1, 2]);
    let (tags_string, set_tags_string) = create_signal("".to_string());

    let input_title: NodeRef<Input> = create_node_ref();
    let input_content: NodeRef<Input> = create_node_ref();
    let input_image_url: NodeRef<Input> = create_node_ref();
    let input_author: NodeRef<Input> = create_node_ref();
    let input_tags: NodeRef<Input> = create_node_ref();

    // TODO. Improve this with a function
    let handle_success = {
        let all_blogs_query = Rc::clone(&all_blogs_query);
        move || {
            show_toast(toast_context, ToastVariant::Success, "✅ Blog added");

            let _ = all_blogs_query.invalidate_query(AllBlogsQKey);
        }
    };




    let on_submit = {
        // let all_server_blogs_query = Rc::clone(&all_server_blogs_query);

        // TODO 🐛 This is called when just refreshing the page, not when calling the on_submit...
        // TODO 🐛 This is called when just refreshing the page, not when calling the on_submit...
        // TODO 🐛 This is called when just refreshing the page, not when calling the on_submit...
        println!("Entering the on_submit function..");

        
        move |ev: SubmitEvent| {
            ev.prevent_default();

            // TODO. We do not even get there... Why ? 🤔
            println!("🐛🐛 WE DO NOT EVEN GET THERE... WHY ? 🤔");

            let value_input_title = input_title().expect("<input> should be mounted").value();
            let value_input_content = input_content().expect("<input> should be mounted").value();
            let value_input_author = input_author().expect("<input> should be mounted").value();
            let value_input_image_url = input_image_url().expect("<input> should be mounted").value();
            let value_input_tags = input_tags().expect("<input> should be mounted").value();
            let parsed_tags: Vec<u32> = value_input_tags
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            set_tags(parsed_tags.clone());

            set_title(value_input_title.clone());
            set_content(value_input_content.clone());
            set_author(value_input_author.clone());
            set_image_url(value_input_image_url.clone());

            let new_blog = NewBlog {
                title: value_input_title.clone(),
                content: value_input_content.clone(),
                author: value_input_author.clone(),
                image_url: value_input_image_url.clone(),
                tags: parsed_tags,
            };

            // TODO 🐛 This never prints...
            println!("[FORM] --> new_blog: {:?}", new_blog);

            let new_blog_json = serde_json::to_string(&new_blog).unwrap_or_else(|_| "Failed to serialize new_blog".to_string());
            console::warn_1(&new_blog_json.into());

            set_tags_string(value_input_tags.clone());

            wasm_bindgen_futures::spawn_local({
                let clone_handle_success = handle_success.clone();
                async move {
                    match add_blog(new_blog).await {
                        Ok(_server_blog) => clone_handle_success(),
                        Err(_err) => handle_error_toast(toast_context),
                    }
                }
            });
        }
    };

    view! {
        <form on:submit=on_submit class="flex flex-col gap-2 p-1 w-full border max-w-[520px]">
            <H2>"Simple (Toast)"</H2>

            <div class="flex flex-row gap-2 items-center">
                <p class="w-[140px]">"TITLE"</p>
                <Input value=Some(title) node_ref=input_title />
            </div>
            <div class="flex flex-row gap-2 items-center">
                <p class="w-[140px]">"CONTENT"</p>
                <Input value=Some(content) node_ref=input_content />
            </div>
            <div class="flex flex-row gap-2 items-center">
                <p class="w-[140px]">"AUTHOR"</p>
                <Input value=Some(author) node_ref=input_author />
            </div>
            <div class="flex flex-row gap-2 items-center">
                <p class="w-[140px]">"IMAGE URL"</p>
                <Input value=Some(image_url) node_ref=input_image_url />
            </div>
            <div class="flex flex-row gap-2 items-center">
                <p class="w-[140px]">"TAGS"</p>
                <Input value=Some(tags_string) node_ref=input_tags />
            </div>

            <Button r#type="submit">"Submit"</Button>
        </form>
    }
}

