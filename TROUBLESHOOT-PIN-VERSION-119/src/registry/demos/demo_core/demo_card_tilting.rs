use leptos::*;

#[component]
pub fn DemoCardTilting() -> impl IntoView {
    view! {
        <div class="grid place-items-center min-h-[500px]">

            <div class="relative tilting-card-wrapper" style="width: 15rem; aspect-ratio: 1 / 2;">
                <div class="absolute mouse-position-tracker" />
                <div class="absolute mouse-position-tracker" />
                <div class="absolute mouse-position-tracker" />
                <div class="absolute mouse-position-tracker" />
                <div class="absolute mouse-position-tracker" />
                <div class="absolute mouse-position-tracker" />
                <div class="absolute mouse-position-tracker" />
                <div class="absolute mouse-position-tracker" />
                <div class="absolute mouse-position-tracker" />
                <div
                    class="grid absolute inset-0 place-content-center text-center bg-neutral-500 tilting-card-body"
                    style="background-image: url('https://images.unsplash.com/photo-1528321917418-af00f8823ded?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=MnwzMjM4NDZ8MHwxfHJhbmRvbXx8fHx8fHx8fDE2NTg0MjY1Mzg&ixlib=rb-1.2.1&q=80&w=800'); background-size: cover; background-position: center;"
                >
                    <h1 class="p-0.5 m-0 bg-neutral-300">Tilting Card</h1>
                    <p class="p-0.5 m-0 bg-neutral-300">With CSS only</p>
                </div>
            </div>
        </div>
    }
}
