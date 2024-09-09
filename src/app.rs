use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

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
    let count = RwSignal::new(0);
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-bug-double-suspend.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("/other") view=move || OtherPage(OtherPageProps{ count })/>
                </Routes>
            </main>
        </Router>
    }
}

#[server]
pub async fn get_thing(thing: ()) -> Result<(), ServerFnError> {
    Ok(thing)
}

#[component]
fn OtherPage(count: RwSignal<usize>) -> impl IntoView {
    let resource = Resource::new(|| (), get_thing);
    view! {
        <Suspense fallback=move || view!{<div>"Loading"</div>}>
            {move || {
                Suspend::new(async move {
                    count.update_untracked(|x| *x += 1);
                    log::debug!("In Suspend - called {} times", count.get_untracked());
                    let _ = resource.await;
                    view!{<div>"Loaded"</div>}
                })
            }}
        </Suspense>
        <a href="/">"Home"</a>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <a href="/other">"Other page"</a>
    }
}
