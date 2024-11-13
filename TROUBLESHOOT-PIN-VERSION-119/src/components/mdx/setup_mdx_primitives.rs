use crate::{
    __registry__::mdx_demos_primitives::{
        mdx_demo_aspect_ratio::MdxDemoAspectRatio,
        mdx_demo_primitive_label::MdxDemoPrimitiveLabel,
        mdx_demo_tabs::MdxDemoTabs,
        mdx_demo_slider::MdxDemoSlider,
        mdx_demo_scroll_area::MdxDemoScrollArea,
        mdx_demo_primitive_checkbox::MdxDemoPrimitiveCheckbox,
        mdx_demo_collapsible::MdxDemoCollapsible,
        mdx_demo_toolbar::MdxDemoToolbar,
        mdx_demo_toggle::MdxDemoToggle,
        mdx_demo_avatar::MdxDemoAvatar,
        mdx_demo_switch::MdxDemoSwitch,
        mdx_demo_accordion::MdxDemoAccordion,
        mdx_demo_radio_group::MdxDemoRadioGroup,
        mdx_demo_toggle_group::MdxDemoToggleGroup,
        mdx_demo_tabs_custom::MdxDemoTabsCustom,
        mdx_demo_toggle_custom::MdxDemoToggleCustom,
        mdx_demo_progress::MdxDemoProgress,
    },
    utils::mdx::mdx_leptos::Components,
};

pub fn setup_mdx_primitives() -> Components {
    let mut components = Components::new();

    components.add("MdxDemoAspectRatio".into(), MdxDemoAspectRatio);
    components.add("MdxDemoPrimitiveLabel".into(), MdxDemoPrimitiveLabel);
    components.add("MdxDemoTabs".into(), MdxDemoTabs);
    components.add("MdxDemoSlider".into(), MdxDemoSlider);
    components.add("MdxDemoScrollArea".into(), MdxDemoScrollArea);
    components.add("MdxDemoPrimitiveCheckbox".into(), MdxDemoPrimitiveCheckbox);
    components.add("MdxDemoCollapsible".into(), MdxDemoCollapsible);
    components.add("MdxDemoToolbar".into(), MdxDemoToolbar);
    components.add("MdxDemoToggle".into(), MdxDemoToggle);
    components.add("MdxDemoAvatar".into(), MdxDemoAvatar);
    components.add("MdxDemoSwitch".into(), MdxDemoSwitch);
    components.add("MdxDemoAccordion".into(), MdxDemoAccordion);
    components.add("MdxDemoRadioGroup".into(), MdxDemoRadioGroup);
    components.add("MdxDemoToggleGroup".into(), MdxDemoToggleGroup);
    components.add("MdxDemoTabsCustom".into(), MdxDemoTabsCustom);
    components.add("MdxDemoToggleCustom".into(), MdxDemoToggleCustom);
    components.add("MdxDemoProgress".into(), MdxDemoProgress);

    components
}
