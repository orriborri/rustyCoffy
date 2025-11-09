use dioxus::prelude::*;
use crate::ui::state::AppState;

#[cfg(feature = "database")]
use crate::models::BrewingSession;

/// Session detail page showing comprehensive session information
#[component]
pub fn SessionDetail(id: String) -> Element {
    let app_state = use_context::<AppState>();
    let mut duplicate_msg = use_signal(|| String::new());
    
    rsx! {
        div {
            class: "page session-detail-page",
            h2 { "Brewing Session Details" }
            
            if !duplicate_msg().is_empty() {
                div { class: "success-message", "{duplicate_msg()}" }
            }
            
            {render_session_detail(&app_state, &id, duplicate_msg)}
        }
    }
}

#[cfg(feature = "database")]
fn render_session_detail(app_state: &AppState, id: &str, mut duplicate_msg: Signal<String>) -> Element {
    let session = app_state.sessions.iter()
        .find(|s| s.id.to_string() == id);
    
    if let Some(session) = session {
        rsx! {
            SessionDetailView {
                session: session.clone(),
                on_duplicate: move |_| {
                    duplicate_msg.set("Session duplicated! Go to New Brew to create it.".to_string());
                    
                    spawn(async move {
                        gloo_timers::future::TimeoutFuture::new(3000).await;
                        duplicate_msg.set(String::new());
                    });
                }
            }
        }
    } else {
        rsx! {
            div { class: "error-message",
                p { "Session not found" }
            }
        }
    }
}

#[cfg(not(feature = "database"))]
fn render_session_detail(_app_state: &AppState, _id: &str, _duplicate_msg: Signal<String>) -> Element {
    rsx! {
        p { "Database features not enabled" }
    }
}

#[cfg(feature = "database")]
#[component]
fn SessionDetailView(session: BrewingSession, on_duplicate: EventHandler<()>) -> Element {
    use dioxus_router::prelude::*;
    use crate::ui::app::Route;
    
    let rating_stars = (0..5).map(|i| {
        if session.rating.unwrap_or(0.0) >= (i + 1) as f32 {
            "★"
        } else if session.rating.unwrap_or(0.0) >= (i as f32 + 0.5) {
            "⯨"
        } else {
            "☆"
        }
    }).collect::<String>();
    
    rsx! {
        div { class: "session-detail-content",
            div { class: "detail-header",
                h3 { "Session #{session.id}" }
                span { class: "session-date", "{session.created_at.format(\"%B %d, %Y at %H:%M\")}" }
            }
            
            div { class: "detail-sections",
                section { class: "detail-section",
                    h4 { "Brewing Parameters" }
                    div { class: "detail-grid",
                        div { class: "detail-item",
                            span { class: "label", "Method:" }
                            span { class: "value", "{session.brewing_method:?}" }
                        }
                        div { class: "detail-item",
                            span { class: "label", "Coffee Amount:" }
                            span { class: "value", "{session.coffee_grams}g" }
                        }
                        div { class: "detail-item",
                            span { class: "label", "Water Amount:" }
                            span { class: "value", "{session.water_ml}ml" }
                        }
                        div { class: "detail-item",
                            span { class: "label", "Coffee-to-Water Ratio:" }
                            span { class: "value", "{(session.water_ml / session.coffee_grams):.1}:1" }
                        }
                        div { class: "detail-item",
                            span { class: "label", "Grind Setting:" }
                            span { class: "value", "{session.grind_setting}" }
                        }
                        if let Some(brew_time) = session.brew_time_seconds {
                            div { class: "detail-item",
                                span { class: "label", "Brew Time:" }
                                span { class: "value", "{brew_time}s ({brew_time / 60}m {brew_time % 60}s)" }
                            }
                        }
                        if let Some(temp) = session.water_temp_celsius {
                            div { class: "detail-item",
                                span { class: "label", "Water Temperature:" }
                                span { class: "value", "{temp}°C" }
                            }
                        }
                    }
                }
                
                section { class: "detail-section",
                    h4 { "Quality Assessment" }
                    if let Some(rating) = session.rating {
                        div { class: "rating-display-large",
                            span { class: "stars", "{rating_stars}" }
                            span { class: "rating-value", "{rating:.1} / 5.0" }
                        }
                    } else {
                        p { "No rating provided" }
                    }
                    
                    if let Some(notes) = &session.tasting_notes {
                        div { class: "tasting-notes",
                            h5 { "Tasting Notes" }
                            p { "{notes}" }
                        }
                    }
                }
                
                section { class: "detail-section",
                    h4 { "Equipment Used" }
                    div { class: "detail-grid",
                        div { class: "detail-item",
                            span { class: "label", "Bean ID:" }
                            span { class: "value", "#{session.bean_id}" }
                        }
                        div { class: "detail-item",
                            span { class: "label", "Grinder ID:" }
                            span { class: "value", "#{session.grinder_id}" }
                        }
                    }
                }
            }
            
            div { class: "session-actions",
                button {
                    class: "btn-primary",
                    onclick: move |_| on_duplicate.call(()),
                    "Duplicate This Session"
                }
                Link {
                    to: Route::Sessions {},
                    class: "btn-secondary",
                    "Back to Sessions"
                }
            }
        }
    }
}
