#[cfg(test)]
mod tests {
    use crate::models::*;
    use proptest::prelude::*;
    use chrono::{NaiveDate, Utc};
    use crate::validation::*;

    // Property-based test strategies
    prop_compose! {
        fn valid_coffee_amount()(amount in COFFEE_AMOUNT_MIN..=COFFEE_AMOUNT_MAX) -> f32 {
            amount
        }
    }

    prop_compose! {
        fn valid_water_amount()(amount in WATER_AMOUNT_MIN..=WATER_AMOUNT_MAX) -> f32 {
            amount
        }
    }

    prop_compose! {
        fn valid_rating()(rating in 1.0f32..=10.0f32) -> f32 {
            // Ensure rating is in 0.5 increments
            (rating * 2.0).floor() / 2.0
        }
    }

    prop_compose! {
        fn valid_grind_setting(min: i32, max: i32)(setting in min..=max) -> i32 {
            setting
        }
    }

    prop_compose! {
        fn valid_coffee_ratio_pair()(
            coffee in COFFEE_AMOUNT_MIN..=COFFEE_AMOUNT_MAX,
            ratio in COFFEE_RATIO_MIN..=COFFEE_RATIO_MAX
        ) -> (f32, f32) {
            let water = coffee * ratio;
            (coffee, water.min(WATER_AMOUNT_MAX).max(WATER_AMOUNT_MIN))
        }
    }

    prop_compose! {
        fn past_date()(days_ago in 0i64..365) -> NaiveDate {
            let today = Utc::now().naive_utc().date();
            today - chrono::Duration::days(days_ago)
        }
    }

    prop_compose! {
        fn non_empty_string()(s in "[a-zA-Z][a-zA-Z0-9 ]{1,50}") -> String {
            s
        }
    }

    // Tests for NewBrewingSession validation
    proptest! {
        #[test]
        fn test_valid_brewing_session_always_passes(
            bean_id in 1i32..1000,
            grinder_id in 1i32..100,
            grind_setting in 1i32..50,
            brewing_method in prop_oneof![
                Just("V60".to_string()),
                Just("Chemex".to_string()),
                Just("FrenchPress".to_string()),
                Just("AeroPress".to_string()),
                Just("Espresso".to_string()),
            ],
            (coffee_grams, water_grams) in valid_coffee_ratio_pair(),
            rating in proptest::option::of(valid_rating()),
            brew_time in proptest::option::of(BREW_TIME_MIN..=BREW_TIME_MAX),
            water_temp in proptest::option::of(WATER_TEMP_MIN..=WATER_TEMP_MAX),
        ) {
            let session = NewBrewingSession {
                bean_id,
                grinder_id,
                grind_setting,
                brewing_method,
                water_temp_celsius: water_temp,
                brew_time_seconds: brew_time,
                coffee_grams,
                water_grams,
                tasting_notes: None,
                rating,
            };

            prop_assert!(session.validate().is_ok());
        }

        #[test]
        fn test_invalid_coffee_ratio_fails(
            bean_id in 1i32..1000,
            grinder_id in 1i32..100,
            grind_setting in 1i32..50,
            coffee_grams in valid_coffee_amount(),
            // Create invalid ratios
            ratio in prop_oneof![
                (1.0f32..COFFEE_RATIO_MIN), // Too low
                (COFFEE_RATIO_MAX + 0.1..25.0f32) // Too high
            ]
        ) {
            let water_grams = coffee_grams * ratio;
            let session = NewBrewingSession {
                bean_id,
                grinder_id,
                grind_setting,
                brewing_method: "V60".to_string(),
                water_temp_celsius: None,
                brew_time_seconds: None,
                coffee_grams,
                water_grams,
                tasting_notes: None,
                rating: None,
            };

            prop_assert!(session.validate().is_err());
        }

        #[test]
        fn test_invalid_rating_fails(
            bean_id in 1i32..1000,
            grinder_id in 1i32..100,
            grind_setting in 1i32..50,
            (coffee_grams, water_grams) in valid_coffee_ratio_pair(),
            // Invalid ratings (not in 0.5 increments or out of range)
            rating in prop_oneof![
                (0.0f32..1.0f32), // Too low
                (10.1f32..15.0f32), // Too high
                (1.1f32..9.9f32).prop_filter("Not in 0.5 increments", |r| (r * 2.0) % 1.0 != 0.0)
            ]
        ) {
            let session = NewBrewingSession {
                bean_id,
                grinder_id,
                grind_setting,
                brewing_method: "V60".to_string(),
                water_temp_celsius: None,
                brew_time_seconds: None,
                coffee_grams,
                water_grams,
                tasting_notes: None,
                rating: Some(rating),
            };

            prop_assert!(session.validate().is_err());
        }

        #[test]
        fn test_coffee_amount_bounds(
            bean_id in 1i32..1000,
            grinder_id in 1i32..100,
            grind_setting in 1i32..50,
            coffee_grams in prop_oneof![
                (0.0f32..COFFEE_AMOUNT_MIN), // Too low
                (COFFEE_AMOUNT_MAX + 0.1..200.0f32) // Too high
            ]
        ) {
            let session = NewBrewingSession {
                bean_id,
                grinder_id,
                grind_setting,
                brewing_method: "V60".to_string(),
                water_temp_celsius: None,
                brew_time_seconds: None,
                coffee_grams,
                water_grams: 320.0, // Valid water amount
                tasting_notes: None,
                rating: None,
            };

            prop_assert!(session.validate().is_err());
        }

        #[test]
        fn test_water_amount_bounds(
            bean_id in 1i32..1000,
            grinder_id in 1i32..100,
            grind_setting in 1i32..50,
            water_grams in prop_oneof![
                (0.0f32..WATER_AMOUNT_MIN), // Too low
                (WATER_AMOUNT_MAX + 0.1..2000.0f32) // Too high
            ]
        ) {
            let session = NewBrewingSession {
                bean_id,
                grinder_id,
                grind_setting,
                brewing_method: "V60".to_string(),
                water_temp_celsius: None,
                brew_time_seconds: None,
                coffee_grams: 20.0, // Valid coffee amount
                water_grams,
                tasting_notes: None,
                rating: None,
            };

            prop_assert!(session.validate().is_err());
        }
    }

