use leptos::*;

use crate::registry::icons::others::text_align_center::TextAlignCenter;
use crate::registry::icons::others::text_align_left::TextAlignLeft;
use crate::registry::icons::others::text_align_right::TextAlignRight;
use crate::registry::primitives::p_toggle_group::{
    ToggleGroupKind, ToggleGroupMultiple, ToggleGroupSingle,
};
use crate::registry::primitives::p_toolbar::{
    ToolbarButton, ToolbarLink, ToolbarRoot, ToolbarSeparator, ToolbarToggleGroup,
    ToolbarToggleItem,
};

#[component]
pub fn DemoToolbar() -> impl IntoView {
    view! {
        <ToolbarRoot
            attr:class="flex p-[10px] w-full min-w-max rounded-md"
            attr:aria-label="Formatting options"
        >
            <ToolbarToggleGroup
                kind=ToggleGroupKind::Multiple {
                    value: ToggleGroupMultiple::none().into(),
                    default_value: ToggleGroupMultiple::none().into(),
                    on_value_change: None,
                }
                attr:aria-label="Text formatting"
            >
                <ToolbarToggleItem
                    attr:class="flex-shrink-0 mr-0.5 flex-grow-0 basis-auto text-neutral-700 h-[25px] px-[5px] rounded inline-flex text-[13px] leading-none items-center justify-center bg-white ml-0.5 outline-none hover:bg-neutral-300 hover:text-neutral-900 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-neutral-500 first:ml-0 data-[state=on]:bg-neutral-400 data-[state=on]:text-neutral-900"
                    value="bold"
                    attr:aria-label="Bold"
                >
                    <FontBoldIcon />
                </ToolbarToggleItem>
                <ToolbarToggleItem
                    attr:class="flex-shrink-0 mr-0.5 flex-grow-0 basis-auto text-neutral-700 h-[25px] px-[5px] rounded inline-flex text-[13px] leading-none items-center justify-center bg-white ml-0.5 outline-none hover:bg-neutral-300 hover:text-neutral-900 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-neutral-500 first:ml-0 data-[state=on]:bg-neutral-400 data-[state=on]:text-neutral-900"
                    value="italic"
                    attr:aria-label="Italic"
                >
                    <FontItalicIcon />
                </ToolbarToggleItem>
                <ToolbarToggleItem
                    attr:class="flex-shrink-0 flex-grow-0 basis-auto text-neutral-700 h-[25px] px-[5px] rounded inline-flex text-[13px] leading-none items-center justify-center bg-white ml-0.5 outline-none hover:bg-neutral-300 hover:text-neutral-900 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-neutral-500 first:ml-0 data-[state=on]:bg-neutral-400 data-[state=on]:text-neutral-900"
                    value="strikethrough"
                    attr:aria-label="Strike through"
                >
                    <StrikethroughIcon />
                </ToolbarToggleItem>
            </ToolbarToggleGroup>
            <ToolbarSeparator attr:class="w-[1px] bg-neutral-300 mx-[10px]" />
            <ToolbarToggleGroup
                kind=ToggleGroupKind::Single {
                    value: ToggleGroupSingle::none().into(),
                    default_value: "center".into(),
                    on_value_change: None,
                }
                attr:aria-label="Text alignment"
            >
                <ToolbarToggleItem
                    attr:class="flex-shrink-0 flex-grow-0 mr-0.5 basis-auto text-neutral-700 h-[25px] px-[5px] rounded inline-flex text-[13px] leading-none items-center justify-center bg-white ml-0.5 outline-none hover:bg-neutral-300 hover:text-neutral-900 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-neutral-500 first:ml-0 data-[state=on]:bg-neutral-400 data-[state=on]:text-neutral-900"
                    value="left"
                    attr:aria-label="Left aligned"
                >
                    <TextAlignLeft />
                </ToolbarToggleItem>
                <ToolbarToggleItem
                    attr:class="flex-shrink-0 flex-grow-0 mr-0.5 basis-auto text-neutral-700 h-[25px] px-[5px] rounded inline-flex text-[13px] leading-none items-center justify-center bg-white ml-0.5 outline-none hover:bg-neutral-300 hover:text-neutral-900 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-neutral-500 first:ml-0 data-[state=on]:bg-neutral-400 data-[state=on]:text-neutral-900"
                    value="center"
                    attr:aria-label="Center aligned"
                >
                    <TextAlignCenter />
                </ToolbarToggleItem>
                <ToolbarToggleItem
                    attr:class="flex-shrink-0 flex-grow-0 basis-auto text-neutral-700 h-[25px] px-[5px] rounded inline-flex text-[13px] leading-none items-center justify-center bg-white ml-0.5 outline-none hover:bg-neutral-300 hover:text-neutral-900 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-neutral-500 first:ml-0 data-[state=on]:bg-neutral-400 data-[state=on]:text-neutral-900"
                    value="right"
                    attr:aria-label="Right aligned"
                >
                    <TextAlignRight />
                </ToolbarToggleItem>
            </ToolbarToggleGroup>
            <ToolbarSeparator attr:class="w-[1px] bg-neutral-300 mx-[10px]" />
            <ToolbarLink
                attr:class="bg-transparent mr-[4px] text-neutral-700 hidden sm:inline-flex justify-center items-center hover:bg-transparent hover:cursor-pointer flex-shrink-0 flex-grow-0 basis-auto h-[25px] px-[5px] rounded text-[13px] leading-none bg-white ml-0.5 outline-none hover:bg-neutral-300 hover:text-neutral-900 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-neutral-500 first:ml-0 data-[state=on]:bg-neutral-400 data-[state=on]:text-neutral-900"
                attr:href="#"
                attr:target="_blank"
            >
                "Edited 2 hours ago"
            </ToolbarLink>
            <ToolbarButton attr:class="px-[10px] text-white ml-auto bg-neutral-800 flex-shrink-0 flex-grow-0 basis-auto h-[25px] rounded inline-flex text-[13px] leading-none items-center justify-center outline-none hover:bg-neutral-900 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-neutral-500">
                "Share"
            </ToolbarButton>
        </ToolbarRoot>
    }
}

