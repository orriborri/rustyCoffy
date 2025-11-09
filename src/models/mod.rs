#[cfg(test)]
mod tests;

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Serialize, Deserialize};
use crate::validation::{self, BrewingError, Result};

#[cfg(feature = "database")]
use diesel::prelude::*;

// Custom enums with string representation for database compatibility
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BrewingMethod {
    V60,
    Chemex,
    FrenchPress,
    AeroPress,
    Espresso,
    Moka,
    ColdBrew,
    Other,
}

impl Default for BrewingMethod {
    fn default() -> Self {
        BrewingMethod::V60
    }
}

impl BrewingMethod {
    pub fn to_string(&self) -> String {
        match self {
            BrewingMethod::V60 => "V60".to_string(),
            BrewingMethod::Chemex => "Chemex".to_string(),
            BrewingMethod::FrenchPress => "FrenchPress".to_string(),
            BrewingMethod::AeroPress => "AeroPress".to_string(),
            BrewingMethod::Espresso => "Espresso".to_string(),
            BrewingMethod::Moka => "Moka".to_string(),
            BrewingMethod::ColdBrew => "ColdBrew".to_string(),
            BrewingMethod::Other => "Other".to_string(),
        }
    }
    
    pub fn from_string(s: &str) -> Result<Self> {
        match s {
            "V60" => Ok(BrewingMethod::V60),
            "Chemex" => Ok(BrewingMethod::Chemex),
            "FrenchPress" => Ok(BrewingMethod::FrenchPress),
            "AeroPress" => Ok(BrewingMethod::AeroPress),
            "Espresso" => Ok(BrewingMethod::Espresso),
            "Moka" => Ok(BrewingMethod::Moka),
            "ColdBrew" => Ok(BrewingMethod::ColdBrew),
            "Other" => Ok(BrewingMethod::Other),
            _ => Err(BrewingError::InvalidBeanData {
                message: format!("Invalid brewing method: {}", s),
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GrinderType {
    BurrConical,
    BurrFlat,
    Blade,
    Manual,
}

impl GrinderType {
    pub fn to_string(&self) -> String {
        match self {
            GrinderType::BurrConical => "BurrConical".to_string(),
            GrinderType::BurrFlat => "BurrFlat".to_string(),
            GrinderType::Blade => "Blade".to_string(),
            GrinderType::Manual => "Manual".to_string(),
        }
    }
    
    pub fn from_string(s: &str) -> Result<Self> {
        match s {
            "BurrConical" => Ok(GrinderType::BurrConical),
            "BurrFlat" => Ok(GrinderType::BurrFlat),
            "Blade" => Ok(GrinderType::Blade),
            "Manual" => Ok(GrinderType::Manual),
            _ => Err(BrewingError::InvalidGrinderSettings {
                message: format!("Invalid grinder type: {}", s),
            }),
        }
    }
}

// Coffee bean information with inventory tracking
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "database", derive(Queryable, Identifiable, Selectable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::schema::coffee_beans))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct CoffeeBean {
    pub id: i32,
    pub name: String,
    pub origin: String,
    pub roast_date: NaiveDate,
    pub purchase_date: NaiveDate,
    pub remaining_grams: Option<f32>,
    pub variety: Option<String>,
    pub processing_method: Option<String>,
    pub created_at: NaiveDateTime,
}

impl CoffeeBean {
    /// Checks if there are sufficient beans for a brewing session
    pub fn has_sufficient_quantity(&self, required_grams: f32) -> bool {
        match self.remaining_grams {
            Some(available) => available >= required_grams,
            None => true, // If not tracking quantity, assume sufficient
        }
    }
    
    /// Gets the age of the beans in days since roast date
    pub fn days_since_roast(&self) -> i64 {
        let today = chrono::Utc::now().naive_utc().date();
        (today - self.roast_date).num_days()
    }
    
    /// Checks if beans are still fresh (within 30 days of roast)
    pub fn is_fresh(&self) -> bool {
        self.days_since_roast() <= 30
    }
    
    /// Updates remaining quantity after use
    pub fn use_beans(&mut self, grams_used: f32) -> Result<()> {
        if let Some(ref mut remaining) = self.remaining_grams {
            if *remaining < grams_used {
                return Err(BrewingError::InsufficientBeanQuantity {
                    available: *remaining,
                    required: grams_used,
                });
            }
            *remaining -= grams_used;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "database", derive(Insertable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::schema::coffee_beans))]
pub struct NewCoffeeBean {
    pub name: String,
    pub origin: String,
    pub roast_date: NaiveDate,
    pub purchase_date: NaiveDate,
    pub remaining_grams: Option<f32>,
    pub variety: Option<String>,
    pub processing_method: Option<String>,
}

impl NewCoffeeBean {
    /// Validates coffee bean parameters
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            return Err(BrewingError::InvalidBeanData {
                message: "Bean name cannot be empty".to_string(),
            });
        }
        
        if self.origin.trim().is_empty() {
            return Err(BrewingError::InvalidBeanData {
                message: "Bean origin cannot be empty".to_string(),
            });
        }
        
        if self.roast_date > chrono::Utc::now().naive_utc().date() {
            return Err(BrewingError::InvalidBeanData {
                message: "Roast date cannot be in the future".to_string(),
            });
        }
        
        if self.purchase_date > chrono::Utc::now().naive_utc().date() {
            return Err(BrewingError::InvalidBeanData {
                message: "Purchase date cannot be in the future".to_string(),
            });
        }
        
        if let Some(remaining) = self.remaining_grams {
            if remaining < 0.0 {
                return Err(BrewingError::InvalidBeanData {
                    message: format!("Remaining grams cannot be negative, got: {}", remaining),
                });
            }
        }
        
        Ok(())
    }
    
    /// Creates a new coffee bean with validation
    pub fn new(
        name: String,
        origin: String,
        roast_date: NaiveDate,
        purchase_date: NaiveDate,
    ) -> Result<Self> {
        let bean = Self {
            name,
            origin,
            roast_date,
            purchase_date,
            remaining_grams: None,
            variety: None,
            processing_method: None,
        };
        
        bean.validate()?;
        Ok(bean)
    }
}

// Grinder equipment with setting ranges for validation
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "database", derive(Queryable, Identifiable, Selectable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::schema::grinders))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct Grinder {
    pub id: i32,
    pub brand: String,
    pub model: String,
    pub grinder_type: String,
    pub min_setting: i32,
    pub max_setting: i32,
    pub created_at: NaiveDateTime,
}

impl Grinder {
    /// Gets the full name of the grinder (brand + model)
    pub fn full_name(&self) -> String {
        format!("{} {}", self.brand, self.model)
    }
    
    /// Validates if a grind setting is within this grinder's range
    pub fn is_valid_setting(&self, setting: i32) -> bool {
        setting >= self.min_setting && setting <= self.max_setting
    }
    
    /// Gets the range of valid settings for this grinder
    pub fn setting_range(&self) -> std::ops::RangeInclusive<i32> {
        self.min_setting..=self.max_setting
    }
    
    /// Gets the grinder type as enum
    pub fn get_grinder_type(&self) -> Result<GrinderType> {
        GrinderType::from_string(&self.grinder_type)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "database", derive(Insertable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::schema::grinders))]
pub struct NewGrinder {
    pub brand: String,
    pub model: String,
    pub grinder_type: String,
    pub min_setting: i32,
    pub max_setting: i32,
}

impl NewGrinder {
    /// Validates grinder parameters
    pub fn validate(&self) -> Result<()> {
        if self.min_setting <= 0 {
            return Err(BrewingError::InvalidGrinderSettings {
                message: format!("Minimum setting must be positive, got: {}", self.min_setting),
            });
        }
        
        if self.max_setting <= self.min_setting {
            return Err(BrewingError::InvalidGrinderSettings {
                message: format!(
                    "Maximum setting ({}) must be greater than minimum setting ({})",
                    self.max_setting, self.min_setting
                ),
            });
        }
        
        if self.brand.trim().is_empty() {
            return Err(BrewingError::InvalidGrinderSettings {
                message: "Brand cannot be empty".to_string(),
            });
        }
        
        if self.model.trim().is_empty() {
            return Err(BrewingError::InvalidGrinderSettings {
                message: "Model cannot be empty".to_string(),
            });
        }
        
        // Validate grinder type string
        GrinderType::from_string(&self.grinder_type)?;
        
        Ok(())
    }
    
    /// Creates a new grinder with validation
    pub fn new(
        brand: String,
        model: String,
        grinder_type: GrinderType,
        min_setting: i32,
        max_setting: i32,
    ) -> Result<Self> {
        let grinder = Self {
            brand,
            model,
            grinder_type: grinder_type.to_string(),
            min_setting,
            max_setting,
        };
        
        grinder.validate()?;
        Ok(grinder)
    }
}

// Individual brewing session with comprehensive tracking
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "database", derive(Queryable, Identifiable, Selectable, Associations))]
#[cfg_attr(feature = "database", diesel(belongs_to(CoffeeBean, foreign_key = bean_id)))]
#[cfg_attr(feature = "database", diesel(belongs_to(Grinder, foreign_key = grinder_id)))]
#[cfg_attr(feature = "database", diesel(table_name = crate::schema::brewing_sessions))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct BrewingSession {
    pub id: i32,
    pub bean_id: i32,
    pub grinder_id: i32,
    pub grind_setting: i32,
    pub brewing_method: String,
    pub water_temp_celsius: Option<i32>,
    pub brew_time_seconds: Option<i32>,
    pub coffee_grams: f32,
    pub water_grams: f32,
    pub tasting_notes: Option<String>,
    pub rating: Option<f32>,
    pub created_at: NaiveDateTime,
}

