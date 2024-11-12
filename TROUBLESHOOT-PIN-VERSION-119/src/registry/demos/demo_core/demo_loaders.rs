use leptos::*;

use crate::registry::ui::card::Card;

// SOURCE: https://codepen.io/robstinson/pen/QWBRevm (some are failing)

#[component]
pub fn DemoLoaders() -> impl IntoView {
    let LOADERS: Vec<Box<dyn Fn() -> View>> = vec![
        Box::new(|| view! { <Loader1 /> }),
        Box::new(|| view! { <Loader2 /> }),
        Box::new(|| view! { <Loader3 /> }),
        Box::new(|| view! { <Loader4 /> }),
        Box::new(|| view! { <Loader5 /> }),
        Box::new(|| view! { <Loader6 /> }),
        Box::new(|| view! { <Loader7 /> }),
        Box::new(|| view! { <Loader8 /> }),
        Box::new(|| view! { <Loader9 /> }),
        Box::new(|| view! { <Loader10 /> }),
    ];

    view! {
        <div class="flex flex-col justify-center items-center p-4 w-screen">
            <div class="flex flex-wrap gap-4 justify-center w-full max-w-screen-md">
                {LOADERS
                    .into_iter()
                    .map(|loader| {
                        view! {
                            <Card class="flex justify-center items-center size-32">{loader()}</Card>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
fn Loader1() -> impl IntoView {
    view! {
        <div class="w-10 h-10 rounded-full border-4 animate-spin border-y-primary border-l-primary" />
    }
}

#[component]
fn Loader2() -> impl IntoView {
    view! {
        <div class="flex relative justify-center items-center">
            <div class="absolute w-6 h-6 rounded-full border-2 animate-ping border-primary" />
            <div
                class="absolute w-4 h-4 rounded-full border-2 animate-ping border-primary"
                style="animation-delay: .1s;"
            />
            <div
                class="absolute w-2 h-2 rounded-full border-2 animate-ping border-primary"
                style="animation-delay: .2s;"
            />
        </div>
    }
}

#[component]
fn Loader3() -> impl IntoView {
    view! {
        <div
            class="flex relative justify-center items-center animate-spin"
            style="animation-duration: 3s;"
        >
            <div class="flex absolute flex-col justify-between h-10 rotate-0">
                <div class="w-1 h-3 rounded-full bg-primary" />
                <div class="w-1 h-3 rounded-full bg-primary" />
            </div>
            <div class="flex absolute flex-col justify-between h-10 rotate-45">
                <div class="w-1 h-3 rounded-full bg-primary" />
                <div class="w-1 h-3 rounded-full bg-primary" />
            </div>
            <div class="flex absolute flex-col justify-between h-10 rotate-90">
                <div class="w-1 h-3 rounded-full bg-primary" />
                <div class="w-1 h-3 rounded-full bg-primary" />
            </div>
            <div class="flex absolute flex-col justify-between h-10 -rotate-45">
                <div class="w-1 h-3 rounded-full bg-primary" />
                <div class="w-1 h-3 rounded-full bg-primary" />
            </div>
        </div>
    }
}

#[component]
fn Loader4() -> impl IntoView {
    view! {
        <div class="flex relative justify-center items-center">
            <div class="absolute w-10 h-10 rounded-full border-4 animate-spin border-primary border-l-primary" />
            <div
                class="absolute w-7 h-7 rounded-full border-4 animate-spin border-primary border-r-primary"
                style="animation-direction: reverse;"
            />
        </div>
    }
}

#[component]
fn Loader5() -> impl IntoView {
    view! {
        <div class="flex relative gap-2 justify-center items-center">
            <div class="w-2 h-2 rounded-full animate-ping bg-primary" />
            <div
                class="w-2 h-2 rounded-full animate-ping bg-primary"
                style="animation-delay: .33s;"
            />
            <div
                class="w-2 h-2 rounded-full animate-ping bg-primary"
                style="animation-delay: .66s;"
            />
        </div>
    }
}

#[component]
fn Loader6() -> impl IntoView {
    view! {
        <div
            class="flex relative justify-center items-center animate-spin"
            style="animation-duration: 3s;"
        >
            <div class="flex absolute flex-col justify-between h-8 rotate-0">
                <div class="w-1 h-1 rounded-full bg-primary" />
                <div class="w-1 h-1 rounded-full bg-primary" />
            </div>
            <div class="flex absolute flex-col justify-between h-8 rotate-45">
                <div class="w-1 h-1 rounded-full bg-primary" />
                <div class="w-1 h-1 rounded-full bg-primary" />
            </div>
            <div class="flex absolute flex-col justify-between h-8 rotate-90">
                <div class="w-1 h-1 rounded-full bg-primary" />
                <div class="w-1 h-1 rounded-full bg-primary" />
            </div>
            <div class="flex absolute flex-col justify-between h-8 -rotate-45">
                <div class="w-1 h-1 rounded-full bg-primary" />
                <div class="w-1 h-1 rounded-full bg-primary" />
            </div>
        </div>
    }
}

#[component]
fn Loader7() -> impl IntoView {
    view! {
        <div class="flex relative gap-2 justify-center items-center">
            <div class="w-2 h-2 rounded-full animate-bounce bg-primary" />
            <div
                class="w-2 h-2 rounded-full animate-bounce bg-primary"
                style="animation-delay: .33s;"
            />
            <div
                class="w-2 h-2 rounded-full animate-bounce bg-primary"
                style="animation-delay: .66s;"
            />
        </div>
    }
}

#[component]
fn Loader8() -> impl IntoView {
    view! {
        <div class="w-10 h-10 rounded-full animate-spin bg-primary">
            <div class="flex justify-center items-center w-10 h-10 bg-white rounded-full skew-x-12">
                <div class="w-8 h-8 rounded-full animate-spin bg-primary">
                    <div class="flex justify-center items-center w-8 h-8 bg-white rounded-full rotate-90 skew-y-12" />
                </div>
            </div>
        </div>
    }
}

#[component]
fn Loader9() -> impl IntoView {
    view! {
        <div class="flex relative justify-center items-center">
            <div class="flex absolute justify-center items-center w-10 h-10 rounded-full border-4 animate-spin border-primary border-l-primary">
                <div class="flex absolute justify-center items-center w-7 h-7 rounded-full border-4 animate-spin border-primary border-r-primary" />
            </div>
        </div>
    }
}

#[component]
fn Loader10() -> impl IntoView {
    view! {
        <div class="relative w-8 h-8">
            <div class="absolute w-full h-full animate-spin">
                <div class="absolute top-0 left-0 w-2 h-2 rounded-full bg-primary" />
                <div class="absolute top-0 right-0 w-2 h-2 rounded-full bg-primary" />
                <div class="absolute right-0 bottom-0 w-2 h-2 rounded-full bg-primary" />
                <div class="absolute bottom-0 left-0 w-2 h-2 rounded-full bg-primary" />
            </div>
            <div class="absolute w-full h-full animate-spin" style="animation-direction: reverse;">
                <div class="absolute top-1 left-1 w-1 h-1 rounded-full bg-primary" />
                <div class="absolute top-1 right-1 w-1 h-1 rounded-full bg-primary" />
                <div class="absolute right-1 bottom-1 w-1 h-1 rounded-full bg-primary" />
                <div class="absolute bottom-1 left-1 w-1 h-1 rounded-full bg-primary" />
            </div>
        </div>
    }
}
