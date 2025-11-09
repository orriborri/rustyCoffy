use dioxus::prelude::*;
use crate::ui::state::{AppState, NewSessionState};
use crate::models::{CoffeeBean, Grinder, BrewingMethod};

#[cfg(feature = "database")]
use crate::models::NewBrewingSession;

/// Multi-step brewing session creation page
#[component]
pub fn NewBrew() -> Element {
    let app_state = use_context::<AppState>();
    let mut session_state = use_signal(|| NewSessionState::new());
    let mut current_step = use_signal(|| 1);
    let mut error_msg = use_signal(|| String::new());
    let mut success_msg = use_signal(|| String::new());
    
    rsx! {
        div {
            class: "page new-brew-page",
            h2 { "Create New Brewing Session" }
            
            if !error_msg().is_empty() {
                div { class: "error-message", "{error_msg()}" }
            }
            
            if !success_msg().is_empty() {
                div { class: "success-message", "{success_msg()}" }
            }
            
            div { class: "progress-steps",
                for step in 1..=4 {
                    div {
                        class: if current_step() >= step { "step active" } else { "step" },
                        "{step}"
                    }
                }
            }
            
            div { class: "new-brew-form",
                if current_step() == 1 {
                    BeanSelector {
                        selected: session_state().selected_bean_id,
                        on_select: move |id| {
                            session_state.write().selected_bean_id = Some(id);
                            error_msg.set(String::new());
                            current_step.set(2);
                        }
                    }
                } else if current_step() == 2 {
                    GrinderSelector {
                        selected: session_state().selected_grinder_id,
                        on_select: move |id| {
                            session_state.write().selected_grinder_id = Some(id);
                            error_msg.set(String::new());
                            current_step.set(3);
                        },
                        on_back: move |_| current_step.set(1)
                    }
                } else if current_step() == 3 {
                    BrewingParameters {
                        state: session_state(),
                        on_update: move |state| {
                            session_state.set(state);
                        },
                        on_next: move |_| {
                            // Validate parameters
                            let state = session_state();
                            if state.grind_setting.is_none() {
                                error_msg.set("Grind setting is required".to_string());
                                return;
                            }
                            if state.coffee_amount <= 0.0 {
                                error_msg.set("Coffee amount must be greater than 0".to_string());
                                return;
                            }
                            if state.water_amount <= 0.0 {
                                error_msg.set("Water amount must be greater than 0".to_string());
                                return;
                            }
                            error_msg.set(String::new());
                            current_step.set(4);
                        },
                        on_back: move |_| current_step.set(2)
                    }
                } else {
                    RatingInput {
                        state: session_state(),
                        on_save: move |_| {
                            #[cfg(feature = "database")]
                            {
                                let state = session_state();
                                
                                // Validate rating
                                if let Some(rating) = state.rating {
                                    if rating < 0.5 || rating > 5.0 || (rating * 2.0).fract() != 0.0 {
                                        error_msg.set("Rating must be between 0.5 and 5.0 in 0.5 increments".to_string());
                                        return;
                                    }
                                }
                                
                                // Create new session
                                let new_session = NewBrewingSession {
                                    bean_id: state.selected_bean_id.unwrap(),
                                    grinder_id: state.selected_grinder_id.unwrap(),
                                    brewing_method: state.brewing_method,
                                    grind_setting: state.grind_setting.unwrap(),
                                    coffee_grams: state.coffee_amount,
                                    water_ml: state.water_amount,
                                    brew_time_seconds: state.brew_time,
                                    water_temp_celsius: state.water_temp,
                                    tasting_notes: if state.tasting_notes.is_empty() { None } else { Some(state.tasting_notes.clone()) },
                                    rating: state.rating,
                                };
                                
                                // Save session
                                let service = app_state.brewing_service.borrow();
                                match service.create_session(new_session) {
                                    Ok(session) => {
                                        success_msg.set(format!("Session #{} created successfully!", session.id));
                                        
                                        // Update bean quantity
                                        if let Ok(beans) = service.get_beans() {
                                            app_state.beans = beans;
                                        }
                                        
                                        // Reset form
                                        session_state.set(NewSessionState::new());
                                        current_step.set(1);
                                        
                                        // Clear success message after 3 seconds
                                        spawn(async move {
                                            gloo_timers::future::TimeoutFuture::new(3000).await;
                                            success_msg.set(String::new());
                                        });
                                    }
                                    Err(e) => {
                                        error_msg.set(format!("Failed to save session: {}", e));
                                    }
                                }
                            }
                            
                            #[cfg(not(feature = "database"))]
                            {
                                success_msg.set("Session saved (database not enabled)".to_string());
                                session_state.set(NewSessionState::new());
                                current_step.set(1);
                            }
                        },
                        on_back: move |_| current_step.set(3)
                    }
                }
            }
        }
    }
}

