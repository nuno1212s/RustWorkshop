
use super::*;

// Exercise 1 Tests: Basic Enum Matching
#[test]
fn test_exercise_1_basic_enum() {
    // TODO: Test your Shape enum and shape_name() function
    // Create different shapes and verify the name is returned correctly
}

// Exercise 2 Tests: Enum with Associated Data
#[test]
fn test_exercise_2_shape_area() {
    // TODO: Test your Shape2D enum
    // Test calculate_area() for Circle, Square, and Rectangle
}

#[test]
fn test_exercise_2_shape_perimeter() {
    // TODO: Test calculate_perimeter() for all shapes
}

// Exercise 3 Tests: String and Number Matching
#[test]
fn test_exercise_3_classify_grade() {
    // TODO: Test classify_grade() with scores in each range:
    // 95 -> "A", 85 -> "B", 75 -> "C", 65 -> "D", 45 -> "F"
}

#[test]
fn test_exercise_3_parse_command() {
    // TODO: Test parse_command() with:
    // "start", "stop", "status", and unknown commands
}

// Exercise 4 Tests: Match with Guard Clauses
#[test]
fn test_exercise_4_categorize_transaction() {
    // TODO: Test categorize_transaction() with:
    // amounts < 10.0, between 10-100, and > 100
}

#[test]
fn test_exercise_4_valid_purchase() {
    // TODO: Test is_valid_purchase() with various prices and budgets
}

#[test]
fn test_exercise_4_process_order() {
    // TODO: Test process_order() with different quantities and discounts
}

// Exercise 5 Tests: Option/Result Matching
#[test]
fn test_exercise_5_safe_divide() {
    // TODO: Test safe_divide() with valid division and division by zero
}

#[test]
fn test_exercise_5_unwrap_safely() {
    // TODO: Test unwrap_safely() with Some(value) and None
}

#[test]
fn test_exercise_5_process_result() {
    // TODO: Test process_result() with Ok and Err variants
}

// Exercise 6 Tests: Advanced Pattern Matching
#[test]
fn test_exercise_6_user_access_level() {
    // TODO: Test user_access_level() for all User variants
    // Admin with high/low permissions, Moderator with many/few channels, Regular users
}

#[test]
fn test_exercise_6_can_perform_action() {
    // TODO: Test can_perform_action() for each user type with various actions
    // Admin (any action), Moderator (edit/delete/ban), Regular (view only)
}
