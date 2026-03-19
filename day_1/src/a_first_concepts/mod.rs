use std::cell::RefCell;
use std::rc::Rc;
use crate::a_first_concepts::util::Droppable;

#[cfg(test)]
mod tests;
mod util;

// Exercício 1: Declaração de variáveis
// Modifique este código de maneira a que `var` seja uma variável válida
// com um tipo igual ao tipo retornado pela função
fn declare_var() -> i32 {
    var = 10;

    var + 10
}

// Exercício 2: Tipos de variáveis
// Modifique este código de maneira a que x seja um vec de inteiros signed de 32 bits
fn declare_var_2() {
    let x = vec![];
}

// Exercício 3: Shadowing simples
// Faça com que este código seja válido e retorne x = 10 no fim
// utilizando shadowing de variáveis
fn declare_var_3() -> i32 {
    let x = 5;

    x = 10;

    x
}

// Exercício 4: Tipos
// Corrija os tipos deste código
fn declare_var_4() -> todo!() {
    let x: todo!() = "hello";

    println!("{x}");

    x
}

// Exercício 5: Declaração de variáveis
// Declare a variável x de maneira que seja válida
fn declare_var_5() -> todo!() {
    let x: u32;

    println!("{}", x);

    x
}

// Exercício 6: String parsing e inferência de tipos
// Nesta função, vamos fazer parse de um valor var para um u32.
// Existem duas formas de fazer isto: anotando na variável
// ou anotando na chamada de parse (sintaxe .call::<T>())
// Explore as capacidades do compilador de inferir os tipos associados.
fn string_parsing(string: &str) -> todo!() {
    let var = string.parse().unwrap();

    println!("{var}");

    todo!()
}

// Exercício 7: Scoping - Asserts corretos
// Faça com que os asserts estejam corretos alterando o escopo das variáveis
fn handle_scope() {
    let x = "hello";

    let x = "world";

    assert_eq!(x, "world");
    assert_eq!(x, "hello");
}

// Exercício 8: Scoping - Valor disponível após escopo
// Faça com que o valor string_example fique disponível para fora da scope interna
fn handle_scope_2() -> String {
    let todo!() = {
        let string_example = String::from("test");
    };

    todo!()
}

// Exercício 9: Scoping - Shadowing com tipos diferentes
// Shadowing também funciona com tipos diferentes, podendo alterar o próprio tipo original
// da variável. Faça com que os asserts estejam corretos
fn handle_scope_3() {
    let x = "hello";

    let x = 5;

    assert_eq!(x, 5);

    assert_eq!(x, "hello");
}

// Exercício 10: Scoping - Ordem de drop
// O objetivo é que a variável `segunda` seja Dropped primeiro que a variável `primeira`.
// Use scopes para manipular este comportamento. Não inverta a ordem de criação
fn handle_scope_drop_order() {
    let log = Rc::new(RefCell::new(vec![]));

    let primeira = Droppable::new("Primeira", log.clone());
    let segunda = Droppable::new("Segunda", log.clone());

    assert_eq!(log.borrow().as_slice(), vec!["Segunda", "Primeira"])
}

// Exercício 11: Declaração de função - print
// Crie uma função `print` que aceite um inteiro signed de 64 bits e imprima para a consola

// Exercício 12: Declaração de função - add
// Crie uma função `add` que aceite dois inteiros signed de 32 bits e retorne a soma deles
// como um inteiro signed de 64 bits

// Exercício 13: Tipos compostos - Tuplo para Array
// Transforme o tuplo recebido em um array
fn from_tuple_to_array(tuple: (u32, u32)) -> [u32; 2] {
    todo!()
}

// Exercício 14: Tipos compostos - Array para Tuplo
// Transforme o array recebido em um tuplo
fn from_array_to_tuple(array: [u32; 2]) -> (u32, u32) {
    todo!()
}

// Exercício 15: Tipos compostos - Desconstrução de tuplo
// Desmonte o tuplo nos argumentos de maneira que fiquem imediatamente
// disponíveis como `first_arg` e `second_arg`
fn desmontar_tuplo(todo!(): (i32, i32)) -> i32 {
    first_arg + second_arg
}

// Exercício 16: Controlo de fluxo - Condicional simples
// Retorne uma string em função do valor de input:
// - Caso input == 0: "Zero"
// - Caso input > 0: "Positivo"
// - Caso input < 0: "Negativo"
fn conditional(input: i32) -> &'static str {
    todo!()
}

// Exercício 17: Controlo de fluxo - Condicional com slice
// Retorna true se todos os números da slice forem positivos.
// Retorna false se houver algum número negativo
fn conditional_slice(input: &[i32]) -> bool {
    todo!()
}

// Exercício 18: Controlo de fluxo - Soma de slice
// Faça a soma de todos os números da slice e retorne
fn sum_slice(input: &[i32]) -> i32 {
    todo!()
}

// Exercício 19: Controlo de fluxo - Verificação de número primo
// Verifique se um número é primo utilizando controlos de fluxo
fn prime_checker(number: i32) -> bool {
    todo!()
}

// Exercício 20: Loops - Return em loop
// Retorne o valor de rand_num assim que for par.
// Se não for par, continue a execução
fn loop_return() -> i32 {
    loop {
        let rand_num : i32 = rand::random();

        todo!()
    }
}

// Exercício 21: Loops - Break com valor
// Quando o valor rand_num:
// - < 10 retorne o valor imediatamente
// - > 1000, faça com que o valor seja armazenado em `value`, para que possa ser retornado posteriormente
// - Se não satisfazer nenhuma das condições, prossiga a execução
fn inner_loop_break() -> i32 {

    let value = loop {
        let rand_num : i32 = rand::random();

        todo!()
    };

    // Quando utilizamos o break com um loop,
    // fazemos com que o processamento prossiga para depois do loop
    // Logo quando utilizamos break, o value irá ser negado aqui
    -value
}

// Exercício 22: Loops e geração aleatória - Condicional com random
// Gere um número aleatório utilizando rand::random_range():
// - Para gerar um número positivo: rand::random_range(0..i32::MAX)
// - Para gerar um número negativo: rand::random_range(i32::MIN..0)
// Caso o argumento `negativo` seja true, deve atribuir um número negativo.
// Caso o argumento seja false, atribuir um número positivo
fn conditional_atrib(negativo: bool) -> i32 {
    let val : i32 = todo!();

    val
}

// Exercício 23: Mutação - Multiplicação de array
// Multiplique todos os elementos do array por `x`
fn mutacao_array(array: [u32; 4], x: u32) -> [u32; 4] {
    todo!()
}