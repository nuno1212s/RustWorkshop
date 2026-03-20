#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // EXERCÍCIO 1: Introdução a Option - Básico
    // ========================================================================

    #[test]
    fn test_exercise_1_encontrar_numero_pares_valido() {
        assert_eq!(encontrar_numero_pares(&[1, 2, 3, 4], 1), Some(2));
        assert_eq!(encontrar_numero_pares(&[1, 2, 3, 4], 3), Some(4));
    }

    #[test]
    fn test_exercise_1_encontrar_numero_pares_impar() {
        assert_eq!(encontrar_numero_pares(&[1, 3, 5], 0), None);
        assert_eq!(encontrar_numero_pares(&[1, 2, 3, 4], 0), None);
    }

    #[test]
    fn test_exercise_1_encontrar_numero_pares_fora_dos_limites() {
        assert_eq!(encontrar_numero_pares(&[1, 2, 3], 10), None);
        assert_eq!(encontrar_numero_pares(&[], 0), None);
    }

    #[test]
    fn test_exercise_1_primeiro_maior_que_existe() {
        assert_eq!(primeiro_maior_que(&[1, 2, 3], 2), Some(3));
        assert_eq!(primeiro_maior_que(&[5, 10, 15], 8), Some(10));
    }

    #[test]
    fn test_exercise_1_primeiro_maior_que_nao_existe() {
        assert_eq!(primeiro_maior_que(&[1, 2, 3], 10), None);
        assert_eq!(primeiro_maior_que(&[], 5), None);
    }

    #[test]
    fn test_exercise_1_extrair_valor_some() {
        assert_eq!(extrair_valor(Some("teste".to_string())), "teste");
        assert_eq!(extrair_valor(Some("hello".to_string())), "hello");
    }

    #[test]
    fn test_exercise_1_extrair_valor_none() {
        assert_eq!(extrair_valor(None), "Vazio");
    }

    // ========================================================================
    // EXERCÍCIO 2: Introdução a Result - Básico
    // ========================================================================

    #[test]
    fn test_exercise_2_divisao_segura_valida() {
        assert_eq!(divisao_segura(10.0, 2.0), Ok(5.0));
        assert_eq!(divisao_segura(20.0, 4.0), Ok(5.0));
    }

    #[test]
    fn test_exercise_2_divisao_segura_por_zero() {
        assert_eq!(divisao_segura(10.0, 0.0), Err("Divisão por zero".to_string()));
    }

    #[test]
    fn test_exercise_2_divisao_segura_valores_invalidos() {
        assert!(divisao_segura(f64::NAN, 5.0).is_err());
        assert!(divisao_segura(5.0, f64::INFINITY).is_err());
        assert!(divisao_segura(f64::NEG_INFINITY, 5.0).is_err());
    }

    #[test]
    fn test_exercise_2_processar_idade_adulto() {
        assert_eq!(processar_idade(25), Ok("Adulto".to_string()));
        assert_eq!(processar_idade(18), Ok("Adulto".to_string()));
    }

    #[test]
    fn test_exercise_2_processar_idade_menor() {
        assert_eq!(processar_idade(10), Ok("Menor".to_string()));
        assert_eq!(processar_idade(0), Ok("Menor".to_string()));
    }

    #[test]
    fn test_exercise_2_processar_idade_invalida() {
        assert_eq!(processar_idade(-5), Err("Idade inválida".to_string()));
        assert_eq!(processar_idade(-1), Err("Idade inválida".to_string()));
    }

    #[test]
    fn test_exercise_2_verificar_senha_valida() {
        assert_eq!(verificar_senha("12345678"), Ok(()));
        assert_eq!(verificar_senha("MySecurePassword"), Ok(()));
    }

    #[test]
    fn test_exercise_2_verificar_senha_muito_curta() {
        assert_eq!(verificar_senha("123"), Err("Senha muito curta".to_string()));
        assert_eq!(verificar_senha("pass"), Err("Senha muito curta".to_string()));
    }

    #[test]
    fn test_exercise_2_verificar_senha_vazia() {
        assert_eq!(verificar_senha(""), Err("Senha vazia".to_string()));
    }

    // ========================================================================
    // EXERCÍCIO 3: Conversão entre Option e Result
    // ========================================================================

    #[test]
    fn test_exercise_3_option_para_result_some() {
        assert_eq!(option_para_result(Some(42)), Ok(42));
        assert_eq!(option_para_result(Some(0)), Ok(0));
    }

    #[test]
    fn test_exercise_3_option_para_result_none() {
        assert_eq!(option_para_result(None), Err("Nenhum valor disponível".to_string()));
    }

    #[test]
    fn test_exercise_3_result_para_option_ok() {
        assert_eq!(result_para_option(Ok("teste".to_string())), Some("teste".to_string()));
        assert_eq!(result_para_option(Ok("hello".to_string())), Some("hello".to_string()));
    }

    #[test]
    fn test_exercise_3_result_para_option_err() {
        assert_eq!(result_para_option::<String, String>(Err("erro".to_string())), None);
    }

    #[test]
    fn test_exercise_3_buscar_usuario_alice() {
        assert_eq!(buscar_usuario("alice"), Ok(1));
    }

    #[test]
    fn test_exercise_3_buscar_usuario_bob() {
        assert_eq!(buscar_usuario("bob"), Ok(2));
    }

    #[test]
    fn test_exercise_3_buscar_usuario_desconhecido() {
        assert_eq!(buscar_usuario("unknown"), Err("Utilizador não encontrado".to_string()));
        assert_eq!(buscar_usuario("charlie"), Err("Utilizador não encontrado".to_string()));
    }

    #[test]
    fn test_exercise_3_buscar_usuario_como_option_alice() {
        assert_eq!(buscar_usuario_como_option("alice"), Some(1));
        assert_eq!(buscar_usuario_como_option("bob"), Some(2));
    }

    #[test]
    fn test_exercise_3_buscar_usuario_como_option_desconhecido() {
        assert_eq!(buscar_usuario_como_option("unknown"), None);
    }

    // ========================================================================
    // EXERCÍCIO 4: O Operador ? (try operator)
    // ========================================================================

    #[test]
    fn test_exercise_4_ler_arquivo_valido() {
        // TODO: Teste ler_arquivo_e_processar("valido.txt")
        // Deve retornar Ok(100)
    }

    #[test]
    fn test_exercise_4_ler_arquivo_caminho_vazio() {
        // TODO: Teste ler_arquivo_e_processar("")
        // Deve retornar Err("Caminho vazio")
    }

    #[test]
    fn test_exercise_4_ler_arquivo_nao_encontrado() {
        // TODO: Teste ler_arquivo_e_processar com arquivo inexistente
        // Deve retornar Err("Arquivo não encontrado")
    }

    #[test]
    fn test_exercise_4_processar_numero_de_string_valido() {
        // TODO: Teste processar_numero_de_string("42")
        // Deve retornar Ok(42)
    }

    #[test]
    fn test_exercise_4_processar_numero_de_string_negativo() {
        // TODO: Teste processar_numero_de_string("-5")
        // Deve retornar Err("Número negativo")
    }

    #[test]
    fn test_exercise_4_processar_numero_de_string_nao_numero() {
        // TODO: Teste processar_numero_de_string("abc")
        // Deve retornar Err com mensagem de erro de parse
    }

    #[test]
    fn test_exercise_4_encontrar_primeiro_positivo_existe() {
        // TODO: Teste encontrar_primeiro_positivo com vetor contendo positivos
        // Deve retornar Some(primeiro positivo)
    }

    #[test]
    fn test_exercise_4_encontrar_primeiro_positivo_nenhum() {
        // TODO: Teste encontrar_primeiro_positivo com vetor sem positivos
        // Deve retornar None
    }

    #[test]
    fn test_exercise_4_encontrar_primeiro_positivo_vazio() {
        // TODO: Teste encontrar_primeiro_positivo com vetor vazio
        // Deve retornar None
    }

    // ========================================================================
    // EXERCÍCIO 5: Métodos úteis de Option e Result
    // ========================================================================

    #[test]
    fn test_exercise_5_dobrar_se_existir_some() {
        // TODO: Teste dobrar_se_existir(Some(5))
        // Deve retornar Some(10)
        // Verifica que usa .map()
    }

    #[test]
    fn test_exercise_5_dobrar_se_existir_none() {
        // TODO: Teste dobrar_se_existir(None)
        // Deve retornar None
    }

    #[test]
    fn test_exercise_5_aplicar_desconto_com_preco() {
        // TODO: Teste aplicar_desconto(Some(100.0), 10.0)
        // Deve retornar Some(90.0)
    }

    #[test]
    fn test_exercise_5_aplicar_desconto_sem_preco() {
        // TODO: Teste aplicar_desconto(None, 10.0)
        // Deve retornar None
    }

    #[test]
    fn test_exercise_5_buscar_e_processar_alice() {
        // TODO: Teste buscar_e_processar("alice")
        // Deve retornar Ok("Utilizador: 1")
        // Verifica que usa buscar_usuario() e .map()
    }

    #[test]
    fn test_exercise_5_buscar_e_processar_desconhecido() {
        // TODO: Teste buscar_e_processar com nome desconhecido
        // Deve retornar Err
    }

    #[test]
    fn test_exercise_5_resultado_ou_padrao_ok() {
        // TODO: Teste resultado_ou_padrao(Ok(42), 0)
        // Deve retornar 42
    }

    #[test]
    fn test_exercise_5_resultado_ou_padrao_err() {
        // TODO: Teste resultado_ou_padrao(Err("erro"), 0)
        // Deve retornar 0 (o padrão)
    }

    #[test]
    fn test_exercise_5_combinar_options_ambos_some() {
        // TODO: Teste combinar_options(Some(5), Some(3))
        // Deve retornar Some(8)
        // Verifica que usa .and_then()
    }

    #[test]
    fn test_exercise_5_combinar_options_primeiro_none() {
        // TODO: Teste combinar_options(None, Some(3))
        // Deve retornar None
    }

    #[test]
    fn test_exercise_5_combinar_options_segundo_none() {
        // TODO: Teste combinar_options(Some(5), None)
        // Deve retornar None
    }

    // ========================================================================
    // EXERCÍCIO 6: Cenário Complexo - Sistema de Processamento de Pedidos
    // ========================================================================

    #[test]
    fn test_exercise_6_validar_cliente_nome_valido() {
        // TODO: Teste validar_cliente com nome de 3+ caracteres
        // Deve retornar Ok(nome)
    }

    #[test]
    fn test_exercise_6_validar_cliente_nome_curto() {
        // TODO: Teste validar_cliente com nome <= 2 caracteres
        // Deve retornar Err(ClienteInvalido)
    }

    #[test]
    fn test_exercise_6_validar_quantidade_valida() {
        // TODO: Teste validar_quantidade com quantidade > 0
        // Deve retornar Ok(quantidade)
    }

    #[test]
    fn test_exercise_6_validar_quantidade_invalida() {
        // TODO: Teste validar_quantidade com quantidade <= 0
        // Deve retornar Err(QuantidadeInvalida)
    }

    #[test]
    fn test_exercise_6_validar_preco_valido() {
        // TODO: Teste validar_preco com preço > 0 e válido
        // Deve retornar Ok(preco)
    }

    #[test]
    fn test_exercise_6_validar_preco_zero_ou_negativo() {
        // TODO: Teste validar_preco com preço <= 0
        // Deve retornar Err(PrecoInvalido)
    }

    #[test]
    fn test_exercise_6_validar_preco_nan() {
        // TODO: Teste validar_preco com NaN
        // Deve retornar Err(PrecoInvalido)
    }

    #[test]
    fn test_exercise_6_calcular_total_valido() {
        // TODO: Teste calcular_total com quantidade > 0 e preço > 0
        // Deve retornar Ok(quantidade * preco)
        // Verifica que valida ambos os parâmetros com ?
    }

    #[test]
    fn test_exercise_6_calcular_total_quantidade_invalida() {
        // TODO: Teste calcular_total com quantidade inválida
        // Deve retornar Err(QuantidadeInvalida)
    }

    #[test]
    fn test_exercise_6_calcular_total_preco_invalido() {
        // TODO: Teste calcular_total com preço inválido
        // Deve retornar Err(PrecoInvalido)
    }

    #[test]
    fn test_exercise_6_processar_pedido_valido() {
        // TODO: Crie um Pedido válido:
        // id: 1, cliente_nome: "Alice", quantidade: 5, preco_unitario: 10.0
        // Teste processar_pedido
        // Deve retornar Ok com mensagem formatada
        // Verifica que usa ? para propagar erros
    }

    #[test]
    fn test_exercise_6_processar_pedido_cliente_invalido() {
        // TODO: Crie um Pedido com nome de cliente muito curto (p.ex. "X")
        // Teste processar_pedido
        // Deve retornar Err(ClienteInvalido)
    }

    #[test]
    fn test_exercise_6_processar_pedido_quantidade_invalida() {
        // TODO: Crie um Pedido com quantidade <= 0
        // Teste processar_pedido
        // Deve retornar Err(QuantidadeInvalida)
    }

    #[test]
    fn test_exercise_6_processar_pedido_preco_invalido() {
        // TODO: Crie um Pedido com preço <= 0
        // Teste processar_pedido
        // Deve retornar Err(PrecoInvalido)
    }

    #[test]
    fn test_exercise_6_processar_pedido_com_opcao_sucesso() {
        // TODO: Crie um Pedido válido
        // Teste processar_pedido_com_opcao
        // Deve retornar Some(mensagem)
        // Verifica que reutiliza processar_pedido e converte com .ok()
    }

    #[test]
    fn test_exercise_6_processar_pedido_com_opcao_erro() {
        // TODO: Crie um Pedido inválido
        // Teste processar_pedido_com_opcao
        // Deve retornar None
    }
}

