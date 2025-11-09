// Standalone test file for optimization engine statistical functions
// This can be run with: rustc --test test_optimization_stats.rs && ./test_optimization_stats

#[cfg(test)]
mod tests {
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
        let size_factor = (sample_size as f32).min(10.0) / 10.0;
        let rating_factor = average_rating / 10.0;
        
        let base_score = size_factor * 0.7 + rating_factor * 0.3;
        
        if average_rating < 6.0 {
            base_score * 0.7
        } else {
            base_score
        }.min(1.0)
    }

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
}

fn main() {
    println!("Run with: cargo test --test test_optimization_stats");
}
