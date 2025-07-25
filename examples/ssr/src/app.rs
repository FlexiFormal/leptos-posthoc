use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
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
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/axum-test.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    use thaw::*;

    view! {
        <ConfigProvider>
            <h1>"Welcome to Leptos!"</h1>
            <SomeComplicatedComponent/>
        </ConfigProvider>
    }
}

#[server]
async fn get_some_html() -> Result<String, ServerFnError> {
    Ok(r#"
    <div>
      <div>Some HTML</div>
      <div data-replace-with-leptos class="test-class" style="width:fit-content;margin:auto">
          This text should be wrapped in a red border, and, for good measure, show something on hover.<br>
          We can also nest this:
          <div data-replace-with-leptos class="test-class" style="width:fit-content;margin:auto">
            Here is another div that should also get bounded the same way
          </div>
          and we're back to the first level.
        </div>
      <div>This text should not be touched at all.</div>
    </div>"#.to_string())
}

use leptos_posthoc::*;

#[component]
fn SomeComplicatedComponent() -> impl IntoView {
    let html = Resource::new(|| (), |_| get_some_html());
    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            {move || {
                html.get().map(|html| html.ok().map( |html| {
                    view! { <span>"Here:"
                        <DomStringCont html cont=replace/>
                    </span> }
                }))
            }}
        </Suspense>
    }
}

#[component]
fn MyReplacementComponent(orig: OriginalNode) -> impl IntoView {
    use thaw::*;
    let cont = view!(<DomCont skip_head=true orig cont=replace/>)
        .add_any_attr(leptos::tachys::html::style::style("border: 1px solid red"))
        .add_any_attr(leptos::tachys::html::class::class("foo-bar"))
        .add_any_attr(leptos::tachys::html::attribute::custom::custom_attribute(
            "data-foo", "bar",
        ));
    view! {
        //<div><div style="border: 1px solid red;width:fit-content;margin:auto">
          <Popover>
              <PopoverTrigger slot>
                  {cont}
              </PopoverTrigger>
              <div style="border: 1px solid black;font-weight:bold;">"IT WORKS!"</div>
          </Popover>
       //</div></div>
    }
}

fn replace(
    e: &leptos::web_sys::Element,
) -> (Option<impl FnOnce() -> AnyView>, Option<impl FnOnce()>) {
    let node = e.clone();
    leptos::logging::log!("open");
    leptos::web_sys::console::log_1(&node);
    (
        e.get_attribute("data-replace-with-leptos").map(|_| {
            let orig = e.clone().into();
            || view!(<MyReplacementComponent orig/>).into_any()
        }),
        Some(move || {
            leptos::logging::log!("close");
            leptos::web_sys::console::log_1(&node);
        }),
    )
}
