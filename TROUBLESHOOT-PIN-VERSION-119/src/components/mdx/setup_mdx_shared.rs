use leptos::MaybeSignal;

use crate::{
    components::mdx::mdx_html_elements_to_fix::{
        a1,
        a1Props,
        blockquote1,
        blockquote1Props,
        code1,
        code1Props,
        h1,
        h1Props,
        h2,
        h2Props,
        h3,
        h3Props,
        p1,
        p1Props,
        pre1,
        pre1Props,
        span1,
        span1Props,
        // img1, img1Props,
    },
    registry::ui::separator::{Separator, SeparatorOrientation, SeparatorProps},
    utils::mdx::mdx_leptos::{Components, MdxComponentProps},
};

pub fn setup_mdx_shared() -> Components {
    let mut components = Components::new();

    // ...with props (by defining a mapper from `MdxComponentProps`)

    // TODO. NB: The Separator does not have children (<Separator />)
    // TODO. The class does not seem to apply (content-mdx/docs/introduction.mdx)
    components.add_props("Separator".into(), Separator, |_| SeparatorProps {
        // children: props.children,
        attributes: vec![],
        class: MaybeSignal::from(""), // use empty string as default
        orientation: MaybeSignal::from(SeparatorOrientation::Default), // use a valid default value
    });

    //
    //
    components.add_props("h1".into(), h1, |props: MdxComponentProps| h1Props {
        children: props.children,
    });
    components.add_props("h2".into(), h2, |props: MdxComponentProps| h2Props {
        children: props.children,
    });
    components.add_props("h3".into(), h3, |props: MdxComponentProps| h3Props {
        children: props.children,
    });

    // TODO. Find a way to fix issue with p, a, img, etc. I had to add a number after otherwise the import didn't work
    components.add_props("a".into(), a1, |props: MdxComponentProps| a1Props {
        children: props.children,
    });
    components.add_props("p".into(), p1, |props: MdxComponentProps| p1Props {
        children: props.children,
    });
    components.add_props(
        "blockquote".into(),
        blockquote1,
        |props: MdxComponentProps| blockquote1Props {
            children: props.children,
        },
    );
    components.add_props("pre".into(), pre1, |props: MdxComponentProps| pre1Props {
        children: props.children,
    });
    components.add_props("code".into(), code1, |props: MdxComponentProps| {
        code1Props {
            children: props.children,
        }
    });
    // TODO 🚑 : I needed to override with dark:!text-neutral-100
    components.add_props("span".into(), span1, |props: MdxComponentProps| {
        span1Props {
            children: props.children,
        }
    });

    //
    //
    components
}
