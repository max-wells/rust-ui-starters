use crate::{
    __registry__::mdx_demos_hooks::{
        mdx_demo_use_not::MdxDemoUseNot,
        mdx_demo_use_web_notification::MdxDemoUseWebNotification,
        mdx_demo_use_cookie::MdxDemoUseCookie,
        mdx_demo_use_user_media::MdxDemoUseUserMedia,
        mdx_demo_use_hover::MdxDemoUseHover,
        mdx_demo_use_intl_number_format::MdxDemoUseIntlNumberFormat,
        mdx_demo_use_element_bounding::MdxDemoUseElementBounding,
        mdx_demo_use_throttle::MdxDemoUseThrottle,
        mdx_demo_use_watch_debounced::MdxDemoUseWatchDebounced,
        mdx_demo_use_clipboard::MdxDemoUseClipboard,
        mdx_demo_use_timestamp::MdxDemoUseTimestamp,
        mdx_demo_use_floating_flip::MdxDemoUseFloatingFlip,
        mdx_demo_use_debounce::MdxDemoUseDebounce,
        mdx_demo_use_raf_fn::MdxDemoUseRafFn,
        mdx_demo_use_watch_throttled::MdxDemoUseWatchThrottled,
        mdx_demo_use_timeout_fn::MdxDemoUseTimeoutFn,
        mdx_demo_use_resize_observer::MdxDemoUseResizeObserver,
        mdx_demo_use_locale::MdxDemoUseLocale,
        mdx_demo_use_color_mode::MdxDemoUseColorMode,
        mdx_demo_use_click_outside::MdxDemoUseClickOutside,
        mdx_demo_use_or::MdxDemoUseOr,
        mdx_demo_use_infinite_scroll::MdxDemoUseInfiniteScroll,
        mdx_demo_use_round::MdxDemoUseRound,
        mdx_demo_use_broadcast_channel::MdxDemoUseBroadcastChannel,
        mdx_demo_use_sorted::MdxDemoUseSorted,
        mdx_demo_use_lock_body_scroll::MdxDemoUseLockBodyScroll,
        mdx_demo_use_display_media::MdxDemoUseDisplayMedia,
        mdx_demo_use_mouse_in_element::MdxDemoUseMouseInElement,
        mdx_demo_use_geolocation::MdxDemoUseGeolocation,
        mdx_demo_use_drop_zone::MdxDemoUseDropZone,
        mdx_demo_use_mouse::MdxDemoUseMouse,
        mdx_demo_use_and::MdxDemoUseAnd,
        mdx_demo_use_debounce_fn::MdxDemoUseDebounceFn,
        mdx_demo_use_breakpoints::MdxDemoUseBreakpoints,
        mdx_demo_use_toggle::MdxDemoUseToggle,
        mdx_demo_use_element_size::MdxDemoUseElementSize,
        mdx_demo_use_media_query::MdxDemoUseMediaQuery,
        mdx_demo_use_idle::MdxDemoUseIdle,
        mdx_demo_use_mutation_observer::MdxDemoUseMutationObserver,
        mdx_demo_use_throttle_fn::MdxDemoUseThrottleFn,
        mdx_demo_use_permission::MdxDemoUsePermission,
        mdx_demo_use_window_size::MdxDemoUseWindowSize,
        mdx_demo_use_interval_fn::MdxDemoUseIntervalFn,
        mdx_demo_use_floating_tooltip::MdxDemoUseFloatingTooltip,
        mdx_demo_use_device_pixel_ratio::MdxDemoUseDevicePixelRatio,
        mdx_demo_use_event_listener::MdxDemoUseEventListener,
        mdx_demo_use_window_scroll::MdxDemoUseWindowScroll,
        mdx_demo_use_floor::MdxDemoUseFloor,
        mdx_demo_use_key_press::MdxDemoUseKeyPress,
        mdx_demo_use_interval::MdxDemoUseInterval,
        mdx_demo_use_storage::MdxDemoUseStorage,
        mdx_demo_use_autosize::MdxDemoUseAutosize,
        mdx_demo_use_floating_placement::MdxDemoUseFloatingPlacement,
        mdx_demo_use_watch_pausable::MdxDemoUseWatchPausable,
        mdx_demo_use_floating_shift::MdxDemoUseFloatingShift,
        mdx_demo_use_interval_local_storage::MdxDemoUseIntervalLocalStorage,
        mdx_demo_use_prefers_reduced_motion::MdxDemoUsePrefersReducedMotion,
        mdx_demo_use_sync_signal::MdxDemoUseSyncSignal,
        mdx_demo_use_locales::MdxDemoUseLocales,
        mdx_demo_use_window_focus::MdxDemoUseWindowFocus,
        mdx_demo_use_floating_size::MdxDemoUseFloatingSize,
        mdx_demo_use_intersection_observer::MdxDemoUseIntersectionObserver,
        mdx_demo_use_abs::MdxDemoUseAbs,
        mdx_demo_use_ceil::MdxDemoUseCeil,
        mdx_demo_use_css_var::MdxDemoUseCssVar,
        mdx_demo_use_cycle_list::MdxDemoUseCycleList,
    },
    utils::mdx::mdx_leptos::Components,
};

