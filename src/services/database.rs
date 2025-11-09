use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use chrono::NaiveDate;
use crate::models::*;
use crate::validation::{BrewingError, Result};
use crate::schema::{coffee_beans, grinders, brewing_sessions};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Database service with connection pooling for Postgres
pub struct Database {
    pool: DbPool,
}

impl Database {
    /// Creates a new Database instance with connection pool
    pub fn new(database_url: &str) -> Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .map_err(BrewingError::Pool)?;
        
        Ok(Database { pool })
    }
    
    /// Gets a connection from the pool
    fn get_connection(&self) -> Result<DbConnection> {
        self.pool.get().map_err(BrewingError::Pool)
    }
}

// Coffee Bean CRUD Operations
impl Database {
    /// Creates a new coffee bean
    pub fn create_coffee_bean(&self, new_bean: NewCoffeeBean) -> Result<CoffeeBean> {
        new_bean.validate()?;
        
        let mut conn = self.get_connection()?;
        let bean = diesel::insert_into(coffee_beans::table)
            .values(&new_bean)
            .get_result(&mut conn)
            .map_err(BrewingError::Database)?;
        
        Ok(bean)
    }
    
    /// Gets all coffee beans
    pub fn get_coffee_beans(&self) -> Result<Vec<CoffeeBean>> {
        let mut conn = self.get_connection()?;
        let beans = coffee_beans::table
            .order(coffee_beans::created_at.desc())
            .select(CoffeeBean::as_select())
            .load(&mut conn)
            .map_err(BrewingError::Database)?;
        
        Ok(beans)
    }
    
    /// Gets coffee beans that still have remaining quantity
    pub fn get_active_coffee_beans(&self) -> Result<Vec<CoffeeBean>> {
        let mut conn = self.get_connection()?;
        let beans = coffee_beans::table
            .filter(coffee_beans::remaining_grams.is_not_null())
            .filter(coffee_beans::remaining_grams.gt(0.0))
            .order(coffee_beans::created_at.desc())
            .select(CoffeeBean::as_select())
            .load(&mut conn)
            .map_err(BrewingError::Database)?;
        
        Ok(beans)
    }
    
    /// Gets a coffee bean by ID
    pub fn get_coffee_bean_by_id(&self, id: i32) -> Result<Option<CoffeeBean>> {
        let mut conn = self.get_connection()?;
        let bean = coffee_beans::table
            .find(id)
            .select(CoffeeBean::as_select())
            .first(&mut conn)
            .optional()
            .map_err(BrewingError::Database)?;
        
        Ok(bean)
    }
    
    /// Updates coffee bean quantity after use
    pub fn update_coffee_bean_quantity(&self, bean_id: i32, grams_used: f32) -> Result<()> {
        let mut conn = self.get_connection()?;
        
        // Get current bean to validate quantity
        let bean = self.get_coffee_bean_by_id(bean_id)?
            .ok_or(BrewingError::BeanNotFound(bean_id))?;
        
        if let Some(current_quantity) = bean.remaining_grams {
            if current_quantity < grams_used {
                return Err(BrewingError::InsufficientBeanQuantity {
                    available: current_quantity,
                    required: grams_used,
                });
            }
            
            let new_quantity = current_quantity - grams_used;
            diesel::update(coffee_beans::table.find(bean_id))
                .set(coffee_beans::remaining_grams.eq(new_quantity))
                .execute(&mut conn)
                .map_err(BrewingError::Database)?;
        }
        
        Ok(())
    }
    
    /// Gets usage history for a specific bean
    pub fn get_bean_usage_history(&self, bean_id: i32) -> Result<Vec<BrewingSession>> {
        let mut conn = self.get_connection()?;
        let sessions = brewing_sessions::table
            .filter(brewing_sessions::bean_id.eq(bean_id))
            .order(brewing_sessions::created_at.desc())
            .select(BrewingSession::as_select())
            .load(&mut conn)
            .map_err(BrewingError::Database)?;
        
        Ok(sessions)
    }
}

// Grinder CRUD Operations
impl Database {
    /// Creates a new grinder
    pub fn create_grinder(&self, new_grinder: NewGrinder) -> Result<Grinder> {
        new_grinder.validate()?;
        
        let mut conn = self.get_connection()?;
        let grinder = diesel::insert_into(grinders::table)
            .values(&new_grinder)
            .get_result(&mut conn)
            .map_err(BrewingError::Database)?;
        
        Ok(grinder)
    }
    
