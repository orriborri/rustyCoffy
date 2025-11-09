#[cfg(all(test, feature = "postgres-tests"))]
mod integration_tests {
    use super::super::*;
    use crate::models::*;
    use crate::validation::BrewingError;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use std::env;
    use chrono::Utc;
    
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    
    fn setup_test_service() -> BrewingService {
        let database_url = env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/coffee_tracker_test".to_string());
        
        let db = Database::new(&database_url).expect("Failed to create database");
        
        // Run migrations
        let mut conn = db.get_connection().expect("Failed to get connection");
        conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
        
        // Clean up existing data
        use crate::schema::*;
        diesel::delete(brewing_sessions::table).execute(&mut conn).ok();
        diesel::delete(coffee_beans::table).execute(&mut conn).ok();
        diesel::delete(grinders::table).execute(&mut conn).ok();
        
        BrewingService::from_database(db)
    }
    
    #[test]
    fn test_complete_workflow_create_equipment_and_session() {
        let service = setup_test_service();
        
        // Step 1: Add a coffee bean
        let new_bean = NewCoffeeBean {
            name: "Ethiopian Yirgacheffe".to_string(),
            origin: "Ethiopia".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(500.0),
            variety: Some("Heirloom".to_string()),
            processing_method: Some("Washed".to_string()),
        };
        
        let bean_id = service.add_bean(new_bean).expect("Failed to add bean");
        assert!(bean_id > 0);
        
        // Step 2: Add a grinder
        let new_grinder = NewGrinder {
            brand: "Baratza".to_string(),
            model: "Encore".to_string(),
            grinder_type: GrinderType::BurrConical.to_string(),
            min_setting: 10,
            max_setting: 40,
        };
        
        let grinder_id = service.add_grinder(new_grinder).expect("Failed to add grinder");
        assert!(grinder_id > 0);
        
        // Step 3: Create a brewing session
        let new_session = NewBrewingSession {
            bean_id,
            grinder_id,
            grind_setting: 20,
            brewing_method: BrewingMethod::V60.to_string(),
            water_temp_celsius: Some(95),
            brew_time_seconds: Some(240),
            coffee_grams: 20.0,
            water_grams: 320.0,
            tasting_notes: Some("Bright and fruity".to_string()),
            rating: Some(8.5),
        };
        
        let session_id = service.create_session(new_session).expect("Failed to create session");
        assert!(session_id > 0);
        
        // Step 4: Verify bean quantity was updated
        let bean = service.get_bean_by_id(bean_id)
            .expect("Failed to get bean")
            .expect("Bean not found");
        
        assert_eq!(bean.remaining_grams, Some(480.0));
        
        // Step 5: Verify session was created correctly
        let session = service.get_session_by_id(session_id)
            .expect("Failed to get session")
            .expect("Session not found");
        
        assert_eq!(session.bean_id, bean_id);
        assert_eq!(session.grinder_id, grinder_id);
        assert_eq!(session.coffee_ratio(), 16.0);
    }
    
    #[test]
    fn test_session_filtering_by_rating() {
        let service = setup_test_service();
        
        // Create test data
        let bean_id = service.add_bean(NewCoffeeBean {
            name: "Test Bean".to_string(),
            origin: "Colombia".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(1000.0),
            variety: None,
            processing_method: None,
        }).expect("Failed to add bean");
        
        let grinder_id = service.add_grinder(NewGrinder {
            brand: "Test".to_string(),
            model: "Grinder".to_string(),
            grinder_type: GrinderType::BurrConical.to_string(),
            min_setting: 10,
            max_setting: 40,
        }).expect("Failed to add grinder");
        
        // Create sessions with different ratings
        let ratings = vec![6.0, 7.5, 8.0, 9.0];
        for rating in ratings {
            let session = NewBrewingSession {
                bean_id,
                grinder_id,
                grind_setting: 20,
                brewing_method: BrewingMethod::V60.to_string(),
                water_temp_celsius: Some(95),
                brew_time_seconds: Some(240),
                coffee_grams: 20.0,
                water_grams: 320.0,
                tasting_notes: None,
                rating: Some(rating),
            };
            
            service.create_session(session).expect("Failed to create session");
        }
        
        // Filter by rating >= 8.0
        let filter = SessionFilter {
            rating_min: Some(8.0),
            ..Default::default()
        };
        
        let filtered_sessions = service.get_sessions(filter)
            .expect("Failed to get filtered sessions");
        
        assert_eq!(filtered_sessions.len(), 2);
        for session in filtered_sessions {
            assert!(session.rating.unwrap() >= 8.0);
        }
    }
    
