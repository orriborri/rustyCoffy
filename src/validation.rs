use thiserror::Error;

#[cfg(feature = "database")]
use diesel;
#[cfg(feature = "database")]
use r2d2;

// Domain validation constants
pub const COFFEE_RATIO_MIN: f32 = 15.0;
pub const COFFEE_RATIO_MAX: f32 = 17.0;
pub const RATING_MIN: f32 = 1.0;
pub const RATING_MAX: f32 = 10.0;
pub const RATING_INCREMENT: f32 = 0.5;
pub const COFFEE_AMOUNT_MIN: f32 = 10.0;
pub const COFFEE_AMOUNT_MAX: f32 = 100.0;
pub const WATER_AMOUNT_MIN: f32 = 150.0;
pub const WATER_AMOUNT_MAX: f32 = 1700.0;
pub const BREW_TIME_MIN: i32 = 30;
pub const BREW_TIME_MAX: i32 = 480;
pub const WATER_TEMP_MIN: i32 = 60;
pub const WATER_TEMP_MAX: i32 = 100;

#[derive(Debug, Error)]
pub enum BrewingError {
    #[cfg(feature = "database")]
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
    
    #[cfg(feature = "database")]
    #[error("Connection pool error: {0}")]
    Pool(#[from] r2d2::Error),
    
    #[error("Bean not found with id: {0}")]
    BeanNotFound(i32),
    
    #[error("Grinder not found with id: {0}")]
    GrinderNotFound(i32),
    
    #[error("Grind setting {setting} outside range {min}-{max} for {grinder}")]
    GrindSettingOutOfRange { 
        setting: i32, 
        min: i32, 
        max: i32, 
        grinder: String 
    },
    
    #[error("Coffee ratio {ratio:.1} outside acceptable range {min:.1}-{max:.1}")]
    CoffeeRatioInvalid { 
        ratio: f32, 
        min: f32, 
        max: f32 
    },
    
    #[error("Quality rating must be in 0.5 increments between 1.0-10.0, got: {rating}")]
    InvalidQualityRating { rating: f32 },
    
    #[error("Coffee amount {amount}g outside valid range {min}-{max}g")]
    InvalidCoffeeAmount { 
        amount: f32, 
        min: f32, 
        max: f32 
    },
    
    #[error("Water amount {amount}ml outside valid range {min}-{max}ml")]
    InvalidWaterAmount { 
        amount: f32, 
        min: f32, 
        max: f32 
    },
    
    #[error("Brew time {time}s outside valid range {min}-{max}s")]
    InvalidBrewTime { 
        time: i32, 
        min: i32, 
        max: i32 
    },
    
    #[error("Water temperature {temp}°C outside valid range {min}-{max}°C")]
    InvalidWaterTemperature { 
        temp: i32, 
        min: i32, 
        max: i32 
    },
    
    #[error("Insufficient bean quantity: {available}g available, {required}g required")]
    InsufficientBeanQuantity { 
        available: f32, 
        required: f32 
    },
    
    #[error("Invalid grinder settings: {message}")]
    InvalidGrinderSettings { message: String },
    
    #[error("Invalid bean data: {message}")]
    InvalidBeanData { message: String },
}

pub type Result<T> = std::result::Result<T, BrewingError>;

/// Validates coffee to water ratio
pub fn validate_coffee_ratio(coffee_grams: f32, water_grams: f32) -> Result<()> {
    let ratio = water_grams / coffee_grams;
    if ratio < COFFEE_RATIO_MIN || ratio > COFFEE_RATIO_MAX {
        return Err(BrewingError::CoffeeRatioInvalid {
            ratio,
            min: COFFEE_RATIO_MIN,
            max: COFFEE_RATIO_MAX,
        });
    }
    Ok(())
}

/// Validates quality rating (must be in 0.5 increments)
pub fn validate_rating(rating: f32) -> Result<()> {
    if rating < RATING_MIN || rating > RATING_MAX {
        return Err(BrewingError::InvalidQualityRating { rating });
    }
    
    // Check if rating is in 0.5 increments
    let scaled = rating * 2.0;
    if (scaled - scaled.floor()).abs() > f32::EPSILON {
        return Err(BrewingError::InvalidQualityRating { rating });
    }
    
    Ok(())
}

/// Validates coffee amount
pub fn validate_coffee_amount(amount: f32) -> Result<()> {
    if amount < COFFEE_AMOUNT_MIN || amount > COFFEE_AMOUNT_MAX {
        return Err(BrewingError::InvalidCoffeeAmount {
            amount,
            min: COFFEE_AMOUNT_MIN,
            max: COFFEE_AMOUNT_MAX,
        });
    }
    Ok(())
}

/// Validates water amount
pub fn validate_water_amount(amount: f32) -> Result<()> {
    if amount < WATER_AMOUNT_MIN || amount > WATER_AMOUNT_MAX {
        return Err(BrewingError::InvalidWaterAmount {
            amount,
            min: WATER_AMOUNT_MIN,
            max: WATER_AMOUNT_MAX,
        });
    }
    Ok(())
}

/// Validates brew time
pub fn validate_brew_time(time: i32) -> Result<()> {
    if time < BREW_TIME_MIN || time > BREW_TIME_MAX {
        return Err(BrewingError::InvalidBrewTime {
            time,
            min: BREW_TIME_MIN,
            max: BREW_TIME_MAX,
        });
    }
    Ok(())
}

/// Validates water temperature
pub fn validate_water_temperature(temp: i32) -> Result<()> {
    if temp < WATER_TEMP_MIN || temp > WATER_TEMP_MAX {
        return Err(BrewingError::InvalidWaterTemperature {
            temp,
            min: WATER_TEMP_MIN,
            max: WATER_TEMP_MAX,
        });
    }
    Ok(())
}

/// Validates grind setting against grinder range
pub fn validate_grind_setting(setting: i32, min_setting: i32, max_setting: i32, grinder_name: &str) -> Result<()> {
    if setting < min_setting || setting > max_setting {
        return Err(BrewingError::GrindSettingOutOfRange {
            setting,
            min: min_setting,
            max: max_setting,
            grinder: grinder_name.to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_coffee_ratio() {
        // Valid ratios
        assert!(validate_coffee_ratio(20.0, 320.0).is_ok()); // 16:1 ratio
        assert!(validate_coffee_ratio(20.0, 300.0).is_ok()); // 15:1 ratio
        assert!(validate_coffee_ratio(20.0, 340.0).is_ok()); // 17:1 ratio
        
        // Invalid ratios
        assert!(validate_coffee_ratio(20.0, 280.0).is_err()); // 14:1 ratio (too low)
        assert!(validate_coffee_ratio(20.0, 360.0).is_err()); // 18:1 ratio (too high)
    }

    #[test]
    fn test_validate_rating() {
        // Valid ratings
        assert!(validate_rating(1.0).is_ok());
        assert!(validate_rating(5.5).is_ok());
        assert!(validate_rating(10.0).is_ok());
        assert!(validate_rating(7.5).is_ok());
        
        // Invalid ratings
        assert!(validate_rating(0.5).is_err()); // Too low
        assert!(validate_rating(10.5).is_err()); // Too high
        assert!(validate_rating(5.3).is_err()); // Not in 0.5 increments
        assert!(validate_rating(7.7).is_err()); // Not in 0.5 increments
    }

    #[test]
    fn test_validate_coffee_amount() {
        // Valid amounts
        assert!(validate_coffee_amount(20.0).is_ok());
        assert!(validate_coffee_amount(10.0).is_ok());
        assert!(validate_coffee_amount(100.0).is_ok());
        
        // Invalid amounts
        assert!(validate_coffee_amount(5.0).is_err()); // Too low
        assert!(validate_coffee_amount(150.0).is_err()); // Too high
    }

    #[test]
    fn test_validate_water_amount() {
        // Valid amounts
        assert!(validate_water_amount(320.0).is_ok());
        assert!(validate_water_amount(150.0).is_ok());
        assert!(validate_water_amount(1700.0).is_ok());
        
        // Invalid amounts
        assert!(validate_water_amount(100.0).is_err()); // Too low
        assert!(validate_water_amount(2000.0).is_err()); // Too high
    }

    #[test]
    fn test_validate_grind_setting() {
        // Valid settings
        assert!(validate_grind_setting(15, 10, 20, "Test Grinder").is_ok());
        assert!(validate_grind_setting(10, 10, 20, "Test Grinder").is_ok());
        assert!(validate_grind_setting(20, 10, 20, "Test Grinder").is_ok());
        
        // Invalid settings
        assert!(validate_grind_setting(5, 10, 20, "Test Grinder").is_err()); // Too low
        assert!(validate_grind_setting(25, 10, 20, "Test Grinder").is_err()); // Too high
    }
}