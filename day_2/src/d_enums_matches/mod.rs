mod test;

/// Enums e Data Oriented Design

/// Imagine a seguinte hierarquia numa linguagem OOP, representando formas
/// Geométricas
/// Quadrado, circulo, elipse, Triângulo, Cubo, Cilindro, Esfera

/// Todas essas formas devem implementar métodos para calcular a área
/// e o perímetro.
/// Devem também partilhar alguns atributos:
/// Local base (x,y), cor e ângulo de colocação

/// Para conseguir implementar isto, deve recorrer a Composição e sum types

// ============================================================================
// EXERCISE 3: String and Number Matching
// ============================================================================
/// Practice matching on different types (strings, numbers, etc).
///
/// Requirements:
/// - Implement `fn classify_grade(score: u32) -> &'static str` that:
///   * 90-100 -> "A"
///   * 80-89 -> "B"
///   * 70-79 -> "C"
///   * 60-69 -> "D"
///   * 0-59 -> "F"
/// - Implement `fn parse_command(input: &str) -> String` that:
///   * "start" -> "Starting application..."
///   * "stop" -> "Stopping application..."
///   * "status" -> "Status: running"
///   * anything else -> "Unknown command"
/// - Use match with range patterns for numbers
/// - Use match with string slices for commands
///
/// Example usage:
/// ```
/// assert_eq!(classify_grade(95), "A");
/// assert_eq!(classify_grade(75), "C");
/// assert_eq!(parse_command("start"), "Starting application...");
/// ```

// ============================================================================
// EXERCISE 4: Match with Guard Clauses (if conditions)
// ============================================================================
/// Use match guards to add additional conditions to patterns.
///
/// Requirements:
/// - Create an enum `Transaction` with variant: Payment { amount: f64 }
/// - Implement `fn categorize_transaction(transaction: &Transaction) -> &'static str` that:
///   * amount < 10.0 -> "Small transaction"
///   * 10.0 <= amount <= 100.0 -> "Regular transaction"
///   * amount > 100.0 -> "Large transaction"
/// - Implement `fn is_valid_purchase(item_price: f64, budget: f64) -> bool`
///   * Returns true if purchase is possible and doesn't exceed budget
///   * Use match with guards to check conditions
/// - Implement `fn process_order(quantity: u32, discount_percent: f64) -> f64` that:
///   * Base price per item: 10.0
///   * quantity < 5 && discount_percent == 0.0 -> regular price
///   * quantity >= 5 && quantity < 20 && discount_percent >= 5.0 -> apply 10% discount
///   * quantity >= 20 && discount_percent >= 15.0 -> apply 20% discount
///   * all others -> apply no discount
///
/// Example usage:
/// ```
/// let trans = Transaction::Payment { amount: 50.0 };
/// assert_eq!(categorize_transaction(&trans), "Regular transaction");
/// ```

// ============================================================================
// EXERCISE 5: Pattern Matching with Complex Enums and Option/Result
// ============================================================================
/// Work with Option and Result types using pattern matching.
///
/// Requirements:
/// - Implement `fn safe_divide(a: f64, b: f64) -> Result<f64, &'static str>`
///   * Returns Ok(result) if b != 0.0
///   * Returns Err("Division by zero") otherwise
/// - Implement `fn unwrap_safely(value: Option<i32>) -> i32`
///   * Returns the value if Some
///   * Returns 0 if None
/// - Implement `fn process_result(result: Result<String, String>) -> String` that:
///   * If Ok: return "Success: " + value
///   * If Err: return "Error: " + error_message
/// - Use match to handle all cases explicitly
///
/// Example usage:
/// ```
/// assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
/// assert_eq!(safe_divide(10.0, 0.0), Err("Division by zero"));
/// ```

// ============================================================================
// EXERCISE 6: Advanced Pattern Matching - Multiple Conditions
// ============================================================================
/// Combine multiple patterns and conditions for complex scenarios.
///
/// Requirements:
/// - Create an enum `User` with variants:
///   * Admin { name: String, permissions: u32 }
///   * Moderator { name: String, channels: Vec<String> }
///   * Regular { name: String }
/// - Implement `fn user_access_level(user: &User) -> i32` that:
///   * Admin with permissions >= 100 -> return 5
///   * Admin with permissions < 100 -> return 4
///   * Moderator with more than 3 channels -> return 3
///   * Moderator with 3 or fewer channels -> return 2
///   * Regular users -> return 1
/// - Implement `fn can_perform_action(user: &User, action: &str) -> bool` that:
///   * Admins can always perform any action
///   * Moderators can perform actions: "edit", "delete", "ban"
///   * Regular users can only perform "view"
///   * Use match guards to check the action string
///
/// Example usage:
/// ```
/// let admin = User::Admin { name: "Alice".to_string(), permissions: 150 };
/// assert_eq!(user_access_level(&admin), 5);
/// assert!(can_perform_action(&admin, "anything"));
/// ```

