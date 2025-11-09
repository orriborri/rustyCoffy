use dioxus::prelude::*;
use crate::ui::state::AppState;

#[cfg(feature = "database")]
use crate::models::BrewingSession;

/// Statistics page with detailed analytics dashboard
#[component]
pub fn Statistics() -> Element {
    let app_state = use_context::<AppState>();
    
    rsx! {
        div {
            class: "page statistics-page",
            h2 { "Brewing Statistics & Analytics" }
            
            div { class: "statistics-content",
                {render_statistics(&app_state)}
            }
        }
    }
}

#[cfg(feature = "database")]
fn render_statistics(app_state: &AppState) -> Element {
    let sessions = &app_state.sessions;
    
    if sessions.is_empty() {
        return rsx! {
            div { class: "empty-state",
                p { "No sessions yet. Create some brews to see analytics!" }
            }
        };
    }
    
    // Calculate statistics
    let total_sessions = sessions.len();
    let avg_coffee = sessions.iter().map(|s| s.coffee_grams).sum::<f32>() / total_sessions as f32;
    let avg_water = sessions.iter().map(|s| s.water_ml).sum::<f32>() / total_sessions as f32;
    let avg_ratio = avg_water / avg_coffee;
    
    let rated_sessions: Vec<&BrewingSession> = sessions.iter()
        .filter(|s| s.rating.is_some())
        .collect();
    
    let avg_rating = if !rated_sessions.is_empty() {
        rated_sessions.iter()
            .filter_map(|s| s.rating)
            .sum::<f32>() / rated_sessions.len() as f32
    } else {
        0.0
    };
    
    // Method distribution
    let mut method_counts = std::collections::HashMap::new();
    for session in sessions.iter() {
        *method_counts.entry(format!("{:?}", session.brewing_method)).or_insert(0) += 1;
    }
    
    // Rating distribution
    let excellent = rated_sessions.iter().filter(|s| s.rating.unwrap() >= 4.5).count();
    let good = rated_sessions.iter().filter(|s| s.rating.unwrap() >= 3.5 && s.rating.unwrap() < 4.5).count();
    let average = rated_sessions.iter().filter(|s| s.rating.unwrap() < 3.5).count();
    
    rsx! {
        div { class: "stats-grid",
            section { class: "stat-section",
                h3 { "Overall Statistics" }
                div { class: "stat-cards",
                    div { class: "stat-card",
                        div { class: "stat-value", "{total_sessions}" }
                        div { class: "stat-label", "Total Brews" }
                    }
                    div { class: "stat-card",
                        div { class: "stat-value", "{avg_rating:.2}" }
                        div { class: "stat-label", "Average Rating" }
                    }
                    div { class: "stat-card",
                        div { class: "stat-value", "{avg_ratio:.1}:1" }
                        div { class: "stat-label", "Average Ratio" }
                    }
                    div { class: "stat-card",
                        div { class: "stat-value", "{avg_coffee:.1}g" }
                        div { class: "stat-label", "Avg Coffee" }
                    }
                }
            }
            
            section { class: "stat-section",
                h3 { "Brewing Methods" }
                div { class: "method-distribution",
                    for (method, count) in method_counts.iter() {
                        div { class: "method-bar",
                            div { class: "method-name", "{method}" }
                            div { class: "bar-container",
                                div {
                                    class: "bar-fill",
                                    style: "width: {(count * 100 / total_sessions) as f32}%",
                                }
                            }
                            div { class: "method-count", "{count} brews" }
                        }
                    }
                }
            }
            
            section { class: "stat-section",
                h3 { "Quality Distribution" }
                div { class: "quality-distribution",
                    div { class: "quality-item excellent",
                        div { class: "quality-label", "Excellent (4.5+)" }
                        div { class: "quality-value", "{excellent}" }
                    }
                    div { class: "quality-item good",
                        div { class: "quality-label", "Good (3.5-4.5)" }
                        div { class: "quality-value", "{good}" }
                    }
                    div { class: "quality-item average",
                        div { class: "quality-label", "Average (<3.5)" }
                        div { class: "quality-value", "{average}" }
                    }
                }
            }
            
            section { class: "stat-section",
                h3 { "Recommendations" }
                div { class: "recommendations",
                    if avg_ratio < 14.0 {
                        div { class: "recommendation",
                            "💡 Your ratio is quite strong. Try increasing water for a lighter brew."
                        }
                    } else if avg_ratio > 18.0 {
                        div { class: "recommendation",
                            "💡 Your ratio is quite light. Try using more coffee for a stronger brew."
                        }
                    } else {
                        div { class: "recommendation",
                            "✅ Your coffee-to-water ratio is in the ideal range (14-18:1)."
                        }
                    }
                    
                    if avg_rating < 3.5 {
                        div { class: "recommendation",
                            "💡 Try experimenting with different grind settings to improve quality."
                        }
                    } else if avg_rating >= 4.5 {
                        div { class: "recommendation",
                            "🏆 Excellent brewing! You've mastered your technique."
                        }
                    }
                }
            }
        }
    }
}

#[cfg(not(feature = "database"))]
fn render_statistics(_app_state: &AppState) -> Element {
    rsx! {
        div { class: "empty-state",
            p { "Database features not enabled" }
        }
    }
}
