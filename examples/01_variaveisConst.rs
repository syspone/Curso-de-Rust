// Aula: variáveis e constantes — ver GUIA_RUST.md
// Rode com: cargo run --example 01_variaveisConst

const PI: f64 = 3.14159;
const MAX_ALUNOS: usize = 40;

static TOTAL: i32 = 30;
static NOME_CURSO: &str = "Rust do Zero";

fn main() {
    println!("--- Globais ---");
    println!("Total: {}", TOTAL);
    println!("Curso: {}", NOME_CURSO);
    println!("PI: {}  |  Máximo de alunos: {}", PI, MAX_ALUNOS);

    println!("\n--- Variáveis locais (let) ---");
    let idade = 25;
    println!("Idade: {}", idade);

    let mut contador = 0;
    contador += 1;
    contador += 1;
    println!("Contador: {}", contador);

    let altura: f32 = 1.75;
    let ativo: bool = true;
    let letra: char = 'R';
    let saudacao: &str = "Olá";
    println!("Altura: {}  Ativo: {}  Letra: {}  {}", altura, ativo, letra, saudacao);

    let grande = 1_000_000u64;
    let pequeno = 200u8;
    println!("Grande: {}  Pequeno: {}", grande, pequeno);

    println!("\n--- Shadowing ---");
    let valor = "42";
    let valor: i32 = valor.parse().unwrap();
    let valor = valor * 2;
    println!("Valor: {}", valor);

    println!("\n--- Constantes locais e escopo ---");
    const DESCONTO: f64 = 0.10;
    let preco = 100.0;
    let preco_final = preco * (1.0 - DESCONTO);
    println!("Preço final: {}", preco_final);

    {
        let interna = "só existe aqui dentro";
        println!("{}", interna);
    }

    let dobro = {
        let base = 21;
        base * 2
    };
    println!("Dobro: {}", dobro);

    let resultado: i32;
    if dobro > 40 {
        resultado = 1;
    } else {
        resultado = 0;
    }
    println!("Resultado: {}", resultado);
    let _nao_usada = 99;


    println!("\n--- Constantes ---");

    // const NAME_COMPANY: &str = "code";        // Em const o tipo é obrigatório: não há inferência.
                                                // Nome de const: SCREAMING_SNAKE_CASE.

    let nome = "code";                   // Na ‘string’ não e preciso de inferir o tipo da variável.
                                                // Conversão snake_case.
    println!("Nome: {}", nome);
   // nome = "Treinamento";                     // nome é imutavel. Precisa de let mut nome para execução.

    const MEU_PASSARALHO: &str = "xibata";

    println!("Nome da Empresa: {}", "xibata");      // Compilação no binario final
    println!("Nome da Empresa: {}", MEU_PASSARALHO);
    println!("Nome da Empresa: {}", MEU_PASSARALHO);
    println!("Nome da Empresa: {}", MEU_PASSARALHO);
    println!("Nome: {}", nome);
}