pub fn setup_mdx_hooks() -> Components {
    let mut components = Components::new();

    components.add("MdxDemoUseNot".into(), MdxDemoUseNot);
    components.add("MdxDemoUseWebNotification".into(), MdxDemoUseWebNotification);
    components.add("MdxDemoUseCookie".into(), MdxDemoUseCookie);
    components.add("MdxDemoUseUserMedia".into(), MdxDemoUseUserMedia);
    components.add("MdxDemoUseHover".into(), MdxDemoUseHover);
    components.add("MdxDemoUseIntlNumberFormat".into(), MdxDemoUseIntlNumberFormat);
    components.add("MdxDemoUseElementBounding".into(), MdxDemoUseElementBounding);
    components.add("MdxDemoUseThrottle".into(), MdxDemoUseThrottle);
    components.add("MdxDemoUseWatchDebounced".into(), MdxDemoUseWatchDebounced);
    components.add("MdxDemoUseClipboard".into(), MdxDemoUseClipboard);
    components.add("MdxDemoUseTimestamp".into(), MdxDemoUseTimestamp);
    components.add("MdxDemoUseFloatingFlip".into(), MdxDemoUseFloatingFlip);
    components.add("MdxDemoUseDebounce".into(), MdxDemoUseDebounce);
    components.add("MdxDemoUseRafFn".into(), MdxDemoUseRafFn);
    components.add("MdxDemoUseWatchThrottled".into(), MdxDemoUseWatchThrottled);
    components.add("MdxDemoUseTimeoutFn".into(), MdxDemoUseTimeoutFn);
    components.add("MdxDemoUseResizeObserver".into(), MdxDemoUseResizeObserver);
    components.add("MdxDemoUseLocale".into(), MdxDemoUseLocale);
    components.add("MdxDemoUseColorMode".into(), MdxDemoUseColorMode);
    components.add("MdxDemoUseClickOutside".into(), MdxDemoUseClickOutside);
    components.add("MdxDemoUseOr".into(), MdxDemoUseOr);
    components.add("MdxDemoUseInfiniteScroll".into(), MdxDemoUseInfiniteScroll);
    components.add("MdxDemoUseRound".into(), MdxDemoUseRound);
    components.add("MdxDemoUseBroadcastChannel".into(), MdxDemoUseBroadcastChannel);
    components.add("MdxDemoUseSorted".into(), MdxDemoUseSorted);
    components.add("MdxDemoUseLockBodyScroll".into(), MdxDemoUseLockBodyScroll);
    components.add("MdxDemoUseDisplayMedia".into(), MdxDemoUseDisplayMedia);
    components.add("MdxDemoUseMouseInElement".into(), MdxDemoUseMouseInElement);
    components.add("MdxDemoUseGeolocation".into(), MdxDemoUseGeolocation);
    components.add("MdxDemoUseDropZone".into(), MdxDemoUseDropZone);
    components.add("MdxDemoUseMouse".into(), MdxDemoUseMouse);
    components.add("MdxDemoUseAnd".into(), MdxDemoUseAnd);
    components.add("MdxDemoUseDebounceFn".into(), MdxDemoUseDebounceFn);
    components.add("MdxDemoUseBreakpoints".into(), MdxDemoUseBreakpoints);
    components.add("MdxDemoUseToggle".into(), MdxDemoUseToggle);
    components.add("MdxDemoUseElementSize".into(), MdxDemoUseElementSize);
    components.add("MdxDemoUseMediaQuery".into(), MdxDemoUseMediaQuery);
    components.add("MdxDemoUseIdle".into(), MdxDemoUseIdle);
    components.add("MdxDemoUseMutationObserver".into(), MdxDemoUseMutationObserver);
    components.add("MdxDemoUseThrottleFn".into(), MdxDemoUseThrottleFn);
    components.add("MdxDemoUsePermission".into(), MdxDemoUsePermission);
    components.add("MdxDemoUseWindowSize".into(), MdxDemoUseWindowSize);
    components.add("MdxDemoUseIntervalFn".into(), MdxDemoUseIntervalFn);
    components.add("MdxDemoUseFloatingTooltip".into(), MdxDemoUseFloatingTooltip);
    components.add("MdxDemoUseDevicePixelRatio".into(), MdxDemoUseDevicePixelRatio);
    components.add("MdxDemoUseEventListener".into(), MdxDemoUseEventListener);
    components.add("MdxDemoUseWindowScroll".into(), MdxDemoUseWindowScroll);
    components.add("MdxDemoUseFloor".into(), MdxDemoUseFloor);
    components.add("MdxDemoUseKeyPress".into(), MdxDemoUseKeyPress);
    components.add("MdxDemoUseInterval".into(), MdxDemoUseInterval);
    components.add("MdxDemoUseStorage".into(), MdxDemoUseStorage);
    components.add("MdxDemoUseAutosize".into(), MdxDemoUseAutosize);
    components.add("MdxDemoUseFloatingPlacement".into(), MdxDemoUseFloatingPlacement);
    components.add("MdxDemoUseWatchPausable".into(), MdxDemoUseWatchPausable);
    components.add("MdxDemoUseFloatingShift".into(), MdxDemoUseFloatingShift);
    components.add("MdxDemoUseIntervalLocalStorage".into(), MdxDemoUseIntervalLocalStorage);
    components.add("MdxDemoUsePrefersReducedMotion".into(), MdxDemoUsePrefersReducedMotion);
    components.add("MdxDemoUseSyncSignal".into(), MdxDemoUseSyncSignal);
    components.add("MdxDemoUseLocales".into(), MdxDemoUseLocales);
    components.add("MdxDemoUseWindowFocus".into(), MdxDemoUseWindowFocus);
    components.add("MdxDemoUseFloatingSize".into(), MdxDemoUseFloatingSize);
    components.add("MdxDemoUseIntersectionObserver".into(), MdxDemoUseIntersectionObserver);
    components.add("MdxDemoUseAbs".into(), MdxDemoUseAbs);
    components.add("MdxDemoUseCeil".into(), MdxDemoUseCeil);
    components.add("MdxDemoUseCssVar".into(), MdxDemoUseCssVar);
    components.add("MdxDemoUseCycleList".into(), MdxDemoUseCycleList);

    components
}