    /// Gets all grinders
    pub fn get_grinders(&self) -> Result<Vec<Grinder>> {
        let mut conn = self.get_connection()?;
        let grinders = grinders::table
            .order(grinders::created_at.desc())
            .select(Grinder::as_select())
            .load(&mut conn)
            .map_err(BrewingError::Database)?;
        
        Ok(grinders)
    }
    
    /// Gets a grinder by ID
    pub fn get_grinder_by_id(&self, id: i32) -> Result<Option<Grinder>> {
        let mut conn = self.get_connection()?;
        let grinder = grinders::table
            .find(id)
            .select(Grinder::as_select())
            .first(&mut conn)
            .optional()
            .map_err(BrewingError::Database)?;
        
        Ok(grinder)
    }
    
    /// Gets usage statistics for a grinder
    pub fn get_grinder_usage_stats(&self, grinder_id: i32) -> Result<GrinderStats> {
        let mut conn = self.get_connection()?;
        
        // Get total sessions count
        let total_sessions = brewing_sessions::table
            .filter(brewing_sessions::grinder_id.eq(grinder_id))
            .count()
            .get_result::<i64>(&mut conn)
            .map_err(BrewingError::Database)?;
        
        // Get average rating
        let avg_rating: Option<f64> = brewing_sessions::table
            .filter(brewing_sessions::grinder_id.eq(grinder_id))
            .filter(brewing_sessions::rating.is_not_null())
            .select(diesel::dsl::avg(brewing_sessions::rating))
            .first(&mut conn)
            .map_err(BrewingError::Database)?;
        
        // Get most used grind setting
        let most_used_setting: Option<i32> = brewing_sessions::table
            .filter(brewing_sessions::grinder_id.eq(grinder_id))
            .group_by(brewing_sessions::grind_setting)
            .order(diesel::dsl::count(brewing_sessions::id).desc())
            .select(brewing_sessions::grind_setting)
            .first(&mut conn)
            .optional()
            .map_err(BrewingError::Database)?;
        
        Ok(GrinderStats {
            grinder_id,
            total_sessions,
            average_rating: avg_rating.map(|r| r as f32),
            most_used_setting,
        })
    }
}

// Brewing Session CRUD Operations
impl Database {
    /// Creates a new brewing session
    pub fn create_brewing_session(&self, new_session: NewBrewingSession) -> Result<BrewingSession> {
        new_session.validate()?;
        
        // Validate grind setting against grinder
        let grinder = self.get_grinder_by_id(new_session.grinder_id)?
            .ok_or(BrewingError::GrinderNotFound(new_session.grinder_id))?;
        
        new_session.validate_grind_setting(&grinder)?;
        
        // Check bean exists and has sufficient quantity
        let bean = self.get_coffee_bean_by_id(new_session.bean_id)?
            .ok_or(BrewingError::BeanNotFound(new_session.bean_id))?;
        
        if !bean.has_sufficient_quantity(new_session.coffee_grams) {
            return Err(BrewingError::InsufficientBeanQuantity {
                available: bean.remaining_grams.unwrap_or(0.0),
                required: new_session.coffee_grams,
            });
        }
        
        let mut conn = self.get_connection()?;
        
        // Create the session
        let session = diesel::insert_into(brewing_sessions::table)
            .values(&new_session)
            .get_result(&mut conn)
            .map_err(BrewingError::Database)?;
        
        // Update bean quantity if tracking
        if bean.remaining_grams.is_some() {
            self.update_coffee_bean_quantity(new_session.bean_id, new_session.coffee_grams)?;
        }
        
        Ok(session)
    }
    
    /// Gets all brewing sessions with optional filtering
    pub fn get_brewing_sessions(&self, filter: Option<SessionFilter>) -> Result<Vec<BrewingSession>> {
        let mut conn = self.get_connection()?;
        
        // Start with base query
        let mut query = brewing_sessions::table
            .inner_join(coffee_beans::table)
            .inner_join(grinders::table)
            .into_boxed();
        
        if let Some(filter) = filter {
            if let Some(bean_origin) = filter.bean_origin {
                query = query.filter(coffee_beans::origin.eq(bean_origin));
            }
            
            if let Some(roast_date_from) = filter.roast_date_from {
                query = query.filter(coffee_beans::roast_date.ge(roast_date_from));
            }
            
            if let Some(roast_date_to) = filter.roast_date_to {
                query = query.filter(coffee_beans::roast_date.le(roast_date_to));
            }
            
            if let Some(grinder_type) = filter.grinder_type {
                query = query.filter(grinders::grinder_type.eq(grinder_type.to_string()));
            }
            
            if let Some(brewing_method) = filter.brewing_method {
                query = query.filter(brewing_sessions::brewing_method.eq(brewing_method.to_string()));
            }
            
            if let Some(rating_min) = filter.rating_min {
                query = query.filter(brewing_sessions::rating.ge(rating_min));
            }
            
            if let Some(limit) = filter.limit {
                query = query.limit(limit);
            }
        }
        
        let sessions = query
            .order(brewing_sessions::created_at.desc())
            .select(BrewingSession::as_select())
            .load(&mut conn)
            .map_err(BrewingError::Database)?;
        
        Ok(sessions)
    }
    
