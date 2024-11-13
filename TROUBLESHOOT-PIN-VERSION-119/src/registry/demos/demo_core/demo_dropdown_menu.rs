use leptos::*;

// Source : https://codepen.io/hexagoncircle/pen/ExzZMvO

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn DemoDropdownMenu() -> impl IntoView {
    view! {
        <section class="warning">
            <p>
                "CSS anchor positioning is not yet supported in this browser. Try this demo in Chrome 125+"
            </p>
        </section>

        <button class="toggle" data-anchor popovertarget="popover-menu">
            Menu
        </button>
        <div id="popover-menu" popover>
            <ul class="menu" data-list-arrow-keys>
                <li>
                    <button>
                        New Tab <div class="kbd-shortcut">
                            <kbd>"⌘"</kbd>
                            <kbd>T</kbd>
                        </div>
                    </button>
                </li>
                <li>
                    <button>
                        New Window <div class="kbd-shortcut">
                            <kbd>"⌘"</kbd>
                            <kbd>N</kbd>
                        </div>
                    </button>
                </li>
                <li>
                    <button>
                        New Incognito Window <div class="kbd-shortcut">
                            <kbd>"⇧"</kbd>
                            <kbd>"⌘"</kbd>
                            <kbd>N</kbd>
                        </div>
                    </button>
                </li>
                <li>
                    <button>
                        Reopen Closed Tab <div class="kbd-shortcut">
                            <kbd>"⌘"</kbd>
                            <kbd>T</kbd>
                        </div>
                    </button>
                </li>
                <li>
                    <button>
                        Open File... <div class="kbd-shortcut">
                            <kbd>"⌘"</kbd>
                            <kbd>O</kbd>
                        </div>
                    </button>
                </li>
                <li>
                    <button>
                        Open Location... <div class="kbd-shortcut">
                            <kbd>"⌘"</kbd>
                            <kbd>L</kbd>
                        </div>
                    </button>
                </li>
                <li class="separator">
                    <button>
                        Close Window <div class="kbd-shortcut">
                            <kbd>"⇧"</kbd>
                            <kbd>"⌘"</kbd>
                            <kbd>W</kbd>
                        </div>
                    </button>
                </li>
                <li>
                    <button>
                        Close Tab <div class="kbd-shortcut">
                            <kbd>"⌘"</kbd>
                            <kbd>W</kbd>
                        </div>
                    </button>
                </li>
                <li>
                    <button>
                        Save Page As... <div class="kbd-shortcut">
                            <kbd>"⌘"</kbd>
                            <kbd>S</kbd>
                        </div>
                    </button>
                </li>
                <li class="separator" data-anchor>
                    <button>
                        Share
                        <svg
                            fill="currentColor"
                            xmlns="http://www.w3.org/2000/svg"
                            width="14"
                            height="14"
                            viewBox="0 0 256 256"
                        >
                            <path d="M184.49,136.49l-80,80a12,12,0,0,1-17-17L159,128,87.51,56.49a12,12,0,1,1,17-17l80,80A12,12,0,0,1,184.49,136.49Z"></path>
                        </svg>
                    </button>
                    <ul id="share-menu" class="menu submenu">
                        <li>
                            <button>
                                Email Link <div class="kbd-shortcut">
                                    <kbd>"⇧"</kbd>
                                    <kbd>"⌘"</kbd>
                                    <kbd>I</kbd>
                                </div>
                            </button>
                        </li>
                        <li>
                            <button>Messages</button>
                        </li>
                        <li>
                            <button>Airdrop</button>
                        </li>
                        <li>
                            <button>Notes</button>
                        </li>
                    </ul>
                </li>
                <li class="separator">
                    <button>
                        Print <div class="kbd-shortcut">
                            <kbd>"⌘"</kbd>
                            <kbd>P</kbd>
                        </div>
                    </button>
                </li>
            </ul>
        </div>
    }
}
