use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoGalleryClickableTransitions() -> impl IntoView {
    view! {
        <Stylesheet
            id="gallery-clickable-transitions"
            href="/components/gallery-clickable-transitions.css"
        />
        <script src="/components/gallery-clickable-transitions.js" />

        <div class="p-2 max-w-xl border border-sky-500">

            <div class="full-height color-scheme no-view-transition">
                <div class="full-height centered-grid">
                    <fieldset id="gallery" class="hub no-border">
                        <div class="gallery-item box-sizing">
                            <input
                                type="radio"
                                id="image-1"
                                name="gallery"
                                value="image-1"
                                checked
                            />
                            <img
                                class="max-inline-size"
                                src="https://assets.codepen.io/2585/image_fx_a_black_wolf_in_armor_wrecking_a_steampunk_ci.jpg"
                                alt=""
                            />
                            <label for="image-1">Cyber Wolf</label>
                        </div>

                        <div class="gallery-item box-sizing">
                            <input type="radio" id="image-2" name="gallery" value="image-2" />
                            <img
                                class="max-inline-size"
                                src="https://assets.codepen.io/2585/image_fx_cyberpunk_city_scene_with_neon_streaks_of_lig+%283%29.jpg"
                                alt=""
                            />
                            <label for="image-2">Flying cars</label>
                        </div>

                        <div class="gallery-item box-sizing">
                            <input type="radio" id="image-3" name="gallery" value="image-3" />
                            <img
                                class="max-inline-size"
                                src="https://assets.codepen.io/2585/image_fx_cyberpunk_city_scene_with_neon_streaks_of_lig+%282%29.jpg"
                                alt=""
                            />
                            <label for="image-3">Flying cars 2</label>
                        </div>

                        <div class="gallery-item box-sizing">
                            <input type="radio" id="image-4" name="gallery" value="image-4" />
                            <img
                                class="max-inline-size"
                                src="https://assets.codepen.io/2585/image_fx_cyberpunk_city_scene_with_neon_streaks_of_lig+%281%29.jpg"
                                alt=""
                            />
                            <label for="image-4">Flying cars 3</label>
                        </div>

                        <div class="gallery-item box-sizing">
                            <input type="radio" id="image-5" name="gallery" value="image-5" />
                            <img
                                class="max-inline-size"
                                src="https://assets.codepen.io/2585/image_fx_cyberpunk_city_scene_with_neon_streaks_of_lig+%284%29.jpg"
                                alt=""
                            />
                            <label for="image-5">Cyber T-Rex</label>
                        </div>

                        <div class="gallery-item box-sizing">
                            <input type="radio" id="image-6" name="gallery" value="image-6" />
                            <img
                                class="max-inline-size"
                                src="https://assets.codepen.io/2585/image_fx_a_raptor_in_armor_wrecking_a_cyberbunk_city_w.jpg"
                                alt=""
                            />
                            <label for="image-6">Cyber Raptor</label>
                        </div>

                        <div class="gallery-item box-sizing">
                            <input type="radio" id="image-7" name="gallery" value="image-7" />
                            <img
                                class="max-inline-size"
                                src="https://assets.codepen.io/2585/image_fx_cyberpunk_city_scene_with_neon_streaks_of_lig.jpg"
                                alt=""
                            />
                            <label for="image-7">Cyber freeway</label>
                        </div>
                    </fieldset>
                </div>
            </div>

        </div>
    }
}
