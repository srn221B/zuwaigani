use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment,
};

use crate::pages::{HomePage, PlatformsPage};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/frontend.css"/>
        <Title text="zuwaigani"/>

        <Router>
            <header class="navbar bg-base-100 shadow-sm">
                <div class="flex-1">
                    <A href="/" attr:class="btn btn-ghost text-xl">"zuwaigani"</A>
                </div>
                <ul class="menu menu-horizontal px-1">
                    <li>
                        <A href="/platforms">"Platforms"</A>
                    </li>
                </ul>
            </header>
            <main class="mx-auto max-w-4xl p-6">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("platforms") view=PlatformsPage/>
                </Routes>
            </main>
        </Router>
    }
}
