// Premite no poner warnings para dead code (ejercicios)
#![allow(dead_code)]

fn print_variables() {
    let switch = false;
    let volume: u32 = 10;
    //let unsigned = -10;


    println!("Switch is {}", switch);
    println!("Volume is {}", volume);
    println!("\n");
    println!("The switch is in {} state and the volume is in {} state", switch, volume);

    println!("{param1} - {param2} - {param3}", param1="Hola", param2=30, param3=true);

    println!("{hello_str} {salutation}", hello_str="hello", salutation="my friend");
    eprintln!("ERROR");
}
fn integers_overflows() {

    let total = 4;
    let height: u32 = 41;
    let deduction: i32 = 2-200;
    let max_range_16: u16 = 65535;
    let overtime_1: u16 = max_range_16 + 1;
    let overtime_2: u16 = max_range_16 + 2;

    println!("The total is {total}");
    println!("The height is {} and the deduction is {}", height, deduction);

    println!("max_range_16 : {max_range_16}");
    println!("overtime_1: {overtime_1}");
    println!("overtime_2: {overtime_2}");
}
fn mutable_variables() {
    /*
    No va funcionar, no estan mut, por lo que no podemos usarlas.
    let y: u16 = 12;
    y = 15;
     */

    let mut y: u16 = 12;
    println!("Value of y: {}", y);
    y = 15;
    println!("Value of y: {}", y);
}
fn overshadow_variables() {
    let x : u16 = 5;
    println!("{}", x);
    // Overshadow, creamos una nueva variable con el mismo nombre con otro valor
    let x : u16 = 6;
    println!("{}", x);
}
fn overshadow_constants() {
    // Constantes en mayusculas siempre
    const EXAMPLE : u16 = 25;
    println!("Constant: {}", EXAMPLE);
    /* 
    Esto no funciona porque la constante ya esta definida, no se puede hacer
    overshadow en constantes

    const EXAMPLE : u16 = 30;
    println!("Constant: {}", EXAMPLE);
    */
}
fn strings_literal() {
    // str literal -> Modulo std::str
    // Estáticas por defecto, se asegura que va a ser válida para toda la duración del programa.
    // Se pueden declarar estáticas también.
    //let greeting: &str = "Hello world!";
    //println!("{greeting}");

    let bank: &str = "Citi Bank";
    let currency : &str = "Bitcoin";
    println!("The bank is {} and the currency is {}", bank, currency);

    // Definiendo como estáticas explícitamente con &'static TIPO
    let bank2: &'static str = "Sky";
    let currency2 : &'static str = "Ethereum";
    println!("The bank is {} and the currency is {}", bank2, currency2);

}
fn strings_object() {
    /*  El objeto String no forma parte de std. Es mutable, usa UTF-8.
        Se usa para cadenas de texto que puedan ser solicitadas en runtime.
        Los objetos String se guardan en el Heap (dinámicos), después de las variables estáticas,
        crecen hacia las direcciones altas.
     */

    // Nuevo objeto String sin nada mas (tamaño 0)
     let nothing_within: String = String::new();
     // len() finds the length value of a string
     println!("{}", nothing_within.len());

    // Nuevo tipo String con 16 caracteres usando from 
    let great_movie: String = String::from("The Big Lebowski");
    println!("{}", great_movie.len());

    // push_str() -> Añade el caracter al final de la cadena
    let mut greeting: String = String::from("Julia says, ");
    greeting.push_str("hello");
    println!("{}", greeting);

    // to_string() -> Convierte un std::str a un objeto String
    let random_string: String = "Please make me into an object!".to_string();
    println!("{}", random_string);
}
fn strings_manipulation() {
    // 1. Create a string literal named password and assign it the value of "pokemon,"
    // 2. Using the String Object push method, modify the password so that it inclues " gotta catch them all"
    // 3. Print the result
    //let mut password: String = String::from("pokemon,");
    let mut password = "pokemon,".to_string();
    password.push_str(" gotta catch them all");
    println!("{}", password);
}
fn decision_making() {

    // No son necesarios los parentesis si no hay combinatoria en condicional
    // if
    let user: &str = "todd";
    
    if user.len() == 4 {
        println!("Pass");
    }

    // if..else
    let user2 : &str = "fred";
    if user2.len() == 3 {
        println!("Pass");
    } 
    else {
        println!("Fail!")
    }

    // if..else..if anidados
    let password: &str = "sunday";
    if password.len() > 3 {
        println!("Thank you for creating a password!");
    }
    else if password.len() > 2 {
        println!("Please add at lease one more char to your password.");
    }
    else {
        println!("The password is too short. Please make a bigger password.");
    }

    // Match -> Igual que switch en C, se comprueba la variable con una lista de valores.
    let microbiome: &str = "xc12";
    let body_part: &str = match microbiome {
        "xc12" =>   { 
                        println!("Found match for microbiome!"); 
                        "Tummy Biome" 
                    },
        "mpt1" =>   "Eyebniome", 
        "ttw6" =>   "Fingerbiome",
        _ =>        "Unknown" // default
    };

    println!("The biome match is {}", body_part);
}
fn decision_making_exercise() {
    /*
    3. If the check passes print into the console the following string: "fail"
    4. If the check provides a false boolean conduct the following tests:
        check if x is less than y or x is greater than 6
    If the second test passes print the following string into the console: "success"
    If the second test fails print the following string into the console: "please try again"
    */

    // 1. Write two signed 32 bit constants x and y and assign x the value 3 and y the value 4
    const X: i32 = 3;
    const Y: i32 = 4;

    // 2. Check to see whether or not the x is less than y and x is greater than 6
    if (X < Y) && (X > 6) {
        println!("fail");
    }
    else {
        if (X < Y) || (X > 6) {
            println!("success");
        }
        else {
            println!("please try again");
        }
    }
}
fn loops() {
    
    // for - Definite loop
    for a in 1..20 { // 20 not included
        // Skips 2
        if a == 2 {
            continue;
        }
        println!("{}", a);
    }

    // while
    let mut b: u16 = 0;
    while b <= 5 {      
        b += 1;
        println!("loop b value is {}", b);
    }

    // loop
    let mut c: i16 = 0;
    loop {
        c -= 1;
        println!("c = {}", c);

        if c == -10 {
            // Breaks the loop
            break;
        }
    }
}
fn loops_exercise() {
    /*
        1. Create a unsigned mutable variable called count of 32 bits
        2. Write an infinite loop that increments counts +1 and stores the value in count and has the following conditions:
            3. If count is equal to 3 then print the string literal "welcome to miami!" in the console
            4. If the count equals 5 then print the string literal "Time to call it a dayt!" and then exit the loop.
    */

    let mut count: u32 = 0;

    loop {
        count += 1;
        if count == 3 {
            println!("welcome to miami!");
        }
        println!("{}", count);
        if count == 5 {
            println!("Time to call it a day!");
            break;
        }    
    }
}
fn function_example(x: i32) -> bool {
    println!("The value of x is {}", x);
    return true;
}
fn function_exercise() {
    /*
        Write a function called plus_one that takes a signed integer 32 as a parameter and returns the integer added with the value of 1

        In a main function create a variable y which is stores the result of invoking the plus_one function with the argument of 5 and print in
        the main function the result of y.  
    */

    fn plus_one(y: i32) -> i32 {
        return y + 1;
    }
    
    let y: i32 = plus_one(5);
    println!("result : {}", y);

}

fn main() {
    println!("\n");
    //print_variables();            // Initial ones
    //integers_overflows();         // 1.27
    //mutable_variables();          // 1.30
    //overshadow_variables();       // 1.31
    //overshadow_constants();       // 1.32
    //strings_literal();            // 1.33
    //strings_object();             // 1.34, 1.35
    //strings_manipulation();       // 1.36
    //decision_making();            // 1.42
    //decision_making_exercise();   // 1.45
    //loops();                      // 1.47
    //loops_exercise();             // 1.51
    /*
    {
    let mut return_value: bool = false;
    return_value = function_example(10);                  // 1.52
    }
    */
    // function_exercise();             // 11.55
    println!("\n");
    
}
