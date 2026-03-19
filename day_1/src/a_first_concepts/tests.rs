
#[test]
fn test_val() {
    assert_eq!(super::declare_var(), 20);
}

#[test]
fn test_var_2() {
    super::declare_var_2();
}

#[test]
fn test_var_3() {
    assert_eq!(super::declare_var_3(), 10);
}

#[test]
fn test_var_4() {
    assert_eq!(super::declare_var_4(), "hello");
}

#[test]
fn test_var_5() {
    super::declare_var_5();
}

#[test]
fn test_string_parsing() {

}

#[test]
fn test_var_6() {
    assert_eq!(42, super::declare_var_6());
}

#[test]
fn test_print() {
    super::print(5i64);
}

#[test]
fn test_add() {
    assert_eq!(9i64, super::add(5i32, 6i32));

    assert_eq!(i32::MAX as i64 * 2, super::add(i32::MAX, i32::MAX));
}

#[test]
fn handle_scope() {
    super::handle_scope();
}

#[test]
fn test_handle_scope_2() {
    let string = super::handle_scope_2();

    assert_eq!(String::from("test"), string);
}

#[test]
fn test_handle_scope_3() {
    super::handle_scope_3();
}

#[test]
fn test_handle_scope_drop_order() {
    super::handle_scope_drop_order();
}

#[test]
fn from_tuple_to_array() {
    assert_eq!(super::from_tuple_to_array((10, 20)), [10, 20]);
}

#[test]
fn test_conditional() {
    assert_eq!(super::conditional(0), "Zero");
    assert_eq!(super::conditional(1), "Positivo");
    assert_eq!(super::conditional(i32::MAX), "Positivo");
    assert_eq!(super::conditional(-1), "Negativo");
    assert_eq!(super::conditional(i32::MIN), "Negativo");
}

#[test]
fn test_conditional_slice() {
    assert_eq!(super::conditional_slice(&[5, 10]), true);
    assert_eq!(super::conditional_slice(&[-1, 10]), false);
    assert_eq!(super::conditional_slice(&[10, -1]), false);
}

#[test]
fn test_sum_slice() {
    let inputs = [
        vec![0]
    ];

    for input in &inputs {
        assert_eq!(super::sum_slice(input), input.iter().sum());
    }
}

#[test]
fn test_prime_checker() {
    assert!(super::prime_checker(2));
    assert!(super::prime_checker(7));
    assert!(super::prime_checker(7907));

    assert!(!super::prime_checker(4));
    assert!(!super::prime_checker(7908));
}

#[test]
fn test_loop_return() {
    assert_eq!(super::loop_return() % 2, 0);
}

#[test]
fn inner_loop_break() {
    let inner_loop_break = super::inner_loop_break();
    
    if inner_loop_break.abs() < 10 {
        assert!(inner_loop_break > 0);
    } else if inner_loop_break.abs() > 1000 {
        assert!(inner_loop_break < 0);
    } else {
        panic!("Should not reach here!");
    }
}

#[test]
fn test_conditional_atrib() {
    assert!(super::conditional_atrib(true) < 0);
    assert!(super::conditional_atrib(false) >= 0);
}

#[test]
fn test_mutacao_array() {
    assert_eq!([0, 0, 0, 0], super::mutacao_array([1, 2, 3, 4], 0));
    assert_eq!([4, 8, 12, 16], super::mutacao_array([1, 2, 3, 4], 4));
}