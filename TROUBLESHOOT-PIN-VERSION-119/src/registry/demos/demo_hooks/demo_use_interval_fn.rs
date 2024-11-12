use leptos::*;

#[cfg(feature = "hydrate")]
use crate::registry::ui::button::Button;
#[cfg(feature = "hydrate")]
use leptos_use::use_interval_fn;
#[cfg(feature = "hydrate")]
use leptos_use::utils::Pausable;

// TODO 🪝. Input

#[component]
pub fn DemoUseIntervalFn() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let interval_component = {
        let greetings = [
            "Hello",
            "Hi",
            "Yo!",
            "Hey",
            "Hola",
            "こんにちは",
            "Bonjour",
            "Salut!",
            "你好",
            "Привет",
        ];

        let (word, set_word) = create_signal(greetings[0]);
        let (interval, set_interval) = create_signal(500_u64);
        let (index, set_index) = create_signal(0);

        let Pausable {
            pause,
            resume,
            is_active,
        } = use_interval_fn(
            move || {
                set_index.set((index.get() + 1) % greetings.len());
                set_word.set(greetings[index.get()]);
            },
            interval,
        );

        view! {
            <p>{move || word.get()}</p>
            <p>
                "Interval:"
                <input
                    prop:value=move || interval.get()
                    on:input=move |e| set_interval.set(event_target_value(&e).parse().unwrap())
                    type="number"
                    placeholder="interval"
                />
            </p>

            <Show
                when=move || is_active.get()
                fallback=move || {
                    let resume = resume.clone();
                    view! { <Button on:click=move |_| resume()>"Resume"</Button> }
                }
            >
                {
                    let pause = pause.clone();
                    view! { <Button on:click=move |_| pause()>"Pause"</Button> }
                }
            </Show>
        }
    };

    #[cfg(not(feature = "hydrate"))]
    let interval_component = view! {
        <p>
            "Interval functionality is not available on the server. This component requires client-side hydration."
        </p>
    };

    view! { <div>{interval_component}</div> }
}
