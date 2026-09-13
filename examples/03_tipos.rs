// Aula: tipos escalares e compostos — ver GUIA_RUST.md
// Rode com: cargo run --example 03_tipos

use std::mem::size_of;

fn main() {
    println!("# Inteiros\n");

    // ---------- tabela: bits | signed | unsigned ----------
    println!("| bits | signed | unsigned |");
    println!("|------|--------|----------|");
    println!("| {:>4} | {:<6} | {:<8} |", size_of::<i8>() * 8, "i8", "u8");
    println!("| {:>4} | {:<6} | {:<8} |", size_of::<i16>() * 8, "i16", "u16");
    println!("| {:>4} | {:<6} | {:<8} |", size_of::<i32>() * 8, "i32", "u32");
    println!("| {:>4} | {:<6} | {:<8} |", size_of::<i64>() * 8, "i64", "u64");
    println!("| {:>4} | {:<6} | {:<8} |", size_of::<i128>() * 8, "i128", "u128");
    println!("| {:>4} | {:<6} | {:<8} |", size_of::<isize>() * 8, "isize", "usize");
    println!("  (arch = {} bits nesta máquina)\n", size_of::<usize>() * 8);

    // ---------- signed: -(2^(n-1)) até 2^(n-1) - 1 ----------
    println!("## signed");
    println!("range: -(2^(n-1)) até 2^(n-1) - 1");
    println!("i8:    {} até {}", i8::MIN, i8::MAX);
    println!("i16:   {} até {}", i16::MIN, i16::MAX);
    println!("i32:   {} até {}", i32::MIN, i32::MAX);
    println!("i64:   {} até {}", i64::MIN, i64::MAX);
    println!("i128:  {} até {}", i128::MIN, i128::MAX);
    println!("isize: {} até {}", isize::MIN, isize::MAX);
    println!("(Recebem numeros positivos e negativos)\n");

    // conferindo a fórmula no i8: [-(2^7) até 2^7 - 1]
    let n: u32 = 8;
    let min_i8 = -(2_i32.pow(n - 1));
    let max_i8 = 2_i32.pow(n - 1) - 1;
    println!("fórmula i8: [-(2^7) até 2^7 - 1] = [{} até {}]\n", min_i8, max_i8);

    // ---------- unsigned: 0 até 2^n - 1 ----------
    println!("## unsigned");
    println!("range: 0 até 2^n - 1");
    println!("u8:    {} até {}", u8::MIN, u8::MAX);
    println!("u16:   {} até {}", u16::MIN, u16::MAX);
    println!("u32:   {} até {}", u32::MIN, u32::MAX);
    println!("u64:   {} até {}", u64::MIN, u64::MAX);
    println!("u128:  {} até {}", u128::MIN, u128::MAX);
    println!("usize: {} até {}", usize::MIN, usize::MAX);
    println!("(Nao permiti armazenamento de sinal, entao sempre positivo)\n");

    // conferindo a fórmula no u8: [0 até 2^8 - 1]
    let max_u8 = 2_u32.pow(n) - 1;
    println!("fórmula u8: [0 até 2^8 - 1] = [0 até {}]\n", max_u8);

    // ---------- o que acontece quando estoura o range ----------
    println!("## overflow");
    let x: u8 = 255;
    println!("255u8.checked_add(1)  = {:?}", x.checked_add(1)); // None
    println!("255u8.wrapping_add(1) = {}", x.wrapping_add(1)); // 0 (dá a volta)
    println!("255u8.saturating_add(1) = {}", x.saturating_add(1)); // 255 (trava no máximo)

    let y: i8 = -128;
    println!("(-128i8).checked_sub(1) = {:?}", y.checked_sub(1)); // None
    println!("(-128i8).wrapping_sub(1) = {}", y.wrapping_sub(1)); // 127
}