use leptos::*;

// SOURCE : https://codepen.io/argyleink/pen/ZEdrzJZ

#[component]
pub fn DemoParallaxZoomWords() -> impl IntoView {
    view! {
        <div class="stuck-grid">
            <div class="grid-item">"oklch()"</div>
            <div class="grid-item">"scroll()"</div>
            <div class="grid-item">"text-box-trim"</div>
            <div class="grid-item">"pow()"</div>
            <div class="grid-item">"@property"</div>
            <div class="grid-item">"top-layer"</div>
            <div class="grid-item">"@view-transition"</div>
            <div class="grid-item">"var()"</div>
            <div class="grid-item">"clamp()"</div>
            <div class="grid-item">"view()"</div>
            <div class="grid-item special">
                <b>CSS</b>
            </div>
            <div class="grid-item">"@layer"</div>
            <div class="grid-item">"@swash"</div>
            <div class="grid-item">"subgrid"</div>
            <div class="grid-item">"in oklab"</div>
            <div class="grid-item">":popover-open"</div>
            <div class="grid-item">"abs()"</div>
            <div class="grid-item">"sin()"</div>
            <div class="grid-item">":has()"</div>
            <div class="grid-item">"::marker"</div>
            <div class="grid-item">"1cap"</div>
            <div class="grid-item">"scrollbar-color"</div>
            <div class="grid-item">"scroll-timeline"</div>
            <div class="grid-item">"view-timeline"</div>
            <div class="grid-item">"overlay"</div>
            <div class="grid-item">"scale"</div>
            <div class="grid-item">"ascent-override"</div>
            <div class="grid-item">"initial-letter"</div>
            <div class="grid-item">"inset"</div>
            <div class="grid-item">"@container"</div>
            <div class="grid-item">"accent-color"</div>
            <div class="grid-item">"color-mix()"</div>
            <div class="grid-item">"@scope"</div>
            <div class="grid-item">"@starting-style"</div>
            <div class="grid-item">"override-colors"</div>
            <div class="grid-item">"anchor()"</div>
            <div class="grid-item">"scroll-snap"</div>
            <div class="grid-item">"::backdrop"</div>
            <div class="grid-item">"::cue"</div>
            <div class="grid-item">":focus-visible"</div>
            <div class="grid-item">":user-valid"</div>
            <div class="grid-item">":fullscreen"</div>
            <div class="grid-item">":dir()"</div>
            <div class="grid-item">"caret-color"</div>
            <div class="grid-item">"aspect-ratio"</div>
            <div class="grid-item">"cross-fade()"</div>
            <div class="grid-item">"image-set()"</div>
            <div class="grid-item">"env()"</div>
            <div class="grid-item">"place-content"</div>
            <div class="grid-item">"gap"</div>
        </div>

        <style>
            {r#"
            @keyframes parallax-zoom-words {{
            0% {{
            transform: translateZ(-1000px);
            opacity: 0;
            filter: blur(5px);
            }}
            50% {{
            transform: translateZ(0px);
            opacity: 1;
            filter: blur(0px);
            }}
            100% {{
            transform: translateZ(1000px);
            opacity: 0;
            filter: blur(5px);
            }}
            }}
            
            .stuck-grid {{
            block-size: 100svh;
            perspective: 1000px;
            transform-style: preserve-3d;
            
            display: grid;
            grid: repeat(4, 25dvh) / repeat(4, 25dvw);;
            place-items: center;
            
            position: sticky;
            top: 0;
            
            overflow: clip;
            
            > .grid-item {{
            transform-style: preserve-3d;
            
            font-size: 5vmin;
            font-weight: lighter;
            
            text-wrap: nowrap;
            
            @supports (animation-timeline: scroll()) {{
            @media (prefers-reduced-motion: no-preference) {{
            animation: parallax-zoom-words linear both;
            animation-timeline: scroll(root block);
            will-change: transform, opacity, filter;
            }}
            }}
            
            &.special.special {{
            grid-row: 2 / span 2;
            grid-column: 2 / span 2;
            }}
            
            > b {{
            font-size: 15vmin;
            }}
            
            &:nth-of-type(1)  {{ animation-range: 40% 50% }}
            &:nth-of-type(2)  {{ animation-range: 20% 30% }}
            &:nth-of-type(3)  {{ animation-range: 52% 62% }}
            &:nth-of-type(4)  {{ animation-range: 50% 60% }}
            &:nth-of-type(5)  {{ animation-range: 45% 55% }}
            &:nth-of-type(6)  {{ animation-range: 10% 20% }}
            &:nth-of-type(7)  {{ animation-range: 90% 100% }}
            &:nth-of-type(8)  {{ animation-range: 30% 40% }}
            &:nth-of-type(9)  {{ animation-range: 80% 90% }}
            &:nth-of-type(10) {{ animation-range: 70% 80% }}
            &:nth-of-type(11) {{ animation-range: -10% 50% }}
            &:nth-of-type(12) {{ animation-range: 52% 62% }}
            &:nth-of-type(13) {{ animation-range: 15% 25% }}
            &:nth-of-type(14) {{ animation-range: 7% 17% }}
            &:nth-of-type(15) {{ animation-range: 75% 85% }}
            &:nth-of-type(16) {{ animation-range: 3% 13% }}
            &:nth-of-type(17) {{ animation-range: 87% 97% }}
            &:nth-of-type(18) {{ animation-range: 42% 52% }}
            &:nth-of-type(19) {{ animation-range: 57% 67% }}
            &:nth-of-type(20) {{ animation-range: 37% 47% }}
            &:nth-of-type(21) {{ animation-range: 12% 22% }}
            &:nth-of-type(22) {{ animation-range: 8% 18% }}
            &:nth-of-type(23) {{ animation-range: 84% 94% }}
            &:nth-of-type(24) {{ animation-range: 33% 43% }}
            &:nth-of-type(25) {{ animation-range: 48% 58% }}
            &:nth-of-type(26) {{ animation-range: 13% 23% }}
            &:nth-of-type(27) {{ animation-range: 78% 88% }}
            &:nth-of-type(28) {{ animation-range: 62% 72% }}
            &:nth-of-type(29) {{ animation-range: 31% 41% }}
            &:nth-of-type(30) {{ animation-range: 8% 18% }}
            &:nth-of-type(31) {{ animation-range: 4% 14% }}
            &:nth-of-type(32) {{ animation-range: 74% 84% }}
            &:nth-of-type(33) {{ animation-range: 61% 71% }}
            &:nth-of-type(34) {{ animation-range: 26% 36% }}
            &:nth-of-type(35) {{ animation-range: 63% 73% }}
            &:nth-of-type(36) {{ animation-range: 11% 21% }}
            &:nth-of-type(37) {{ animation-range: 89% 99% }}
            &:nth-of-type(38) {{ animation-range: 33% 43% }}
            &:nth-of-type(39) {{ animation-range: 88% 98% }}
            &:nth-of-type(40) {{ animation-range: 22% 32% }}
            &:nth-of-type(41) {{ animation-range: 16% 26% }}
            &:nth-of-type(42) {{ animation-range: 26% 36% }}
            &:nth-of-type(43) {{ animation-range: 66% 76% }}
            &:nth-of-type(44) {{ animation-range: 3% 13% }}
            &:nth-of-type(45) {{ animation-range: 44% 54% }}
            &:nth-of-type(46) {{ animation-range: 11% 21% }}
            &:nth-of-type(47) {{ animation-range: 23% 33% }}
            &:nth-of-type(48) {{ animation-range: 39% 49% }}
            &:nth-of-type(49) {{ animation-range: 59% 69% }}
            &:nth-of-type(50) {{ animation-range: 6% 16% }}
            
            @supports (animation-timeline: scroll()) {{
            
            &:nth-of-type(1) {{ grid-area: 1/1 }}
            &:nth-of-type(2) {{ grid-area: 1/2 }}
            &:nth-of-type(3) {{ grid-area: 1/3 }}
            &:nth-of-type(4) {{ grid-area: 1/4 }}
            &:nth-of-type(5) {{ grid-area: 2/1 }}
            &:nth-of-type(6) {{ grid-area: 2/2 }}
            &:nth-of-type(7) {{ grid-area: 2/3 }}
            &:nth-of-type(8) {{ grid-area: 2/4 }}
            &:nth-of-type(9) {{ grid-area: 3/1 }}
            &:nth-of-type(10) {{ grid-area: 3/2 }}
            &:nth-of-type(11) {{ grid-area: 3/3 }}
            &:nth-of-type(12) {{ grid-area: 3/4 }}
            &:nth-of-type(13) {{ grid-area: 4/1 }}
            &:nth-of-type(14) {{ grid-area: 4/2 }}
            &:nth-of-type(15) {{ grid-area: 4/3 }}
            &:nth-of-type(16) {{ grid-area: 4/4 }}
            &:nth-of-type(17) {{ grid-area: 2/1 }}
            &:nth-of-type(18) {{ grid-area: 2/2 }}
            &:nth-of-type(19) {{ grid-area: 2/3 }}
            &:nth-of-type(20) {{ grid-area: 2/4 }}
            &:nth-of-type(21) {{ grid-area: 3/1 }}
            &:nth-of-type(22) {{ grid-area: 3/2 }}
            &:nth-of-type(23) {{ grid-area: 3/3 }}
            &:nth-of-type(24) {{ grid-area: 3/4 }}
            &:nth-of-type(25) {{ grid-area: 1/1 }}
            &:nth-of-type(26) {{ grid-area: 1/2 }}
            &:nth-of-type(27) {{ grid-area: 1/3 }}
            &:nth-of-type(28) {{ grid-area: 1/4 }}
            &:nth-of-type(29) {{ grid-area: 4/1 }}
            &:nth-of-type(30) {{ grid-area: 4/2 }}
            &:nth-of-type(31) {{ grid-area: 4/3 }}
            &:nth-of-type(32) {{ grid-area: 4/4 }}
            &:nth-of-type(33) {{ grid-area: 2/1 }}
            &:nth-of-type(34) {{ grid-area: 2/2 }}
            &:nth-of-type(35) {{ grid-area: 2/3 }}
            &:nth-of-type(36) {{ grid-area: 2/4 }}
            &:nth-of-type(37) {{ grid-area: 3/1 }}
            &:nth-of-type(38) {{ grid-area: 3/2 }}
            &:nth-of-type(39) {{ grid-area: 3/3 }}
            &:nth-of-type(40) {{ grid-area: 3/4 }}
            &:nth-of-type(41) {{ grid-area: 1/1 }}
            &:nth-of-type(42) {{ grid-area: 1/2 }}
            &:nth-of-type(43) {{ grid-area: 1/3 }}
            &:nth-of-type(44) {{ grid-area: 1/4 }}
            &:nth-of-type(45) {{ grid-area: 4/1 }}
            &:nth-of-type(46) {{ grid-area: 4/2 }}
            &:nth-of-type(47) {{ grid-area: 4/3 }}
            &:nth-of-type(48) {{ grid-area: 4/4 }}
            &:nth-of-type(49) {{ grid-area: 3/1 }}
            &:nth-of-type(50) {{ grid-area: 3/2 }}
            &:nth-of-type(51) {{ grid-area: 3/3 }}
            &:nth-of-type(52) {{ grid-area: 3/4 }}
            }}
            }}
            }}
            "#}
        </style>
    }
}