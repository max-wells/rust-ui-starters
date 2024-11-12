use leptos::*;

pub fn use_lock_body_scroll(initial_locked: bool) -> (Signal<bool>, SignalSetter<bool>) {
    let (locked, set_locked) = create_signal(initial_locked);

    create_effect(move |_| {
        if locked.get() {
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .style()
                .set_property("overflow", "hidden")
                .unwrap();
        } else {
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .style()
                .remove_property("overflow")
                .unwrap();
        }
    });

    (locked.into(), set_locked.into())
}