    /// Gets a brewing session by ID
    pub fn get_brewing_session_by_id(&self, id: i32) -> Result<Option<BrewingSession>> {
        let mut conn = self.get_connection()?;
        let session = brewing_sessions::table
            .find(id)
            .select(BrewingSession::as_select())
            .first(&mut conn)
            .optional()
            .map_err(BrewingError::Database)?;
        
        Ok(session)
    }
    
    /// Searches brewing sessions by multiple criteria
    pub fn search_brewing_sessions(&self, query: &str) -> Result<Vec<BrewingSession>> {
        let mut conn = self.get_connection()?;
        let search_term = format!("%{}%", query.to_lowercase());
        
        let sessions = brewing_sessions::table
            .inner_join(coffee_beans::table)
            .inner_join(grinders::table)
            .filter(
                coffee_beans::name.ilike(&search_term)
                    .or(coffee_beans::origin.ilike(&search_term))
                    .or(grinders::brand.ilike(&search_term))
                    .or(grinders::model.ilike(&search_term))
                    .or(brewing_sessions::tasting_notes.ilike(&search_term))
            )
            .select(BrewingSession::as_select())
            .order(brewing_sessions::created_at.desc())
            .load(&mut conn)
            .map_err(BrewingError::Database)?;
        
        Ok(sessions)
    }
    
    /// Duplicates a brewing session (creates a new session with same parameters)
    pub fn duplicate_brewing_session(&self, session_id: i32) -> Result<BrewingSession> {
        let original_session = self.get_brewing_session_by_id(session_id)?
            .ok_or(BrewingError::Database(diesel::result::Error::NotFound))?;
        
        let new_session = original_session.to_new_session();
        self.create_brewing_session(new_session)
    }
}

/// Filter struct for session queries
#[derive(Debug, Clone)]
pub struct SessionFilter {
    pub bean_origin: Option<String>,
    pub roast_date_from: Option<NaiveDate>,
    pub roast_date_to: Option<NaiveDate>,
    pub grinder_type: Option<GrinderType>,
    pub brewing_method: Option<BrewingMethod>,
    pub rating_min: Option<f32>,
    pub limit: Option<i64>,
}

impl Default for SessionFilter {
    fn default() -> Self {
        Self {
            bean_origin: None,
            roast_date_from: None,
            roast_date_to: None,
            grinder_type: None,
            brewing_method: None,
            rating_min: None,
            limit: Some(100), // Default limit to prevent huge queries
        }
    }
}

/// Statistics for grinder usage
#[derive(Debug, Clone)]
pub struct GrinderStats {
    pub grinder_id: i32,
    pub total_sessions: i64,
    pub average_rating: Option<f32>,
    pub most_used_setting: Option<i32>,
}

#[cfg(all(test, feature = "postgres-tests"))]
mod tests {
    use super::*;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use std::env;
    
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    
    fn setup_test_db() -> Database {
        let database_url = env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/coffee_tracker_test".to_string());
        
        let db = Database::new(&database_url).expect("Failed to create database");
        
        // Run migrations
        let mut conn = db.get_connection().expect("Failed to get connection");
        conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
        
        // Clean up existing data
        diesel::delete(brewing_sessions::table).execute(&mut conn).ok();
        diesel::delete(coffee_beans::table).execute(&mut conn).ok();
        diesel::delete(grinders::table).execute(&mut conn).ok();
        
        db
    }
    
    fn create_test_bean(db: &Database) -> CoffeeBean {
        let new_bean = NewCoffeeBean {
            name: "Test Bean".to_string(),
            origin: "Test Origin".to_string(),
            roast_date: chrono::Utc::now().naive_utc().date(),
            purchase_date: chrono::Utc::now().naive_utc().date(),
            remaining_grams: Some(500.0),
            variety: Some("Arabica".to_string()),
            processing_method: Some("Washed".to_string()),
        };
        
        db.create_coffee_bean(new_bean).expect("Failed to create test bean")
    }
    
    fn create_test_grinder(db: &Database) -> Grinder {
        let new_grinder = NewGrinder {
            brand: "Test Brand".to_string(),
            model: "Test Model".to_string(),
            grinder_type: "BurrConical".to_string(),
            min_setting: 10,
            max_setting: 40,
        };
        
        db.create_grinder(new_grinder).expect("Failed to create test grinder")
    }
    