impl BrewingSession {
    /// Calculates the coffee to water ratio
    pub fn coffee_ratio(&self) -> f32 {
        self.water_grams / self.coffee_grams
    }
    
    /// Checks if this session has a high rating (>= 7.0)
    pub fn is_high_quality(&self) -> bool {
        self.rating.map_or(false, |r| r >= 7.0)
    }
    
    /// Gets the brewing method as enum
    pub fn get_brewing_method(&self) -> Result<BrewingMethod> {
        BrewingMethod::from_string(&self.brewing_method)
    }
    
    /// Creates a NewBrewingSession from this session for duplication
    pub fn to_new_session(&self) -> NewBrewingSession {
        NewBrewingSession {
            bean_id: self.bean_id,
            grinder_id: self.grinder_id,
            grind_setting: self.grind_setting,
            brewing_method: self.brewing_method.clone(),
            water_temp_celsius: self.water_temp_celsius,
            brew_time_seconds: self.brew_time_seconds,
            coffee_grams: self.coffee_grams,
            water_grams: self.water_grams,
            tasting_notes: None, // Don't copy tasting notes
            rating: None, // Don't copy rating
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "database", derive(Insertable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::schema::brewing_sessions))]
pub struct NewBrewingSession {
    pub bean_id: i32,
    pub grinder_id: i32,
    pub grind_setting: i32,
    pub brewing_method: String,
    pub water_temp_celsius: Option<i32>,
    pub brew_time_seconds: Option<i32>,
    pub coffee_grams: f32,
    pub water_grams: f32,
    pub tasting_notes: Option<String>,
    pub rating: Option<f32>,
}

impl NewBrewingSession {
    /// Validates all brewing session parameters
    pub fn validate(&self) -> Result<()> {
        // Validate coffee amount
        validation::validate_coffee_amount(self.coffee_grams)?;
        
        // Validate water amount
        validation::validate_water_amount(self.water_grams)?;
        
        // Validate coffee to water ratio
        validation::validate_coffee_ratio(self.coffee_grams, self.water_grams)?;
        
        // Validate brewing method string
        BrewingMethod::from_string(&self.brewing_method)?;
        
        // Validate rating if provided
        if let Some(rating) = self.rating {
            validation::validate_rating(rating)?;
        }
        
        // Validate brew time if provided
        if let Some(brew_time) = self.brew_time_seconds {
            validation::validate_brew_time(brew_time)?;
        }
        
        // Validate water temperature if provided
        if let Some(water_temp) = self.water_temp_celsius {
            validation::validate_water_temperature(water_temp)?;
        }
        
        Ok(())
    }
    
    /// Validates grind setting against a specific grinder
    pub fn validate_grind_setting(&self, grinder: &Grinder) -> Result<()> {
        let grinder_name = format!("{} {}", grinder.brand, grinder.model);
        validation::validate_grind_setting(
            self.grind_setting,
            grinder.min_setting,
            grinder.max_setting,
            &grinder_name,
        )
    }
    
    /// Creates a new brewing session with validation
    pub fn new(
        bean_id: i32,
        grinder_id: i32,
        grind_setting: i32,
        brewing_method: BrewingMethod,
        coffee_grams: f32,
        water_grams: f32,
    ) -> Result<Self> {
        let session = Self {
            bean_id,
            grinder_id,
            grind_setting,
            brewing_method: brewing_method.to_string(),
            water_temp_celsius: None,
            brew_time_seconds: None,
            coffee_grams,
            water_grams,
            tasting_notes: None,
            rating: None,
        };
        
        session.validate()?;
        Ok(session)
    }
}