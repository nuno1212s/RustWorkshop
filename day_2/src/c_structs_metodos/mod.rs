use std::fs::copy;

#[cfg(test)]
mod tests;

// Exercícios de structs

// Faça um struct normal chamado `Quadrado` que armazene os seguintes campos:
// `lado` que é um numero de virgula flutuante de 64 bits
// `cor` que é uma String


// Agora, implemente uma função associada chamada `new` que aceita como argumentos
// `lado` e `cor` e que retorne uma instância de Self
// Isto é o "construtor" do Rust


// Agora, implemente métodos:
// `area` -> retorna a área do quadrado, numero de virgula flutuante de 64 bits
// `set_lado` -> altera o valor de `lado` do quadrado. Aceita um numero de virgula flutuante de 64 bits
// `set_cor` -> altera o valor de `cor` do quadrado. Aceita uma string

#[derive(Debug)]
struct Quadrado {
    lado: f64,
    cor: String
}

impl Quadrado {

    fn new(lado: f64, cor: String) -> Self {
        Quadrado {
            lado, cor
        }
    }

    fn new_2(lado: f64, cor: String) -> Quadrado {
        Self {
            lado, cor
        }
    }

    fn copia_quadrado_cor_dif(existing: Quadrado, cor: String) -> Self {
        Self {
            cor,
            .. existing
        }
    }

    fn set_lado(&mut self, lado: f64) {
        self.lado = lado;
    }

    fn set_lado_extenso(self: &mut Self, lado: f64) {
        self.lado = lado;
    }

    fn set_cor(&mut self, cor: String) {
        self.cor = cor;
    }

    fn consome_quadrado(self: Self) {
        // Tomo a ownership da instância
    }

}

fn consome_quadrado_analg(quadrado: Quadrado) {
    // Tomo a ownership da instância
}

fn test_consumir() {
    let quadrado = Quadrado::new(2.0, String::from("vermelho"));
    println!("{:?}", quadrado);
    quadrado.consome_quadrado();
    // quadrado já não é valido
    println!("{:?}", quadrado)
}

// Faça um Tuple Struct `Ponto3D` que represente um ponto tri dimensional de f64

struct Ponto3D(f64, f64, f64);

// Desconstrua o struct no nome do argumento, utilizando as variáveis `x`, `y`, `z`
fn deconstruct_tuple(Ponto3D(x, y, z): Ponto3D) -> (f64, f64, f64) {
    (x, y, z)
}

fn construct_tuple(x: f64, y: f64, z: f64) -> Ponto3D {
    Ponto3D(x, y, z)
}

fn first_member(ponto: Ponto3D) -> f64 {
    ponto.0
}

fn third_member(ponto: Ponto3D) -> f64 {
    todo!()
}

// Structs e ownerships
struct TestStruct {
    test_clone_obj: String,
    test_copy_obj: u32
}

// Deconstrua o struct `test_struct` de maneira a obter a ownership dos
// objetos contidos
fn deconstruct_struct(test_struct: TestStruct) -> (String, u32) {
    let TestStruct {
        test_clone_obj, test_copy_obj
    } = test_struct;

    (test_clone_obj, test_copy_obj)
}

// Extraia o `test_copy_obj` de `test_struct` sem desconstruir o struct
fn take_copy_obj(test_struct: TestStruct) -> (TestStruct, u32) {
    let copy_obj = test_struct.test_copy_obj;

    (test_struct, copy_obj)
}

// Extraia o `test_clone_obj` de `test_struct` sem desconstruir o struct
fn take_cloneable_obj(test_struct: TestStruct) -> (TestStruct, String) {
    let clone_obj = test_struct.test_clone_obj.clone();

    (test_struct, clone_obj)
}

// Extraia o `test_clone_obj` da ref a `test_struct`.
fn take_cloneable_obj_from_ref(test_struct: &TestStruct) -> String {
    test_struct.test_clone_obj.clone()
}

/// Modelação de dados:
/// Nesta secção vamos ver alguns exemplos de como a modelação correta de dados
/// Tem um impacto forte com o modelo de ownership

// Corrija a estrutura para que test_fn não perca o objetivo (alterar o valor de test_var)
// Mas de maneira a que o modelo de ownership funcione
struct ComplexOwnershipProblem {
    test_var: String,
    inner_struct: InnerComplexOwnershipProblem
}

struct InnerComplexOwnershipProblem {
    test_obj: TestStruct
}

impl ComplexOwnershipProblem {

    fn test_fn(&mut self) {
        self.inner_struct.test_fn(&mut self.test_var);
    }

}

impl InnerComplexOwnershipProblem {

    fn test_fn(&mut self, test_var: &mut String) {
        *test_var = self.test_obj.test_clone_obj.clone();
    }

}

/// O problema da utilização circular
/// Objetivo: Entender as limitações do ownership com estruturas circulares

struct Node {
    valor: i32,
    proximo: Option<Box<Node>>
}

struct Lista {
    inicio: Option<Node>,
}

impl Lista {
    // Crie uma LinkedList simples e implemente:

    // 1. Adicionar um elemento ao final
    fn adicionar(&mut self, valor: i32) {
        todo!()
    }

    // 2. Contar elementos (sem consumir a lista)
    fn contar(&self) -> usize {
        todo!()
    }

    // 3. Retornar o último elemento (por referência)
    fn ultimo(&self) -> Option<&Node> {
        todo!()
    }
}


///  Padrão de "Swap" para contornar limitações
/// Objetivo: Usar swap para rearranjar dados mantendo ownership

struct Pagina {
    numero: u32,
    conteudo: String
}

struct Livro2 {
    pagina1: Pagina,
    pagina2: Pagina,
    pagina3: Pagina
}

impl Livro2 {
    // Implemente um método que inverte a ordem das páginas
    // usando `std::mem::swap` ou o operador `=`
    fn inverter_ordem(&mut self) {
        todo!()
    }

    // Extraia uma página e substitua-a por outra
    fn trocar_pagina(&mut self, indice: u32, nova_pagina: Pagina) -> Pagina {
        todo!()
    }
}