    // Tests for NewGrinder validation
    proptest! {
        #[test]
        fn test_valid_grinder_always_passes(
            brand in non_empty_string(),
            model in non_empty_string(),
            grinder_type in prop_oneof![
                Just("BurrConical".to_string()),
                Just("BurrFlat".to_string()),
                Just("Blade".to_string()),
                Just("Manual".to_string()),
            ],
            min_setting in 1i32..20,
            extra_range in 1i32..30,
        ) {
            let max_setting = min_setting + extra_range;
            let grinder = NewGrinder {
                brand,
                model,
                grinder_type,
                min_setting,
                max_setting,
            };

            prop_assert!(grinder.validate().is_ok());
        }

        #[test]
        fn test_invalid_grinder_settings_fail(
            brand in non_empty_string(),
            model in non_empty_string(),
            min_setting in prop_oneof![
                (-100i32..=0), // Invalid min (non-positive)
                (1i32..50) // Valid min for max test
            ],
            max_offset in prop_oneof![
                (-50i32..=0), // Max <= min
                (1i32..30) // Valid offset for min test
            ]
        ) {
            let max_setting = if min_setting <= 0 {
                min_setting + 10 // Valid max for invalid min test
            } else {
                min_setting + max_offset // This will be <= min when max_offset <= 0
            };

            let grinder = NewGrinder {
                brand,
                model,
                grinder_type: "BurrConical".to_string(),
                min_setting,
                max_setting,
            };

            if min_setting <= 0 || max_setting <= min_setting {
                prop_assert!(grinder.validate().is_err());
            }
        }

        #[test]
        fn test_empty_brand_or_model_fails(
            brand in prop_oneof![
                Just("".to_string()),
                Just("   ".to_string()), // Whitespace only
                non_empty_string()
            ],
            model in prop_oneof![
                Just("".to_string()),
                Just("   ".to_string()), // Whitespace only
                non_empty_string()
            ]
        ) {
            let grinder = NewGrinder {
                brand: brand.clone(),
                model: model.clone(),
                grinder_type: "BurrConical".to_string(),
                min_setting: 1,
                max_setting: 10,
            };

            if brand.trim().is_empty() || model.trim().is_empty() {
                prop_assert!(grinder.validate().is_err());
            } else {
                prop_assert!(grinder.validate().is_ok());
            }
        }
    }

