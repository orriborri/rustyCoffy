use crate::models::*;
use crate::services::database::{Database, SessionFilter, GrinderStats};
use crate::validation::{BrewingError, Result};

/// Business logic service wrapping database operations
pub struct BrewingService {
    db: Database,
}

impl BrewingService {
    /// Creates a new BrewingService instance
    pub fn new(database_url: &str) -> Result<Self> {
        let db = Database::new(database_url)?;
        Ok(BrewingService { db })
    }
    
    /// Creates a new BrewingService from an existing Database
    pub fn from_database(db: Database) -> Self {
        BrewingService { db }
    }
}

// Session management methods
impl BrewingService {
    /// Creates a new brewing session with comprehensive validation
    pub fn create_session(&self, session: NewBrewingSession) -> Result<i32> {
        // Validate the session parameters
        session.validate()?;
        
        // Validate that the grinder exists and grind setting is in range
        let grinder = self.db.get_grinder_by_id(session.grinder_id)?
            .ok_or(BrewingError::GrinderNotFound(session.grinder_id))?;
        
        session.validate_grind_setting(&grinder)?;
        
        // Validate that the bean exists and has sufficient quantity
        let bean = self.db.get_coffee_bean_by_id(session.bean_id)?
            .ok_or(BrewingError::BeanNotFound(session.bean_id))?;
        
        if !bean.has_sufficient_quantity(session.coffee_grams) {
            return Err(BrewingError::InsufficientBeanQuantity {
                available: bean.remaining_grams.unwrap_or(0.0),
                required: session.coffee_grams,
            });
        }
        
        // Create the session
        let created_session = self.db.create_brewing_session(session)?;
        
        Ok(created_session.id)
    }
    
    /// Gets brewing sessions with optional filtering
    pub fn get_sessions(&self, filter: SessionFilter) -> Result<Vec<BrewingSession>> {
        self.db.get_brewing_sessions(Some(filter))
    }
    
    /// Gets all brewing sessions without filtering
    pub fn get_all_sessions(&self) -> Result<Vec<BrewingSession>> {
        self.db.get_brewing_sessions(None)
    }
    
    /// Gets a brewing session by ID
    pub fn get_session_by_id(&self, id: i32) -> Result<Option<BrewingSession>> {
        self.db.get_brewing_session_by_id(id)
    }
    
    /// Duplicates a brewing session for reproducing successful brews
    pub fn duplicate_session(&self, id: i32) -> Result<i32> {
        let duplicated_session = self.db.duplicate_brewing_session(id)?;
        Ok(duplicated_session.id)
    }
    
    /// Searches brewing sessions by query string
    pub fn search_sessions(&self, query: &str) -> Result<Vec<BrewingSession>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }
        
        self.db.search_brewing_sessions(query)
    }
}

// Bean inventory management methods
impl BrewingService {
    /// Adds a new coffee bean with validation
    pub fn add_bean(&self, bean: NewCoffeeBean) -> Result<i32> {
        bean.validate()?;
        let created_bean = self.db.create_coffee_bean(bean)?;
        Ok(created_bean.id)
    }
    
    /// Gets all coffee beans
    pub fn get_beans(&self) -> Result<Vec<CoffeeBean>> {
        self.db.get_coffee_beans()
    }
    
    /// Gets only active coffee beans (with remaining quantity > 0)
    pub fn get_active_beans(&self) -> Result<Vec<CoffeeBean>> {
        self.db.get_active_coffee_beans()
    }
    
    /// Gets a coffee bean by ID
    pub fn get_bean_by_id(&self, id: i32) -> Result<Option<CoffeeBean>> {
        self.db.get_coffee_bean_by_id(id)
    }
    
    /// Updates bean quantity after manual adjustment
    pub fn update_bean_quantity(&self, id: i32, grams_used: f32) -> Result<()> {
        if grams_used < 0.0 {
            return Err(BrewingError::InvalidBeanData {
                message: format!("Grams used cannot be negative, got: {}", grams_used),
            });
        }
        
        self.db.update_coffee_bean_quantity(id, grams_used)
    }
    
    /// Gets usage history for a specific bean
    pub fn get_bean_usage_history(&self, bean_id: i32) -> Result<Vec<BrewingSession>> {
        // Verify bean exists
        self.db.get_coffee_bean_by_id(bean_id)?
            .ok_or(BrewingError::BeanNotFound(bean_id))?;
        
        self.db.get_bean_usage_history(bean_id)
    }
}

// Grinder equipment management methods
impl BrewingService {
    /// Adds a new grinder with validation
    pub fn add_grinder(&self, grinder: NewGrinder) -> Result<i32> {
        grinder.validate()?;
        let created_grinder = self.db.create_grinder(grinder)?;
        Ok(created_grinder.id)
    }
    
    /// Gets all grinders
    pub fn get_grinders(&self) -> Result<Vec<Grinder>> {
        self.db.get_grinders()
    }
    
    /// Gets a grinder by ID
    pub fn get_grinder_by_id(&self, id: i32) -> Result<Option<Grinder>> {
        self.db.get_grinder_by_id(id)
    }
    
    /// Gets usage statistics for a grinder
    pub fn get_grinder_usage_stats(&self, grinder_id: i32) -> Result<GrinderStats> {
        // Verify grinder exists
        self.db.get_grinder_by_id(grinder_id)?
            .ok_or(BrewingError::GrinderNotFound(grinder_id))?;
        
        self.db.get_grinder_usage_stats(grinder_id)
    }
    
    /// Validates a grind setting against a specific grinder
    pub fn validate_grind_setting(&self, grinder_id: i32, setting: i32) -> Result<()> {
        let grinder = self.db.get_grinder_by_id(grinder_id)?
            .ok_or(BrewingError::GrinderNotFound(grinder_id))?;
        
        if !grinder.is_valid_setting(setting) {
            return Err(BrewingError::GrindSettingOutOfRange {
                setting,
                min: grinder.min_setting,
                max: grinder.max_setting,
                grinder: grinder.full_name(),
            });
        }
        
        Ok(())
    }
}

#[cfg(test)]
#[path = "brewing_service_tests.rs"]
mod brewing_service_tests;

#[cfg(all(test, feature = "postgres-tests"))]
#[path = "brewing_service_integration_tests.rs"]
mod brewing_service_integration_tests;