    #[test]
    fn test_session_search() {
        let service = setup_test_service();
        
        // Create test data with searchable content
        let bean_id = service.add_bean(NewCoffeeBean {
            name: "Ethiopian Sidamo".to_string(),
            origin: "Ethiopia".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(500.0),
            variety: None,
            processing_method: None,
        }).expect("Failed to add bean");
        
        let grinder_id = service.add_grinder(NewGrinder {
            brand: "Comandante".to_string(),
            model: "C40".to_string(),
            grinder_type: GrinderType::Manual.to_string(),
            min_setting: 1,
            max_setting: 30,
        }).expect("Failed to add grinder");
        
        let session = NewBrewingSession {
            bean_id,
            grinder_id,
            grind_setting: 15,
            brewing_method: BrewingMethod::V60.to_string(),
            water_temp_celsius: Some(95),
            brew_time_seconds: Some(240),
            coffee_grams: 20.0,
            water_grams: 320.0,
            tasting_notes: Some("Blueberry notes with chocolate finish".to_string()),
            rating: Some(9.0),
        };
        
        service.create_session(session).expect("Failed to create session");
        
        // Search by bean name
        let results = service.search_sessions("Ethiopian")
            .expect("Failed to search sessions");
        assert_eq!(results.len(), 1);
        
        // Search by grinder brand
        let results = service.search_sessions("Comandante")
            .expect("Failed to search sessions");
        assert_eq!(results.len(), 1);
        
        // Search by tasting notes
        let results = service.search_sessions("blueberry")
            .expect("Failed to search sessions");
        assert_eq!(results.len(), 1);
        
        // Search with no results
        let results = service.search_sessions("nonexistent")
            .expect("Failed to search sessions");
        assert_eq!(results.len(), 0);
    }
    
    #[test]
    fn test_session_duplication() {
        let service = setup_test_service();
        
        // Create test data
        let bean_id = service.add_bean(NewCoffeeBean {
            name: "Test Bean".to_string(),
            origin: "Brazil".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(500.0),
            variety: None,
            processing_method: None,
        }).expect("Failed to add bean");
        
        let grinder_id = service.add_grinder(NewGrinder {
            brand: "Test".to_string(),
            model: "Grinder".to_string(),
            grinder_type: GrinderType::BurrConical.to_string(),
            min_setting: 10,
            max_setting: 40,
        }).expect("Failed to add grinder");
        
        // Create original session
        let original_session = NewBrewingSession {
            bean_id,
            grinder_id,
            grind_setting: 25,
            brewing_method: BrewingMethod::AeroPress.to_string(),
            water_temp_celsius: Some(90),
            brew_time_seconds: Some(120),
            coffee_grams: 15.0,
            water_grams: 240.0,
            tasting_notes: Some("Original notes".to_string()),
            rating: Some(8.0),
        };
        
        let original_id = service.create_session(original_session)
            .expect("Failed to create original session");
        
        // Duplicate the session
        let duplicated_id = service.duplicate_session(original_id)
            .expect("Failed to duplicate session");
        
        assert_ne!(duplicated_id, original_id);
        
        // Verify duplicated session has same parameters but no rating/notes
        let original = service.get_session_by_id(original_id)
            .expect("Failed to get original")
            .expect("Original not found");
        
        let duplicated = service.get_session_by_id(duplicated_id)
            .expect("Failed to get duplicated")
            .expect("Duplicated not found");
        
        assert_eq!(duplicated.bean_id, original.bean_id);
        assert_eq!(duplicated.grinder_id, original.grinder_id);
        assert_eq!(duplicated.grind_setting, original.grind_setting);
        assert_eq!(duplicated.brewing_method, original.brewing_method);
        assert_eq!(duplicated.coffee_grams, original.coffee_grams);
        assert_eq!(duplicated.water_grams, original.water_grams);
        
        // Rating and notes should be None
        assert_eq!(duplicated.rating, None);
        assert_eq!(duplicated.tasting_notes, None);
    }
    
