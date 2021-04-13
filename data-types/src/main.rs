
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
    println!("{}", int1);


    // inteiros assinalado de 8 bits
    // u8  de -(2 ** (n - 1))  á (2 ** (n-1)) - 1, onde n é o numero de bits
    // -128 a 127
    // setar como 128 irá retornar estouro de inteiro.
    let int2: i8 = 127; 
    println!("{}", int2);
}

fn main() {
    test_parse();
    inteiro_8_bits();
}
