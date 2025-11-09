use dioxus::prelude::*;
use crate::ui::state::AppState;

#[cfg(feature = "database")]
use crate::models::{BrewingSession, BrewingMethod, GrinderType};

/// Sessions page with filterable and searchable session list
#[component]
pub fn Sessions() -> Element {
    let app_state = use_context::<AppState>();
    let mut filter_method = use_signal(|| String::from("all"));
    let mut filter_rating = use_signal(|| 0.0f32);
    let mut search_query = use_signal(|| String::new());
    
    rsx! {
        div {
            class: "page sessions-page",
            h2 { "Brewing Sessions" }
            
            SessionFilter {
                filter_method: filter_method(),
                filter_rating: filter_rating(),
                search_query: search_query(),
                on_method_change: move |method| filter_method.set(method),
                on_rating_change: move |rating| filter_rating.set(rating),
                on_search_change: move |query| search_query.set(query)
            }
            
            div { class: "sessions-content",
                {render_sessions_list(&app_state, filter_method(), filter_rating(), search_query())}
            }
        }
    }
}

#[cfg(feature = "database")]
fn render_sessions_list(app_state: &AppState, filter_method: String, filter_rating: f32, search_query: String) -> Element {
    let sessions = &app_state.sessions;
    
    let filtered_sessions: Vec<&BrewingSession> = sessions.iter()
        .filter(|s| {
            let method_match = filter_method == "all" || 
                format!("{:?}", s.brewing_method).to_lowercase() == filter_method.to_lowercase();
            
            let rating_match = filter_rating == 0.0 || 
                s.rating.unwrap_or(0.0) >= filter_rating;
            
            let search_match = search_query.is_empty() || 
                s.tasting_notes.as_ref().map_or(false, |n| 
                    n.to_lowercase().contains(&search_query.to_lowercase())
                );
            
            method_match && rating_match && search_match
        })
        .collect();
    
    if filtered_sessions.is_empty() {
        rsx! {
            div { class: "empty-state",
                p { "No sessions found matching your filters." }
            }
        }
    } else {
        rsx! {
            div { class: "session-list",
                for session in filtered_sessions.iter() {
                    BrewingSessionCard { session: (*session).clone() }
                }
            }
        }
    }
}

#[cfg(not(feature = "database"))]
fn render_sessions_list(_app_state: &AppState, _filter_method: String, _filter_rating: f32, _search_query: String) -> Element {
    rsx! {
        p { "Database features not enabled" }
    }
}

#[component]
fn SessionFilter(
    filter_method: String,
    filter_rating: f32,
    search_query: String,
    on_method_change: EventHandler<String>,
    on_rating_change: EventHandler<f32>,
    on_search_change: EventHandler<String>
) -> Element {
    rsx! {
        section { class: "session-filters",
            h3 { "Filter Sessions" }
            
            div { class: "filter-group",
                label { "Brewing Method" }
                select {
                    value: "{filter_method}",
                    onchange: move |e| on_method_change.call(e.value()),
                    option { value: "all", "All Methods" }
                    option { value: "v60", "V60" }
                    option { value: "chemex", "Chemex" }
                    option { value: "frenchpress", "French Press" }
                    option { value: "aeropress", "AeroPress" }
                    option { value: "espresso", "Espresso" }
                }
            }
            
            div { class: "filter-group",
                label { "Minimum Rating" }
                select {
                    value: "{filter_rating}",
                    onchange: move |e| {
                        if let Ok(val) = e.value().parse::<f32>() {
                            on_rating_change.call(val);
                        }
                    },
                    option { value: "0", "All Ratings" }
                    option { value: "3.0", "3.0+" }
                    option { value: "4.0", "4.0+" }
                    option { value: "4.5", "4.5+" }
                }
            }
            
            div { class: "filter-group",
                label { "Search Notes" }
                input {
                    r#type: "text",
                    placeholder: "Search tasting notes...",
                    value: "{search_query}",
                    oninput: move |e| on_search_change.call(e.value())
                }
            }
            
            button {
                class: "btn-secondary",
                onclick: move |_| {
                    on_method_change.call("all".to_string());
                    on_rating_change.call(0.0);
                    on_search_change.call(String::new());
                },
                "Clear Filters"
            }
        }
    }
}

#[cfg(feature = "database")]
#[component]
fn BrewingSessionCard(session: BrewingSession) -> Element {
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
        div { class: "session-card",
            onclick: move |_| {
                // Navigate to detail page
            },
            
            div { class: "session-header",
                h4 { "Session #{session.id}" }
                span { class: "session-date", "{session.created_at.format(\"%Y-%m-%d %H:%M\")}" }
            }
            
            div { class: "session-details",
                div { class: "detail-row",
                    span { class: "label", "Method:" }
                    span { "{session.brewing_method:?}" }
                }
                div { class: "detail-row",
                    span { class: "label", "Coffee:" }
                    span { "{session.coffee_grams}g" }
                }
                div { class: "detail-row",
                    span { class: "label", "Water:" }
                    span { "{session.water_ml}ml" }
                }
                div { class: "detail-row",
                    span { class: "label", "Ratio:" }
                    span { "{(session.water_ml / session.coffee_grams):.1}:1" }
                }
                div { class: "detail-row",
                    span { class: "label", "Grind:" }
                    span { "{session.grind_setting}" }
                }
                if let Some(rating) = session.rating {
                    div { class: "detail-row",
                        span { class: "label", "Rating:" }
                        span { class: "rating-display", "{rating_stars} ({rating:.1})" }
                    }
                }
            }
            
            if let Some(notes) = &session.tasting_notes {
                div { class: "session-notes",
                    p { "{notes}" }
                }
            }
            
            div { class: "session-actions",
                Link {
                    to: Route::SessionDetail { id: session.id.to_string() },
                    class: "btn-link",
                    "View Details"
                }
            }
        }
    }
}

#[cfg(not(feature = "database"))]
#[component]
fn BrewingSessionCard(session: ()) -> Element {
    rsx! {
        div { class: "session-card",
            p { "Database not enabled" }
        }
    }
}
