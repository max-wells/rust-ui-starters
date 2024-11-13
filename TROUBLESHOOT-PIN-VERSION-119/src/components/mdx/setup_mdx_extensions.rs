use crate::{
    __registry__::mdx_demos_extensions::{
        mdx_demo_tabs_shadow::MdxDemoTabsShadow,
        mdx_demo_select_price_table::MdxDemoSelectPriceTable,
        mdx_demo_masonry::MdxDemoMasonry,
        mdx_demo_steps_indicator::MdxDemoStepsIndicator,
        mdx_demo_carousel_snap_scroll::MdxDemoCarouselSnapScroll,
        mdx_demo_sticky_cursor_links::MdxDemoStickyCursorLinks,
        mdx_demo_css_pill_ocean::MdxDemoCssPillOcean,
        mdx_demo_wheel_headings::MdxDemoWheelHeadings,
        mdx_demo_css_pill_mountain::MdxDemoCssPillMountain,
        mdx_demo_cards_follow_shadow::MdxDemoCardsFollowShadow,
        mdx_demo_css_pill_lighthouse::MdxDemoCssPillLighthouse,
        mdx_demo_cards_stacking::MdxDemoCardsStacking,
        mdx_demo_carousel3d_rotating::MdxDemoCarousel3dRotating,
        mdx_demo_triggered_highlight::MdxDemoTriggeredHighlight,
        mdx_demo_card3d_flip_rotation::MdxDemoCard3dFlipRotation,
        mdx_demo_gallery_clickable_transitions::MdxDemoGalleryClickableTransitions,
        mdx_demo_cards_repushing::MdxDemoCardsRepushing,
        mdx_demo_card_focus::MdxDemoCardFocus,
        mdx_demo_universe_rotating::MdxDemoUniverseRotating,
        mdx_demo_cards_slider::MdxDemoCardsSlider,
        mdx_demo_rain_letters::MdxDemoRainLetters,
        mdx_demo_gallery_zoom::MdxDemoGalleryZoom,
        mdx_demo_always_great_grid::MdxDemoAlwaysGreatGrid,
        mdx_demo_cards_glowing::MdxDemoCardsGlowing,
        mdx_demo_cta_animation_on_hover::MdxDemoCtaAnimationOnHover,
        mdx_demo_card_wobble::MdxDemoCardWobble,
        mdx_demo_cursor_multi_color::MdxDemoCursorMultiColor,
        mdx_demo_docker::MdxDemoDocker,
        mdx_demo_particles_vercel::MdxDemoParticlesVercel,
        mdx_demo_radar_mini::MdxDemoRadarMini,
        mdx_demo_scroll_light::MdxDemoScrollLight,
        mdx_demo_hamburger_menu::MdxDemoHamburgerMenu,
        mdx_demo_parallax1::MdxDemoParallax1,
    },
    utils::mdx::mdx_leptos::Components,
};

pub fn setup_mdx_extensions() -> Components {
    let mut components = Components::new();

    components.add("MdxDemoTabsShadow".into(), MdxDemoTabsShadow);
    components.add("MdxDemoSelectPriceTable".into(), MdxDemoSelectPriceTable);
    components.add("MdxDemoMasonry".into(), MdxDemoMasonry);
    components.add("MdxDemoStepsIndicator".into(), MdxDemoStepsIndicator);
    components.add("MdxDemoCarouselSnapScroll".into(), MdxDemoCarouselSnapScroll);
    components.add("MdxDemoStickyCursorLinks".into(), MdxDemoStickyCursorLinks);
    components.add("MdxDemoCssPillOcean".into(), MdxDemoCssPillOcean);
    components.add("MdxDemoWheelHeadings".into(), MdxDemoWheelHeadings);
    components.add("MdxDemoCssPillMountain".into(), MdxDemoCssPillMountain);
    components.add("MdxDemoCardsFollowShadow".into(), MdxDemoCardsFollowShadow);
    components.add("MdxDemoCssPillLighthouse".into(), MdxDemoCssPillLighthouse);
    components.add("MdxDemoCardsStacking".into(), MdxDemoCardsStacking);
    components.add("MdxDemoCarousel3dRotating".into(), MdxDemoCarousel3dRotating);
    components.add("MdxDemoTriggeredHighlight".into(), MdxDemoTriggeredHighlight);
    components.add("MdxDemoCard3dFlipRotation".into(), MdxDemoCard3dFlipRotation);
    components.add("MdxDemoGalleryClickableTransitions".into(), MdxDemoGalleryClickableTransitions);
    components.add("MdxDemoCardsRepushing".into(), MdxDemoCardsRepushing);
    components.add("MdxDemoCardFocus".into(), MdxDemoCardFocus);
    components.add("MdxDemoUniverseRotating".into(), MdxDemoUniverseRotating);
    components.add("MdxDemoCardsSlider".into(), MdxDemoCardsSlider);
    components.add("MdxDemoRainLetters".into(), MdxDemoRainLetters);
    components.add("MdxDemoGalleryZoom".into(), MdxDemoGalleryZoom);
    components.add("MdxDemoAlwaysGreatGrid".into(), MdxDemoAlwaysGreatGrid);
    components.add("MdxDemoCardsGlowing".into(), MdxDemoCardsGlowing);
    components.add("MdxDemoCtaAnimationOnHover".into(), MdxDemoCtaAnimationOnHover);
    components.add("MdxDemoCardWobble".into(), MdxDemoCardWobble);
    components.add("MdxDemoCursorMultiColor".into(), MdxDemoCursorMultiColor);
    components.add("MdxDemoDocker".into(), MdxDemoDocker);
    components.add("MdxDemoParticlesVercel".into(), MdxDemoParticlesVercel);
    components.add("MdxDemoRadarMini".into(), MdxDemoRadarMini);
    components.add("MdxDemoScrollLight".into(), MdxDemoScrollLight);
    components.add("MdxDemoHamburgerMenu".into(), MdxDemoHamburgerMenu);
    components.add("MdxDemoParallax1".into(), MdxDemoParallax1);

    components
}
