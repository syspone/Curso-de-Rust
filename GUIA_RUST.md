# Guia de Rust

Guia de consulta do curso. Cada capítulo corresponde a uma aula e aponta para o
exemplo executável em `examples/`. Novos capítulos são adicionados no final conforme
o curso avança.

## Índice

1. [Como rodar os exemplos](#1-como-rodar-os-exemplos)
2. [Variáveis](#2-variáveis)
3. [Shadowing](#3-shadowing)
4. [Constantes e estáticos](#4-constantes-e-estáticos)
5. [Escopo](#5-escopo)
6. [Convenção de nomes](#6-convenção-de-nomes)
7. [Saída no terminal](#7-saída-no-terminal)
8. [Tipos escalares e compostos](#8-tipos-escalares-e-compostos)
9. [Tuplas e destructuring](#9-tuplas-e-destructuring)
10. [Arrays](#10-arrays)
11. [Ownership e borrowing](#11-ownership-e-borrowing)

Exemplos por capítulo:

| Capítulos | Arquivo | Comando |
|---|---|---|
| 2, 3, 4 | `examples/01_variaveisConst.rs` | `cargo run --example 01_variaveisConst` |
| 5 | `examples/02_escopo.rs` | `cargo run --example 02_escopo` |
| 8 | `examples/03_tipos.rs` | `cargo run --example 03_tipos` |
| 9 | `examples/04_Destructuring.rs` | `cargo run --example 04_Destructuring` |
| 10 | `examples/05_arrays.rs` | `cargo run --example 05_arrays` |
| 11 | `examples/06_OwnershipBorrowing.rs` | `cargo run --example 06_OwnershipBorrowing` |

---

## 1. Como rodar os exemplos

Cada arquivo em `examples/` é um programa independente: tem o seu próprio `main()`,
não precisa de `mod` no `main.rs` e não entra no build normal.

```bash
cargo run --example NOME      # roda examples/NOME.rs
cargo build --examples        # compila todos os exemplos sem rodar
cargo build                   # ignora a pasta examples/
```

O nome usado no comando é o nome do arquivo sem `.rs`.

---

## 2. Variáveis

### Imutável por padrão

```rust
let idade = 25;
// idade = 26; // ERRO: cannot assign twice to immutable variable
```

O compilador infere o tipo a partir do valor (aqui `i32`). Depois de declarada,
uma variável `let` não pode ser reatribuída.

### Mutável (`let mut`)

```rust
let mut contador = 0;
contador += 1;
contador += 1; // contador == 2
```

`mut` libera a reatribuição, mas o **tipo** continua fixo: `contador = "x"` não compila.

### Anotação de tipo

Use quando a inferência não basta ou para deixar a intenção clara:

```rust
let altura: f32 = 1.75;
let ativo: bool = true;
let letra: char = 'R';
let saudacao: &str = "Olá";
```

Em literais numéricos dá para colocar o tipo como sufixo e usar `_` como separador visual:

```rust
let grande = 1_000_000u64; // u64
let pequeno = 200u8;       // u8
```

### Declarar sem inicializar

Permitido, desde que a variável receba um valor em **todos** os caminhos antes de ser lida:

```rust
let resultado: i32;
if dobro > 40 {
    resultado = 1;
} else {
    resultado = 0;
}
println!("{}", resultado);
```

### Variável não utilizada

O compilador avisa sobre variáveis nunca lidas. Prefixe com `_` para silenciar:

```rust
let _nao_usada = 99;
```

---

## 3. Shadowing

Declarar de novo com `let` cria uma **nova** variável com o mesmo nome, que "esconde" a anterior.
Diferente de `mut`, o shadowing permite trocar o tipo:

```rust
let valor = "42";                          // &str
let valor: i32 = valor.parse().unwrap();   // i32
let valor = valor * 2;                     // i32, novo valor (84)
```

| | `let mut` | shadowing (`let` de novo) |
|---|---|---|
| Reatribuir valor | sim | sim (cria outra variável) |
| Trocar o tipo | não | sim |
| Vale fora do bloco atual | sim | só dentro do bloco onde foi redeclarada |

---

## 4. Constantes e estáticos

Ambos exigem tipo explícito e nome em `SCREAMING_SNAKE_CASE`. Podem ser declarados
fora de qualquer função (globais) ou dentro de uma função (locais).

```rust
const PI: f64 = 3.14159;
const MAX_ALUNOS: usize = 40;

static TOTAL: i32 = 30;
static NOME_CURSO: &str = "Rust do Zero";

fn main() {
    const DESCONTO: f64 = 0.10; // const local
    let preco_final = 100.0 * (1.0 - DESCONTO);
}
```

### `let` × `const`

```rust
let nome = "code";                     // variável
const MEU_PASSARALHO: &str = "xibata"; // constante
```

| | `let` | `const` |
|---|---|---|
| Anotação de tipo | opcional (o compilador infere) | **obrigatória** |
| Valor | pode vir do runtime (entrada do usuário, cálculo, retorno de função) | só expressão avaliável em tempo de compilação |
| Mutável | sim, com `let mut` | nunca |
| Onde pode ser declarada | só dentro de função ou bloco | dentro **ou fora** de função, em qualquer módulo |
| Na memória | ocupa espaço na stack e tem endereço | é copiada ("inlined") em cada uso |
| Convenção de nome | `snake_case` | `SCREAMING_SNAKE_CASE` |

Como a constante é copiada em cada uso, estas duas linhas geram exatamente o mesmo
código no binário final — a constante em si não sobra lá:

```rust
println!("Nome da Empresa: {}", MEU_PASSARALHO);
println!("Nome da Empresa: {}", "xibata");
```

Em `const MEU_PASSARALHO: &str = "xibata";` o tipo completo é `&'static str`: o tempo
de vida `'static` é implícito em constantes.

Reatribuir uma variável imutável é **erro de compilação**, não aviso:

```rust
let nome = "code";
nome = "Treinamento"; // erro: cannot assign twice to immutable variable
```

### `const` × `static`

| | `const` | `static` |
|---|---|---|
| Onde vive | copiado ("inlined") em cada uso | um único endereço na memória durante todo o programa |
| Tem endereço fixo? | não | sim (dá para pegar `&TOTAL`) |
| Valor | deve ser calculável em tempo de compilação | idem |
| Mutável? | nunca | só com `static mut`, exige `unsafe` (evitar) |
| Uso típico | números, limites, fatores | dados grandes ou compartilhados, strings globais |

Na dúvida, use `const`. Prefira `static` apenas quando precisar da **mesma** referência
em vários lugares.

---

## 5. Escopo

Escopo é a região do código em que uma variável existe e pode ser usada. Em Rust o
escopo é delimitado por chaves `{}`: a variável nasce no `let` e é destruída quando
o bloco em que foi declarada termina.

```rust
fn main() {
    let esc_ex = 30;               // vive até o `}` de main()

    {                              // abre um novo escopo
        let esc_int = esc_ex + 20; // pode usar esc_ex (escopo externo)
        println!("{}", esc_int);
    }                              // esc_int é destruída aqui

    println!("{}", esc_ex);        // ok
    // println!("{}", esc_int);    // ERRO: cannot find value `esc_int` in this scope
}
```

Regras:

- Um bloco interno enxerga as variáveis do bloco externo.
- O bloco externo **não** enxerga as variáveis do bloco interno.
- Ao sair do bloco a variável deixa de existir e a memória é liberada (isso é a base do
  *ownership*, tema das próximas aulas).

### Bloco como expressão

Um bloco pode produzir um valor: a última linha **sem `;`** é o resultado.

```rust
let dobro = {
    let base = 21;
    base * 2      // sem ponto e vírgula
};                // dobro == 42
```

---

## 6. Convenção de nomes

O compilador emite `warning` quando o nome foge do padrão (`non_snake_case`, etc.).
Não é erro, mas vale seguir:

| Item | Estilo | Exemplo |
|---|---|---|
| variáveis, funções, módulos | `snake_case` | `esc_ex`, `preco_final` |
| `const` e `static` | `SCREAMING_SNAKE_CASE` | `TOTAL`, `MAX_ALUNOS` |
| structs, enums, traits | `CamelCase` | `Aluno`, `Escopo` |

Para manter um nome fora do padrão de propósito, use `#[allow(non_snake_case)]`
acima do item.

---

## 7. Saída no terminal

`println!` acrescenta quebra de linha no final; `print!` não. Em programas de
terminal use `println!`, senão o prompt do shell aparece colado na saída.

```rust
println!("Total: {}", total);          // posicional
println!("Total: {total}");            // captura direta da variável
println!("{}  |  {}", a, b);           // vários valores
```

---

## 8. Tipos escalares e compostos

**Escalares** representam um único valor dentro de uma escala conhecida, o que
permite comparação direta entre valores:

| Tipo | Nome | Exemplo |
|---|---|---|
| inteiro | `integer` | `5` |
| flutuante | `float point` | `42.1` |
| booleano | `bool` | `true`, `false` |
| carácter | `char` | `'a'`, `'虫'`, `'😂'` |

**Compostos** agregam múltiplos valores:

| Tipo | Nome | Exemplo |
|---|---|---|
| tupla | `tuple` | `(5, true, 42.1, 'a')` |
| matriz | `array` | `[1, 2, 3, 4, 5, 6]` |

### Inteiros: tamanho e sinal

Cada inteiro tem um número de bits e um sinal. `size_of::<T>()` devolve o tamanho
em bytes; multiplicando por 8 chega-se aos bits:

| bits | signed | unsigned |
|---|---|---|
| 8 | `i8` | `u8` |
| 16 | `i16` | `u16` |
| 32 | `i32` | `u32` |
| 64 | `i64` | `u64` |
| 128 | `i128` | `u128` |
| arch | `isize` | `usize` |

`isize`/`usize` acompanham a arquitetura da máquina (64 bits num PC comum). `usize`
é o tipo usado para índices e tamanhos de coleções.

Os limites saem direto da quantidade de bits:

- **signed** (`i*`): de `-(2^(n-1))` até `2^(n-1) - 1` — aceita negativos.
  Em `i8`: `-128` até `127`.
- **unsigned** (`u*`): de `0` até `2^n - 1` — não guarda sinal, sempre positivo.
  Em `u8`: `0` até `255`.

Todo tipo numérico expõe as constantes `MIN` e `MAX`:

```rust
println!("i8: {} até {}", i8::MIN, i8::MAX);   // -128 até 127
println!("u8: {} até {}", u8::MIN, u8::MAX);   // 0 até 255
```

### Overflow

Estourar o range não é ignorado: em modo debug o programa entra em `panic`. Para
decidir o comportamento explicitamente, use os métodos da família:

```rust
let x: u8 = 255;
x.checked_add(1)     // None          -> devolve Option, None quando estoura
x.wrapping_add(1)    // 0             -> dá a volta no range
x.saturating_add(1)  // 255           -> trava no limite

let y: i8 = -128;
y.checked_sub(1)     // None
y.wrapping_sub(1)    // 127
```

Em release o comportamento padrão é o `wrapping`, então nunca dependa do `panic`
para detectar overflow — escolha o método que expressa a intenção.

---

## 9. Tuplas e destructuring

Uma tupla agrupa um número **fixo** de valores, que podem ser de **tipos diferentes**.
O tipo da tupla é a lista dos tipos dos seus campos, na ordem:

```rust
let numbers = (1, 2, 3);                              // (i32, i32, i32)
let misto: (i32, bool, f64, char) = (5, true, 42.1, 'a');
```

A tupla vazia `()` é chamada de *unit* e representa "nenhum valor" — é o que uma
função sem `return` devolve.

### Acesso por índice

Os campos são acessados por posição, com `.0`, `.1`, `.2`. O índice é parte da
sintaxe: precisa ser um número literal, não uma variável.

```rust
let numbers = (1, 2, 3);
println!("{}", numbers.0);   // 1
```

Para **alterar** um campo a tupla precisa ser `mut` — isso é mutação de um campo,
não shadowing (ver [3. Shadowing](#3-shadowing)):

```rust
let mut numbers = (1, 2, 3);
numbers.0 = 50;              // ok, numbers == (50, 2, 3)
```

### Imprimir

Tupla não implementa `Display`, então `{}` não funciona com ela toda. Use `{:?}`
(`Debug`) para imprimir a tupla inteira, ou `{}` em cada campo:

```rust
println!("{:?}", numbers);   // (50, 2, 3)
println!("{}", numbers.0);   // 50
```

### Destructuring

Destructuring é quebrar a tupla em variáveis separadas, casando a forma do `let`
com a forma do valor:

```rust
let numbers = (50, 2, 3);
let (a, b, c) = numbers;
println!("a: {}, b: {}, c: {}", a, b, c);   // a: 50, b: 2, c: 3
```

Variações úteis:

```rust
let (x, _, z) = numbers;        // `_` descarta o campo do meio
let ((p, q), r) = ((1, 2), 3);  // tuplas aninhadas
let (mut m, n) = (1, 2);        // `mut` vale por variável
```

Como o lado direito é avaliado inteiro antes da atribuição, dá para trocar dois
valores em uma linha:

```rust
let (s, t) = (10, 20);
let (s, t) = (t, s);            // s == 20, t == 10
```

O mesmo padrão funciona em parâmetros de função:

```rust
fn distancia((x, y): (i32, i32)) -> i32 {
    x.abs() + y.abs()
}
```

| | acesso por índice | destructuring |
|---|---|---|
| Sintaxe | `t.0` | `let (a, b) = t;` |
| Pega quantos campos | um por vez | todos de uma vez |
| Serve para escrever | sim (com `mut`) | não, só para ler |
| Quando usar | um campo pontual | quando os campos ganham nomes próprios |

---

## 10. Arrays

Um array guarda vários valores do **mesmo tipo**, em quantidade **fixa** definida
em tempo de compilação. O tamanho faz parte do tipo: `[i32; 5]` e `[i32; 3]` são
tipos diferentes.

```rust
let numbers = [1, 2, 3, 4, 5];        // [i32; 5]
let explicito: [i32; 3] = [7, 8, 9];
let zeros = [0u8; 5];                 // repete o valor: [0, 0, 0, 0, 0]
```

### Acesso

O índice começa em `0` e, diferente da tupla, pode ser uma variável:

```rust
println!("{:?}", numbers[0]);   // 1
println!("{}", numbers.len());  // 5
```

Estourar o limite com um índice **literal** é erro de compilação:

```rust
// println!("{}", numbers[10]);
// error: this operation will panic at runtime
// index out of bounds: the length is 5 but the index is 10
```

Com um índice calculado em tempo de execução o programa entra em `panic`. Para
tratar o caso sem quebrar, use `.get()`, que devolve `Option`:

```rust
numbers.get(0)    // Some(1)
numbers.get(10)   // None
```

### Mutação

Como nas variáveis, é preciso `mut` — e ele muda o conteúdo, nunca o tamanho:

```rust
let mut mutavel = [1, 2, 3];
mutavel[0] = 99;              // [99, 2, 3]
```

### Percorrer

```rust
for n in numbers {
    println!("{}", n);
}

for (i, n) in numbers.iter().enumerate() {
    println!("{}: {}", i, n);   // índice junto com o valor
}
```

### Fatias (slices)

Uma fatia é uma referência a um pedaço do array; o range `a..b` inclui `a` e
exclui `b`:

```rust
let fatia = &numbers[1..4];   // [2, 3, 4]
println!("{}", fatia.len());  // 3
```

### Cópia e arrays aninhados

Se o tipo dos elementos é `Copy` (números, `bool`, `char`), o array inteiro é
copiado na atribuição — o original continua válido:

```rust
let copia = numbers;
println!("{:?} {:?}", numbers, copia);   // os dois funcionam
```

Um array de arrays serve como matriz:

```rust
let matriz = [[1, 2], [3, 4]];
println!("{}", matriz[1][0]);   // 3
```

Destructuring também vale para arrays, com `..` para pular o meio:

```rust
let [primeiro, .., ultimo] = numbers;   // 1 e 5
```

| | tupla | array |
|---|---|---|
| Tipos dos elementos | podem ser diferentes | todos iguais |
| Tamanho | fixo, na declaração | fixo, parte do tipo |
| Índice | literal (`t.0`) | expressão (`a[i]`) |
| Impressão | `{:?}` | `{:?}` |

---

## 11. Ownership e borrowing

Ownership (posse) é o jeito que o Rust tem de liberar memória sem garbage collector
e sem `free` manual. O compilador confere tudo em tempo de compilação, com três regras:

1. Todo valor tem **um dono**: a variável que o guarda.
2. Só existe **um dono por vez**.
3. Quando o dono sai de escopo, o valor é destruído (ver [5. Escopo](#5-escopo)).

### Stack e heap

| | stack | heap |
|---|---|---|
| Tamanho do dado | fixo, conhecido na compilação | pode crescer em tempo de execução |
| Exemplos | `i32`, `f64`, `bool`, `char`, arrays e tuplas desses tipos | o conteúdo de `String`, `Vec` |
| Custo de copiar | barato (copia os bytes) | caro (teria que duplicar o dado inteiro) |

Uma `String` é dividida em duas partes. Na stack ficam o ponteiro, o tamanho e a
capacidade; o texto em si fica na heap. É por isso que ela não é copiada na atribuição.

### Move

Quando um valor que está na heap é atribuído a outra variável, a **posse passa para
ela**. A variável antiga fica inválida:

```rust
let texto = String::from("Hello");
let texto2 = texto;                  // move: texto2 agora é o dono

println!("{}", texto2);              // ok
// println!("{}", texto);            // ERRO E0382: borrow of moved value: `texto`
```

Nenhum byte do texto é copiado. Só o ponteiro muda de dono, e continua existindo
um único responsável por liberar a memória.

Passar um valor para uma função também move:

```rust
fn tamanho(s: String) -> usize {
    s.len()
}                                    // s sai de escopo: a String é liberada aqui

let nome = String::from("Rust");
let n = tamanho(nome);               // nome foi movido para dentro da função
// println!("{}", nome);             // ERRO E0382
```

### Copy

Tipos de tamanho fixo que vivem inteiros na stack implementam `Copy`. Nesse caso a
atribuição **duplica** o valor e as duas variáveis continuam válidas:

```rust
let a = 1;
let b = a;                           // cópia, não move
println!("a: {}, b: {}", a, b);      // ok
```

São `Copy`: todos os inteiros e floats, `bool`, `char`, e tuplas e arrays formados
só por esses tipos. `String` e `Vec` **não** são.

### Clone

Quando você realmente quer duas cópias independentes de um dado na heap, peça isso
explicitamente com `.clone()`:

```rust
let c = String::from("Cristian");
let d = c.clone();                   // duplica o texto na heap
println!("c: {}, d: {}", c, d);      // os dois continuam válidos
```

`.clone()` custa memória e tempo. Por isso ele é explícito: quando aparece no código,
fica visível que existe uma cópia ali.

### Referências (`&`): emprestar sem tomar posse

O operador `&` cria uma **referência**. Ela deixa usar o valor sem virar dona dele,
e esse ato de pegar emprestado se chama *borrowing*. Quando a referência sai de
escopo, nada é liberado, porque ela nunca foi dona:

```rust
fn tamanho(s: &String) -> usize {
    s.len()
}

let nome = String::from("Rust");
let n = tamanho(&nome);              // empresta
println!("{} tem {} letras", nome, n);  // nome continua válido
```

Uma referência comum é **só leitura**.

### Referências mutáveis (`&mut`)

Para alterar o valor emprestado, a variável precisa ser `mut` e a referência precisa
ser `&mut`:

```rust
fn adicionar(s: &mut String) {
    s.push_str(" é legal");
}

let mut nome = String::from("Rust");
adicionar(&mut nome);                // nome == "Rust é legal"
```

Com números, use `*` para chegar ao valor por trás da referência:

```rust
let mut valor = 50;
let r = &mut valor;
*r += 1;                             // valor == 51
```

### A regra do borrow checker

Em qualquer momento, um valor pode ter **ou** várias referências `&`, **ou** uma
única `&mut`. Nunca as duas coisas ao mesmo tempo.

```rust
let mut valor = 50;
let referencia = &valor;             // empréstimo de leitura
let referencia_mut = &mut valor;     // ERRO E0502 se `referencia` ainda for usada
*referencia_mut += 1;
println!("{}", referencia);          // ...e aqui ela é usada
// cannot borrow `valor` as mutable because it is also borrowed as immutable
```

Um empréstimo dura só **até o último uso** da referência, e não até o fim do bloco.
Por isso o exemplo da aula compila: `_referencia` nunca mais é lida depois do
`&mut`. Basta descomentar o `println!` de `_referencia` para o erro aparecer.

A regra evita que alguém leia um dado enquanto outra parte do código o altera.
Com isso, esse tipo de bug é pego na compilação, e não em produção.

### Referências nunca ficam penduradas

O compilador não deixa uma referência viver mais que o seu dono:

```rust
fn pendurada() -> &String {          // ERRO E0106: missing lifetime specifier
    let s = String::from("x");
    &s                               // s seria destruída no fim da função
}
```

A saída é devolver o próprio valor (`-> String`) e mover a posse para quem chamou a
função. O porquê do erro falar em *lifetime* fica para o capítulo de lifetimes.

### Resumo

| Operação | Sintaxe | Original continua válido? | Pode alterar? |
|---|---|---|---|
| move | `let b = a;` (tipo não `Copy`) | não | sim, se `b` for `mut` |
| copy | `let b = a;` (tipo `Copy`) | sim | cada um altera o seu |
| clone | `let b = a.clone();` | sim | cada um altera o seu |
| empréstimo | `let b = &a;` | sim | não |
| empréstimo mutável | `let b = &mut a;` | sim, mas sem uso enquanto `b` estiver em uso | sim, por `b` |
