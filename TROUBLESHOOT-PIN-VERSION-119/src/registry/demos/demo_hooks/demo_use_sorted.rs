use leptos::*;
use leptos_use::use_sorted;

// TODO. Input

const DEMO_VEC: [i32; 10] = [4, 2, 67, 34, 76, 22, 2, 4, 65, 23];

#[component]
pub fn DemoUseSorted() -> impl IntoView {
    let (list, set_list) = create_signal::<Vec<i32>>(DEMO_VEC.to_vec());

    let sorted: Signal<Vec<i32>> = use_sorted(list);

    let on_input = move |evt| {
        set_list.update(|list| {
            *list = event_target_value(&evt)
                .split(",")
                .map(|n| n.parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>()
        });
    };

    let input_text = move || string_list(&list());
    let sorted_text = move || string_list(&sorted());

    view! {
        <div class="flex flex-col gap-4">
            <p>Input:</p>
            <input prop:value=input_text on:input=on_input type="text" />
            <p>Output: {sorted_text}</p>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

fn string_list(list: &[i32]) -> String {
    list.iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join(", ") // Added join to convert Vec<String> to a single String
}
