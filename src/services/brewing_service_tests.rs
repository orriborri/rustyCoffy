#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::*;
    use crate::validation::BrewingError;
    use chrono::Utc;
    use proptest::prelude::*;
    
    // Helper function to create a valid test bean
    fn create_test_bean() -> NewCoffeeBean {
        NewCoffeeBean {
            name: "Test Bean".to_string(),
            origin: "Ethiopia".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(500.0),
            variety: Some("Arabica".to_string()),
            processing_method: Some("Washed".to_string()),
        }
    }
    
    // Helper function to create a valid test grinder
    fn create_test_grinder() -> NewGrinder {
        NewGrinder {
            brand: "Baratza".to_string(),
            model: "Encore".to_string(),
            grinder_type: GrinderType::BurrConical.to_string(),
            min_setting: 10,
            max_setting: 40,
        }
    }
    
    // Helper function to create a valid test session
    fn create_test_session(bean_id: i32, grinder_id: i32) -> NewBrewingSession {
        NewBrewingSession {
            bean_id,
            grinder_id,
            grind_setting: 20,
            brewing_method: BrewingMethod::V60.to_string(),
            water_temp_celsius: Some(95),
            brew_time_seconds: Some(240),
            coffee_grams: 20.0,
            water_grams: 320.0,
            tasting_notes: Some("Great coffee!".to_string()),
            rating: Some(8.5),
        }
    }
    
    // Property-based tests for validation logic
    proptest! {
        #[test]
        fn test_coffee_ratio_validation(
            coffee_grams in 10.0f32..100.0f32,
            ratio in 15.0f32..17.0f32
        ) {
            let water_grams = coffee_grams * ratio;
            let session = NewBrewingSession {
                bean_id: 1,
                grinder_id: 1,
                grind_setting: 20,
                brewing_method: BrewingMethod::V60.to_string(),
                water_temp_celsius: Some(95),
                brew_time_seconds: Some(240),
                coffee_grams,
                water_grams,
                tasting_notes: None,
                rating: None,
            };
            
            // Should pass validation for valid ratios
            prop_assert!(session.validate().is_ok());
        }
        
        #[test]
        fn test_invalid_coffee_ratio(
            coffee_grams in 10.0f32..100.0f32,
            ratio in prop::sample::select(vec![10.0f32, 14.0, 18.0, 20.0])
        ) {
            let water_grams = coffee_grams * ratio;
            let session = NewBrewingSession {
                bean_id: 1,
                grinder_id: 1,
                grind_setting: 20,
                brewing_method: BrewingMethod::V60.to_string(),
                water_temp_celsius: Some(95),
                brew_time_seconds: Some(240),
                coffee_grams,
                water_grams,
                tasting_notes: None,
                rating: None,
            };
            
            // Should fail validation for invalid ratios
            prop_assert!(session.validate().is_err());
        }
        
        #[test]
        fn test_rating_validation(
            rating in prop::sample::select(vec![1.0f32, 1.5, 2.0, 5.5, 7.0, 9.5, 10.0])
        ) {
            let session = NewBrewingSession {
                bean_id: 1,
                grinder_id: 1,
                grind_setting: 20,
                brewing_method: BrewingMethod::V60.to_string(),
                water_temp_celsius: Some(95),
                brew_time_seconds: Some(240),
                coffee_grams: 20.0,
                water_grams: 320.0,
                tasting_notes: None,
                rating: Some(rating),
            };
            
            // Should pass validation for valid ratings
            prop_assert!(session.validate().is_ok());
        }
        
        #[test]
        fn test_invalid_rating(
            rating in prop::sample::select(vec![0.5f32, 0.7, 5.3, 7.7, 10.5, 11.0])
        ) {
            let session = NewBrewingSession {
                bean_id: 1,
                grinder_id: 1,
                grind_setting: 20,
                brewing_method: BrewingMethod::V60.to_string(),
                water_temp_celsius: Some(95),
                brew_time_seconds: Some(240),
                coffee_grams: 20.0,
                water_grams: 320.0,
                tasting_notes: None,
                rating: Some(rating),
            };
            
            // Should fail validation for invalid ratings
            prop_assert!(session.validate().is_err());
        }
        
        #[test]
        fn test_grind_setting_validation(
            setting in 10i32..=40i32
        ) {
            let grinder = Grinder {
                id: 1,
                brand: "Test".to_string(),
                model: "Grinder".to_string(),
                grinder_type: GrinderType::BurrConical.to_string(),
                min_setting: 10,
                max_setting: 40,
                created_at: Utc::now().naive_utc(),
            };
            
            // Should pass validation for settings in range
            prop_assert!(grinder.is_valid_setting(setting));
        }
        
        #[test]
        fn test_invalid_grind_setting(
            setting in prop::sample::select(vec![5i32, 9, 41, 50, 100])
        ) {
            let grinder = Grinder {
                id: 1,
                brand: "Test".to_string(),
                model: "Grinder".to_string(),
                grinder_type: GrinderType::BurrConical.to_string(),
                min_setting: 10,
                max_setting: 40,
                created_at: Utc::now().naive_utc(),
            };
            
            // Should fail validation for settings outside range
            prop_assert!(!grinder.is_valid_setting(setting));
        }
    }
    
    // Unit tests for business logic
    #[test]
    fn test_new_bean_validation_empty_name() {
        let bean = NewCoffeeBean {
            name: "".to_string(),
            origin: "Ethiopia".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(500.0),
            variety: None,
            processing_method: None,
        };
        
        assert!(bean.validate().is_err());
    }
    
    #[test]
    fn test_new_bean_validation_empty_origin() {
        let bean = NewCoffeeBean {
            name: "Test Bean".to_string(),
            origin: "".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(500.0),
            variety: None,
            processing_method: None,
        };
        
        assert!(bean.validate().is_err());
    }
    
    #[test]
    fn test_new_bean_validation_negative_quantity() {
        let bean = NewCoffeeBean {
            name: "Test Bean".to_string(),
            origin: "Ethiopia".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(-10.0),
            variety: None,
            processing_method: None,
        };
        
        assert!(bean.validate().is_err());
    }
    
    #[test]
    fn test_new_grinder_validation_invalid_range() {
        let grinder = NewGrinder {
            brand: "Test".to_string(),
            model: "Grinder".to_string(),
            grinder_type: GrinderType::BurrConical.to_string(),
            min_setting: 40,
            max_setting: 10, // Max less than min
        };
        
        assert!(grinder.validate().is_err());
    }
    
    #[test]
    fn test_new_grinder_validation_zero_min() {
        let grinder = NewGrinder {
            brand: "Test".to_string(),
            model: "Grinder".to_string(),
            grinder_type: GrinderType::BurrConical.to_string(),
            min_setting: 0,
            max_setting: 40,
        };
        
        assert!(grinder.validate().is_err());
    }
    
    #[test]
    fn test_new_grinder_validation_empty_brand() {
        let grinder = NewGrinder {
            brand: "".to_string(),
            model: "Grinder".to_string(),
            grinder_type: GrinderType::BurrConical.to_string(),
            min_setting: 10,
            max_setting: 40,
        };
        
        assert!(grinder.validate().is_err());
    }
    
    #[test]
    fn test_session_validation_valid() {
        let session = create_test_session(1, 1);
        assert!(session.validate().is_ok());
    }
    
    #[test]
    fn test_session_validation_invalid_coffee_amount() {
        let mut session = create_test_session(1, 1);
        session.coffee_grams = 5.0; // Too low
        assert!(session.validate().is_err());
        
        session.coffee_grams = 150.0; // Too high
        assert!(session.validate().is_err());
    }
    
    #[test]
    fn test_session_validation_invalid_water_amount() {
        let mut session = create_test_session(1, 1);
        session.water_grams = 100.0; // Too low
        assert!(session.validate().is_err());
        
        session.water_grams = 2000.0; // Too high
        assert!(session.validate().is_err());
    }
    
    #[test]
    fn test_session_validation_invalid_brew_time() {
        let mut session = create_test_session(1, 1);
        session.brew_time_seconds = Some(20); // Too low
        assert!(session.validate().is_err());
        
        session.brew_time_seconds = Some(500); // Too high
        assert!(session.validate().is_err());
    }
    
    #[test]
    fn test_session_validation_invalid_water_temp() {
        let mut session = create_test_session(1, 1);
        session.water_temp_celsius = Some(50); // Too low
        assert!(session.validate().is_err());
        
        session.water_temp_celsius = Some(110); // Too high
        assert!(session.validate().is_err());
    }
    
    #[test]
    fn test_coffee_bean_has_sufficient_quantity() {
        let bean = CoffeeBean {
            id: 1,
            name: "Test".to_string(),
            origin: "Ethiopia".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(100.0),
            variety: None,
            processing_method: None,
            created_at: Utc::now().naive_utc(),
        };
        
        assert!(bean.has_sufficient_quantity(50.0));
        assert!(bean.has_sufficient_quantity(100.0));
        assert!(!bean.has_sufficient_quantity(150.0));
    }
    
    #[test]
    fn test_coffee_bean_use_beans() {
        let mut bean = CoffeeBean {
            id: 1,
            name: "Test".to_string(),
            origin: "Ethiopia".to_string(),
            roast_date: Utc::now().naive_utc().date(),
            purchase_date: Utc::now().naive_utc().date(),
            remaining_grams: Some(100.0),
            variety: None,
            processing_method: None,
            created_at: Utc::now().naive_utc(),
        };
        
        assert!(bean.use_beans(30.0).is_ok());
        assert_eq!(bean.remaining_grams, Some(70.0));
        
        assert!(bean.use_beans(80.0).is_err()); // Not enough remaining
    }
    
    #[test]
    fn test_brewing_session_coffee_ratio() {
        let session = BrewingSession {
            id: 1,
            bean_id: 1,
            grinder_id: 1,
            grind_setting: 20,
            brewing_method: BrewingMethod::V60.to_string(),
            water_temp_celsius: Some(95),
            brew_time_seconds: Some(240),
            coffee_grams: 20.0,
            water_grams: 320.0,
            tasting_notes: None,
            rating: Some(8.5),
            created_at: Utc::now().naive_utc(),
        };
        
        assert_eq!(session.coffee_ratio(), 16.0);
    }
    
    #[test]
    fn test_brewing_session_is_high_quality() {
        let mut session = BrewingSession {
            id: 1,
            bean_id: 1,
            grinder_id: 1,
            grind_setting: 20,
            brewing_method: BrewingMethod::V60.to_string(),
            water_temp_celsius: Some(95),
            brew_time_seconds: Some(240),
            coffee_grams: 20.0,
            water_grams: 320.0,
            tasting_notes: None,
            rating: Some(8.5),
            created_at: Utc::now().naive_utc(),
        };
        
        assert!(session.is_high_quality());
        
        session.rating = Some(6.5);
        assert!(!session.is_high_quality());
        
        session.rating = None;
        assert!(!session.is_high_quality());
    }
    
    #[test]
    fn test_brewing_session_to_new_session() {
        let session = BrewingSession {
            id: 1,
            bean_id: 1,
            grinder_id: 1,
            grind_setting: 20,
            brewing_method: BrewingMethod::V60.to_string(),
            water_temp_celsius: Some(95),
            brew_time_seconds: Some(240),
            coffee_grams: 20.0,
            water_grams: 320.0,
            tasting_notes: Some("Original notes".to_string()),
            rating: Some(8.5),
            created_at: Utc::now().naive_utc(),
        };
        
        let new_session = session.to_new_session();
        
        assert_eq!(new_session.bean_id, session.bean_id);
        assert_eq!(new_session.grinder_id, session.grinder_id);
        assert_eq!(new_session.grind_setting, session.grind_setting);
        assert_eq!(new_session.brewing_method, session.brewing_method);
        assert_eq!(new_session.coffee_grams, session.coffee_grams);
        assert_eq!(new_session.water_grams, session.water_grams);
        
        // Tasting notes and rating should not be copied
        assert_eq!(new_session.tasting_notes, None);
        assert_eq!(new_session.rating, None);
    }
    
    #[test]
    fn test_grinder_full_name() {
        let grinder = Grinder {
            id: 1,
            brand: "Baratza".to_string(),
            model: "Encore".to_string(),
            grinder_type: GrinderType::BurrConical.to_string(),
            min_setting: 10,
            max_setting: 40,
            created_at: Utc::now().naive_utc(),
        };
        
        assert_eq!(grinder.full_name(), "Baratza Encore");
    }
    
    #[test]
    fn test_grinder_setting_range() {
        let grinder = Grinder {
            id: 1,
            brand: "Test".to_string(),
            model: "Grinder".to_string(),
            grinder_type: GrinderType::BurrConical.to_string(),
            min_setting: 10,
            max_setting: 40,
            created_at: Utc::now().naive_utc(),
        };
        
        let range = grinder.setting_range();
        assert_eq!(*range.start(), 10);
        assert_eq!(*range.end(), 40);
    }
}
