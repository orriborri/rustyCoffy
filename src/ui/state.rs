#[cfg(feature = "database")]
use std::cell::RefCell;
#[cfg(feature = "database")]
use std::rc::Rc;
use serde::{Deserialize, Serialize};

#[cfg(feature = "database")]
use crate::services::BrewingService;
#[cfg(feature = "database")]
use crate::models::{BrewingSession, CoffeeBean, Grinder, BrewingMethod};

/// Global application state with comprehensive data management
#[derive(Clone)]
pub struct AppState {
    #[cfg(feature = "database")]
    pub brewing_service: Rc<RefCell<BrewingService>>,
    #[cfg(feature = "database")]
    pub sessions: Vec<BrewingSession>,
    #[cfg(feature = "database")]
    pub beans: Vec<CoffeeBean>,
    #[cfg(feature = "database")]
    pub grinders: Vec<Grinder>,
    pub loading: bool,
    pub error: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "database")]
            brewing_service: Rc::new(RefCell::new(
                BrewingService::new("postgresql://localhost/coffee_tracker")
                    .expect("Failed to initialize brewing service")
            )),
            #[cfg(feature = "database")]
            sessions: Vec::new(),
            #[cfg(feature = "database")]
            beans: Vec::new(),
            #[cfg(feature = "database")]
            grinders: Vec::new(),
            loading: false,
            error: None,
        }
    }

    #[cfg(feature = "database")]
    pub fn load_data(&mut self) -> Result<(), String> {
        self.loading = true;
        self.error = None;

        let service = self.brewing_service.borrow();
        
        match service.get_beans() {
            Ok(beans) => self.beans = beans,
            Err(e) => {
                self.error = Some(format!("Failed to load beans: {}", e));
                self.loading = false;
                return Err(self.error.clone().unwrap());
            }
        }
        
        match service.get_grinders() {
            Ok(grinders) => self.grinders = grinders,
            Err(e) => {
                self.error = Some(format!("Failed to load grinders: {}", e));
                self.loading = false;
                return Err(self.error.clone().unwrap());
            }
        }
        
        match service.get_sessions(Default::default()) {
            Ok(sessions) => self.sessions = sessions,
            Err(e) => {
                self.error = Some(format!("Failed to load sessions: {}", e));
                self.loading = false;
                return Err(self.error.clone().unwrap());
            }
        }

        self.loading = false;
        Ok(())
    }
    
    #[cfg(feature = "database")]
    pub fn clear_error(&mut self) {
        self.error = None;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Default brewing parameters for consistent UX
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BrewingDefaults {
    pub coffee_amount: f32,    // 20.0 grams
    pub water_amount: f32,     // 320.0 ml (maintains 16:1 ratio)
    pub brew_time: i32,        // 240 seconds (4 minutes)
    #[cfg(feature = "database")]
    pub method: BrewingMethod, // PourOver
}

impl Default for BrewingDefaults {
    fn default() -> Self {
        Self {
            coffee_amount: 20.0,
            water_amount: 320.0,
            brew_time: 240,
            #[cfg(feature = "database")]
            method: BrewingMethod::V60,
        }
    }
}

/// Form state management for multi-step brewing session creation
#[derive(Clone, Default, Debug, PartialEq)]
pub struct NewSessionState {
    pub selected_bean_id: Option<i32>,
    pub selected_grinder_id: Option<i32>,
    pub grind_setting: Option<i32>,
    #[cfg(feature = "database")]
    pub brewing_method: BrewingMethod,
    pub coffee_amount: f32,
    pub water_amount: f32,
    pub brew_time: Option<i32>,
    pub water_temp: Option<i32>,
    pub tasting_notes: String,
    pub rating: Option<f32>,
    pub validation_errors: Vec<String>,
}

impl NewSessionState {
    pub fn new() -> Self {
        let defaults = BrewingDefaults::default();
        Self {
            selected_bean_id: None,
            selected_grinder_id: None,
            grind_setting: None,
            #[cfg(feature = "database")]
            brewing_method: defaults.method,
            coffee_amount: defaults.coffee_amount,
            water_amount: defaults.water_amount,
            brew_time: Some(defaults.brew_time),
            water_temp: None,
            tasting_notes: String::new(),
            rating: None,
            validation_errors: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn calculate_ratio(&self) -> f32 {
        if self.coffee_amount > 0.0 {
            self.water_amount / self.coffee_amount
        } else {
            0.0
        }
    }
}