#[component]
fn FontBoldIcon() -> impl IntoView {
    view! {
        <svg
            width="15"
            height="15"
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M5.10505 12C4.70805 12 4.4236 11.912 4.25171 11.736C4.0839 11.5559 4 11.2715 4 10.8827V4.11733C4 3.72033 4.08595 3.43588 4.25784 3.26398C4.43383 3.08799 4.71623 3 5.10505 3C6.42741 3 8.25591 3 9.02852 3C10.1373 3 11.0539 3.98153 11.0539 5.1846C11.0539 6.08501 10.6037 6.81855 9.70327 7.23602C10.8657 7.44851 11.5176 8.62787 11.5176 9.48128C11.5176 10.5125 10.9902 12 9.27734 12C8.77742 12 6.42626 12 5.10505 12ZM8.37891 8.00341H5.8V10.631H8.37891C8.9 10.631 9.6296 10.1211 9.6296 9.29877C9.6296 8.47643 8.9 8.00341 8.37891 8.00341ZM5.8 4.36903V6.69577H8.17969C8.53906 6.69577 9.27734 6.35939 9.27734 5.50002C9.27734 4.64064 8.48047 4.36903 8.17969 4.36903H5.8Z"
                fill="currentColor"
            ></path>
        </svg>
    }
}

#[component]
fn FontItalicIcon() -> impl IntoView {
    view! {
        <svg
            width="15"
            height="15"
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M5.67494 3.50017C5.67494 3.25164 5.87641 3.05017 6.12494 3.05017H10.6249C10.8735 3.05017 11.0749 3.25164 11.0749 3.50017C11.0749 3.7487 10.8735 3.95017 10.6249 3.95017H9.00587L7.2309 11.05H8.87493C9.12345 11.05 9.32493 11.2515 9.32493 11.5C9.32493 11.7486 9.12345 11.95 8.87493 11.95H4.37493C4.1264 11.95 3.92493 11.7486 3.92493 11.5C3.92493 11.2515 4.1264 11.05 4.37493 11.05H5.99397L7.76894 3.95017H6.12494C5.87641 3.95017 5.67494 3.7487 5.67494 3.50017Z"
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
            ></path>
        </svg>
    }
}

#[component]
fn StrikethroughIcon() -> impl IntoView {
    view! {
        <svg
            width="15"
            height="15"
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M5.00003 3.25C5.00003 2.97386 4.77617 2.75 4.50003 2.75C4.22389 2.75 4.00003 2.97386 4.00003 3.25V7.10003H2.49998C2.27906 7.10003 2.09998 7.27912 2.09998 7.50003C2.09998 7.72094 2.27906 7.90003 2.49998 7.90003H4.00003V8.55C4.00003 10.483 5.56703 12.05 7.50003 12.05C9.43303 12.05 11 10.483 11 8.55V7.90003H12.5C12.7209 7.90003 12.9 7.72094 12.9 7.50003C12.9 7.27912 12.7209 7.10003 12.5 7.10003H11V3.25C11 2.97386 10.7762 2.75 10.5 2.75C10.2239 2.75 10 2.97386 10 3.25V7.10003H5.00003V3.25ZM5.00003 7.90003V8.55C5.00003 9.93071 6.11932 11.05 7.50003 11.05C8.88074 11.05 10 9.93071 10 8.55V7.90003H5.00003Z"
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
            ></path>
        </svg>
    }
}