    #[test]
    fn test_create_and_get_coffee_bean() {
        let db = setup_test_db();
        let bean = create_test_bean(&db);
        
        assert_eq!(bean.name, "Test Bean");
        assert_eq!(bean.origin, "Test Origin");
        assert_eq!(bean.remaining_grams, Some(500.0));
        
        let retrieved_bean = db.get_coffee_bean_by_id(bean.id)
            .expect("Failed to get bean")
            .expect("Bean not found");
        
        assert_eq!(retrieved_bean.id, bean.id);
        assert_eq!(retrieved_bean.name, bean.name);
    }
    
    #[test]
    fn test_create_and_get_grinder() {
        let db = setup_test_db();
        let grinder = create_test_grinder(&db);
        
        assert_eq!(grinder.brand, "Test Brand");
        assert_eq!(grinder.model, "Test Model");
        assert_eq!(grinder.grinder_type, "BurrConical".to_string());
        
        let retrieved_grinder = db.get_grinder_by_id(grinder.id)
            .expect("Failed to get grinder")
            .expect("Grinder not found");
        
        assert_eq!(retrieved_grinder.id, grinder.id);
        assert_eq!(retrieved_grinder.brand, grinder.brand);
    }
    
    #[test]
    fn test_create_brewing_session() {
        let db = setup_test_db();
        let bean = create_test_bean(&db);
        let grinder = create_test_grinder(&db);
        
        let new_session = NewBrewingSession {
            bean_id: bean.id,
            grinder_id: grinder.id,
            grind_setting: 20,
            brewing_method: "V60".to_string(),
            water_temp_celsius: Some(95),
            brew_time_seconds: Some(240),
            coffee_grams: 20.0,
            water_grams: 320.0,
            tasting_notes: Some("Great coffee!".to_string()),
            rating: Some(8.5),
        };
        
        let session = db.create_brewing_session(new_session)
            .expect("Failed to create brewing session");
        
        assert_eq!(session.bean_id, bean.id);
        assert_eq!(session.grinder_id, grinder.id);
        assert_eq!(session.grind_setting, 20);
        assert_eq!(session.coffee_ratio(), 16.0);
        
        // Check that bean quantity was updated
        let updated_bean = db.get_coffee_bean_by_id(bean.id)
            .expect("Failed to get bean")
            .expect("Bean not found");
        
        assert_eq!(updated_bean.remaining_grams, Some(480.0));
    }
    
    #[test]
    fn test_session_filtering() {
        let db = setup_test_db();
        let bean = create_test_bean(&db);
        let grinder = create_test_grinder(&db);
        
        // Create multiple sessions
        for i in 0..3 {
            let new_session = NewBrewingSession {
                bean_id: bean.id,
                grinder_id: grinder.id,
                grind_setting: 20 + i,
                brewing_method: "V60".to_string(),
                water_temp_celsius: Some(95),
                brew_time_seconds: Some(240),
                coffee_grams: 20.0,
                water_grams: 320.0,
                tasting_notes: None,
                rating: Some(7.0 + i as f32),
            };
            
            db.create_brewing_session(new_session)
                .expect("Failed to create brewing session");
        }
        
        // Test filtering by rating
        let filter = SessionFilter {
            rating_min: Some(8.0),
            ..Default::default()
        };
        
        let filtered_sessions = db.get_brewing_sessions(Some(filter))
            .expect("Failed to get filtered sessions");
        
        assert_eq!(filtered_sessions.len(), 2);
        for session in filtered_sessions {
            assert!(session.rating.unwrap() >= 8.0);
        }
    }
    
    #[test]
    fn test_search_sessions() {
        let db = setup_test_db();
        let bean = create_test_bean(&db);
        let grinder = create_test_grinder(&db);
        
        let new_session = NewBrewingSession {
            bean_id: bean.id,
            grinder_id: grinder.id,
            grind_setting: 20,
            brewing_method: "V60".to_string(),
            water_temp_celsius: Some(95),
            brew_time_seconds: Some(240),
            coffee_grams: 20.0,
            water_grams: 320.0,
            tasting_notes: Some("Excellent Ethiopian coffee".to_string()),
            rating: Some(9.0),
        };
        
        db.create_brewing_session(new_session)
            .expect("Failed to create brewing session");
        
        // Search by tasting notes
        let results = db.search_brewing_sessions("Ethiopian")
            .expect("Failed to search sessions");
        
        assert_eq!(results.len(), 1);
        assert!(results[0].tasting_notes.as_ref().unwrap().contains("Ethiopian"));
    }
    
