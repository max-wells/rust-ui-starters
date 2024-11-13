use leptos::html::Input;
use leptos::*;
use leptos_use::{watch_pausable, WatchPausableReturn};

// TODO 🪝💪 Improve this later (issue with Button props)
// TODO later: Remove #[allow(unused_variables)]

#[allow(unused_variables)]
#[component]
pub fn DemoUseWatchPausable() -> impl IntoView {
    let input = create_node_ref::<Input>();
    let (log, set_log) = create_signal("".to_string());
    let (source, set_source) = create_signal("".to_string());

    let WatchPausableReturn {
        pause,
        resume,
        is_active,
        ..
    } = watch_pausable(
        move || source.get(),
        move |v, _, _| {
            set_log.update(|log| *log = format!("{log}Changed to \"{v}\"\n"));
        },
    );

    let clear = move |_| set_log.set("".to_string());

    let pause = move |_| {
        set_log.update(|log| *log = format!("{log}Paused\n"));
        pause();
    };

    let resume = move |_| {
        set_log.update(|log| *log = format!("{log}Resumed\n"));
        resume();
    };

    view! {
        <div class="flex flex-col gap-4">
            <p class="mb-2">"Type something below to trigger the watch"</p>
            <input
                node_ref=input
                class="block"
                prop:value=move || source.get()
                on:input=move |e| set_source.set(event_target_value(&e))
                type="text"
            />
            <p>"Value: " {source}</p>
            <button prop:disabled=move || !is_active.get() class="orange" on:click=pause>
                "Pause"
            </button>
            <button prop:disabled=move || is_active.get() on:click=resume>
                "Resume"
            </button>
            <button on:click=clear>"Clear Log"</button>
            <br />
            <br />
            <p>"Log"</p>
            <pre>{log}</pre>

        </div>
    }
}
