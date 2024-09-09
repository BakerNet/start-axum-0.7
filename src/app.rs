use codee::string::FromToStringCodec;
use leptos::{html::Div, logging::log, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos_use::{use_websocket, UseWebSocketReturn};

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
    tokio::time::sleep(std::time::Duration::from_millis(25)).await;
    Ok(thing)
}

#[component]
fn OtherPage(count: RwSignal<usize>) -> impl IntoView {
    let resource = Resource::new(|| (), get_thing);
    let div_ref = NodeRef::<Div>::new();

    Effect::new(move || {
        // get_untracked will only be Some on the hydration, not on CSR route
        if let Some(div) = div_ref.get_untracked() {
            div.set_text_content(Some(&format!(
                "Frontend Suspend rendered {} times",
                count.get_untracked()
            )));
        }
    });

    view! {
        <Suspense fallback=move || view!{<div>"Loading"</div>}>
            {move || {
                Suspend::new(async move {
                    let _ = resource.await;
                    count.update_untracked(|x| *x += 1);
                    view!{<OtherInner count />}
                })
            }}
        </Suspense>
        <div node_ref=div_ref></div>
        <a href="/">"Home"</a>
    }
}

#[component]
fn OtherInner(count: RwSignal<usize>) -> impl IntoView {
    log!("rendering OtherInner");
    let UseWebSocketReturn { ready_state, .. } =
        use_websocket::<String, String, FromToStringCodec>("wss://echo.websocket.org/");

    Effect::watch(
        move || ready_state.get(),
        move |state, _, _| {
            log!("State: {:?}", state);
        },
        true,
    );

    view! {<div>{format!("OtherPage Suspend has been rendered {} times", count.get_untracked())}</div>}
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