    #[test]
    fn test_grinder_usage_stats() {
        let db = setup_test_db();
        let bean = create_test_bean(&db);
        let grinder = create_test_grinder(&db);
        
        // Create multiple sessions with different ratings
        let ratings = vec![7.5, 8.0, 9.0];
        for rating in ratings {
            let new_session = NewBrewingSession {
                bean_id: bean.id,
                grinder_id: grinder.id,
                grind_setting: 20,
                brewing_method: "V60".to_string(),
                water_temp_celsius: Some(95),
                brew_time_seconds: Some(240),
                coffee_grams: 20.0,
                water_grams: 320.0,
                tasting_notes: None,
                rating: Some(rating),
            };
            
            db.create_brewing_session(new_session)
                .expect("Failed to create brewing session");
        }
        
        let stats = db.get_grinder_usage_stats(grinder.id)
            .expect("Failed to get grinder stats");
        
        assert_eq!(stats.total_sessions, 3);
        assert_eq!(stats.average_rating, Some(8.166667)); // Average of 7.5, 8.0, 9.0
        assert_eq!(stats.most_used_setting, Some(20));
    }
    
    #[test]
    fn test_duplicate_session() {
        let db = setup_test_db();
        let bean = create_test_bean(&db);
        let grinder = create_test_grinder(&db);
        
        let new_session = NewBrewingSession {
            bean_id: bean.id,
            grinder_id: grinder.id,
            grind_setting: 20,
            brewing_method: "V60".to_string(),
            water_temp_celsius: Some(95),
            brew_time_seconds: Some(240),
            coffee_grams: 20.0,
            water_grams: 320.0,
            tasting_notes: Some("Original session".to_string()),
            rating: Some(8.5),
        };
        
        let original_session = db.create_brewing_session(new_session)
            .expect("Failed to create original session");
        
        let duplicated_session = db.duplicate_brewing_session(original_session.id)
            .expect("Failed to duplicate session");
        
        // Check that parameters match but ID and timestamps are different
        assert_ne!(duplicated_session.id, original_session.id);
        assert_eq!(duplicated_session.bean_id, original_session.bean_id);
        assert_eq!(duplicated_session.grinder_id, original_session.grinder_id);
        assert_eq!(duplicated_session.grind_setting, original_session.grind_setting);
        assert_eq!(duplicated_session.brewing_method, original_session.brewing_method);
        
        // Tasting notes and rating should be None for duplicated session
        assert_eq!(duplicated_session.tasting_notes, None);
        assert_eq!(duplicated_session.rating, None);
    }
    
    #[test]
    fn test_insufficient_bean_quantity() {
        let db = setup_test_db();
        let mut bean = create_test_bean(&db);
        bean.remaining_grams = Some(10.0); // Only 10g available
        
        let grinder = create_test_grinder(&db);
        
        let new_session = NewBrewingSession {
            bean_id: bean.id,
            grinder_id: grinder.id,
            grind_setting: 20,
            brewing_method: "V60".to_string(),
            water_temp_celsius: Some(95),
            brew_time_seconds: Some(240),
            coffee_grams: 20.0, // Trying to use 20g when only 10g available
            water_grams: 320.0,
            tasting_notes: None,
            rating: None,
        };
        
        let result = db.create_brewing_session(new_session);
        assert!(result.is_err());
        
        if let Err(BrewingError::InsufficientBeanQuantity { available, required }) = result {
            assert_eq!(available, 500.0); // Original amount from create_test_bean
            assert_eq!(required, 20.0);
        } else {
            panic!("Expected InsufficientBeanQuantity error");
        }
    }
    
    #[test]
    fn test_invalid_grind_setting() {
        let db = setup_test_db();
        let bean = create_test_bean(&db);
        let grinder = create_test_grinder(&db); // Range 10-40
        
        let new_session = NewBrewingSession {
            bean_id: bean.id,
            grinder_id: grinder.id,
            grind_setting: 50, // Outside range
            brewing_method: "V60".to_string(),
            water_temp_celsius: Some(95),
            brew_time_seconds: Some(240),
            coffee_grams: 20.0,
            water_grams: 320.0,
            tasting_notes: None,
            rating: None,
        };
        
        let result = db.create_brewing_session(new_session);
        assert!(result.is_err());
        
        if let Err(BrewingError::GrindSettingOutOfRange { setting, min, max, .. }) = result {
            assert_eq!(setting, 50);
            assert_eq!(min, 10);
            assert_eq!(max, 40);
        } else {
            panic!("Expected GrindSettingOutOfRange error");
        }
    }
}