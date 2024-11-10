use leptos::*;
use leptos_meta::Stylesheet;
use tailwind_fuse::*;

#[component]
pub fn DemoParallax1() -> impl IntoView {
    view! {
        <Stylesheet id="parallax-1" href="/components/parallax-1.css" />

        <div class="mainDiv">

            <div class="overflow-y-auto overflow-x-hidden h-[100vh]" style="perspective: 5px;">

                <section
                    class="flex relative justify-center items-center my-24 first:mt-0 last:mb-0 parallax min-h-[300px] h-[60vh]"
                    style="transform-style: preserve-3d; z-index: -1;"
                >
                    <ParallaxDivImage class="background">
                        <img
                            src="https://images.pexels.com/photos/36717/amazing-animal-beautiful-beautifull.jpg?auto=compress&cs=tinysrgb&dpr=2&h=1080&w=1920"
                            alt="Background Image"
                            class="object-cover w-full h-full"
                        />
                    </ParallaxDivImage>
                    <ParallaxDivImage class="foreground">
                        <img
                            src="https://images.pexels.com/photos/1509534/pexels-photo-1509534.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=1080&w=1920"
                            alt="Foreground Image"
                            class="object-cover w-full h-full"
                        />
                    </ParallaxDivImage>
                    <h2 class="z-10 text-4xl font-bold">"Keep Scrolling 👇"</h2>
                </section>

                <section class="px-12 bg-blue-900 first:mt-0 last:mb-0 content-wrapper">
                    <div class="flex relative flex-col gap-4 mx-auto max-w-3xl w-[85%]">
                        <p>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec malesuada consectetur tortor vitae tincidunt. Sed luctus placerat lacinia. Vestibulum dignissim, nulla et semper dictum, diam nisl molestie lorem, eget feugiat ligula augue ut neque. Vestibulum quis dignissim risus, ut consectetur mi. Vivamus volutpat eros odio, eu aliquet nulla feugiat ut. Nulla in purus tristique, lobortis mi non, mattis ligula. Nunc turpis neque, tristique eu semper at, aliquet ac urna. Morbi varius quam efficitur, sollicitudin dui vitae, dignissim nisl. Curabitur fringilla aliquam felis vel vestibulum. Proin non rutrum turpis. Fusce turpis erat, vestibulum vel consectetur non, sodales et nibh. Praesent sit amet hendrerit eros. In a neque volutpat, iaculis ex at, convallis sem. Vestibulum dapibus nec nisi eget finibus. Donec vitae velit dignissim, semper arcu a, placerat sapien. Donec cursus purus enim, id euismod ipsum mattis eget.
                        </p>
                        <p>
                            Sed ac aliquet tortor. Quisque tempus nisi vel ligula commodo suscipit. Curabitur dui lorem, posuere ut facilisis et, mollis ut leo. Duis varius leo ante, vitae consectetur est egestas a. Fusce in nisl ut nibh dictum gravida sed sed nulla. Nunc ultricies lectus sit amet diam blandit sodales. Mauris fermentum mi dignissim diam venenatis tempor id quis nibh. Interdum et malesuada fames ac ante ipsum primis in faucibus. Sed vel ex commodo, mattis sem at, mattis nulla. Curabitur lobortis justo quis tellus sodales, eu suscipit augue laoreet. Cras pellentesque finibus nunc id efficitur. Aliquam aliquam quis magna sed lobortis. Pellentesque non consequat lectus. Duis eu justo ac tellus convallis accumsan.
                        </p>
                        <p>
                            Donec at luctus lorem. Cras venenatis semper massa, in viverra elit imperdiet ac. Nunc malesuada odio sed felis aliquam, in sagittis diam consectetur. Aliquam id nulla sed lacus pharetra volutpat id at erat. Phasellus tempor magna sit amet blandit convallis. Nullam id felis nisl. Vestibulum vulputate mauris nunc, vel ultrices mi mollis ut. Sed et convallis augue, a scelerisque enim. Sed massa metus, accumsan non porta in, lacinia vitae urna. Donec id convallis lorem. Nulla facilisi. Maecenas a pharetra massa. Suspendisse mattis in tellus non faucibus. Ut vitae egestas neque.
                        </p>
                        <p>
                            Nulla scelerisque nulla felis. Suspendisse laoreet leo vitae ornare mattis. Nam facilisis nunc at orci dictum, vitae tempor felis cursus. Suspendisse potenti. Ut eu nunc metus. Maecenas ac justo diam. Proin molestie orci id tellus molestie rutrum in in lorem. Nam efficitur metus lorem, a feugiat lacus porta quis. Praesent nec interdum turpis. Fusce efficitur, tortor et dictum aliquam, sem lectus ornare purus, quis bibendum neque ligula cursus magna. Proin ipsum massa, tempus sit amet consectetur ut, laoreet ut urna.
                        </p>
                        <p>
                            Praesent eu laoreet augue, vel dignissim tellus. Aliquam urna dui, condimentum quis lacinia at, pharetra id erat. Sed nec nisi sed lectus pharetra accumsan in quis tortor. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vivamus congue euismod nisl, at gravida arcu ultrices vitae. Aenean a urna a ipsum porttitor pulvinar convallis ac orci. Fusce malesuada id orci varius sodales. Nam semper ut lorem quis maximus. Suspendisse porta velit et purus tempor, dapibus varius nibh pretium. Duis viverra, tellus at auctor faucibus, elit nibh lacinia elit, ac interdum elit magna quis nisl. Sed elementum nec dolor efficitur dapibus. Nullam sed ante feugiat, aliquam ipsum nec, efficitur tellus. Sed blandit justo id tempor pulvinar. Cras metus turpis, efficitur vel aliquet sit amet, posuere quis nunc.
                        </p>
                        <p>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce finibus nisi augue, sit amet elementum sem mattis a. Sed lectus diam, lacinia sed mauris vitae, tristique hendrerit mi. Sed eget condimentum tellus. Donec sed libero posuere, imperdiet mauris tempor, sollicitudin est. In at posuere ipsum, sit amet pulvinar diam. Aliquam erat volutpat. In hac habitasse platea dictumst.
                        </p>
                    </div>
                </section>

                <section
                    class="flex relative justify-center items-center first:mt-0 last:mb-0 min-h-[300px] h-[60vh] parallax"
                    style="transform-style: preserve-3d; z-index: -1;"
                >
                    <ParallaxDivImage class="background">
                        <img
                            src="https://images.pexels.com/photos/1509534/pexels-photo-1509534.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=1080&w=1920"
                            alt="Background Image"
                            class="object-cover w-full h-full"
                        />
                    </ParallaxDivImage>
                    <ParallaxDivImage class="foreground">
                        <img
                            src="https://images.pexels.com/photos/36717/amazing-animal-beautiful-beautifull.jpg?auto=compress&cs=tinysrgb&dpr=2&h=1080&w=1920"
                            alt="Foreground Image"
                            class="object-cover w-full h-full"
                        />
                    </ParallaxDivImage>
                    <h2 class="relative z-10 text-5xl font-bold">The End!</h2>
                </section>
            </div>

        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn ParallaxDivImage(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children, // Accept children
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "absolute top-0 left-0 w-full h-full pointer-events-none",
            class()
        )
    });

    view! {
        <div {..attributes} class=class>
            // Render children
            {children()}
        </div>
    }
}
