use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoDragAndDrop() -> impl IntoView {
    view! {
        <Stylesheet id="drag-and-drop" href="/components-nooo/drag-and-drop.css" />
        <script src="/components/drag-and-drop.js" />

        <ol id="$demo">
            <li draggable="true" style="view-transition-name: card-1">
                CSS
            </li>
            <li draggable="true" style="view-transition-name: card-2">
                HTML
            </li>
            <li draggable="true" style="view-transition-name: card-3">
                JS
            </li>
            <li
                draggable="true"
                style="view-transition-name: yes-this-keeps-animating-during-morph"
            >
                <picture>
                    <source
                        srcset="https://fonts.gstatic.com/s/e/notoemoji/latest/1f92f/512.webp"
                        type="image/webp"
                    />
                    <img
                        src="https://fonts.gstatic.com/s/e/notoemoji/latest/1f92f/512.gif"
                        alt="🤯"
                        width="125"
                        height="125"
                    />
                </picture>
            </li>
            <li draggable="true" style="view-transition-name: a-really-long-list-item-for-funsies">
                If yore gonna build a time machine into a car, why not do it with some style?
            </li>
        </ol>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/
