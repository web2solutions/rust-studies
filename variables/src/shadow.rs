fn main() { 
    
    // Sombreamento - shadowing
    // se tentar atribuir valor a x sem usar let resultará em erro
    // podemos mudar o tipo de variável
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is {}", x);

    let x = "abc";

    println!("The value of x is {}", x);

    println!("The length of x is {}", x.len());

}
