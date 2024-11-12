use leptos::*;

use crate::registry::{
    icons::others::plus::Plus,
    ui::faq::{Faq, FaqContent, FaqDescription, FaqInput, FaqLabel, FaqSection, FaqTitle},
};

#[component]
pub fn DemoFaq() -> impl IntoView {
    view! {
        <section class="flex flex-col justify-center items-center py-6 px-12 w-screen h-[600px] bg-zinc-900 text-zinc-50">

            <Faq>
                <FaqSection>
                    <FaqInput id="faq1" />
                    <FaqLabel for_attr="faq1">
                        <FaqTitle>
                            {"What is the meaning of life, the universe, and everything?"}
                        </FaqTitle>
                        <Plus class="size-5 text-primary" />
                    </FaqLabel>
                    <FaqContent>
                        <FaqDescription>{DUMMY_CONTENT}</FaqDescription>
                    </FaqContent>
                </FaqSection>

                <FaqSection>
                    <FaqInput id="faq2" />
                    <FaqLabel for_attr="faq2">
                        <FaqTitle>{"What is the average airspeed of an unladen swallow?"}</FaqTitle>
                        <Plus class="size-5 text-primary" />
                    </FaqLabel>
                    <FaqContent>
                        <FaqDescription>{DUMMY_CONTENT}</FaqDescription>
                    </FaqContent>
                </FaqSection>

                <FaqSection>
                    <FaqInput id="faq3" />
                    <FaqLabel for_attr="faq3">
                        <FaqTitle>{"Why did not they just take the eagles to Mordor?"}</FaqTitle>
                        <Plus class="size-5 text-primary" />
                    </FaqLabel>
                    <FaqContent>
                        <FaqDescription>{DUMMY_CONTENT}</FaqDescription>
                    </FaqContent>
                </FaqSection>
            </Faq>

        </section>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ CONSTANTS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const DUMMY_CONTENT: &str = "Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante. Donec eu libero sit amet quam egestas semper. Aenean ultricies mi vitae est. Mauris placerat eleifend leo.";

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// TODO UI. Refactor {FaqInput, FaqLabel, FaqTitle} into a single component (FaqHeader)
// * └──> Tried but did not work.. :/ Maybe because we need a Fragment ?

// <FaqHeader
// id="faq1"
// for_attr="faq1"
// title="What is the meaning of life, the universe, and everything?"
// />

// #[component]
// pub fn FaqHeader(
//     #[prop(into)] id: &'static str,
//     #[prop(into)] for_attr: &'static str,
//     #[prop(into)] title: &'static str,
// ) -> impl IntoView {
//     view! {
//         <>
//             <input id=id type="checkbox" class="ml-auto sr-only peer" />
//             <label
//                 for_attr=for_attr
//                 class="flex items-center justify-between w-full px-4 py-2 mt-2 cursor-pointer"
//             >
//                 <FaqTitle>{title}</FaqTitle>
//                 <Plus class="size-5 text-primary" />
//             </label>
//         </>
//     }
// }
