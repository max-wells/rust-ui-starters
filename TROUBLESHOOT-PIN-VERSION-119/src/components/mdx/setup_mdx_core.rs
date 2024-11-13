use crate::{
    __registry__::mdx_demos_core::{
        mdx_demo_navigation_menu::MdxDemoNavigationMenu,
        mdx_demo_button_reactive::MdxDemoButtonReactive,
        mdx_demo_orbiting_circles::MdxDemoOrbitingCircles,
        mdx_demo_button_sizes::MdxDemoButtonSizes,
        mdx_demo_command::MdxDemoCommand,
        mdx_demo_badge::MdxDemoBadge,
        mdx_demo_dropzone::MdxDemoDropzone,
        mdx_demo_blurry_blob::MdxDemoBlurryBlob,
        mdx_demo_meteor_effect::MdxDemoMeteorEffect,
        mdx_demo_floating_button_menu::MdxDemoFloatingButtonMenu,
        mdx_demo_button::MdxDemoButton,
        mdx_demo_radar::MdxDemoRadar,
        mdx_demo_marquee::MdxDemoMarquee,
        mdx_demo_command_dialog::MdxDemoCommandDialog,
        mdx_demo_headings::MdxDemoHeadings,
        mdx_demo_badge_variants::MdxDemoBadgeVariants,
        mdx_demo_skeleton_image::MdxDemoSkeletonImage,
        mdx_demo_navigation_menu_complex::MdxDemoNavigationMenuComplex,
        mdx_demo_multi_select::MdxDemoMultiSelect,
        mdx_demo_animate::MdxDemoAnimate,
        mdx_demo_bento_grid4::MdxDemoBentoGrid4,
        mdx_demo_headings_variants::MdxDemoHeadingsVariants,
        mdx_demo_tooltip::MdxDemoTooltip,
        mdx_demo_status::MdxDemoStatus,
        mdx_demo_button_variants::MdxDemoButtonVariants,
        mdx_demo_slider_hover::MdxDemoSliderHover,
        mdx_demo_parallax_simple::MdxDemoParallaxSimple,
        mdx_demo_loaders::MdxDemoLoaders,
        mdx_demo_spotlight::MdxDemoSpotlight,
        mdx_demo_dropdown_menu::MdxDemoDropdownMenu,
        mdx_demo_alert_dialog::MdxDemoAlertDialog,
        mdx_demo_separator::MdxDemoSeparator,
        mdx_demo_breadcrumb::MdxDemoBreadcrumb,
        mdx_demo_skeleton::MdxDemoSkeleton,
        mdx_demo_select::MdxDemoSelect,
        mdx_demo_bento_grid5::MdxDemoBentoGrid5,
        mdx_demo_cards_direction_aware::MdxDemoCardsDirectionAware,
        mdx_demo_gradient::MdxDemoGradient,
        mdx_demo_carousel_parallax::MdxDemoCarouselParallax,
        mdx_demo_button_override::MdxDemoButtonOverride,
        mdx_demo_sheet_directions::MdxDemoSheetDirections,
        mdx_demo_background_retro::MdxDemoBackgroundRetro,
        mdx_demo_alert::MdxDemoAlert,
        mdx_demo_bento_grid6::MdxDemoBentoGrid6,
        mdx_demo_card::MdxDemoCard,
        mdx_demo_expandable::MdxDemoExpandable,
        mdx_demo_card_tilting::MdxDemoCardTilting,
        mdx_demo_headings_animate::MdxDemoHeadingsAnimate,
        mdx_demo_tailwind_scroll_only::MdxDemoTailwindScrollOnly,
        mdx_demo_table::MdxDemoTable,
        mdx_demo_status_variants::MdxDemoStatusVariants,
        mdx_demo_text_area::MdxDemoTextArea,
        mdx_demo_card3d_hover::MdxDemoCard3dHover,
        mdx_demo_headings_motion::MdxDemoHeadingsMotion,
        mdx_demo_charts_many::MdxDemoChartsMany,
        mdx_demo_beam_border::MdxDemoBeamBorder,
        mdx_demo_text_slide::MdxDemoTextSlide,
        mdx_demo_input::MdxDemoInput,
        mdx_demo_checkbox::MdxDemoCheckbox,
        mdx_demo_label::MdxDemoLabel,
        mdx_demo_hamburger_menus::MdxDemoHamburgerMenus,
        mdx_demo_popover_modal::MdxDemoPopoverModal,
        mdx_demo_faq::MdxDemoFaq,
        mdx_demo_toast::MdxDemoToast,
        mdx_demo_blockquote::MdxDemoBlockquote,
        mdx_demo_dialog_form::MdxDemoDialogForm,
        mdx_demo_parallax_zoom_words::MdxDemoParallaxZoomWords,
        mdx_demo_dropdown::MdxDemoDropdown,
        mdx_demo_announcement::MdxDemoAnnouncement,
        mdx_demo_avatar_simple::MdxDemoAvatarSimple,
        mdx_demo_animate_group::MdxDemoAnimateGroup,
        mdx_demo_sheet::MdxDemoSheet,
        mdx_demo_toast_variants::MdxDemoToastVariants,
        mdx_demo_badge_custom::MdxDemoBadgeCustom,
    },
    utils::mdx::mdx_leptos::Components,
};

