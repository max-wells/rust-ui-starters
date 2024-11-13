use leptos::*;

use crate::registry::icons::chevrons::chevron_down::ChevronDown;
use crate::registry::primitives::p_accordion::{
    AccordionKind, AccordionSingle, PrimitiveAccordionContent, PrimitiveAccordionHeader,
    PrimitiveAccordionItem, PrimitiveAccordionRoot, PrimitiveAccordionTrigger,
};

#[component]
pub fn DemoAccordion() -> impl IntoView {
    view! {
        <PrimitiveAccordionRoot
            attr:class="w-[500px]"
            kind=AccordionKind::Single {
                value: AccordionSingle::none().into(),
                default_value: "item-1".into(),
                collapsible: true.into(),
                on_value_change: None,
            }
        >
            // kind=AccordionKind::Multiple {
            // value: AccordionMultiple::none(),
            // default_value: AccordionMultiple::none(),
            // on_value_change: None
            // }
            <AccordionItemDemo value="item-1">
                <AccordionTriggerDemo>"Is it accessible?"</AccordionTriggerDemo>
                <AccordionContentDemo>
                    "Yes. It adheres to the WAI-ARIA design pattern."
                </AccordionContentDemo>
            </AccordionItemDemo>

            <AccordionItemDemo value="item-2">
                <AccordionTriggerDemo>"Is it unstyled?"</AccordionTriggerDemo>
                <AccordionContentDemo>
                    "Yes. It's unstyled by default, giving you freedom over the look and feel."
                </AccordionContentDemo>
            </AccordionItemDemo>

            <AccordionItemDemo value="item-3">
                <AccordionTriggerDemo>"Can it be animated?"</AccordionTriggerDemo>
                <AccordionContentDemo>
                    "Yes! You can animate the Accordion with CSS or Rust."
                </AccordionContentDemo>
            </AccordionItemDemo>
        </PrimitiveAccordionRoot>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
fn AccordionItemDemo(
    #[prop(into)] value: MaybeSignal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <PrimitiveAccordionItem value=value attr:class="border-b">
            {children()}
        </PrimitiveAccordionItem>
    }
}

#[component]
fn AccordionTriggerDemo(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <PrimitiveAccordionHeader attr:class="flex">
            <PrimitiveAccordionTrigger attr:class="flex items-center justify-between flex-1 py-4 font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180">
                {children.with_value(|children| children())}
                <ChevronDown
                    attr:class="transition-transform duration-200 shrink-0 size-4"
                    attr:aria-hidden=true.into_attribute()
                />
            </PrimitiveAccordionTrigger>
        </PrimitiveAccordionHeader>
    }
}

#[component]
fn AccordionContentDemo(children: ChildrenFn) -> impl IntoView {
    view! {
        <PrimitiveAccordionContent attr:class="mb-4 overflow-hidden text-sm transition-all">
            <div class="px-5 py-[15px] data-[state=closed]:animate-accordionUp data-[state=open]:animate-accordionDown">
                {children()}
            </div>
        </PrimitiveAccordionContent>
    }
}