    #[test]
    fn test_grinder_usage_statistics() {
        let service = setup_test_service();
        
        // Create test data
        let bean_id = service.add_bean(NewCoffeeBean {
            name: "Test Bean".to_string(),
            origin: "Kenya".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(1000.0),
            variety: None,
            processing_method: None,
        }).expect("Failed to add bean");
        
        let grinder_id = service.add_grinder(NewGrinder {
            brand: "Test".to_string(),
            model: "Grinder".to_string(),
            grinder_type: GrinderType::BurrFlat.to_string(),
            min_setting: 1,
            max_setting: 50,
        }).expect("Failed to add grinder");
        
        // Create multiple sessions with different ratings and settings
        let sessions_data = vec![
            (20, 7.5),
            (20, 8.0),
            (25, 8.5),
            (20, 9.0),
        ];
        
        for (setting, rating) in sessions_data {
            let session = NewBrewingSession {
                bean_id,
                grinder_id,
                grind_setting: setting,
                brewing_method: BrewingMethod::Espresso.to_string(),
                water_temp_celsius: Some(93),
                brew_time_seconds: Some(30),
                coffee_grams: 18.0,
                water_grams: 288.0,
                tasting_notes: None,
                rating: Some(rating),
            };
            
            service.create_session(session).expect("Failed to create session");
        }
        
        // Get usage statistics
        let stats = service.get_grinder_usage_stats(grinder_id)
            .expect("Failed to get grinder stats");
        
        assert_eq!(stats.total_sessions, 4);
        assert_eq!(stats.most_used_setting, Some(20));
        
        // Average rating should be (7.5 + 8.0 + 8.5 + 9.0) / 4 = 8.25
        assert!(stats.average_rating.is_some());
        let avg = stats.average_rating.unwrap();
        assert!((avg - 8.25).abs() < 0.01);
    }
    
    #[test]
    fn test_bean_usage_history() {
        let service = setup_test_service();
        
        // Create test data
        let bean_id = service.add_bean(NewCoffeeBean {
            name: "Test Bean".to_string(),
            origin: "Guatemala".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(500.0),
            variety: None,
            processing_method: None,
        }).expect("Failed to add bean");
        
        let grinder_id = service.add_grinder(NewGrinder {
            brand: "Test".to_string(),
            model: "Grinder".to_string(),
            grinder_type: GrinderType::BurrConical.to_string(),
            min_setting: 10,
            max_setting: 40,
        }).expect("Failed to add grinder");
        
        // Create multiple sessions with this bean
        for i in 0..3 {
            let session = NewBrewingSession {
                bean_id,
                grinder_id,
                grind_setting: 20 + i,
                brewing_method: BrewingMethod::V60.to_string(),
                water_temp_celsius: Some(95),
                brew_time_seconds: Some(240),
                coffee_grams: 20.0,
                water_grams: 320.0,
                tasting_notes: None,
                rating: Some(8.0),
            };
            
            service.create_session(session).expect("Failed to create session");
        }
        
        // Get usage history
        let history = service.get_bean_usage_history(bean_id)
            .expect("Failed to get bean usage history");
        
        assert_eq!(history.len(), 3);
        
        // Verify all sessions use the correct bean
        for session in history {
            assert_eq!(session.bean_id, bean_id);
        }
    }
    
    #[test]
    fn test_insufficient_bean_quantity_error() {
        let service = setup_test_service();
        
        // Create bean with limited quantity
        let bean_id = service.add_bean(NewCoffeeBean {
            name: "Limited Bean".to_string(),
            origin: "Costa Rica".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(10.0), // Only 10g available
            variety: None,
            processing_method: None,
        }).expect("Failed to add bean");
        
        let grinder_id = service.add_grinder(NewGrinder {
            brand: "Test".to_string(),
            model: "Grinder".to_string(),
            grinder_type: GrinderType::BurrConical.to_string(),
            min_setting: 10,
            max_setting: 40,
        }).expect("Failed to add grinder");
        
        // Try to create session requiring more beans than available
        let session = NewBrewingSession {
            bean_id,
            grinder_id,
            grind_setting: 20,
            brewing_method: BrewingMethod::V60.to_string(),
            water_temp_celsius: Some(95),
            brew_time_seconds: Some(240),
            coffee_grams: 20.0, // Requires 20g but only 10g available
            water_grams: 320.0,
            tasting_notes: None,
            rating: None,
        };
        
        let result = service.create_session(session);
        assert!(result.is_err());
        
        match result {
            Err(BrewingError::InsufficientBeanQuantity { available, required }) => {
                assert_eq!(available, 10.0);
                assert_eq!(required, 20.0);
            }
            _ => panic!("Expected InsufficientBeanQuantity error"),
        }
    }
    
