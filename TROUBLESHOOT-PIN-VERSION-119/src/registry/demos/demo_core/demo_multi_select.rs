use leptos::*;
use leptos_meta::Stylesheet;

// TODO --> Switch from JS to Rust 🦀 💪

#[component]
pub fn DemoMultiSelect() -> impl IntoView {
    view! {
        <Stylesheet id="multiselect" href="/components/multi-select.css" />
        <script src="/components/multi-select.js" />
        // * 💁 For the moment, both CSS and JS files are located in public folder.
        // * Waiting to implement this fully in Rust 🦀

        <div class="mx-auto w-full max-w-2xl">
            <form action="" method="get">
                <div>
                    <label for="countries">"Countries"</label>
                    <select name="countries" id="countries" multiple>
                        <option value="1">"Afghanistan"</option>
                        <option value="2">"Australia"</option>
                        <option value="3">"Germany"</option>
                        <option value="4">"Canada"</option>
                        <option value="5">"Russia"</option>
                        <option value="6">"United Kingdom"</option>
                        <option value="7">"France"</option>
                        <option value="8">"Brazil"</option>
                        <option value="9">"Italy"</option>
                        <option value="10">"Zambia"</option>
                    </select>
                </div>
            </form>
        </div>

        <script>"new MultiSelectTag('countries');"</script>
    }
}
