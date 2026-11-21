use leptos::prelude::*;
use zuwaigani_core::capabilities::Capabilities;

const CAPABILITIES_JSON: &str = include_str!("../../data/capabilities.json");

/// Which checks each platform supports.
#[component]
pub fn PlatformsPage() -> impl IntoView {
    let capabilities: Capabilities = serde_json::from_str(CAPABILITIES_JSON)
        .expect("data/capabilities.json matches the core types");

    let header = capabilities
        .platforms
        .iter()
        .map(|p| view! { <th scope="col">{p.to_string()}</th> })
        .collect_view();

    let rows = capabilities
        .checks
        .iter()
        .map(|c| {
            let cells = capabilities
                .platforms
                .iter()
                .map(|p| {
                    if c.supported_by.contains(p) {
                        view! { <td><span class="badge badge-success">"Yes"</span></td> }
                    } else {
                        view! { <td><span class="badge badge-error">"No"</span></td> }
                    }
                })
                .collect_view();
            view! {
                <tr>
                    <th scope="row" class="font-mono">{c.check.to_string()}</th>
                    {cells}
                </tr>
            }
        })
        .collect_view();

    view! {
        <h1 class="text-3xl font-bold">"Platforms"</h1>
        <p class="py-2 opacity-70">"Which checks each platform supports."</p>
        <div class="card bg-base-100 shadow-sm mt-4">
            <div class="card-body overflow-x-auto">
                <table class="table table-zebra">
                    <thead>
                        <tr>
                            <th scope="col">"check"</th>
                            {header}
                        </tr>
                    </thead>
                    <tbody>{rows}</tbody>
                </table>
            </div>
        </div>
    }
}