pub fn setup_mdx_core() -> Components {
    let mut components = Components::new();

    components.add("MdxDemoNavigationMenu".into(), MdxDemoNavigationMenu);
    components.add("MdxDemoButtonReactive".into(), MdxDemoButtonReactive);
    components.add("MdxDemoOrbitingCircles".into(), MdxDemoOrbitingCircles);
    components.add("MdxDemoButtonSizes".into(), MdxDemoButtonSizes);
    components.add("MdxDemoCommand".into(), MdxDemoCommand);
    components.add("MdxDemoBadge".into(), MdxDemoBadge);
    components.add("MdxDemoDropzone".into(), MdxDemoDropzone);
    components.add("MdxDemoBlurryBlob".into(), MdxDemoBlurryBlob);
    components.add("MdxDemoMeteorEffect".into(), MdxDemoMeteorEffect);
    components.add("MdxDemoFloatingButtonMenu".into(), MdxDemoFloatingButtonMenu);
    components.add("MdxDemoButton".into(), MdxDemoButton);
    components.add("MdxDemoRadar".into(), MdxDemoRadar);
    components.add("MdxDemoMarquee".into(), MdxDemoMarquee);
    components.add("MdxDemoCommandDialog".into(), MdxDemoCommandDialog);
    components.add("MdxDemoHeadings".into(), MdxDemoHeadings);
    components.add("MdxDemoBadgeVariants".into(), MdxDemoBadgeVariants);
    components.add("MdxDemoSkeletonImage".into(), MdxDemoSkeletonImage);
    components.add("MdxDemoNavigationMenuComplex".into(), MdxDemoNavigationMenuComplex);
    components.add("MdxDemoMultiSelect".into(), MdxDemoMultiSelect);
    components.add("MdxDemoAnimate".into(), MdxDemoAnimate);
    components.add("MdxDemoBentoGrid4".into(), MdxDemoBentoGrid4);
    components.add("MdxDemoHeadingsVariants".into(), MdxDemoHeadingsVariants);
    components.add("MdxDemoTooltip".into(), MdxDemoTooltip);
    components.add("MdxDemoStatus".into(), MdxDemoStatus);
    components.add("MdxDemoButtonVariants".into(), MdxDemoButtonVariants);
    components.add("MdxDemoSliderHover".into(), MdxDemoSliderHover);
    components.add("MdxDemoParallaxSimple".into(), MdxDemoParallaxSimple);
    components.add("MdxDemoLoaders".into(), MdxDemoLoaders);
    components.add("MdxDemoSpotlight".into(), MdxDemoSpotlight);
    components.add("MdxDemoDropdownMenu".into(), MdxDemoDropdownMenu);
    components.add("MdxDemoAlertDialog".into(), MdxDemoAlertDialog);
    components.add("MdxDemoSeparator".into(), MdxDemoSeparator);
    components.add("MdxDemoBreadcrumb".into(), MdxDemoBreadcrumb);
    components.add("MdxDemoSkeleton".into(), MdxDemoSkeleton);
    components.add("MdxDemoSelect".into(), MdxDemoSelect);
    components.add("MdxDemoBentoGrid5".into(), MdxDemoBentoGrid5);
    components.add("MdxDemoCardsDirectionAware".into(), MdxDemoCardsDirectionAware);
    components.add("MdxDemoGradient".into(), MdxDemoGradient);
    components.add("MdxDemoCarouselParallax".into(), MdxDemoCarouselParallax);
    components.add("MdxDemoButtonOverride".into(), MdxDemoButtonOverride);
    components.add("MdxDemoSheetDirections".into(), MdxDemoSheetDirections);
    components.add("MdxDemoBackgroundRetro".into(), MdxDemoBackgroundRetro);
    components.add("MdxDemoAlert".into(), MdxDemoAlert);
    components.add("MdxDemoBentoGrid6".into(), MdxDemoBentoGrid6);
    components.add("MdxDemoCard".into(), MdxDemoCard);
    components.add("MdxDemoExpandable".into(), MdxDemoExpandable);
    components.add("MdxDemoCardTilting".into(), MdxDemoCardTilting);
    components.add("MdxDemoHeadingsAnimate".into(), MdxDemoHeadingsAnimate);
    components.add("MdxDemoTailwindScrollOnly".into(), MdxDemoTailwindScrollOnly);
    components.add("MdxDemoTable".into(), MdxDemoTable);
    components.add("MdxDemoStatusVariants".into(), MdxDemoStatusVariants);
    components.add("MdxDemoTextArea".into(), MdxDemoTextArea);
    components.add("MdxDemoCard3dHover".into(), MdxDemoCard3dHover);
    components.add("MdxDemoHeadingsMotion".into(), MdxDemoHeadingsMotion);
    components.add("MdxDemoChartsMany".into(), MdxDemoChartsMany);
    components.add("MdxDemoBeamBorder".into(), MdxDemoBeamBorder);
    components.add("MdxDemoTextSlide".into(), MdxDemoTextSlide);
    components.add("MdxDemoInput".into(), MdxDemoInput);
    components.add("MdxDemoCheckbox".into(), MdxDemoCheckbox);
    components.add("MdxDemoLabel".into(), MdxDemoLabel);
    components.add("MdxDemoHamburgerMenus".into(), MdxDemoHamburgerMenus);
    components.add("MdxDemoPopoverModal".into(), MdxDemoPopoverModal);
    components.add("MdxDemoFaq".into(), MdxDemoFaq);
    components.add("MdxDemoToast".into(), MdxDemoToast);
    components.add("MdxDemoBlockquote".into(), MdxDemoBlockquote);
    components.add("MdxDemoDialogForm".into(), MdxDemoDialogForm);
    components.add("MdxDemoParallaxZoomWords".into(), MdxDemoParallaxZoomWords);
    components.add("MdxDemoDropdown".into(), MdxDemoDropdown);
    components.add("MdxDemoAnnouncement".into(), MdxDemoAnnouncement);
    components.add("MdxDemoAvatarSimple".into(), MdxDemoAvatarSimple);
    components.add("MdxDemoAnimateGroup".into(), MdxDemoAnimateGroup);
    components.add("MdxDemoSheet".into(), MdxDemoSheet);
    components.add("MdxDemoToastVariants".into(), MdxDemoToastVariants);
    components.add("MdxDemoBadgeCustom".into(), MdxDemoBadgeCustom);

    components
}
