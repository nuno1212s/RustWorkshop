use thiserror::Error;


/// ============================================================================
/// EXERCÍCIO 1: Introdução a Option - Básico
/// ============================================================================
/// Aprenda a trabalhar com Option<T> para representar valores que podem estar
/// ausentes (None) ou presentes (Some(valor)).
///
/// Requisitos:
/// - Implemente `fn encontrar_numero_pares(numeros: &[i32], indice: usize) -> Option<i32>`
///   * Retorna Some(valor) se o número no índice for par
///   * Retorna None se o número for ímpar ou se o índice está fora dos limites
///
/// - Implemente `fn primeiro_maior_que(numeros: &[i32], limite: i32) -> Option<i32>`
///   * Retorna Some(primeiro número maior que limite) se existir
///   * Retorna None caso não exista nenhum
///
/// - Implemente `fn extrair_valor(opcao: Option<String>) -> String`
///   * Retorna o valor se Some
///   * Retorna "Vazio" se None
///   * Use match para tratar ambos os casos


// ============================================================================
// EXERCÍCIO 2: Introdução a Result - Básico
// ============================================================================
/// Aprenda a trabalhar com Result<T, E> para representar operações que podem
/// falhar com um erro.
///
/// Primeiro, declare um novo tipo de erro, utilizando o `thiserror` de maneira
/// a que os erros sejam programaticamente identificáveis, e não apenas strings.
/// Analise o tipo [ExampleError] para ver o pretendido.
///
///
/// Requisitos:
/// - Implemente `fn divisao_segura(dividendo: f64, divisor: f64) -> Result<f64, Erro>`
///   * Retorna Ok(resultado) se divisor != 0.0
///   * Retorna Err(Erro) se divisor == 0.0
///   * Retorna Err(Erro) se algum dos valores for NaN ou infinito
///
/// - Implemente `fn processar_idade(idade: i32) -> Result<String, Erro>`
///   * Retorna Ok("Adulto") se idade >= 18
///   * Retorna Ok("Menor") se idade < 18 e idade >= 0
///   * Retorna Err(Erro) se idade < 0
///
/// - Implemente `fn verificar_senha(senha: &str) -> Result<(), Erro>`
///   * Retorna Ok(()) se a senha tem pelo menos 8 caracteres
///   * Retorna Err(Erro) se tem menos de 8 caracteres
///   * Retorna Err(Erro) se está vazia


// ============================================================================
// EXERCÍCIO 3: Conversão entre Option e Result
// ============================================================================
/// Trabalhe com conversões entre Option<T> e Result<T, E>, e vice-versa.
/// Isto é essencial para trabalhar com códigos que usam tipos diferentes.
///
/// Requisitos:
/// - Implemente `fn option_para_result(opcao: Option<i32>) -> Result<i32, Erro>`
///   * Converte Some(valor) em Ok(valor)
///   * Converte None em Err(Erro)
///
/// - Implemente `fn result_para_option(resultado: Result<String, String>) -> Option<String>`
///   * Converte Ok(valor) em Some(valor)
///   * Converte Err(_) em None
///
/// - Implemente `fn buscar_usuario(nome: &str) -> Result<u32, Erro>`
///   * Se o nome é "alice", retorna Ok(1)
///   * Se o nome é "bob", retorna Ok(2)
///   * Caso contrário, retorna Err(Erro)
///
/// - Implemente `fn buscar_usuario_como_option(nome: &str) -> Option<u32>`
///   * Reutiliza buscar_usuario internamente
///   * Usa result_para_option para converter o resultado
///   * Utilize os examplos `alice` e `bob`. Onde alice é um utilizador válido e bob
///  não
fn  option_para_result(opcao: Option<i32>) -> Result<i32, ExampleError> {
    let valor = opcao.ok_or(ExampleError::DivPorZero)?;

    todo!()
}

// ============================================================================
// EXERCÍCIO 4: O Operador ? (try operator)
// ============================================================================
/// Aprenda a usar o operador ? para simplificar tratamento de erros em funções
/// que retornam Result ou Option.
///
/// Requisitos:
/// - Implemente `fn ler_arquivo_e_processar(caminho: &str) -> Result<usize, Erro>`
///   * Simule a leitura de um arquivo:
///     - Se caminho == "valido.txt", retorna Ok(conteúdo com comprimento 100)
///     - Se caminho == "", retorna Err(Erro)
///     - Caso contrário, retorna Err(Erro)
///   * Use ? para propagar o erro da leitura
///   * Retorna Ok(comprimento_do_conteudo)
///
/// - Implemente `fn processar_numero_de_string(s: &str) -> Result<i32, Erro>`
///   * Use ? para tratar o erro de parse (se falhar)
///   * Valide que o número é >= 0, senão retorna Err(Erro)
///   * Use ? para esta validação também
///   * Retorna Ok(numero)
///
/// - Implemente `fn encontrar_primeiro_positivo(numeros: &[i32]) -> Option<i32>`
///   * Use ? para retornar None se a lista está vazia
///   * Use ? para retornar None se nenhum número é positivo
///   * Retorna Some(primeiro número positivo)


