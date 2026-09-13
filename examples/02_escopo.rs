// Aula: escopo de variáveis — ver GUIA_RUST.md
// Rode com: cargo run --example 02_escopo

fn main() {
    let esc_ex = 30;
    {
        let esc_int = esc_ex + 20;
        println!("Escopo Interno: {}", esc_int);
    }   // Fim do bloco interno
    println!("Escopo Externo {}", esc_ex);
}
