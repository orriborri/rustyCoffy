use dioxus::prelude::*;
use dioxus_router::{Router, Link, Outlet, Routable};
use crate::ui::state::AppState;
use crate::ui::pages::{Sessions, SessionDetail, Equipment, Statistics, NewBrew, Dashboard};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/dashboard")]
        Dashboard {},
        #[route("/brew")]
        NewBrew {},
        #[route("/sessions")]
        Sessions {},
        #[route("/sessions/:id")]
        SessionDetail { id: String },
        #[route("/equipment")]
        Equipment {},
        #[route("/statistics")]
        Statistics {},
}

#[component]
fn NavBar() -> Element {
    rsx! {
        nav {
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::NewBrew {}, "New Brew" }
            Link { to: Route::Sessions {}, "Sessions" }
            Link { to: Route::Equipment {}, "Equipment" }
            Link { to: Route::Statistics {}, "Statistics" }
        }
        Outlet::<Route> {}
    }
}

/// Main Dioxus app component with routing and global state management
#[allow(non_snake_case)]
pub fn App() -> Element {
    use_context_provider(|| AppState::new());
    let app_state = use_context::<AppState>();

    rsx! {
        div { class: "app-container",
            if app_state.loading {
                div { class: "loading-overlay",
                    div { class: "loading-spinner" }
                    p { "Loading..." }
                }
            }
            
            if let Some(error) = &app_state.error {
                div { class: "global-error",
                    span { "⚠️ {error}" }
                    button {
                        onclick: move |_| {
                            #[cfg(feature = "database")]
                            {
                                let mut state = app_state.clone();
                                state.clear_error();
                            }
                        },
                        "Dismiss"
                    }
                }
            }
            
            Router::<Route> {}
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "page home-page",
            h1 { "Coffee Brewing Tracker" }
            p { "Welcome to your coffee brewing companion!" }
            
            div { class: "home-actions",
                Link {
                    to: Route::Dashboard {},
                    class: "btn-primary",
                    "Go to Dashboard"
                }
                Link {
                    to: Route::NewBrew {},
                    class: "btn-primary",
                    "Create New Brew"
                }
            }
        }
    }
}
