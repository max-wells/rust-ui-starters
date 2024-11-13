use leptos::*;

#[component]
pub fn DemoNavigationMenuComplex() -> impl IntoView {
    view! {
        <nav class="clearfix navigation-menu-demo">
            <ul>
                <li>
                    <a href="#">Home</a>
                    <ul>
                        <li>
                            <a href="#">Menu Item 1</a>
                        </li>
                        <li class="submenu">
                            <a href="#">Menu Item 2</a>
                            <ul>
                                <li>
                                    <a href="#">Subitem 1</a>
                                </li>
                                <li class="submenu">
                                    <a href="#">Subitem 2</a>
                                    <ul>
                                        <li>
                                            <a href="#">Subitem 1</a>
                                        </li>
                                        <li>
                                            <a href="#">Subitem 2</a>
                                        </li>
                                        <li>
                                            <a href="#">Subitem 3</a>
                                        </li>
                                    </ul>
                                </li>
                            </ul>
                        </li>
                        <li class="submenu">
                            <a href="#">Menu Item 3</a>
                            <ul>
                                <li>
                                    <a href="#">Subitem 1</a>
                                </li>
                                <li>
                                    <a href="#">Subitem 2</a>
                                </li>
                                <li>
                                    <a href="#">Subitem 3</a>
                                </li>
                            </ul>
                        </li>
                        <li>
                            <a href="#">Menu Item 4</a>
                        </li>
                    </ul>
                </li>
                <li>
                    <a href="#">About</a>
                    <ul>
                        <li>
                            <a href="#">Menu Item 1</a>
                        </li>
                        <li>
                            <a href="#">Menu Item 2</a>
                        </li>
                        <li>
                            <a href="#">Menu Item 3</a>
                        </li>
                    </ul>
                </li>
                <li>
                    <a href="#">Products</a>
                    <ul>
                        <li class="submenu">
                            <a href="#">Menu Item 1</a>
                            <ul>
                                <li>
                                    <a href="#">Subitem 1</a>
                                </li>
                                <li>
                                    <a href="#">Subitem 2</a>
                                </li>
                                <li>
                                    <a href="#">Subitem 3</a>
                                </li>
                            </ul>
                        </li>
                        <li>
                            <a href="#">Menu Item 2</a>
                        </li>
                        <li>
                            <a href="#">Menu Item 3</a>
                        </li>
                    </ul>
                </li>
                <li>
                    <a href="#">Contact</a>
                </li>
            </ul>
        </nav>
    }
}
