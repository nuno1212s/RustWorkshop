# Workshop de Rust - Exercícios

Bem-vindo ao repositório de exercícios do Workshop de Rust! Este repositório contém exercícios e exemplos práticos para aprender Rust, organizados por dias.

## Estrutura do Repositório

```
Exercises/
├── day_1/           # Exercícios do Dia 1
└── day_2/           # Exercícios do Dia 2
```

## Objetivos

Este workshop cobre os conceitos fundamentais de Rust através de exercícios práticos:

- **Dia 1**: Conceitos básicos, variáveis, tipos, scoping, funções e controlo de fluxo
- **Dia 2**: Structs, métodos, enums e pattern matching

## Como Começar

### 1. Clone ou Faça Fork do Repositório

```bash
# Clone
git clone <repository-url>

# Ou faça fork e depois clone
git clone <your-fork-url>
```

### 2. Navegue até a Pasta do Dia

```bash
cd day_1
# ou
cd day_2
```

### 3. Complete os Exercícios

Abra os ficheiros de código e implemente as soluções conforme indicado nos comentários.

### 4. Execute os Testes

```bash
# Executar todos os testes
cargo test

# Executar testes com output verboso
cargo test -- --nocapture
```

## Conteúdo do Workshop

### Dia 1 - Conceitos Fundamentais

Os exercícios cobrem:
- Declaração de variáveis
- Tipos de dados
- Shadowing
- Scoping e lifetimes
- Funções
- Controlo de fluxo (if/else, loops)
- Mutação e arrays

### Dia 2 - Tipos Compostos

Os exercícios cobrem:
- Structs e métodos
- Enums e pattern matching

## Instruções para Completar

1. **Leia os comentários** - Cada exercício tem instruções detalhadas em comentários
2. **Implemente a solução** - Complete o código conforme pedido
3. **Execute os testes** - Verifique se a sua solução está correta. (Note que visto que alguns exercícios previnem o código de compilar, terá que resolver todos os exercícios para poder usufruir dos testes unitários)

## Pré-requisitos

- [Rust instalado](https://www.rust-lang.org/tools/install)
- Editor de código (VS Code, JetBrains IDE, etc.)
- Conhecimento básico de terminal

## Dependências

O projeto usa as seguintes dependências externas:
- `rand` - Para geração de números aleatórios

As dependências já estão configuradas no `Cargo.toml` de cada dia.
