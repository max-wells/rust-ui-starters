use leptos::*;

use crate::registry::{
    icons::chevrons::chevron_right::ChevronRight,
    ui::announcement::{Announcement, AnnouncementDescription, AnnouncementLineEffect},
};

#[component]
pub fn DemoAnnouncement() -> impl IntoView {
    view! {
        <Announcement>
            <AnnouncementLineEffect />
            "🎉"
            <hr class="mx-2 h-4 bg-gray-300 w-[1px] shrink-0" />
            <AnnouncementDescription>"Introducing new feature"</AnnouncementDescription>
            <ChevronRight class="ml-1 transition-transform duration-300 ease-in-out group-hover:translate-x-0.5 size-3" />
        </Announcement>
    }
}
