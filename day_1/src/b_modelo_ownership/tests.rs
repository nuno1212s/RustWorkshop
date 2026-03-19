// TESTES PARA SEÇÃO 1: MOVE SEMANTICS

#[test]
fn test_move_string() {
    let original = String::from("Hello, Rust!");
    let result = super::move_string(original);
    assert_eq!(result, "Hello, Rust!");
}

#[test]
fn test_swap_strings() {
    let first = String::from("First");
    let second = String::from("Second");
    let (result_first, result_second) = super::swap_strings(first, second);
    assert_eq!(result_first, "Second");
    assert_eq!(result_second, "First");
}

#[test]
fn test_sum_integers() {
    assert_eq!(super::sum_integers(5, 3), 8);
    assert_eq!(super::sum_integers(10, 20), 30);
    assert_eq!(super::sum_integers(-5, 5), 0);
}

// TESTES PARA SEÇÃO 2: BORROWING (EMPRÉSTIMO IMUTÁVEL)

#[test]
fn test_string_length() {
    let s = String::from("Hello");
    assert_eq!(super::string_length(&s), 5);
    assert_eq!(super::string_length(&s), 5); // Pode usar múltiplas vezes
}

#[test]
fn test_concat_strings() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("Rust!");
    let result = super::concat_strings(&s1, &s2);
    assert_eq!(result, "Hello, Rust!");
    // Ambos ainda estão disponíveis (não foram movidos)
    assert_eq!(s1, "Hello, ");
    assert_eq!(s2, "Rust!");
}

#[test]
fn test_count_even() {
    assert_eq!(super::count_even(&[1, 2, 3, 4, 5, 6]), 3);
    assert_eq!(super::count_even(&[1, 3, 5, 7]), 0);
    assert_eq!(super::count_even(&[2, 4, 6, 8]), 4);
}

#[test]
fn test_find_in_vector() {
    let vec = vec![10, 20, 30, 40, 50];
    assert_eq!(super::find_in_vector(&vec, 30), Some(2));
    assert_eq!(super::find_in_vector(&vec, 10), Some(0));
    assert_eq!(super::find_in_vector(&vec, 99), None);
}

// TESTES PARA SEÇÃO 3: BORROWING MUTÁVEL

#[test]
fn test_add_to_vector() {
    let mut vec = vec![1, 2, 3];
    super::add_to_vector(&mut vec, 4);
    assert_eq!(vec, vec![1, 2, 3, 4]);
}

#[test]
fn test_multiply_vector() {
    let mut vec = vec![1, 2, 3, 4];
    super::multiply_vector(&mut vec, 2);
    assert_eq!(vec, vec![2, 4, 6, 8]);
}

#[test]
fn test_uppercase_string() {
    let mut s = String::from("hello world");
    super::uppercase_string(&mut s);
    assert_eq!(s, "HELLO WORLD");
}

#[test]
fn test_swap_elements() {
    let mut vec = vec![1, 2, 3, 4, 5];
    super::swap_elements(&mut vec, 0, 4);
    assert_eq!(vec, vec![5, 2, 3, 4, 1]);
}

#[test]
fn test_reverse_vector() {
    let mut vec = vec![1, 2, 3, 4, 5];
    super::reverse_vector(&mut vec);
    assert_eq!(vec, vec![5, 4, 3, 2, 1]);
}

// TESTES PARA SEÇÃO 4: OWNERSHIP E FUNÇÕES

#[test]
fn test_create_range_vector() {
    let result = super::create_range_vector(5);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);

    let result2 = super::create_range_vector(3);
    assert_eq!(result2, vec![1, 2, 3]);
}

#[test]
fn test_remove_first() {
    let vec = vec![1, 2, 3, 4];
    let result = super::remove_first(vec);
    assert_eq!(result, vec![2, 3, 4]);
}

#[test]
fn test_remove_first_ref_mut() {
    let mut vec = vec![1, 2, 3, 4];
    super::remove_first_ref_mut(&mut vec);

    assert_eq!(vec, vec![2, 3, 4]);
}

#[test]
fn test_contains_all() {
    let haystack = vec![1, 2, 3, 4, 5];
    let needle1 = vec![2, 4];
    let needle2 = vec![1, 2];
    let needle3 = vec![6, 7];

    assert!(super::contains_all(&haystack, &needle1));
    assert!(super::contains_all(&haystack, &needle2));
    assert!(!super::contains_all(&haystack, &needle3));
}

// TESTES PARA SEÇÃO 5: CONFLITOS DE BORROW

#[test]
fn test_borrow_conflict() {
    assert_eq!(super::borrow_conflict(), 20);
}

#[test]
fn test_mixed_borrows() {
    assert_eq!(super::mixed_borrows(), 40);
}

// TESTES PARA SEÇÃO 6: CASOS DE USO PRÁTICOS

#[test]
fn test_pessoa_new() {
    let pessoa = super::Pessoa::new(String::from("João"), 30);
    assert_eq!(pessoa.nome, "João");
    assert_eq!(pessoa.idade, 30);
}

#[test]
fn test_pessoa_consumir() {
    let pessoa = super::Pessoa::new(String::from("Maria"), 25);
    let result = pessoa.consumir();
    assert_eq!(result, "Nome: Maria, Idade: 25");
}

#[test]
fn test_pessoa_fazer_aniversario() {
    let mut pessoa = super::Pessoa::new(String::from("Pedro"), 35);
    pessoa.fazer_aniversario();
    assert_eq!(pessoa.idade, 36);

    pessoa.fazer_aniversario();
    assert_eq!(pessoa.idade, 37);
}

