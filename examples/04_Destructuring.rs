// Aula: destructuring de tuplas — ver GUIA_RUST.md
// Rode com: cargo run --example 04_Destructuring

fn main() {
    // tupla
    let mut numbers = (1, 2, 3);            // mut permite alterar os campos depois
    numbers.0 = 50;                         // muta o campo 0, não redeclara a tupla
    println!("{:?}", numbers);

    let (a, b, c) = numbers;
    println!("a: {}, b: {}, c: {}", a, b, c);
}