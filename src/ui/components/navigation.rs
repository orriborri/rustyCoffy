use dioxus::prelude::*;
use dioxus_router::Link;
use crate::ui::app::Route;

/// Navigation component with proper routing links
#[component]
pub fn Navigation() -> Element {
    rsx! {
        nav {
            class: "navigation",
            div {
                class: "nav-brand",
                h1 { "☕ Coffee Brewing Tracker" }
            }
            ul {
                class: "nav-links",
                li {
                    Link {
                        to: Route::Home {},
                        class: "nav-link",
                        "Home"
                    }
                }
                li {
                    Link {
                        to: Route::Dashboard {},
                        class: "nav-link",
                        "Dashboard"
                    }
                }
                li {
                    Link {
                        to: Route::NewBrew {},
                        class: "nav-link",
                        "New Brew"
                    }
                }
                li {
                    Link {
                        to: Route::Sessions {},
                        class: "nav-link",
                        "Sessions"
                    }
                }
                li {
                    Link {
                        to: Route::Equipment {},
                        class: "nav-link",
                        "Equipment"
                    }
                }
                li {
                    Link {
                        to: Route::Statistics {},
                        class: "nav-link",
                        "Statistics"
                    }
                }
            }
        }
    }
}
