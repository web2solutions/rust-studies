fn make_mutable () {
    // transformar em mutavel - make mutable
    let mut x = 5;
    println!("The value of x  is {}", x);

    x = 6;

    println!("The value of x  is {}", x);
}

fn shadow () {
    // Sombreamento - shadowing
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is {}", x)
}

fn main() {
    
    make_mutable();

    shadow();

    
}
