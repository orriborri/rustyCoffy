use dioxus::prelude::*;
use crate::ui::state::AppState;
use crate::models::{CoffeeBean, Grinder};

#[cfg(feature = "database")]
use chrono::NaiveDate;

#[cfg(feature = "database")]
use crate::models::{NewCoffeeBean, NewGrinder, GrinderType};

#[cfg(feature = "database")]
use crate::services::database::GrinderStats;

/// Equipment page combining bean and grinder management with tabs
#[component]
pub fn Equipment() -> Element {
    let mut active_tab = use_signal(|| "beans");
    
    rsx! {
        div {
            class: "page equipment-page",
            h2 { "Equipment Management" }
            
            div {
                class: "equipment-tabs",
                button {
                    class: if active_tab() == "beans" { "tab-button active" } else { "tab-button" },
                    onclick: move |_| active_tab.set("beans"),
                    "Coffee Beans"
                }
                button {
                    class: if active_tab() == "grinders" { "tab-button active" } else { "tab-button" },
                    onclick: move |_| active_tab.set("grinders"),
                    "Grinders"
                }
            }
            
            div {
                class: "equipment-content",
                
                if active_tab() == "beans" {
                    BeansSection {}
                } else {
                    GrindersSection {}
                }
            }
        }
    }
}

/// Beans section with add form and list
#[component]
fn BeansSection() -> Element {
    let mut show_form = use_signal(|| false);
    
    rsx! {
        section {
            class: "beans-section",
            
            div {
                class: "section-header",
                h3 { "Coffee Beans" }
                button {
                    class: "btn-primary",
                    onclick: move |_| show_form.set(!show_form()),
                    if show_form() { "Cancel" } else { "Add Bean" }
                }
            }
            
            if show_form() {
                AddBeanForm {}
            }
            
            BeanList {}
        }
    }
}

/// Grinders section with add form and list
#[component]
fn GrindersSection() -> Element {
    let mut show_form = use_signal(|| false);
    
    rsx! {
        section {
            class: "grinders-section",
            
            div {
                class: "section-header",
                h3 { "Grinders" }
                button {
                    class: "btn-primary",
                    onclick: move |_| show_form.set(!show_form()),
                    if show_form() { "Cancel" } else { "Add Grinder" }
                }
            }
            
            if show_form() {
                AddGrinderForm {}
            }
            
            GrinderList {}
        }
    }
}

/// Add bean form component
#[component]
fn AddBeanForm() -> Element {
    rsx! {
        div {
            class: "add-form",
            p { "Add bean form placeholder" }
        }
    }
}

/// Bean list component
#[component]
fn BeanList() -> Element {
    let _app_state = use_context::<AppState>();
    
    #[cfg(feature = "database")]
    let beans = &app_state.beans;
    
    #[cfg(not(feature = "database"))]
    let beans: &Vec<CoffeeBean> = &Vec::new();
    
    rsx! {
        div {
            class: "bean-list",
            
            if beans.is_empty() {
                div { class: "empty-state", "No coffee beans added yet. Add your first bean above!" }
            } else {
                div {
                    class: "bean-grid",
                    for bean in beans.iter() {
                        BeanCard { bean: bean.clone() }
                    }
                }
            }
        }
    }
}

/// Individual bean card component
#[cfg(feature = "database")]
#[component]
fn BeanCard(bean: CoffeeBean) -> Element {
    use crate::models::BrewingSession;
    
    let mut show_usage = use_signal(|| false);
    let mut usage_history = use_signal(|| Vec::<BrewingSession>::new());
    
    let days_since_roast = bean.days_since_roast();
    let is_fresh = bean.is_fresh();
    
    rsx! {
        div {
            class: "bean-card",
            
            div {
                class: "bean-header",
                h4 { "{bean.name}" }
                span {
                    class: if is_fresh { "freshness-badge fresh" } else { "freshness-badge aged" },
                    if is_fresh { "Fresh" } else { "Aged" }
                }
            }
            
            div {
                class: "bean-details",
                div { class: "detail-row",
                    span { class: "label", "Origin:" }
                    span { "{bean.origin}" }
                }
                div { class: "detail-row",
                    span { class: "label", "Roast Date:" }
                    span { "{bean.roast_date} ({days_since_roast} days ago)" }
                }
                if let Some(variety) = &bean.variety {
                    div { class: "detail-row",
                        span { class: "label", "Variety:" }
                        span { "{variety}" }
                    }
                }
                if let Some(processing) = &bean.processing_method {
                    div { class: "detail-row",
                        span { class: "label", "Processing:" }
                        span { "{processing}" }
                    }
                }
                if let Some(remaining) = bean.remaining_grams {
                    div { class: "detail-row",
                        span { class: "label", "Remaining:" }
                        span {
                            class: if remaining < 50.0 { "low-stock" } else { "" },
                            "{remaining:.1}g"
                        }
                    }
                }
            }
        }
    }
}

