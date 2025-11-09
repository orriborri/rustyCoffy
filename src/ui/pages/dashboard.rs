use dioxus::prelude::*;
use crate::ui::state::AppState;

#[cfg(feature = "database")]
use crate::models::BrewingSession;

/// Dashboard component with recent sessions overview and quick statistics
#[component]
pub fn Dashboard() -> Element {
    let app_state = use_context::<AppState>();
    
    rsx! {
        div {
            class: "page dashboard-page",
            h2 { "Dashboard" }
            
            div { class: "dashboard-content",
                {render_dashboard(&app_state)}
            }
        }
    }
}

#[cfg(feature = "database")]
fn render_dashboard(app_state: &AppState) -> Element {
    let sessions = &app_state.sessions;
    let beans = &app_state.beans;
    let grinders = &app_state.grinders;
    
    // Calculate statistics
    let total_sessions = sessions.len();
    let avg_rating = if !sessions.is_empty() {
        sessions.iter()
            .filter_map(|s| s.rating)
            .sum::<f32>() / sessions.iter().filter(|s| s.rating.is_some()).count() as f32
    } else {
        0.0
    };
    
    let recent_sessions: Vec<&BrewingSession> = sessions.iter()
        .rev()
        .take(5)
        .collect();
    
    // Find best rated session
    let best_session = sessions.iter()
        .filter(|s| s.rating.is_some())
        .max_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap());
    
    rsx! {
        div { class: "dashboard-grid",
            section { class: "dashboard-card stats-card",
                h3 { "Quick Statistics" }
                div { class: "stats-grid",
                    div { class: "stat-item",
                        div { class: "stat-value", "{total_sessions}" }
                        div { class: "stat-label", "Total Sessions" }
                    }
                    div { class: "stat-item",
                        div { class: "stat-value", "{beans.len()}" }
                        div { class: "stat-label", "Coffee Beans" }
                    }
                    div { class: "stat-item",
                        div { class: "stat-value", "{grinders.len()}" }
                        div { class: "stat-label", "Grinders" }
                    }
                    div { class: "stat-item",
                        div { class: "stat-value", "{avg_rating:.1}" }
                        div { class: "stat-label", "Avg Rating" }
                    }
                }
            }
            
            section { class: "dashboard-card recent-sessions-card",
                h3 { "Recent Brewing Sessions" }
                if recent_sessions.is_empty() {
                    p { "No sessions yet. Create your first brew!" }
                } else {
                    div { class: "recent-sessions-list",
                        for session in recent_sessions.iter() {
                            RecentSessionItem { session: (*session).clone() }
                        }
                    }
                }
            }
            
            section { class: "dashboard-card suggestions-card",
                h3 { "Brewing Suggestions" }
                if let Some(best) = best_session {
                    div { class: "suggestion",
                        h4 { "🏆 Your Best Brew" }
                        p { "Session #{best.id} with rating {best.rating.unwrap():.1}" }
                        p { class: "suggestion-detail",
                            "Method: {best.brewing_method:?}, Grind: {best.grind_setting}, Ratio: {(best.water_ml / best.coffee_grams):.1}:1"
                        }
                    }
                } else {
                    p { "Create more sessions to get personalized suggestions!" }
                }
                
                if total_sessions > 0 {
                    div { class: "suggestion",
                        h4 { "💡 Tip" }
                        p { "Try experimenting with different grind settings to find your perfect brew." }
                    }
                }
            }
            
            section { class: "dashboard-card inventory-card",
                h3 { "Inventory Status" }
                if beans.is_empty() {
                    p { "No beans in inventory. Add some in Equipment page!" }
                } else {
                    div { class: "inventory-list",
                        for bean in beans.iter() {
                            if let Some(remaining) = bean.remaining_grams {
                                div {
                                    class: if remaining < 50.0 { "inventory-item low-stock" } else { "inventory-item" },
                                    span { class: "bean-name", "{bean.name}" }
                                    span { class: "bean-remaining", "{remaining:.0}g" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(not(feature = "database"))]
fn render_dashboard(_app_state: &AppState) -> Element {
    rsx! {
        div { class: "empty-state",
            p { "Database features not enabled" }
        }
    }
}

#[cfg(feature = "database")]
#[component]
fn RecentSessionItem(session: BrewingSession) -> Element {
    use dioxus_router::prelude::*;
    use crate::ui::app::Route;
    
    rsx! {
        Link {
            to: Route::SessionDetail { id: session.id.to_string() },
            class: "recent-session-item",
            
            div { class: "session-info",
                span { class: "session-id", "#{session.id}" }
                span { class: "session-method", "{session.brewing_method:?}" }
                span { class: "session-date", "{session.created_at.format(\"%m/%d %H:%M\")}" }
            }
            
            if let Some(rating) = session.rating {
                div { class: "session-rating",
                    span { class: "stars", "★".repeat(rating.floor() as usize) }
                    span { class: "rating-value", "{rating:.1}" }
                }
            }
        }
    }
}