// ============================================================================
// EXERCÍCIO 5: Métodos úteis de Option e Result
// ============================================================================
/// Trabalhe com métodos como map(), and_then(), unwrap_or(), ok_or(), etc.
///
/// Requisitos:
/// - Implemente `fn dobrar_se_existir(opcao: Option<i32>) -> Option<i32>`
///   * Use .map() para dobrar o valor se Some
///   * Retorna None se a opção era None
///
/// - Implemente `fn aplicar_desconto(preco: Option<f64>, desconto_percentual: f64) -> Option<f64>`
///   * Use .map() para aplicar o desconto se preco é Some
///   * Retorna None caso contrário
///
/// - Implemente `fn buscar_e_processar(nome: &str) -> Result<String, String>`
///   * Use buscar_usuario() (do exercício 3)
///   * Use .map() para converter o ID em "Utilizador: {id}"
///   * Retorna o resultado mapeado
///
/// - Implemente `fn resultado_ou_padrao(resultado: Result<i32, Erro>, padrao: i32) -> i32`
///   * Use .unwrap_or() para retornar o valor se Ok
///   * Retorna o padrão se Err
///
/// - Implemente `fn combinar_options(a: Option<i32>, b: Option<i32>) -> Option<i32>`
///   * Use .and_then() para combinar dois Options
///   * Retorna Some(a + b) se ambos são Some
///   * Retorna None caso contrário

// ============================================================================
// EXERCÍCIO 6: Cenário Complexo - Sistema de Processamento de Pedidos
// ============================================================================
/// Combine Option, Result e o operador ? em um cenário realista: processar
/// um pedido que envolve múltiplas validações e conversões.
///
/// Requisitos:
/// - Crie um struct `Pedido`:
///   * id: u32
///   * cliente_nome: String
///   * quantidade: i32
///   * preco_unitario: f64
///
/// - Crie um enum `ErroProcessamento`:
///   * ClienteInvalido
///   * QuantidadeInvalida
///   * PrecoInvalido
///   * DivisaoPorZero
///
/// - Implemente `fn validar_cliente(nome: &str) -> Result<String, ErroProcessamento>`
///   * Retorna Ok(nome) se o nome tem mais de 2 caracteres
///   * Retorna Err(ClienteInvalido) caso contrário
///
/// - Implemente `fn validar_quantidade(quantidade: i32) -> Result<i32, ErroProcessamento>`
///   * Retorna Ok(quantidade) se quantidade > 0
///   * Retorna Err(QuantidadeInvalida) caso contrário
///
/// - Implemente `fn validar_preco(preco: f64) -> Result<f64, ErroProcessamento>`
///   * Retorna Ok(preco) se preco > 0.0 e não é NaN/infinito
///   * Retorna Err(PrecoInvalido) caso contrário
///
/// - Implemente `fn calcular_total(quantidade: i32, preco_unitario: f64) -> Result<f64, ErroProcessamento>`
///   * Valida quantidade usando validar_quantidade
///   * Valida preco usando validar_preco
///   * Usa ? para propagar erros
///   * Retorna Ok(total) = quantidade * preco_unitario
///
/// - Implemente `fn processar_pedido(pedido: &Pedido) -> Result<String, ErroProcessamento>`
///   * Valida cliente usando validar_cliente
///   * Calcula total usando calcular_total
///   * Usa ? para propagar todos os erros
///   * Retorna Ok(mensagem) = "Pedido #{id} de {cliente}: Total {total}"
///
/// - Implemente `fn processar_pedido_com_opcao(pedido: &Pedido) -> Option<String>`
///   * Reutiliza processar_pedido
///   * Converte Result para Option usando .ok()
///   * Retorna Some(mensagem) se sucesso, None se erro
///
/// Exemplos:
/// ```
/// let pedido_valido = Pedido {
///     id: 1,
///     cliente_nome: "Alice".to_string(),
///     quantidade: 5,
///     preco_unitario: 10.5,
/// };
/// assert!(processar_pedido(&pedido_valido).is_ok());
/// assert!(processar_pedido_com_opcao(&pedido_valido).is_some());
///
/// let pedido_invalido = Pedido {
///     id: 2,
///     cliente_nome: "X".to_string(),  // muito curto
///     quantidade: 5,
///     preco_unitario: 10.5,
/// };
/// assert!(processar_pedido(&pedido_invalido).is_err());
/// ```

#[derive(Debug, Error)]
enum ExampleError {
    #[error("Divisão por zero")]
    DivPorZero
}
