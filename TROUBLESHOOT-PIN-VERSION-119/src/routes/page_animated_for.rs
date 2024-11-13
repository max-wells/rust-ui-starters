use leptos::*;
use leptos_animated_for::AnimatedFor;
use rand::{seq::SliceRandom, thread_rng, Rng};

const INITIAL_ITEM_COUNT: usize = 40;

#[component]
pub fn PageAnimatedFor() -> impl IntoView {
    let next_item = StoredValue::new(INITIAL_ITEM_COUNT);

    let items = RwSignal::new((0..INITIAL_ITEM_COUNT).collect::<Vec<_>>());

    let create_new_item = Callback::new(move |()| {
        let item = next_item();
        next_item.set_value(item + 1);
        item
    });

    let rng = StoredValue::new(thread_rng());

    let shuffle = move |_| {
        items.update(|items| {
            rng.update_value(|rng| {
                items.shuffle(rng);
            });
        });
    };

    let add_start = move |_| {
        update!(|items| {
            items.insert(0, create_new_item(()));
        });
    };

    let add_end = move |_| {
        update!(|items| {
            items.push(create_new_item(()));
        });
    };

    let add_random = move |_| {
        items.update(|items| {
            let item = create_new_item(());
            rng.update_value(|rng| {
                items.insert(rng.gen_range(0..items.len()), item);
            });
        });
    };

    let remove_start = move |_| {
        update!(|items| {
            if items.is_empty() {
                return;
            }

            items.remove(0);
        });
    };

    let remove_end = move |_| {
        update!(|items| {
            items.pop();
        });
    };

    let remove_random = move |_| {
        items.update(|items| {
            if items.is_empty() {
                return;
            }

            rng.update_value(|rng| {
                items.remove(rng.gen_range(0..items.len()));
            });
        });
    };

    view! {
        <main class="py-8 min-h-screen font-bold text-white">
            <div class="flex flex-wrap gap-2 justify-center">
                <Button on:click=add_start>{"Add start"}</Button>
                <Button on:click=add_end>{"Add end"}</Button>
                <Button on:click=add_random>{"Add random"}</Button>
                <Button on:click=remove_start>{"Remove start"}</Button>
                <Button on:click=remove_end>{"Remove end"}</Button>
                <Button on:click=remove_random>{"Remove random"}</Button>
                <Button on:click=shuffle>{"Shuffle"}</Button>
            </div>

            <div class="grid gap-2 justify-center mt-4 text-xl grid-cols-[repeat(5,auto)] [&>*]:w-16 [&>*]:h-16">
                <AnimatedFor
                    each=items
                    key=|item| *item
                    children=|item| {
                        view! {
                            <div class="flex justify-center items-center rounded bg-blue-500/70">
                                {item}
                            </div>
                        }
                    }

                    enter_from_class="opacity-0"
                    enter_class="duration-1000"
                    move_class="duration-1000"
                    leave_class="opacity-0 duration-1000"
                />

            </div>
        </main>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
fn Button(children: Children) -> impl IntoView {
    view! {
        <button class="py-1 px-4 bg-purple-500 rounded transition cursor-pointer select-none hover:bg-purple-600">
            {children()}
        </button>
    }
}
