
fn test_parse () {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess)
}

/**
    Integegers (i inteiros com sinal) (u inteiros sem sinal)
    8 bits	i8	u8
    16 bits	i16	u16
    32 bits	i32	u32
    64 bits	i64	u64
    128 bits	i128	u128
    arco	isize	usize
*/

fn inteiro_8_bits () {
    // inteiros não assinalado de 8 bits
    // u8  de 0 á (2 ** n) - 1, onde n é o numero de bits
    // 0 a 255
    // setar como 256 irá retornar estouro de inteiro.
    let int1: u8 = 255; 
    println!("maior inteiro não assinado de 8 bits: {}", int1);


    // inteiros assinalado de 8 bits
    // u8  de -(2 ** (n - 1))  á (2 ** (n-1)) - 1, onde n é o numero de bits
    // -128 a 127
    // setar como 128 irá retornar estouro de inteiro.
    let int2: i8 = 127; 
    println!("maior inteiro assinado de 8 bits: {}", int2);
}


fn inteiro_16_bits () {
    // inteiros não assinalado de 8 bits
    // u8  de 0 á (2 ** n) - 1, onde n é o numero de bits
    // 0 a 65535
    // setar como 65536 irá retornar estouro de inteiro.
    let int1: u16 = 65535; 
    println!("maior inteiro não assinado de 16 bits: {}", int1);


    // inteiros assinalado de 8 bits
    // u8  de -(2 ** (n - 1))  á (2 ** (n - 1)) - 1, onde n é o numero de bits
    // -32768 a 32767
    // setar como 32768 irá retornar estouro de inteiro.
    let int2: i16 = 32767; 
    println!("maior inteiro assinado de 16 bits: {}", int2);
}

fn main() {
    test_parse();
    inteiro_8_bits();
    inteiro_16_bits();
}
