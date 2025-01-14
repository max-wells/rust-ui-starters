use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoTriggeredHighlight() -> impl IntoView {
    view! {
        <Stylesheet id="triggered-highlight" href="/components/triggered-highlight.css" />
        <script src="/components/triggered-highlight.js" />

        <header>
            <button aria-pressed="true">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    class="w-6 h-6"
                >
                    <path
                        fill-rule="evenodd"
                        d="M9.528 1.718a.75.75 0 01.162.819A8.97 8.97 0 009 6a9 9 0 009 9 8.97 8.97 0 003.463-.69.75.75 0 01.981.98 10.503 10.503 0 01-9.694 6.46c-5.799 0-10.5-4.701-10.5-10.5 0-4.368 2.667-8.112 6.46-9.694a.75.75 0 01.818.162z"
                        clip-rule="evenodd"
                    />
                    <path d="M12 2.25a.75.75 0 01.75.75v2.25a.75.75 0 01-1.5 0V3a.75.75 0 01.75-.75zM7.5 12a4.5 4.5 0 119 0 4.5 4.5 0 01-9 0zM18.894 6.166a.75.75 0 00-1.06-1.06l-1.591 1.59a.75.75 0 101.06 1.061l1.591-1.59zM21.75 12a.75.75 0 01-.75.75h-2.25a.75.75 0 010-1.5H21a.75.75 0 01.75.75zM17.834 18.894a.75.75 0 001.06-1.06l-1.59-1.591a.75.75 0 10-1.061 1.06l1.59 1.591zM12 18a.75.75 0 01.75.75V21a.75.75 0 01-1.5 0v-2.25A.75.75 0 0112 18zM7.758 17.303a.75.75 0 00-1.061-1.06l-1.591 1.59a.75.75 0 001.06 1.061l1.591-1.59zM6 12a.75.75 0 01-.75.75H3a.75.75 0 010-1.5h2.25A.75.75 0 016 12zM6.697 7.757a.75.75 0 001.06-1.06l-1.59-1.591a.75.75 0 00-1.061 1.06l1.59 1.591z" />
                </svg>
            </button>
            <svg viewBox="0 0 76 65" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M37.5274 0L75.0548 65H0L37.5274 0Z" fill="#000000" />
            </svg>
        </header>
        <main>
            <section>
                <h1>Next.js on Vercel</h1>
                <span>
                    Vercel is the native Next.js platform, designed to enhance the
                    Next.js experience.
                </span>
                <p>
                    Next.js is a fullstack React framework for the web, maintained by
                    Vercel.
                </p>
                <p>
                    While Next.js works when self-hosting, deploying to Vercel is
                    zero-configuration and provides additional enhancements for
                    <mark data-author="JT">
                        <span>scalability, availability, and performance globally.</span>
                    </mark>
                </p>
            </section>
            <section>
                <h2 id="getting-started">Getting started</h2>
                <p>There are multiple ways to get started with Next.js on Vercel:</p>
                <ul>
                    <li>
                        If you already have a project with Next.js, install Vercel CLI and
                        run the vercel command from your project&#39;s root directory
                    </li>
                    <li>
                        Clone one of our Next.js example repos to your favorite git provider
                        and deploy it on Vercel with the button below:
                    </li>
                    <li>Or, choose a template from Vercel&#39;s marketplace:</li>
                </ul>
                <p>
                    <mark data-author="RH">
                        <span>
                            Vercel deployments can integrate with your git provider to
                            generate preview URLs for each pull request you make
                        </span>
                    </mark>
                    to your Next.js project.
                </p>
            </section>
            <section>
                <h2 id="incremental-static-regeneration">Incremental Static Regeneration</h2>
                <p>
                    Incremental Static Regeneration (ISR) allows you to
                    <mark data-author="AB" style="--hue: 280">
                        <span>create or update content without redeploying your site</span>
                    </mark>. ISR has three main benefits for developers: better performance,
                    improved security, and faster build times.
                </p>
                <p>
                    When self-hosting, (ISR) is limited to a single region workload.
                    Statically generated pages are not distributed closer to visitors by
                    default, without additional configuration or vendoring of a CDN. By
                    default, self-hosted ISR does not persist generated pages to durable
                    storage. Instead, these files are located in the Next.js cache (which
                    expires).
                </p>
                <p>
                    <mark data-author="GR" style="--hue: 30">
                        <span>
                            To enable ISR with Next.js in the app router, add an options
                            object with a revalidate property to your fetch requests
                        </span>
                    </mark>
                    .
                </p>
                <p>To summarize, using ISR with Next.js on Vercel:</p>
                <ul>
                    <li>Better performance with our global Edge Network</li>
                    <li>Zero-downtime rollouts to previously statically generated pages</li>
                    <li>
                        Framework-aware infrastructure enables global content updates in
                        300ms
                    </li>
                    <li>Generated pages are both cached and persisted to durable storage</li>
                </ul>
            </section>
            <section>
                <h2 id="server-side-rendering-ssr-">Server-Side Rendering (SSR)</h2>
                <p>
                    Server-Side Rendering (SSR) allows you to render pages dynamically on
                    the server. This is useful for pages where the rendered data needs to
                    be unique on every request. For example, checking authentication or
                    looking at the location of an incoming request.
                </p>
                <p>
                    On Vercel, <mark data-author="LR" style="--hue: 220">
                        <span>
                            you can server-render Next.js applications in either the Node.js
                            runtime (default) with Serverless Functions or the Edge runtime
                            with Edge Functions
                        </span>
                    </mark>. This allows you to pick the best rendering strategy on a per-page
                    basis.
                </p>
                <p>To summarize, SSR with Next.js on Vercel:</p>
                <ul>
                    <li>Scales to zero when not in use</li>
                    <li>Scales automatically with traffic increases</li>
                    <li>
                        Has zero-configuration support for Cache-Control headers, including
                        stale-while-revalidate
                    </li>
                    <li>
                        Framework-aware infrastructure enables switching rendering between
                        Edge/Node.js runtimes
                    </li>
                </ul>
            </section>
            <section>
                <h2 id="image-optimization">Image Optimization</h2>
                <p>
                    Image Optimization helps you achieve faster page loads by reducing the
                    size of images and using modern image formats.
                </p>
                <p>
                    When deploying to Vercel, <mark data-author="MN" style="--hue: 140">
                        <span>images are automatically optimized on demand</span>
                    </mark>, keeping your build times fast while improving your page load
                    performance and Core Web Vitals.
                </p>
                <p>
                    When self-hosting, Image Optimization uses the default Next.js server
                    for optimization. This server manages the rendering of pages and
                    serving of static files.
                </p>
                <p>
                    To use Image Optimization with Next.js on Vercel,
                    <mark data-author="IR" style="--hue: 10">
                        <span>
                            import the next/image component into the component you&#39;d like
                            to add an image to
                        </span>
                    </mark>.
                </p>
                <p>To summarize, using Image Optimization with Next.js on Vercel:</p>
                <ul>
                    <li>Zero-configuration Image Optimization when using next/image</li>
                    <li>Helps your team ensure great performance by default</li>
                    <li>
                        <mark data-author="SH" style="--hue: 190">
                            <span>Keeps your builds fast by optimizing images on-demand</span>
                        </mark>
                    </li>
                    <li>Requires No additional services needed to procure or set up</li>
                </ul>
            </section>
            <hr />
            <p>
                <a href="https://vercel.com/docs/frameworks/nextjs">
                    Check out the Vercel Docs for more.
                </a>
            </p>
        </main>
    }
}
