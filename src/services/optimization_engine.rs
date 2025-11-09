use crate::models::{BrewingSession, BrewingMethod};
use crate::services::database::Database;
use crate::validation::Result;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Analytics and optimization engine for brewing insights
pub struct OptimizationEngine {
    db: Database,
}

impl OptimizationEngine {
    /// Creates a new OptimizationEngine instance
    pub fn new(database_url: &str) -> Result<Self> {
        let db = Database::new(database_url)?;
        Ok(OptimizationEngine { db })
    }
    
    /// Creates a new OptimizationEngine from an existing Database
    pub fn from_database(db: Database) -> Self {
        OptimizationEngine { db }
    }
    
    /// Suggests optimal grind settings based on historical ratings
    pub fn suggest_grind_setting(
        &self,
        bean_id: i32,
        method: BrewingMethod,
    ) -> Result<Vec<GrindSuggestion>> {
        let sessions = self.db.get_brewing_sessions(None)?;
        
        // Filter sessions by bean and method
        let filtered: Vec<&BrewingSession> = sessions
            .iter()
            .filter(|s| {
                s.bean_id == bean_id
                    && s.brewing_method == method.to_string()
                    && s.rating.is_some()
            })
            .collect();
        
        if filtered.is_empty() {
            return Ok(Vec::new());
        }
        
        // Group by grind setting and calculate average rating
        let mut grind_stats: HashMap<i32, (f32, i64)> = HashMap::new();
        
        for session in filtered {
            let rating = session.rating.unwrap();
            let entry = grind_stats.entry(session.grind_setting).or_insert((0.0, 0));
            entry.0 += rating;
            entry.1 += 1;
        }
        
        // Calculate suggestions with confidence scores
        let mut suggestions: Vec<GrindSuggestion> = grind_stats
            .into_iter()
            .map(|(setting, (total_rating, count))| {
                let avg_rating = total_rating / count as f32;
                let confidence = calculate_confidence_score(count, avg_rating);
                
                GrindSuggestion {
                    grind_setting: setting,
                    average_rating: avg_rating,
                    session_count: count,
                    confidence_score: confidence,
                }
            })
            .collect();
        
        // Sort by average rating descending
        suggestions.sort_by(|a, b| {
            b.average_rating
                .partial_cmp(&a.average_rating)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        Ok(suggestions)
    }
    
    /// Analyzes correlation between grind size and brew quality
    pub fn analyze_grind_quality_correlation(&self) -> Result<CorrelationData> {
        let sessions = self.db.get_brewing_sessions(None)?;
        
        // Filter sessions with ratings
        let rated_sessions: Vec<&BrewingSession> = sessions
            .iter()
            .filter(|s| s.rating.is_some())
            .collect();
        
        if rated_sessions.is_empty() {
            return Ok(CorrelationData {
                grind_to_rating_correlation: 0.0,
                method_performance: Vec::new(),
                bean_grind_preferences: Vec::new(),
            });
        }
        
        // Calculate overall correlation
        let correlation = calculate_correlation(
            &rated_sessions
                .iter()
                .map(|s| s.grind_setting as f32)
                .collect::<Vec<_>>(),
            &rated_sessions
                .iter()
                .map(|s| s.rating.unwrap())
                .collect::<Vec<_>>(),
        );
        
        // Analyze method performance
        let method_performance = self.analyze_method_performance(&sessions)?;
        
        // Analyze bean grind preferences
        let bean_grind_preferences = self.analyze_bean_grind_preferences(&sessions)?;
        
        Ok(CorrelationData {
            grind_to_rating_correlation: correlation,
            method_performance,
            bean_grind_preferences,
        })
    }
    
    /// Analyzes brewing patterns and trends
    pub fn analyze_brewing_patterns(&self) -> Result<PatternAnalysis> {
        let sessions = self.db.get_brewing_sessions(None)?;
        
        if sessions.is_empty() {
            return Ok(PatternAnalysis {
                most_successful_combinations: Vec::new(),
                improvement_suggestions: Vec::new(),
                consistency_metrics: ConsistencyMetrics {
                    grind_setting_variance: 0.0,
                    rating_variance: 0.0,
                    coffee_ratio_variance: 0.0,
                },
            });
        }
        
        // Find most successful combinations
        let successful_combinations = self.find_successful_combinations(&sessions)?;
        
        // Generate improvement suggestions
        let improvement_suggestions = self.generate_improvement_suggestions(&sessions)?;
        
        // Calculate consistency metrics
        let consistency_metrics = self.calculate_consistency_metrics(&sessions)?;
        
        Ok(PatternAnalysis {
            most_successful_combinations: successful_combinations,
            improvement_suggestions,
            consistency_metrics,
        })
    }
    
    /// Analyzes equipment performance
    pub fn analyze_equipment_performance(&self) -> Result<EquipmentAnalysis> {
        let sessions = self.db.get_brewing_sessions(None)?;
        let grinders = self.db.get_grinders()?;
        let beans = self.db.get_coffee_beans()?;
        
        // Analyze grinder performance
        let mut grinder_performance = Vec::new();
        for grinder in grinders {
            let grinder_sessions: Vec<&BrewingSession> = sessions
                .iter()
                .filter(|s| s.grinder_id == grinder.id && s.rating.is_some())
                .collect();
            
            if !grinder_sessions.is_empty() {
                let avg_rating = grinder_sessions
                    .iter()
                    .map(|s| s.rating.unwrap())
                    .sum::<f32>()
                    / grinder_sessions.len() as f32;
                
                let success_rate = grinder_sessions
                    .iter()
                    .filter(|s| s.rating.unwrap() >= 7.0)
                    .count() as f32
                    / grinder_sessions.len() as f32;
                
                grinder_performance.push(GrinderPerformance {
                    grinder_id: grinder.id,
                    grinder_name: grinder.full_name(),
                    average_rating: avg_rating,
                    session_count: grinder_sessions.len() as i64,
                    success_rate,
                });
            }
        }
        
        // Analyze bean performance
        let mut bean_performance = Vec::new();
        for bean in beans {
            let bean_sessions: Vec<&BrewingSession> = sessions
                .iter()
                .filter(|s| s.bean_id == bean.id && s.rating.is_some())
                .collect();
            
            if !bean_sessions.is_empty() {
                let avg_rating = bean_sessions
                    .iter()
                    .map(|s| s.rating.unwrap())
                    .sum::<f32>()
                    / bean_sessions.len() as f32;
                
                let success_rate = bean_sessions
                    .iter()
                    .filter(|s| s.rating.unwrap() >= 7.0)
                    .count() as f32
                    / bean_sessions.len() as f32;
                
                bean_performance.push(BeanPerformance {
                    bean_id: bean.id,
                    bean_name: bean.name.clone(),
                    bean_origin: bean.origin.clone(),
                    average_rating: avg_rating,
                    session_count: bean_sessions.len() as i64,
                    success_rate,
                });
            }
        }
        
        Ok(EquipmentAnalysis {
            grinder_performance,
            bean_performance,
        })
    }
    
    // Private helper methods
    
    fn analyze_method_performance(&self, sessions: &[BrewingSession]) -> Result<Vec<MethodPerformance>> {
        let mut method_stats: HashMap<String, (f32, i64)> = HashMap::new();
        
        for session in sessions {
            if let Some(rating) = session.rating {
                let entry = method_stats
                    .entry(session.brewing_method.clone())
                    .or_insert((0.0, 0));
                entry.0 += rating;
                entry.1 += 1;
            }
        }
        
        let mut performance: Vec<MethodPerformance> = method_stats
            .into_iter()
            .map(|(method, (total_rating, count))| MethodPerformance {
                brewing_method: method,
                average_rating: total_rating / count as f32,
                session_count: count,
            })
            .collect();
        
        performance.sort_by(|a, b| {
            b.average_rating
                .partial_cmp(&a.average_rating)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        Ok(performance)
    }
    
    fn analyze_bean_grind_preferences(&self, sessions: &[BrewingSession]) -> Result<Vec<BeanGrindPreference>> {
        let mut bean_grind_stats: HashMap<i32, HashMap<i32, (f32, i64)>> = HashMap::new();
        
        for session in sessions {
            if let Some(rating) = session.rating {
                let grind_stats = bean_grind_stats.entry(session.bean_id).or_insert_with(HashMap::new);
                let entry = grind_stats.entry(session.grind_setting).or_insert((0.0, 0));
                entry.0 += rating;
                entry.1 += 1;
            }
        }
        
        let mut preferences = Vec::new();
        for (bean_id, grind_stats) in bean_grind_stats {
            if let Some((best_setting, (total_rating, count))) = grind_stats
                .iter()
                .max_by(|(_, (r1, c1)), (_, (r2, c2))| {
                    let avg1 = r1 / *c1 as f32;
                    let avg2 = r2 / *c2 as f32;
                    avg1.partial_cmp(&avg2).unwrap_or(std::cmp::Ordering::Equal)
                })
            {
                preferences.push(BeanGrindPreference {
                    bean_id,
                    preferred_grind_setting: *best_setting,
                    average_rating: total_rating / *count as f32,
                    session_count: *count,
                });
            }
        }
        
        Ok(preferences)
    }
    
    fn find_successful_combinations(&self, sessions: &[BrewingSession]) -> Result<Vec<SuccessfulCombination>> {
        let high_rated: Vec<&BrewingSession> = sessions
            .iter()
            .filter(|s| s.rating.map_or(false, |r| r >= 8.0))
            .collect();
        
        let mut combinations: HashMap<(i32, i32, String), (f32, i64)> = HashMap::new();
        
        for session in high_rated {
            let key = (session.bean_id, session.grinder_id, session.brewing_method.clone());
            let entry = combinations.entry(key).or_insert((0.0, 0));
            entry.0 += session.rating.unwrap();
            entry.1 += 1;
        }
        
        let mut successful: Vec<SuccessfulCombination> = combinations
            .into_iter()
            .filter(|(_, (_, count))| *count >= 2) // At least 2 successful sessions
            .map(|((bean_id, grinder_id, method), (total_rating, count))| {
                SuccessfulCombination {
                    bean_id,
                    grinder_id,
                    brewing_method: method,
                    average_rating: total_rating / count as f32,
                    session_count: count,
                }
            })
            .collect();
        
        successful.sort_by(|a, b| {
            b.average_rating
                .partial_cmp(&a.average_rating)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        Ok(successful)
    }
    
    fn generate_improvement_suggestions(&self, sessions: &[BrewingSession]) -> Result<Vec<ImprovementSuggestion>> {
        let mut suggestions = Vec::new();
        
        // Analyze low-rated sessions for improvement opportunities
        let low_rated: Vec<&BrewingSession> = sessions
            .iter()
            .filter(|s| s.rating.map_or(false, |r| r < 6.0))
            .collect();
        
        if !low_rated.is_empty() {
            // Group by bean and find patterns
            let mut bean_issues: HashMap<i32, Vec<&BrewingSession>> = HashMap::new();
            for session in low_rated {
                bean_issues.entry(session.bean_id).or_insert_with(Vec::new).push(session);
            }
            
            for (bean_id, bean_sessions) in bean_issues {
                if bean_sessions.len() >= 2 {
                    // Calculate average grind setting for low-rated sessions
                    let avg_grind = bean_sessions
                        .iter()
                        .map(|s| s.grind_setting as f32)
                        .sum::<f32>()
                        / bean_sessions.len() as f32;
                    
                    suggestions.push(ImprovementSuggestion {
                        bean_id: Some(bean_id),
                        grinder_id: None,
                        suggestion_type: "grind_adjustment".to_string(),
                        description: format!(
                            "Consider adjusting grind setting (currently averaging {:.0}) for better results",
                            avg_grind
                        ),
                        expected_impact: "medium".to_string(),
                    });
                }
            }
        }
        
        // Check for inconsistent ratios
        let ratios: Vec<f32> = sessions.iter().map(|s| s.coffee_ratio()).collect();
        if !ratios.is_empty() {
            let variance = calculate_variance(&ratios);
            if variance > 1.0 {
                suggestions.push(ImprovementSuggestion {
                    bean_id: None,
                    grinder_id: None,
                    suggestion_type: "consistency".to_string(),
                    description: "Coffee-to-water ratio varies significantly. Consider standardizing your measurements".to_string(),
                    expected_impact: "high".to_string(),
                });
            }
        }
        
        Ok(suggestions)
    }
    
    fn calculate_consistency_metrics(&self, sessions: &[BrewingSession]) -> Result<ConsistencyMetrics> {
        if sessions.is_empty() {
            return Ok(ConsistencyMetrics {
                grind_setting_variance: 0.0,
                rating_variance: 0.0,
                coffee_ratio_variance: 0.0,
            });
        }
        
        let grind_settings: Vec<f32> = sessions.iter().map(|s| s.grind_setting as f32).collect();
        let ratings: Vec<f32> = sessions
            .iter()
            .filter_map(|s| s.rating)
            .collect();
        let ratios: Vec<f32> = sessions.iter().map(|s| s.coffee_ratio()).collect();
        
        Ok(ConsistencyMetrics {
            grind_setting_variance: calculate_variance(&grind_settings),
            rating_variance: if !ratings.is_empty() {
                calculate_variance(&ratings)
            } else {
                0.0
            },
            coffee_ratio_variance: calculate_variance(&ratios),
        })
    }
}

// Data structures for analytics

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GrindSuggestion {
    pub grind_setting: i32,
    pub average_rating: f32,
    pub session_count: i64,
    pub confidence_score: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CorrelationData {
    pub grind_to_rating_correlation: f32,
    pub method_performance: Vec<MethodPerformance>,
    pub bean_grind_preferences: Vec<BeanGrindPreference>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MethodPerformance {
    pub brewing_method: String,
    pub average_rating: f32,
    pub session_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BeanGrindPreference {
    pub bean_id: i32,
    pub preferred_grind_setting: i32,
    pub average_rating: f32,
    pub session_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PatternAnalysis {
    pub most_successful_combinations: Vec<SuccessfulCombination>,
    pub improvement_suggestions: Vec<ImprovementSuggestion>,
    pub consistency_metrics: ConsistencyMetrics,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SuccessfulCombination {
    pub bean_id: i32,
    pub grinder_id: i32,
    pub brewing_method: String,
    pub average_rating: f32,
    pub session_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ImprovementSuggestion {
    pub bean_id: Option<i32>,
    pub grinder_id: Option<i32>,
    pub suggestion_type: String,
    pub description: String,
    pub expected_impact: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConsistencyMetrics {
    pub grind_setting_variance: f32,
    pub rating_variance: f32,
    pub coffee_ratio_variance: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EquipmentAnalysis {
    pub grinder_performance: Vec<GrinderPerformance>,
    pub bean_performance: Vec<BeanPerformance>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GrinderPerformance {
    pub grinder_id: i32,
    pub grinder_name: String,
    pub average_rating: f32,
    pub session_count: i64,
    pub success_rate: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BeanPerformance {
    pub bean_id: i32,
    pub bean_name: String,
    pub bean_origin: String,
    pub average_rating: f32,
    pub session_count: i64,
    pub success_rate: f32,
}

// Statistical helper functions

/// Calculates Pearson correlation coefficient between two datasets
fn calculate_correlation(x: &[f32], y: &[f32]) -> f32 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }
    
    let n = x.len() as f32;
    let mean_x = x.iter().sum::<f32>() / n;
    let mean_y = y.iter().sum::<f32>() / n;
    
    let mut numerator = 0.0;
    let mut sum_sq_x = 0.0;
    let mut sum_sq_y = 0.0;
    
    for i in 0..x.len() {
        let diff_x = x[i] - mean_x;
        let diff_y = y[i] - mean_y;
        numerator += diff_x * diff_y;
        sum_sq_x += diff_x * diff_x;
        sum_sq_y += diff_y * diff_y;
    }
    
    let denominator = (sum_sq_x * sum_sq_y).sqrt();
    if denominator == 0.0 {
        return 0.0;
    }
    
    numerator / denominator
}

/// Calculates variance of a dataset
fn calculate_variance(data: &[f32]) -> f32 {
    if data.is_empty() {
        return 0.0;
    }
    
    let mean = data.iter().sum::<f32>() / data.len() as f32;
    let sum_sq_diff = data.iter().map(|x| (x - mean).powi(2)).sum::<f32>();
    sum_sq_diff / data.len() as f32
}

/// Calculates confidence score based on sample size and rating
fn calculate_confidence_score(sample_size: i64, average_rating: f32) -> f32 {
    // Confidence increases with sample size, modulated by rating quality
    let size_factor = (sample_size as f32).min(10.0) / 10.0; // Cap at 10 samples
    let rating_factor = average_rating / 10.0;
    
    // Weight sample size more heavily, but rating still matters
    let base_score = size_factor * 0.7 + rating_factor * 0.3;
    
    // Apply a penalty for low ratings even with high sample size
    if average_rating < 6.0 {
        base_score * 0.7
    } else {
        base_score
    }.min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_correlation_perfect_positive() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let correlation = calculate_correlation(&x, &y);
        assert!((correlation - 1.0).abs() < 0.001, "Expected correlation ~1.0, got {}", correlation);
    }

    #[test]
    fn test_calculate_correlation_perfect_negative() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![10.0, 8.0, 6.0, 4.0, 2.0];
        let correlation = calculate_correlation(&x, &y);
        assert!((correlation + 1.0).abs() < 0.001, "Expected correlation ~-1.0, got {}", correlation);
    }

    #[test]
    fn test_calculate_correlation_no_correlation() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![5.0, 3.0, 7.0, 2.0, 8.0];
        let correlation = calculate_correlation(&x, &y);
        assert!(correlation.abs() < 0.5, "Expected low correlation, got {}", correlation);
    }

    #[test]
    fn test_calculate_correlation_empty() {
        let x: Vec<f32> = vec![];
        let y: Vec<f32> = vec![];
        let correlation = calculate_correlation(&x, &y);
        assert_eq!(correlation, 0.0);
    }

    #[test]
    fn test_calculate_correlation_mismatched_lengths() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![1.0, 2.0];
        let correlation = calculate_correlation(&x, &y);
        assert_eq!(correlation, 0.0);
    }

    #[test]
    fn test_calculate_variance_uniform() {
        let data = vec![5.0, 5.0, 5.0, 5.0];
        let variance = calculate_variance(&data);
        assert_eq!(variance, 0.0);
    }

    #[test]
    fn test_calculate_variance_simple() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let variance = calculate_variance(&data);
        // Mean is 3.0, variance should be 2.0
        assert!((variance - 2.0).abs() < 0.001, "Expected variance ~2.0, got {}", variance);
    }

    #[test]
    fn test_calculate_variance_empty() {
        let data: Vec<f32> = vec![];
        let variance = calculate_variance(&data);
        assert_eq!(variance, 0.0);
    }

    #[test]
    fn test_calculate_confidence_score_high_sample_high_rating() {
        let score = calculate_confidence_score(10, 9.0);
        assert!(score > 0.9, "Expected high confidence score, got {}", score);
    }

    #[test]
    fn test_calculate_confidence_score_low_sample_high_rating() {
        let score = calculate_confidence_score(2, 9.0);
        assert!(score > 0.3 && score < 0.5, "Expected medium-low confidence score, got {}", score);
    }

    #[test]
    fn test_calculate_confidence_score_high_sample_low_rating() {
        let score = calculate_confidence_score(10, 5.0);
        assert!(score > 0.3 && score < 0.6, "Expected medium confidence score with penalty, got {}", score);
    }

    #[test]
    fn test_calculate_confidence_score_low_sample_low_rating() {
        let score = calculate_confidence_score(1, 3.0);
        assert!(score < 0.3, "Expected low confidence score, got {}", score);
    }

    #[test]
    fn test_calculate_confidence_score_caps_at_one() {
        let score = calculate_confidence_score(100, 10.0);
        assert!(score <= 1.0, "Confidence score should not exceed 1.0, got {}", score);
    }

    #[cfg(feature = "postgres-tests")]
    mod integration_tests {
        use super::*;
        use crate::services::database::Database;
        
        fn setup_test_db() -> Database {
            let database_url = std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/coffee_tracker_test".to_string());
            Database::new(&database_url).expect("Failed to create test database")
        }
        
        fn create_test_bean(db: &Database, name: &str) -> i32 {
            let bean = NewCoffeeBean {
                name: name.to_string(),
                origin: "Test Origin".to_string(),
                roast_date: Utc::now().naive_utc().date(),
                purchase_date: Utc::now().naive_utc().date(),
                remaining_grams: Some(500.0),
                variety: None,
                processing_method: None,
            };
            db.create_coffee_bean(bean).expect("Failed to create bean").id
        }
        
        fn create_test_grinder(db: &Database, name: &str) -> i32 {
            let grinder = NewGrinder {
                brand: "Test".to_string(),
                model: name.to_string(),
                grinder_type: GrinderType::BurrConical.to_string(),
                min_setting: 1,
                max_setting: 40,
            };
            db.create_grinder(grinder).expect("Failed to create grinder").id
        }
        
        fn create_test_session(
            db: &Database,
            bean_id: i32,
            grinder_id: i32,
            grind_setting: i32,
            rating: f32,
        ) -> i32 {
            let session = NewBrewingSession {
                bean_id,
                grinder_id,
                grind_setting,
                brewing_method: BrewingMethod::V60.to_string(),
                water_temp_celsius: Some(93),
                brew_time_seconds: Some(240),
                coffee_grams: 20.0,
                water_grams: 320.0,
                tasting_notes: None,
                rating: Some(rating),
            };
            db.create_brewing_session(session).expect("Failed to create session").id
        }
        
        #[test]
        fn test_suggest_grind_setting_no_data() {
            let db = setup_test_db();
            let engine = OptimizationEngine::from_database(db.clone());
            
            let bean_id = create_test_bean(&db, "Test Bean");
            let suggestions = engine
                .suggest_grind_setting(bean_id, BrewingMethod::V60)
                .expect("Failed to get suggestions");
            
            assert!(suggestions.is_empty(), "Should return empty suggestions with no data");
        }
        
        #[test]
        fn test_suggest_grind_setting_with_data() {
            let db = setup_test_db();
            let engine = OptimizationEngine::from_database(db.clone());
            
            let bean_id = create_test_bean(&db, "Test Bean");
            let grinder_id = create_test_grinder(&db, "Test Grinder");
            
            // Create sessions with different grind settings and ratings
            create_test_session(&db, bean_id, grinder_id, 15, 7.0);
            create_test_session(&db, bean_id, grinder_id, 15, 7.5);
            create_test_session(&db, bean_id, grinder_id, 20, 8.5);
            create_test_session(&db, bean_id, grinder_id, 20, 9.0);
            create_test_session(&db, bean_id, grinder_id, 25, 6.0);
            
            let suggestions = engine
                .suggest_grind_setting(bean_id, BrewingMethod::V60)
                .expect("Failed to get suggestions");
            
            assert!(!suggestions.is_empty(), "Should return suggestions with data");
            
            // Best suggestion should be grind setting 20 (avg rating 8.75)
            let best = &suggestions[0];
            assert_eq!(best.grind_setting, 20);
            assert!((best.average_rating - 8.75).abs() < 0.01);
            assert_eq!(best.session_count, 2);
        }
        
        #[test]
        fn test_suggest_grind_setting_filters_by_method() {
            let db = setup_test_db();
            let engine = OptimizationEngine::from_database(db.clone());
            
            let bean_id = create_test_bean(&db, "Test Bean");
            let grinder_id = create_test_grinder(&db, "Test Grinder");
            
            // Create V60 sessions
            create_test_session(&db, bean_id, grinder_id, 15, 8.0);
            
            // Create Chemex session (different method)
            let chemex_session = NewBrewingSession {
                bean_id,
                grinder_id,
                grind_setting: 25,
                brewing_method: BrewingMethod::Chemex.to_string(),
                water_temp_celsius: Some(93),
                brew_time_seconds: Some(240),
                coffee_grams: 20.0,
                water_grams: 320.0,
                tasting_notes: None,
                rating: Some(9.0),
            };
            db.create_brewing_session(chemex_session).expect("Failed to create session");
            
            let suggestions = engine
                .suggest_grind_setting(bean_id, BrewingMethod::V60)
                .expect("Failed to get suggestions");
            
            // Should only include V60 sessions
            assert_eq!(suggestions.len(), 1);
            assert_eq!(suggestions[0].grind_setting, 15);
        }
        
        #[test]
        fn test_analyze_grind_quality_correlation_empty() {
            let db = setup_test_db();
            let engine = OptimizationEngine::from_database(db);
            
            let correlation_data = engine
                .analyze_grind_quality_correlation()
                .expect("Failed to analyze correlation");
            
            assert_eq!(correlation_data.grind_to_rating_correlation, 0.0);
            assert!(correlation_data.method_performance.is_empty());
            assert!(correlation_data.bean_grind_preferences.is_empty());
        }
        
        #[test]
        fn test_analyze_grind_quality_correlation_with_data() {
            let db = setup_test_db();
            let engine = OptimizationEngine::from_database(db.clone());
            
            let bean_id = create_test_bean(&db, "Test Bean");
            let grinder_id = create_test_grinder(&db, "Test Grinder");
            
            // Create sessions with positive correlation (higher grind = higher rating)
            create_test_session(&db, bean_id, grinder_id, 10, 6.0);
            create_test_session(&db, bean_id, grinder_id, 15, 7.0);
            create_test_session(&db, bean_id, grinder_id, 20, 8.0);
            create_test_session(&db, bean_id, grinder_id, 25, 9.0);
            
            let correlation_data = engine
                .analyze_grind_quality_correlation()
                .expect("Failed to analyze correlation");
            
            // Should show positive correlation
            assert!(
                correlation_data.grind_to_rating_correlation > 0.8,
                "Expected strong positive correlation, got {}",
                correlation_data.grind_to_rating_correlation
            );
            
            // Should have method performance data
            assert!(!correlation_data.method_performance.is_empty());
            assert_eq!(correlation_data.method_performance[0].brewing_method, "V60");
        }
        
        #[test]
        fn test_analyze_brewing_patterns_empty() {
            let db = setup_test_db();
            let engine = OptimizationEngine::from_database(db);
            
            let pattern_analysis = engine
                .analyze_brewing_patterns()
                .expect("Failed to analyze patterns");
            
            assert!(pattern_analysis.most_successful_combinations.is_empty());
            assert!(pattern_analysis.improvement_suggestions.is_empty());
            assert_eq!(pattern_analysis.consistency_metrics.grind_setting_variance, 0.0);
        }
        
        #[test]
        fn test_analyze_brewing_patterns_finds_successful_combinations() {
            let db = setup_test_db();
            let engine = OptimizationEngine::from_database(db.clone());
            
            let bean_id = create_test_bean(&db, "Test Bean");
            let grinder_id = create_test_grinder(&db, "Test Grinder");
            
            // Create multiple high-rated sessions with same combination
            create_test_session(&db, bean_id, grinder_id, 20, 8.5);
            create_test_session(&db, bean_id, grinder_id, 20, 9.0);
            create_test_session(&db, bean_id, grinder_id, 20, 8.0);
            
            let pattern_analysis = engine
                .analyze_brewing_patterns()
                .expect("Failed to analyze patterns");
            
            assert!(!pattern_analysis.most_successful_combinations.is_empty());
            let combo = &pattern_analysis.most_successful_combinations[0];
            assert_eq!(combo.bean_id, bean_id);
            assert_eq!(combo.grinder_id, grinder_id);
            assert!(combo.average_rating >= 8.0);
        }
        
        #[test]
        fn test_analyze_brewing_patterns_calculates_consistency() {
            let db = setup_test_db();
            let engine = OptimizationEngine::from_database(db.clone());
            
            let bean_id = create_test_bean(&db, "Test Bean");
            let grinder_id = create_test_grinder(&db, "Test Grinder");
            
            // Create sessions with varying grind settings
            create_test_session(&db, bean_id, grinder_id, 10, 7.0);
            create_test_session(&db, bean_id, grinder_id, 20, 8.0);
            create_test_session(&db, bean_id, grinder_id, 30, 7.5);
            
            let pattern_analysis = engine
                .analyze_brewing_patterns()
                .expect("Failed to analyze patterns");
            
            // Should have non-zero variance due to varying grind settings
            assert!(
                pattern_analysis.consistency_metrics.grind_setting_variance > 0.0,
                "Expected non-zero grind setting variance"
            );
        }
        
        #[test]
        fn test_analyze_brewing_patterns_generates_improvement_suggestions() {
            let db = setup_test_db();
            let engine = OptimizationEngine::from_database(db.clone());
            
            let bean_id = create_test_bean(&db, "Test Bean");
            let grinder_id = create_test_grinder(&db, "Test Grinder");
            
            // Create multiple low-rated sessions
            create_test_session(&db, bean_id, grinder_id, 15, 4.0);
            create_test_session(&db, bean_id, grinder_id, 16, 4.5);
            create_test_session(&db, bean_id, grinder_id, 17, 5.0);
            
            let pattern_analysis = engine
                .analyze_brewing_patterns()
                .expect("Failed to analyze patterns");
            
            // Should generate improvement suggestions for low-rated sessions
            assert!(
                !pattern_analysis.improvement_suggestions.is_empty(),
                "Expected improvement suggestions for low-rated sessions"
            );
        }
        
        #[test]
        fn test_analyze_equipment_performance_empty() {
            let db = setup_test_db();
            let engine = OptimizationEngine::from_database(db);
            
            let equipment_analysis = engine
                .analyze_equipment_performance()
                .expect("Failed to analyze equipment");
            
            assert!(equipment_analysis.grinder_performance.is_empty());
            assert!(equipment_analysis.bean_performance.is_empty());
        }
        
        #[test]
        fn test_analyze_equipment_performance_with_data() {
            let db = setup_test_db();
            let engine = OptimizationEngine::from_database(db.clone());
            
            let bean_id = create_test_bean(&db, "Test Bean");
            let grinder_id = create_test_grinder(&db, "Test Grinder");
            
            // Create sessions with high ratings
            create_test_session(&db, bean_id, grinder_id, 20, 8.0);
            create_test_session(&db, bean_id, grinder_id, 20, 9.0);
            create_test_session(&db, bean_id, grinder_id, 20, 7.5);
            
            let equipment_analysis = engine
                .analyze_equipment_performance()
                .expect("Failed to analyze equipment");
            
            // Should have grinder performance data
            assert!(!equipment_analysis.grinder_performance.is_empty());
            let grinder_perf = &equipment_analysis.grinder_performance[0];
            assert_eq!(grinder_perf.grinder_id, grinder_id);
            assert!(grinder_perf.average_rating > 7.0);
            assert_eq!(grinder_perf.session_count, 3);
            
            // Should have bean performance data
            assert!(!equipment_analysis.bean_performance.is_empty());
            let bean_perf = &equipment_analysis.bean_performance[0];
            assert_eq!(bean_perf.bean_id, bean_id);
            assert!(bean_perf.average_rating > 7.0);
        }
        
        #[test]
        fn test_analyze_equipment_performance_calculates_success_rate() {
            let db = setup_test_db();
            let engine = OptimizationEngine::from_database(db.clone());
            
            let bean_id = create_test_bean(&db, "Test Bean");
            let grinder_id = create_test_grinder(&db, "Test Grinder");
            
            // Create 3 high-rated and 2 low-rated sessions (60% success rate)
            create_test_session(&db, bean_id, grinder_id, 20, 8.0);
            create_test_session(&db, bean_id, grinder_id, 20, 7.5);
            create_test_session(&db, bean_id, grinder_id, 20, 9.0);
            create_test_session(&db, bean_id, grinder_id, 20, 6.0);
            create_test_session(&db, bean_id, grinder_id, 20, 5.5);
            
            let equipment_analysis = engine
                .analyze_equipment_performance()
                .expect("Failed to analyze equipment");
            
            let grinder_perf = &equipment_analysis.grinder_performance[0];
            assert!(
                (grinder_perf.success_rate - 0.6).abs() < 0.01,
                "Expected success rate ~0.6, got {}",
                grinder_perf.success_rate
            );
        }
    }
}
