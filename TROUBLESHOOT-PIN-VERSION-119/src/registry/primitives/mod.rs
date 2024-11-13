pub mod p_accordion;
pub mod p_aspect_ratio;
pub mod p_avatar;
pub mod p_checkbox;
pub mod p_collapsible;
pub mod p_label;
pub mod p_primitive;
pub mod p_progress;
pub(crate) mod p_radio;
pub mod p_radio_group;
pub mod p_scroll_area;
pub mod p_separator;
pub mod p_slider;
pub mod p_slot;
pub mod p_switch;
pub mod p_tabs;
pub mod p_toggle;
pub mod p_toggle_group;
pub mod p_toolbar;

// UTILS
pub mod p_utils_create_controllable_signal;
pub mod p_utils_create_id;
pub mod p_utils_create_previous;
pub mod p_utils_create_state_machine;

// TODO. CUSTOM
pub mod p_primitive_custom;
pub mod p_tabs_custom;
pub mod p_toggle_custom;

pub(crate) mod p_collection;
pub(crate) mod p_presence;
pub(crate) mod p_roving_focus;

#[derive(Default, Clone, PartialEq, Copy, strum_macros::Display)]
pub enum Direction {
    #[default]
    #[strum(to_string = "ltr")]
    LeftToRight,
    #[strum(to_string = "rtl")]
    RightToLeft,
}

#[derive(Default, Clone, PartialEq, Copy, strum_macros::Display)]
pub enum Orientation {
    #[default]
    #[strum(to_string = "horizontal")]
    Horizontal,
    #[strum(to_string = "vertical")]
    Vertical,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                       ✨ UTILS ✨                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub(crate) fn linear_scale(
    (input_start, input_end): (f64, f64),
    (output_start, output_end): (f64, f64),
) -> impl Fn(f64) -> f64 {
    move |value| {
        if input_start == input_end || output_start == output_end {
            return output_start;
        }

        let ratio = (output_end - output_start) / (input_end - input_start);
        output_start + ratio * (value - input_start)
    }
}

pub(crate) type AttributePair = (&'static str, leptos::Attribute);
pub(crate) type Attributes = Vec<AttributePair>;
