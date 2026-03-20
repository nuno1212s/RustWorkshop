#[cfg(test)]
mod tests;

// Exercícios do modelo de ownership
// Nestes exercícios vamos explorar os conceitos principais de ownership:
// - Move semantics (transferência de propriedade)
// - Borrowing (empréstimo de referências)
// - Mutable vs Immutable references
// - Lifetime annotations
// - Clone vs Copy

/// SECÇÃO 1: MOVE SEMANTICS
///
/// Nestes exercícios aprendemos como o Rust move valores entre variáveis
/// e como isso afeta a disponibilidade das variáveis originais

// Exercício 1: Mover uma String de uma variável para outra
// Implemente esta função para que `original` seja movida para `copia`
// de forma que `copia` contenha o valor original e `original` não possa ser utilizada depois
fn move_string(s: String) -> String {
    todo!()
}

// Exercício 2: Mover múltiplos valores
// Implemente esta função para que ela receba dois Strings e os retorne na ordem inversa
// O primeiro argumento deve ser retornado em segundo, e vice-versa
fn swap_strings(first: String, second: String) -> (String, String) {
    todo!()
}

// Exercício 2.1:
// Utilizando a função anterior, faça com que s1 e s2 sejam acessíveis após a execução
// Do swapped strings.
// Aproveite o facto de `String` ser `Clone`
fn use_swapped_strings() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    let (swapped_s1, swapped_s2) = swap_strings(s1, s2);

    // Note como, após a execução da função, s1 e s2 já não estão disponíveis
    assert_eq!("world", swapped_s1);
    assert_eq!("hello", swapped_s2);
    assert_eq!(s1, swapped_s2);
    assert_eq!(s2, swapped_s1);
}

// Exercício 3: Move vs Copy
// Integers implementam Copy, portanto são copiados implicitamente
// Implemente a função para que retorne a soma de ambos os argumentos
// Os argumentos devem estar disponíveis após a chamada (graças a Copy)
fn swap_integers(a: i32, b: i32) -> (i32, i32) {
    todo!()
}

/// SEÇÃO 2: BORROWING (EMPRÉSTIMO IMUTÁVEL)
///
/// Nestes exercícios aprendemos a utilizar referências imutáveis (&T)
/// As referências não transferem propriedade, apenas "emprestam" o valor

// Exercício 4: Referência Imutável
// Implemente esta função para que aceite uma referência imutável a um String
// e retorne o comprimento desse String sem modificá-lo
fn string_length(s: &String) -> usize {
    todo!()
}

// Exercício 5: Múltiplas Referências Imutáveis
// Implemente esta função para que concatene dois Strings referenciados
// O resultado deve ser um novo String contendo ambos
fn concat_strings(s1: &String, s2: &String) -> String {
    todo!()
}

// Exercício 6: Referência a slice
// Implemente esta função para contar quantos números da slice são pares
fn count_even(numbers: &[i32]) -> usize {
    todo!()
}

// Exercício 7: Buscar em um vetor
// Implemente esta função para retornar a primeira posição onde o valor foi encontrado
// Se não encontrar, retorne None
fn find_in_vector(vec: &[i32], target: i32) -> Option<usize> {
    todo!()
}

/// SEÇÃO 3: BORROWING MUTÁVEL (EMPRÉSTIMO MUTÁVEL)
///
/// Nestes exercícios aprendemos a utilizar referências mutáveis (&mut T)
/// Permitem modificar o valor emprestado, mas apenas uma referência mútavel pode existir por vez

// Exercício 8: Referência Mutável Simples
// Implemente esta função para que aceite uma referência mutável a um vetor
// e adicione o valor `element` ao final do vetor
fn add_to_vector(vec: &mut Vec<i32>, element: i32) {
    todo!()
}

// Exercício 9: Modificar elementos do vetor
// Implemente esta função para que multiplique todos os elementos do vetor por `factor`
fn multiply_vector(vec: &mut [i32], factor: i32) {
    todo!()
}

// Exercício 10: Modificar String através de referência mutável
// Implemente esta função para que converta o String para maiúsculas no lugar
fn uppercase_string(s: &mut String) {
    todo!()
}

// Exercício 11: Swap de elementos
// Implemente esta função para que troque os elementos nas posições `i` e `j`
fn swap_elements(vec: &mut [i32], i: usize, j: usize) {
    todo!()
}

// Exercício 12: Inverter um vetor
// Implemente esta função para que inverta os elementos do vetor no lugar
fn reverse_vector(vec: &mut [i32]) {
    todo!()
}

/// SEÇÃO 4: OWNERSHIP E FUNÇÕES
///
/// Nestes exercícios aprendemos como o ownership funciona ao passar valores para funções
/// e como controlar se queremos transferir propriedade ou apenas emprestar

// Exercício 13: Retornar propriedade
// Implemente esta função para que crie um novo vetor, o preencha com números
// de 1 até `n` (inclusive) e o retorne transferindo a propriedade
fn create_range_vector(n: u32) -> Vec<i32> {
    todo!()
}

// Exercício 14: Processar e retornar
// Implemente esta função para que aceite um vetor, remova o primeiro elemento
// e retorne o vetor modificado
fn remove_first(mut vec: Vec<i32>) -> Vec<i32> {
    todo!()
}

// Exercício 14.1: Processar por referência
fn remove_first_ref_mut(vec: &mut Vec<i32>) {
    todo!()
}

// Exercício 15: Dupla referência
// Implemente esta função para que aceite referências a dois vetores
// e retorne true se o primeiro vetor contém todos os elementos do segundo
fn contains_all(haystack: &[i32], needle: &[i32]) -> bool {
    todo!()
}

/// SEÇÃO 5: CONFLITOS DE BORROW (BORROW CHECKER)
///
/// Nestes exercícios o Rust impedirá código inválido durante compilação
/// Você precisa refatorar o código de forma que seja válido

// Exercício 16: Uma só referência mutável
// O código abaixo viola a regra: só pode haver uma referência mutável ativa de cada vez
// Refaça o código para que seja válido sem usar move
fn borrow_conflict() -> i32 {
    let mut num = 10;

    // Descomente a linha abaixo e implemente de forma que o código compile
    let ref1 = &mut num;
    let ref2 = &mut num;

    *ref1 = 5;
    *ref2 = 5;

    todo!()
}

// Exercício 17: Referência mutável vs imutável
// Não podemos ter referências imutáveis enquanto há uma referência mutável ativa
// Refaça o código para que seja válido
fn mixed_borrows() -> i32 {
    let mut num = 20;

    // Descomente as linhas abaixo e refatore o código para que compile
    // let mutable_ref = &mut num;
    // let immutable_ref = &num;
    // mutable_ref + immutable_ref

    todo!()
}