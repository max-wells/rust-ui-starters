use leptos::*;

#[component]
pub fn DemoBackgroundRetro() -> impl IntoView {
    view! {
        <div class="flex overflow-hidden relative justify-center items-center p-20 w-full h-full rounded-lg border md:shadow-xl min-h-[300px] bg-background">
            <p class="z-10 text-5xl font-medium tracking-tighter text-center text-black whitespace-pre-wrap dark:text-white">
                Retro
            </p>
            <div class="absolute w-full h-full opacity-50 [perspective:200px]">
                <div class="absolute inset-0 [transform:rotateX(35deg)]">
                    <div class="animate-grid [background-image:linear-gradient(to_right,rgba(0,0,0,0.3)_1px,transparent_0),linear-gradient(to_bottom,rgba(0,0,0,0.3)_1px,transparent_0)] [background-repeat:repeat] [background-size:60px_60px] [height:300vh] [inset:0%_0px] [margin-left:-50%] [transform-origin:100%_0_0] [width:600vw] dark:[background-image:linear-gradient(to_right,rgba(255,255,255,0.2)_1px,transparent_0),linear-gradient(to_bottom,rgba(255,255,255,0.2)_1px,transparent_0)]"></div>
                </div>
                <div class="absolute inset-0 bg-gradient-to-t from-white to-transparent dark:from-black to-90%"></div>
            </div>
        </div>
    }
}
