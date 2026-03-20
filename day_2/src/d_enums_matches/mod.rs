mod test;

/// Enums e Data Oriented Design

/// Imagine a seguinte hierarquia numa linguagem OOP, representando formas
/// Geométricas
/// Quadrado, circulo, elipse, Triângulo, Cubo, Cilindro, Esfera

/// Todas essas formas devem implementar métodos para calcular a área
/// ,o perímetro e para retornar o nome da forma
/// Devem também partilhar alguns atributos:
/// Local base (x,y), cor e ângulo de colocação

/// Para conseguir implementar isto, deve recorrer a Composição e sum types

// ============================================================================
// EXERCÍCIO 3: Correspondência de Strings e Números
// ============================================================================
/// Pratique a correspondência em diferentes tipos (strings, números, etc).
///
/// Requisitos:
/// - Implemente `fn classificar_nota(pontuacao: u32) -> &'static str` que:
///   * 90-100 -> "A"
///   * 80-89 -> "B"
///   * 70-79 -> "C"
///   * 60-69 -> "D"
///   * 0-59 -> "F"
/// - Implemente `fn processar_comando(entrada: &str) -> String` que:
///   * "iniciar" -> "Iniciando aplicação..."
///   * "parar" -> "Parando aplicação..."
///   * "estado" -> "Estado: em execução"
///   * qualquer outra coisa -> "Comando desconhecido"
/// - Use match com padrões de intervalo para números
/// - Use match com fatias de string para comandos
///
/// Exemplo de utilização:
/// ```
/// assert_eq!(classificar_nota(95), "A");
/// assert_eq!(classificar_nota(75), "C");
/// assert_eq!(processar_comando("iniciar"), "Iniciando aplicação...");
/// ```

// ============================================================================
// EXERCÍCIO 4: Correspondência com Cláusulas de Proteção (if)
// ============================================================================
/// Use proteções de correspondência para adicionar condições adicionais aos padrões.
///
/// Requisitos:
/// - Crie um enum `Transacao` com variante: Pagamento { valor: f64 }
/// - Implemente `fn categorizar_transacao(transacao: &Transacao) -> &'static str` que:
///   * valor < 10.0 -> "Transação pequena"
///   * 10.0 <= valor <= 100.0 -> "Transação regular"
///   * valor > 100.0 -> "Transação grande"
/// - Implemente `fn compra_valida(preco_item: f64, orcamento: f64) -> bool`
///   * Retorna verdadeiro se a compra é possível e não excede o orçamento
///   * Use match com proteções para verificar condições
/// - Implemente `fn processar_pedido(quantidade: u32, desconto_percentual: f64) -> f64` que:
///   * Preço base por item: 10.0
///   * quantidade < 5 && desconto_percentual == 0.0 -> preço regular
///   * quantidade >= 5 && quantidade < 20 && desconto_percentual >= 5.0 -> aplicar 10% de desconto
///   * quantidade >= 20 && desconto_percentual >= 15.0 -> aplicar 20% de desconto
///   * todos os outros -> sem desconto
///
/// Exemplo de utilização:
/// ```
/// let trans = Transacao::Pagamento { valor: 50.0 };
/// assert_eq!(categorizar_transacao(&trans), "Transação regular");
/// ```

// ============================================================================
// EXERCÍCIO 5: Correspondência de Padrões com Enums Complexos e Option/Result
// ============================================================================
/// Trabalhe com tipos Option e Result usando correspondência de padrões.
///
/// Requisitos:
/// - Implemente `fn divisao_segura(a: f64, b: f64) -> Result<f64, &'static str>`
///   * Retorna Ok(resultado) se b != 0.0
///   * Retorna Err("Divisão por zero") caso contrário
/// - Implemente `fn desembrulhar_seguro(valor: Option<i32>) -> i32`
///   * Retorna o valor se Some
///   * Retorna 0 se None
/// - Implemente `fn processar_resultado(resultado: Result<String, String>) -> String` que:
///   * Se Ok: retorna "Sucesso: " + valor
///   * Se Err: retorna "Erro: " + mensagem_de_erro
/// - Use match para tratar todos os casos explicitamente
///
/// Exemplo de utilização:
/// ```
/// assert_eq!(divisao_segura(10.0, 2.0), Ok(5.0));
/// assert_eq!(divisao_segura(10.0, 0.0), Err("Divisão por zero"));
/// ```

// ============================================================================
// EXERCÍCIO 6: Correspondência Avançada de Padrões - Múltiplas Condições
// ============================================================================
/// Combine múltiplos padrões e condições para cenários complexos.
///
/// Requisitos:
/// - Crie um enum `Utilizador` com variantes:
///   * Admin { nome: String, permissoes: u32 }
///   * Moderador { nome: String, canais: Vec<String> }
///   * Regular { nome: String }
/// - Implemente `fn nivel_acesso_utilizador(utilizador: &Utilizador) -> i32` que:
///   * Admin com permissoes >= 100 -> retorna 5
///   * Admin com permissoes < 100 -> retorna 4
///   * Moderador com mais de 3 canais -> retorna 3
///   * Moderador com 3 ou menos canais -> retorna 2
///   * Utilizadores Regular -> retorna 1
/// - Implemente `fn pode_executar_acao(utilizador: &Utilizador, acao: &str) -> bool` que:
///   * Admins podem sempre executar qualquer ação
///   * Moderadores podem executar ações: "editar", "eliminar", "banir"
///   * Utilizadores Regular podem apenas "visualizar"
///   * Use proteções de correspondência para verificar a string de ação
///
/// Exemplo de utilização:
/// ```
/// let admin = Utilizador::Admin { nome: "Alice".to_string(), permissoes: 150 };
/// assert_eq!(nivel_acesso_utilizador(&admin), 5);
/// assert!(pode_executar_acao(&admin, "qualquer coisa"));
/// ```

