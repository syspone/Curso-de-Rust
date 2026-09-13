// Aula: ownership e borrowing — ver GUIA_RUST.md
// Rode com: cargo run --example 06_OwnershipBorrowing

fn main() {

    println!("\n--- Ownership e Referencias e Emprestimos(Borrowing) ---");

    let texto = String::from ("Hello");
    let texto2 = texto;

    println!("Texto : {}", texto2);

    let mut valor = 50;
    let _referencia = &valor;

    let _referencia_mut = &mut valor;

    //println!( "Referencia : {}", _referencia );

    // Outros exemplos

    /* Borrowing (Copia)*/
    let a = 1;      // Copy (f64, bool, i32, char)
    let b = &a;    // O & é o operador de empréstimo (borrow). Ele cria uma referência a "a" em vez de copiar ou mover o valor.

    println!("a: {}, b: {}", a, b);

    /* Ownership (movimento) */
    let c = String::from("Cristian");
    let d = c;

    println!("d: {}", d);
}