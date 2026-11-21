use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="hero py-16">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <h1 class="text-5xl font-bold">"zuwaigani"</h1>
                    <p class="py-6">"Data quality results, checked on every platform."</p>
                    <A href="/platforms" attr:class="btn btn-primary">
                        "See supported checks"
                    </A>
                </div>
            </div>
        </div>
    }
}
