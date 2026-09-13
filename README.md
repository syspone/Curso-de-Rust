<div align="center">

# 🦀 Curso de Rust — do Zero ao Ownership

**Meu caderno de estudos de Rust: uma aula, um exemplo executável, um capítulo no guia.**

![Rust](https://img.shields.io/badge/Rust-1.98-orange?logo=rust)
![Edition](https://img.shields.io/badge/edition-2024-blue)
![IDE](https://img.shields.io/badge/IDE-RustRover-black?logo=jetbrains)
![Aulas](https://img.shields.io/badge/aulas-6-green)
![Status](https://img.shields.io/badge/status-em%20andamento-yellow)

</div>

---

## 📌 Sobre o repositório

Este projeto acompanha o meu curso de Rust. A ideia é simples:

- **`examples/`** guarda o código de cada aula — cada arquivo é um programa
  independente, pronto para rodar.
- **[`GUIA_RUST.md`](GUIA_RUST.md)** é o material de consulta: toda explicação,
  tabela comparativa e pegadinha do compilador fica lá, organizada por capítulo.
- **Este README** é o mapa: mostra onde estou na trilha e como navegar.

> 💡 **Regra da casa:** os exemplos têm só um cabeçalho de duas linhas (aula + comando).
> A teoria mora no guia, não em comentários espalhados pelo código.

---

## 🚀 Começando

Pré-requisito: [rustup](https://rustup.rs) instalado.

```bash
# rodar uma aula específica
cargo run --example 02_escopo

# compilar todos os exemplos de uma vez (bom para checar se nada quebrou)
cargo build --examples
```

O nome no comando é o nome do arquivo em `examples/` **sem** o `.rs`.

No **RustRover**, basta abrir o arquivo do exemplo e clicar no ▶️ ao lado do `fn main()`.

---

## 🗺️ Trilha do curso

| # | Aula | Exemplo | Guia | Status |
|:-:|---|---|---|:-:|
| 01 | Variáveis, shadowing, `const` e `static` | [`01_variaveisConst.rs`](examples/01_variaveisConst.rs) | caps. [2](GUIA_RUST.md#2-variáveis), [3](GUIA_RUST.md#3-shadowing), [4](GUIA_RUST.md#4-constantes-e-estáticos) | ✅ |
| 02 | Escopo e blocos como expressão | [`02_escopo.rs`](examples/02_escopo.rs) | cap. [5](GUIA_RUST.md#5-escopo) | ✅ |
| 03 | Tipos escalares, inteiros e overflow | [`03_tipos.rs`](examples/03_tipos.rs) | cap. [8](GUIA_RUST.md#8-tipos-escalares-e-compostos) | ✅ |
| 04 | Tuplas e destructuring | [`04_Destructuring.rs`](examples/04_Destructuring.rs) | cap. [9](GUIA_RUST.md#9-tuplas-e-destructuring) | ✅ |
| 05 | Arrays, slices e matrizes | [`05_arrays.rs`](examples/05_arrays.rs) | cap. [10](GUIA_RUST.md#10-arrays) | ✅ |
| 06 | Ownership e borrowing | [`06_OwnershipBorrowing.rs`](examples/06_OwnershipBorrowing.rs) | cap. [11](GUIA_RUST.md#11-ownership-e-borrowing) | ✅ |

Capítulos transversais do guia: [como rodar](GUIA_RUST.md#1-como-rodar-os-exemplos) ·
[convenção de nomes](GUIA_RUST.md#6-convenção-de-nomes) ·
[saída no terminal](GUIA_RUST.md#7-saída-no-terminal)

### 🔭 Próximas paradas

- [x] Ownership, move e `Copy`
- [x] Referências `&` e `&mut` — as regras do borrow checker
- [ ] Funções e retorno de valores
- [ ] Controle de fluxo: `if`, `loop`, `while`, `for`
- [ ] `String` × `&str`
- [ ] Structs e métodos
- [ ] Enums, `Option` e `match`
- [ ] Tratamento de erros com `Result` e `?`
- [ ] Coleções: `Vec`, `HashMap`
- [ ] Traits e generics
- [ ] Lifetimes

---

## 📂 Estrutura

```
Curso/
├── Cargo.toml          # pacote "Curso", edition 2024, sem dependências
├── GUIA_RUST.md        # 📖 teoria e referência, um capítulo por tema
├── README.md           # 🗺️ você está aqui
├── src/
│   └── main.rs         # binário padrão (Hello, world!)
└── examples/           # 🧪 uma aula por arquivo, cada um com seu main()
    ├── 01_variaveisConst.rs
    ├── 02_escopo.rs
    ├── 03_tipos.rs
    ├── 04_Destructuring.rs
    ├── 05_arrays.rs
    └── 06_OwnershipBorrowing.rs
```

---

## ✍️ Como adicionar uma nova aula

1. Crie `examples/NN_tema.rs` com o cabeçalho padrão:

   ```rust
   // Aula: <tema> — ver GUIA_RUST.md
   // Rode com: cargo run --example NN_tema
   ```

2. Escreva o capítulo novo **no final** do [`GUIA_RUST.md`](GUIA_RUST.md) e
   atualize o índice e a tabela de exemplos de lá.
3. Adicione a linha na [trilha](#️-trilha-do-curso) acima e marque o item em
   _Próximas paradas_.
4. Confira que tudo compila: `cargo build --examples`.

---

## 🧠 Lembretes que o compilador já me ensinou

| Situação | O que o `rustc` diz | Saída |
|---|---|---|
| Reatribuir `let` sem `mut` | `cannot assign twice to immutable variable` | `let mut` ou shadowing |
| Usar variável fora do bloco | `cannot find value in this scope` | declarar no escopo externo |
| Índice literal fora do array | `this operation will panic at runtime` | `.get(i)` → `Option` |
| Usar `String` depois de mover | `borrow of moved value` | `&` para emprestar ou `.clone()` |
| `u8` passando de 255 | `panic` em debug | `checked_`, `wrapping_`, `saturating_` |

---

## 📚 Referências

- [The Rust Programming Language](https://doc.rust-lang.org/book/) — "o livro"
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) — exercícios curtos
- [Documentação da std](https://doc.rust-lang.org/std/)
- [Rust Playground](https://play.rust-lang.org/) — testar código sem abrir o projeto

---

<div align="center">

Feito com 🦀 e muitas brigas com o borrow checker por **[syspone](https://github.com/syspone)**

</div>