    // Tests for NewCoffeeBean validation
    proptest! {
        #[test]
        fn test_valid_coffee_bean_always_passes(
            name in non_empty_string(),
            origin in non_empty_string(),
            roast_date in past_date(),
            purchase_date in past_date(),
            remaining_grams in proptest::option::of(0.0f32..1000.0f32),
            variety in proptest::option::of(non_empty_string()),
            processing_method in proptest::option::of(non_empty_string()),
        ) {
            let bean = NewCoffeeBean {
                name,
                origin,
                roast_date,
                purchase_date,
                remaining_grams,
                variety,
                processing_method,
            };

            prop_assert!(bean.validate().is_ok());
        }

        #[test]
        fn test_empty_name_or_origin_fails(
            name in prop_oneof![
                Just("".to_string()),
                Just("   ".to_string()),
                non_empty_string()
            ],
            origin in prop_oneof![
                Just("".to_string()),
                Just("   ".to_string()),
                non_empty_string()
            ],
            roast_date in past_date(),
            purchase_date in past_date(),
        ) {
            let bean = NewCoffeeBean {
                name: name.clone(),
                origin: origin.clone(),
                roast_date,
                purchase_date,
                remaining_grams: None,
                variety: None,
                processing_method: None,
            };

            if name.trim().is_empty() || origin.trim().is_empty() {
                prop_assert!(bean.validate().is_err());
            } else {
                prop_assert!(bean.validate().is_ok());
            }
        }

        #[test]
        fn test_future_dates_fail(
            name in non_empty_string(),
            origin in non_empty_string(),
            future_days in 1i64..365,
        ) {
            let today = Utc::now().naive_utc().date();
            let future_date = today + chrono::Duration::days(future_days);
            
            // Test future roast date
            let bean_future_roast = NewCoffeeBean {
                name: name.clone(),
                origin: origin.clone(),
                roast_date: future_date,
                purchase_date: today,
                remaining_grams: None,
                variety: None,
                processing_method: None,
            };
            prop_assert!(bean_future_roast.validate().is_err());

            // Test future purchase date
            let bean_future_purchase = NewCoffeeBean {
                name,
                origin,
                roast_date: today,
                purchase_date: future_date,
                remaining_grams: None,
                variety: None,
                processing_method: None,
            };
            prop_assert!(bean_future_purchase.validate().is_err());
        }

        #[test]
        fn test_negative_remaining_grams_fails(
            name in non_empty_string(),
            origin in non_empty_string(),
            roast_date in past_date(),
            purchase_date in past_date(),
            negative_grams in -1000.0f32..-0.1f32,
        ) {
            let bean = NewCoffeeBean {
                name,
                origin,
                roast_date,
                purchase_date,
                remaining_grams: Some(negative_grams),
                variety: None,
                processing_method: None,
            };

            prop_assert!(bean.validate().is_err());
        }
    }

    // Unit tests for utility methods
    #[test]
    fn test_brewing_session_coffee_ratio() {
        let session = BrewingSession {
            id: 1,
            bean_id: 1,
            grinder_id: 1,
            grind_setting: 15,
            brewing_method: "V60".to_string(),
            water_temp_celsius: Some(95),
            brew_time_seconds: Some(240),
            coffee_grams: 20.0,
            water_grams: 320.0,
            tasting_notes: None,
            rating: Some(8.5),
            created_at: Utc::now().naive_utc(),
        };

        assert_eq!(session.coffee_ratio(), 16.0);
        assert!(session.is_high_quality());
    }

    #[test]
    fn test_grinder_utility_methods() {
        let grinder = Grinder {
            id: 1,
            brand: "Baratza".to_string(),
            model: "Encore".to_string(),
            grinder_type: "BurrConical".to_string(),
            min_setting: 1,
            max_setting: 40,
            created_at: Utc::now().naive_utc(),
        };

        assert_eq!(grinder.full_name(), "Baratza Encore");
        assert!(grinder.is_valid_setting(20));
        assert!(!grinder.is_valid_setting(50));
        assert_eq!(grinder.setting_range(), 1..=40);
    }

    #[test]
    fn test_coffee_bean_utility_methods() {
        let mut bean = CoffeeBean {
            id: 1,
            name: "Ethiopian Yirgacheffe".to_string(),
            origin: "Ethiopia".to_string(),
            roast_date: Utc::now().naive_utc().date() - chrono::Duration::days(7),
            purchase_date: Utc::now().naive_utc().date() - chrono::Duration::days(5),
            remaining_grams: Some(250.0),
            variety: Some("Heirloom".to_string()),
            processing_method: Some("Washed".to_string()),
            created_at: Utc::now().naive_utc(),
        };

        assert!(bean.has_sufficient_quantity(20.0));
        assert!(!bean.has_sufficient_quantity(300.0));
        assert!(bean.is_fresh());
        assert_eq!(bean.days_since_roast(), 7);

        // Test using beans
        assert!(bean.use_beans(20.0).is_ok());
        assert_eq!(bean.remaining_grams, Some(230.0));

        // Test insufficient quantity
        assert!(bean.use_beans(300.0).is_err());
    }

    #[test]
    fn test_grind_setting_validation() {
        let grinder = Grinder {
            id: 1,
            brand: "Test".to_string(),
            model: "Grinder".to_string(),
            grinder_type: "BurrConical".to_string(),
            min_setting: 10,
            max_setting: 20,
            created_at: Utc::now().naive_utc(),
        };

        let session = NewBrewingSession {
            bean_id: 1,
            grinder_id: 1,
            grind_setting: 15,
            brewing_method: "V60".to_string(),
            water_temp_celsius: None,
            brew_time_seconds: None,
            coffee_grams: 20.0,
            water_grams: 320.0,
            tasting_notes: None,
            rating: None,
        };

        assert!(session.validate_grind_setting(&grinder).is_ok());

        let invalid_session = NewBrewingSession {
            grind_setting: 25, // Outside range
            ..session
        };

        assert!(invalid_session.validate_grind_setting(&grinder).is_err());
    }
}