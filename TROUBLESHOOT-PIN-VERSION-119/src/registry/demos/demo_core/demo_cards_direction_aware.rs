use leptos::*;

use crate::registry::ui::button::Button;

#[component]
pub fn DemoCardsDirectionAware() -> impl IntoView {
    view! {
        <section class="flex justify-center content-center">
            <div class="max-w-5xl">
                <div class="services">
                    <ul class="grid grid-cols-3 p-0 m-0 list-none">
                        <li class="relative p-4 cursor-pointer card card-active">
                            <a href="" class="flex flex-col gap-4 items-start no-underline">
                                <h2>Service 1</h2>
                                <p>
                                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                                </p>
                                <Button>Lorem Ipsum</Button>
                            </a>
                        </li>
                        <li class="relative p-4 cursor-pointer card">
                            <a href="" class="flex flex-col gap-4 items-start no-underline">
                                <h2>Service 2</h2>
                                <p>
                                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                                </p>
                                <Button>Lorem Ipsum</Button>
                            </a>
                        </li>
                        <li class="relative p-4 cursor-pointer card">
                            <a href="" class="flex flex-col gap-4 items-start no-underline">
                                <h2>Service 3</h2>
                                <p>
                                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                                </p>
                                <Button>Lorem Ipsum</Button>
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </section>
    }
}