    #[test]
    fn test_invalid_grind_setting_error() {
        let service = setup_test_service();
        
        let bean_id = service.add_bean(NewCoffeeBean {
            name: "Test Bean".to_string(),
            origin: "Peru".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(500.0),
            variety: None,
            processing_method: None,
        }).expect("Failed to add bean");
        
        let grinder_id = service.add_grinder(NewGrinder {
            brand: "Test".to_string(),
            model: "Grinder".to_string(),
            grinder_type: GrinderType::BurrConical.to_string(),
            min_setting: 10,
            max_setting: 40,
        }).expect("Failed to add grinder");
        
        // Try to create session with grind setting outside range
        let session = NewBrewingSession {
            bean_id,
            grinder_id,
            grind_setting: 50, // Outside range 10-40
            brewing_method: BrewingMethod::V60.to_string(),
            water_temp_celsius: Some(95),
            brew_time_seconds: Some(240),
            coffee_grams: 20.0,
            water_grams: 320.0,
            tasting_notes: None,
            rating: None,
        };
        
        let result = service.create_session(session);
        assert!(result.is_err());
        
        match result {
            Err(BrewingError::GrindSettingOutOfRange { setting, min, max, .. }) => {
                assert_eq!(setting, 50);
                assert_eq!(min, 10);
                assert_eq!(max, 40);
            }
            _ => panic!("Expected GrindSettingOutOfRange error"),
        }
    }
    
    #[test]
    fn test_validate_grind_setting_method() {
        let service = setup_test_service();
        
        let grinder_id = service.add_grinder(NewGrinder {
            brand: "Test".to_string(),
            model: "Grinder".to_string(),
            grinder_type: GrinderType::BurrConical.to_string(),
            min_setting: 15,
            max_setting: 35,
        }).expect("Failed to add grinder");
        
        // Valid settings
        assert!(service.validate_grind_setting(grinder_id, 15).is_ok());
        assert!(service.validate_grind_setting(grinder_id, 25).is_ok());
        assert!(service.validate_grind_setting(grinder_id, 35).is_ok());
        
        // Invalid settings
        assert!(service.validate_grind_setting(grinder_id, 10).is_err());
        assert!(service.validate_grind_setting(grinder_id, 40).is_err());
    }
    
    #[test]
    fn test_get_active_beans_only() {
        let service = setup_test_service();
        
        // Add beans with different quantities
        service.add_bean(NewCoffeeBean {
            name: "Active Bean 1".to_string(),
            origin: "Colombia".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(100.0),
            variety: None,
            processing_method: None,
        }).expect("Failed to add bean");
        
        service.add_bean(NewCoffeeBean {
            name: "Empty Bean".to_string(),
            origin: "Brazil".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(0.0),
            variety: None,
            processing_method: None,
        }).expect("Failed to add bean");
        
        service.add_bean(NewCoffeeBean {
            name: "Active Bean 2".to_string(),
            origin: "Ethiopia".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(250.0),
            variety: None,
            processing_method: None,
        }).expect("Failed to add bean");
        
        service.add_bean(NewCoffeeBean {
            name: "No Tracking Bean".to_string(),
            origin: "Kenya".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: None,
            variety: None,
            processing_method: None,
        }).expect("Failed to add bean");
        
        // Get all beans
        let all_beans = service.get_beans().expect("Failed to get all beans");
        assert_eq!(all_beans.len(), 4);
        
        // Get only active beans (with remaining quantity > 0)
        let active_beans = service.get_active_beans().expect("Failed to get active beans");
        assert_eq!(active_beans.len(), 2);
        
        for bean in active_beans {
            assert!(bean.remaining_grams.is_some());
            assert!(bean.remaining_grams.unwrap() > 0.0);
        }
    }
}