#[cfg(not(feature = "database"))]
#[component]
fn BeanCard(bean: CoffeeBean) -> Element {
    let days_since_roast = bean.days_since_roast();
    let is_fresh = bean.is_fresh();
    
    rsx! {
        div {
            class: "bean-card",
            
            div {
                class: "bean-header",
                h4 { "{bean.name}" }
                span {
                    class: if is_fresh { "freshness-badge fresh" } else { "freshness-badge aged" },
                    if is_fresh { "Fresh" } else { "Aged" }
                }
            }
            
            div {
                class: "bean-details",
                div { class: "detail-row",
                    span { class: "label", "Origin:" }
                    span { "{bean.origin}" }
                }
                div { class: "detail-row",
                    span { class: "label", "Roast Date:" }
                    span { "{bean.roast_date} ({days_since_roast} days ago)" }
                }
                if let Some(variety) = &bean.variety {
                    div { class: "detail-row",
                        span { class: "label", "Variety:" }
                        span { "{variety}" }
                    }
                }
                if let Some(processing) = &bean.processing_method {
                    div { class: "detail-row",
                        span { class: "label", "Processing:" }
                        span { "{processing}" }
                    }
                }
                if let Some(remaining) = bean.remaining_grams {
                    div { class: "detail-row",
                        span { class: "label", "Remaining:" }
                        span {
                            class: if remaining < 50.0 { "low-stock" } else { "" },
                            "{remaining:.1}g"
                        }
                    }
                }
            }
        }
    }
}

/// Add grinder form component
#[component]
fn AddGrinderForm() -> Element {
    rsx! {
        div {
            class: "add-form",
            p { "Add grinder form placeholder" }
        }
    }
}

/// Grinder list component
#[component]
fn GrinderList() -> Element {
    let _app_state = use_context::<AppState>();
    
    #[cfg(feature = "database")]
    let grinders = &app_state.grinders;
    
    #[cfg(not(feature = "database"))]
    let grinders: &Vec<Grinder> = &Vec::new();
    
    rsx! {
        div {
            class: "grinder-list",
            
            if grinders.is_empty() {
                div { class: "empty-state", "No grinders added yet. Add your first grinder above!" }
            } else {
                div {
                    class: "grinder-grid",
                    for grinder in grinders.iter() {
                        GrinderCard { grinder: grinder.clone() }
                    }
                }
            }
        }
    }
}

/// Individual grinder card component
#[cfg(feature = "database")]
#[component]
fn GrinderCard(grinder: Grinder) -> Element {
    rsx! {
        div {
            class: "grinder-card",
            
            div {
                class: "grinder-header",
                h4 { "{grinder.full_name()}" }
                span { class: "grinder-type-badge", "{grinder.grinder_type}" }
            }
            
            div {
                class: "grinder-details",
                div { class: "detail-row",
                    span { class: "label", "Setting Range:" }
                    span { "{grinder.min_setting} - {grinder.max_setting}" }
                }
            }
        }
    }
}

#[cfg(not(feature = "database"))]
#[component]
fn GrinderCard(grinder: Grinder) -> Element {
    rsx! {
        div {
            class: "grinder-card",
            
            div {
                class: "grinder-header",
                h4 { "{grinder.full_name()}" }
                span { class: "grinder-type-badge", "{grinder.grinder_type}" }
            }
            
            div {
                class: "grinder-details",
                div { class: "detail-row",
                    span { class: "label", "Setting Range:" }
                    span { "{grinder.min_setting} - {grinder.max_setting}" }
                }
            }
        }
    }
}
