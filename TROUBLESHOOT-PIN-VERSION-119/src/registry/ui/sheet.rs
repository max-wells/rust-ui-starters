use leptos::*;
use tailwind_fuse::*;

use crate::registry::icons::others::x::X;
use crate::registry::ui::button::{Button, ButtonVariant};

// TODO. Improve the use of Button in SheetTrigger
// TODO. USe Heading variants from Headings

pub type SheetVariant = ButtonVariant;

//

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn SheetTrigger(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into, optional)] variant: MaybeSignal<SheetVariant>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <Button {..attributes} class=class variant=variant>
            {children()}
        </Button>
    }
}

#[component]
pub fn SheetDescription(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <p {..attributes} class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn SheetTitle(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("font-bold text-2XL", class()));

    view! {
        <h2 {..attributes} class=class>
            {children()}
        </h2>
    }
}

#[component]
pub fn SheetCancel(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into, optional)] variant: MaybeSignal<ButtonVariant>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <Button {..attributes} class=class variant=variant>
            {children()}
        </Button>
    }
}

//
// Update the SheetContent component
#[component]
pub fn SheetContent(
    #[prop(into)] is_open: MaybeSignal<bool>,
    #[prop(into, optional, default = SheetDirection::Right.into())] direction: MaybeSignal<
        SheetDirection,
    >, // Default to Right
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let outer_class = create_memo(move |_| {
        let direction = direction.get();
        tw_merge!(
            "fixed shadow-lg transform transition-transform duration-300",
            direction.initial_position(),
            direction.to_class(is_open()),
            class()
        )
    });

    let inner_class = create_memo(move |_| {
        let base_class =
            "p-4 h-screen bg-card transition-opacity duration-300 z-50   overflow-y-auto shortfix-sidenav-todo-properly";
        let opacity_class = if is_open() {
            "opacity-100"
        } else {
            "opacity-0 pointer-events-none"
        };
        tw_merge!(base_class, opacity_class)
    });

    view! {
        <div {..attributes} class=outer_class>
            <div class=inner_class>
                <button class="absolute top-0 right-0 m-2">
                    <X class="size-6" />
                </button>
                {children()}
            </div>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// Define the Direction enum
#[derive(Clone, Copy)]
pub enum SheetDirection {
    Right,
    Left,
    Top,
    Bottom,
}

// Implement a method to get the corresponding class for each direction
impl SheetDirection {
    fn to_class(self, is_open: bool) -> &'static str {
        match self {
            SheetDirection::Right => {
                if is_open {
                    "translate-x-0"
                } else {
                    "translate-x-full"
                }
            }
            SheetDirection::Left => {
                if is_open {
                    "translate-x-0"
                } else {
                    "-translate-x-full"
                }
            }
            SheetDirection::Top => {
                if is_open {
                    "translate-y-0"
                } else {
                    "-translate-y-full"
                }
            }
            SheetDirection::Bottom => {
                if is_open {
                    "translate-y-0"
                } else {
                    "translate-y-full"
                }
            }
        }
    }

    fn initial_position(self) -> &'static str {
        match self {
            SheetDirection::Right => "top-0 right-0 h-full w-64",
            SheetDirection::Left => "top-0 left-0 h-full w-64",
            SheetDirection::Top => "top-0 left-0 w-full h-64",
            SheetDirection::Bottom => "bottom-0 left-0 w-full h-64",
        }
    }
}