#[cfg(feature = "database")]
#[component]
fn BeanSelector(selected: Option<i32>, on_select: EventHandler<i32>) -> Element {
    let _app_state = use_context::<AppState>();
    let beans = &_app_state.beans;
    
    rsx! {
        section { class: "form-section",
            h3 { "Step 1: Select Coffee Bean" }
            
            if beans.is_empty() {
                p { "No beans available. Add beans in Equipment page first." }
            } else {
                div { class: "bean-selector",
                    for bean in beans.iter() {
                        div {
                            class: if selected == Some(bean.id) { "bean-option selected" } else { "bean-option" },
                            onclick: move |_| on_select.call(bean.id),
                            h4 { "{bean.name}" }
                            p { "{bean.origin}" }
                            if let Some(remaining) = bean.remaining_grams {
                                p {
                                    class: if remaining < 50.0 { "remaining low-stock" } else { "remaining" },
                                    "{remaining:.0}g remaining"
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
#[component]
fn BeanSelector(selected: Option<i32>, on_select: EventHandler<i32>) -> Element {
    rsx! {
        section { class: "form-section",
            h3 { "Step 1: Select Coffee Bean" }
            p { "Database features not enabled" }
        }
    }
}

#[cfg(feature = "database")]
#[component]
fn GrinderSelector(selected: Option<i32>, on_select: EventHandler<i32>, on_back: EventHandler<()>) -> Element {
    let _app_state = use_context::<AppState>();
    let grinders = &_app_state.grinders;
    
    rsx! {
        section { class: "form-section",
            h3 { "Step 2: Select Grinder" }
            
            if grinders.is_empty() {
                p { "No grinders available. Add grinders in Equipment page first." }
            } else {
                div { class: "grinder-selector",
                    for grinder in grinders.iter() {
                        div {
                            class: if selected == Some(grinder.id) { "grinder-option selected" } else { "grinder-option" },
                            onclick: move |_| on_select.call(grinder.id),
                            h4 { "{grinder.full_name()}" }
                            p { "Settings: {grinder.min_setting} - {grinder.max_setting}" }
                        }
                    }
                }
            }
            
            button { class: "btn-secondary", onclick: move |_| on_back.call(()), "Back" }
        }
    }
}

#[cfg(not(feature = "database"))]
#[component]
fn GrinderSelector(selected: Option<i32>, on_select: EventHandler<i32>, on_back: EventHandler<()>) -> Element {
    rsx! {
        section { class: "form-section",
            h3 { "Step 2: Select Grinder" }
            p { "Database features not enabled" }
            button { class: "btn-secondary", onclick: move |_| on_back.call(()), "Back" }
        }
    }
}

#[component]
fn BrewingParameters(state: NewSessionState, on_update: EventHandler<NewSessionState>, on_next: EventHandler<()>, on_back: EventHandler<()>) -> Element {
    let mut local_state = use_signal(|| state.clone());
    
    rsx! {
        section { class: "form-section",
            h3 { "Step 3: Brewing Parameters" }
            
            div { class: "form-group",
                label { "Brewing Method" }
                select {
                    onchange: move |_e| {
                        #[cfg(feature = "database")]
                        {
                            let method = match _e.value().as_str() {
                                "V60" => BrewingMethod::V60,
                                "Chemex" => BrewingMethod::Chemex,
                                "FrenchPress" => BrewingMethod::FrenchPress,
                                "AeroPress" => BrewingMethod::AeroPress,
                                "Espresso" => BrewingMethod::Espresso,
                                _ => BrewingMethod::V60,
                            };
                            local_state.write().brewing_method = method;
                        }
                    },
                    option { value: "V60", "V60" }
                    option { value: "Chemex", "Chemex" }
                    option { value: "FrenchPress", "French Press" }
                    option { value: "AeroPress", "AeroPress" }
                    option { value: "Espresso", "Espresso" }
                }
            }
            
            div { class: "form-group",
                label { "Coffee Amount (g)" }
                input {
                    r#type: "number",
                    step: "0.1",
                    min: "0",
                    value: "{local_state().coffee_amount}",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<f32>() {
                            local_state.write().coffee_amount = val;
                        }
                    }
                }
            }
            
            div { class: "form-group",
                label { "Water Amount (ml)" }
                input {
                    r#type: "number",
                    step: "1",
                    min: "0",
                    value: "{local_state().water_amount}",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<f32>() {
                            local_state.write().water_amount = val;
                        }
                    }
                }
            }
            
            div { class: "ratio-display",
                p { "Coffee-to-Water Ratio: {local_state().calculate_ratio():.1}:1" }
            }
            
            div { class: "form-group",
                label { "Grind Setting" }
                input {
                    r#type: "number",
                    min: "0",
                    value: "{local_state().grind_setting.unwrap_or(0)}",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<i32>() {
                            local_state.write().grind_setting = Some(val);
                        }
                    }
                }
            }
            
            div { class: "form-group",
                label { "Brew Time (seconds)" }
                input {
                    r#type: "number",
                    min: "0",
                    value: "{local_state().brew_time.unwrap_or(0)}",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<i32>() {
                            local_state.write().brew_time = Some(val);
                        }
                    }
                }
            }
            
            div { class: "form-group",
                label { "Water Temperature (°C)" }
                input {
                    r#type: "number",
                    min: "0",
                    max: "100",
                    value: "{local_state().water_temp.unwrap_or(0)}",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<i32>() {
                            local_state.write().water_temp = Some(val);
                        }
                    }
                }
            }
            
            div { class: "form-actions",
                button { class: "btn-secondary", onclick: move |_| on_back.call(()), "Back" }
                button {
                    class: "btn-primary",
                    onclick: move |_| {
                        on_update.call(local_state());
                        on_next.call(());
                    },
                    "Next"
                }
            }
        }
    }
}

#[component]
fn RatingInput(state: NewSessionState, on_save: EventHandler<()>, on_back: EventHandler<()>) -> Element {
    let mut local_state = use_signal(|| state.clone());
    
    rsx! {
        section { class: "form-section",
            h3 { "Step 4: Quality Rating & Notes" }
            
            div { class: "form-group",
                label { "Rating (0.5 - 5.0 in 0.5 increments)" }
                input {
                    r#type: "number",
                    step: "0.5",
                    min: "0.5",
                    max: "5.0",
                    value: "{local_state().rating.unwrap_or(3.0)}",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<f32>() {
                            local_state.write().rating = Some(val);
                        }
                    }
                }
                div { class: "rating-stars",
                    for i in 1..=5 {
                        span {
                            class: if local_state().rating.unwrap_or(0.0) >= i as f32 { "star filled" } else { "star" },
                            "★"
                        }
                    }
                }
            }
            
            div { class: "form-group",
                label { "Tasting Notes" }
                textarea {
                    rows: "4",
                    value: "{local_state().tasting_notes}",
                    oninput: move |e| {
                        local_state.write().tasting_notes = e.value();
                    },
                    placeholder: "Describe the flavor, aroma, body, acidity, etc..."
                }
            }
            
            div { class: "form-actions",
                button { class: "btn-secondary", onclick: move |_| on_back.call(()), "Back" }
                button {
                    class: "btn-primary",
                    onclick: move |_| on_save.call(()),
                    "Save Brewing Session"
                }
            }
        }
    }
}
