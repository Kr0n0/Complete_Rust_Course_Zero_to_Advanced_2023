#![allow(dead_code)]

// BEGINNER -> INTERMEDIATE LESSONS
// ================================
fn tuples_example() {
    let tuple:(i8, f32, i32) = (2, 3.4, 100);
    // All values
    println!("{:?}", tuple);
    // Access the first one
    println!("The first value is {:?}", tuple.0);
}
fn tuples_exercise() {
    /*
        1. Create a function user_data which takes the tuple x as a parameter containing a 
        signed integer 32 bits, a boolean, and a string literal (hint: use the &str keyword to point
        the reference)mf-install.sh
        2. In the function assign a tuple to distinct variables by naming the integer: age, 
        boolean: active, and string literal: name. BIG HINT: let (integer, bool, and string) = x
        3. Write instructions in the function to print the age, active status and name
        4. In the main function, create a new tuple, user2 and set the user data so that the user is
        30 years old, active status is true, and his name is Jack
        5. Invoke the user_data passing in user2 as the agrument and check the console for your result
    */

    fn user_data (x: (i32, bool, &str)) {
        /*
            Manera clásica:
            let age: i32 = x.0;
            let active: bool = x.1;
            let name: &str = x.2;
        */
        let (age, active, name) = x;

        println!("age: {}", age);
        println!("active: {}", active);
        println!("name: {}", name);
    }

    // No necesita ser mutable, solo la definimos inicialmente y luego establecemos el valor
    let user2: (i32, bool, &str);
    user2 = (30, true, "Jack");
    user_data(user2);
}
fn arrays_example() {
    // Manera 1 de declararlo, con el tipo
    //let arr: [&str; 4] = ["Jerry", "George", "Elaine", "Kramer"];
    
    // Manera 2 de declararlo, que Rust infiera el tipo. Ponemos mut si queremos modificarlo
    let mut arr = ["Jerry", "George", "Elaine", "Kramer"];

    arr[2] = "Elaine Benis";

    // Igual que las tuplas, para mostrarlo con la macro de println! es necesario poner {:?}
    println!("the cast of Seinfeld consist of {:?}", arr);
    println!("The array's total length is {}", arr.len());

    // Iter function
    for value in arr.iter() {
        println!("The value is {}", value);
    }
}
fn arrays_exercise() {
    /*
        1. Create an array with the following integer values: 12,2,3,2,4,5
        2. Write a for loop with can loop through the array and replace all the integer values of 2 with 0.
        Print the amended results tagged along with the index value of each integer.
    */
    let mut array_exercise: [i32; 6] = [12, 2, 3, 2, 4, 5];    
    println!("Beginning array : {:?}", array_exercise);

    for i in 0..array_exercise.len() {
        if array_exercise[i] == 2 {
            array_exercise[i] = 0;
        }
        println!("{i} - {}", array_exercise[i]);
    }
}
fn ownership_example() {

    // Vector -> vector1 own the object in the heap
    // only a single variable own the heap memory at a given time
    let vector1: Vec<i32> = vec![2, 4, 6];

    let vector2: Vec<i32> = vector1;
    // Rust is going to check for memory access and that's really a key
    // selling point for Rust.
    // This works OK
    println!("{:?}", vector2);
    // This not works, because the ownership has changed to vector2.
    // vector1 is not occupying that space, it's actually vector2
    //println!("{:?}", vector1);
}
fn borrowing_example() {
    /*
        Ownership of a variable -> function -> return ownership => Dificil
        Funciones permiten transferir el ownership a otra funcion temporalmente (borrowing)
        Se puede pasar una referencia a la variable con (& variable), en vez de pasar la variable
        de manera directa.
        El ownership de la variable / valor se transfiere al owner original de la variable.
        No se crea un puntero nuevo que apunta a la zona de memoria, sino que se referencia a la memoria ya existente.s
    */
    fn display (x: &Vec<i32>) {
        println!("{:?}", x);
    }
    let vector: Vec<i32> = vec![1,2,3];
    //display(vector); // <- Aquí ya estamos moviendo el ownership a display.
    //println!("{}", vector[1]); // <- Aqui fallaria porque el ownership del vector está en display.

    display(&vector); // <- Este si funciona porque estamos pasando la referencia de la dirección de memoria.
    println!("{:?}", vector[1]); // <- Esto ok porque hemos recuperado el ownership al pasarlo como referencia.
}
fn borrowing_exercise() {
    /*
        Create a function called display2 witch takes a string object as a parameter and pushes onto
        the string an "F8 Tributo" which is an awesome type of Ferrari!

        Create variable car which is a string object with the default "Ferrari". Call display2 passing in the car
        variable as the argument and print the modified version of the variable car.
    */

    /*  No es necesario recoger valores de vuelta porque pasamos la referencia y listo, el objeto se cambia en memoria
        Es necesario pasarle el mut para que sea modificable.
    
    */

    fn display2(_car: &mut String) {
        println!("car value is: {}", _car);
        _car.push_str("F8 Tributo");
    }

    let mut car: String = String::from("Ferrari");
    display2(&mut car);
    println!("the car has been updated to: {}", car);

}
fn borrowing_exercise_myself() {
    /*
        Create a function called display2 witch takes a string object as a parameter and pushes onto
        the string an "F8 Tributo" which is an awesome type of Ferrari!

        Create variable car which is a string object with the default "Ferrari". Call display2 passing in the car
        variable as the argument and print the modified version of the variable car.
    */

    fn display2(_car: &mut String) {
        println!("Original: {}", _car);
        _car.push_str("F8 Tributo");
    }

    let mut car: String = String::from("Ferrari");
    display2(&mut car);
    println!("New: {}", car);
}
fn slices_example() {
    /*
        Slices referencian a una secuencia contigua de datos. Son referencias, no tienen ownership.
        Son como punteros a los datos.
    */

    let game: String = "Mario Brothers".to_string();
    println!("length of the game is {}", game.len());

    // Slice just Mario -> 0..5 
    let slice: &str = &game[0..5];
    println!("{}", slice);
}
fn slices_exercise() {
    /*
        1. Create an array with the values 1 through 5 called nums
        2. Print the value of nums.
        3. Write a function called slice_and_dice which takes an array as a parameter and prints the length of the array.
        It should also print the updated value of the array. This function also replace the first index value in the array to 200.
        4. Call the slice and dice function in the main function and slice out the values 2 and 3 from the array.
    */

    fn slice_and_dice (_nums: &mut [i32; 5]) {
        println!("Array length: {}", _nums.len());
        _nums[0] = 200;
        println!("Updated values: {:?}", _nums);
        let slice: &[i32] = &_nums[1..3];
        println!("Sliced values 2 and 3: {:?}", slice);
    }

    let mut nums: [i32; 5] = [1,2,3,4,5];
    println!("Original values: {:?}", nums);
    slice_and_dice(&mut nums);
}
fn structs_example() {
    /*
        Structs are key/value pairs (como diccionarios en Python)
        No es necesario saber el orden como las tuplas, solo con el key accedemos al value.
        Con los nombres podemos hacer métodos y estructuras complejas en Rust.
        Podemos asociar métodos para la estructura directamente para manejarla!
        struct con nombre con letra mayuscula
    */

    // Blueprint of a House
    struct House {

        door: String,
        garden: String,
        property_value: u32,

    /* TODO Revisar esto
    impl House {

        fn echo (&self) {
            prinln!("Method called.");
        }
    }
     */
    }

    // Currently a House struct variable
    let home1: House = House {
        door: String::from("Blue"),
        garden: String::from("Beautiful"),
        property_value: 1000000
    };

    println!("door is {}, garden is {}, property value is {}", home1.door, home1.garden, home1.property_value);

    //Methods




}
fn structs_exercise() {

        /*
            Write a Triangle Calculator
            1. Write a Triangle strcut that takes the key pair vaules of base and height as unsigned integers.
            2. Write a method for the Triangle which can calculate the area of the triangle (remember the triangle area formula is base * height / 2)
            3. Create a new triangle struct and set the base to 10 and the height to 30
            4. Print the method calculation for the area of the new triangle.
        
        */

        // Blueprint of structure Triangle
        struct Triangle {
            base: u16,
            height: u16
        }

        // Implementation Methods for Triangle
        impl Triangle {
            
            // Area method
            fn area (&self) -> u16 {
                return (self.base * self.height) / 2;
            }
        
        }

        let triangle: Triangle = Triangle {
            base: 10,
            height: 30
        };

        println!("Area calculated: {}", triangle.area());




}
fn structs_exercise_myself() {

    /*
                Write a Triangle Calculator
                1. Write a Triangle strcut that takes the key pair vaules of base and height as unsigned integers.
                2. Write a method for the Triangle which can calculate the area of the triangle (remember the triangle area formula is base * height / 2)
                3. Create a new triangle struct and set the base to 10 and the height to 30
                4. Print the method calculation for the area of the new triangle.
            
    */

    // Define blueprint struct
    struct Triangle {
        base: u16,
        height: u16
    }

    // Define methods for the TAD
    impl Triangle {
        fn area(&self) -> u16 {
            return (self.base * self.height) / 2;
        }
    }

    // New triangle
    let triangle: Triangle = Triangle {
        base: 10,
        height: 30
    };

    println!("The area actually is {}", triangle.area());
}
fn enums_example() {
    // the derive attribute makes the enum printable

    #[derive(Debug)]
    enum TemperatureGrade {
        Hot, Cold, Medium
    }

    #[derive(Debug)]
    struct City {
        name: String,
        temperature: TemperatureGrade
    }

    let c1: City = City {
        // No hace falta definir el tipo de name como String porque ya está definido anteriormente
        //name: String = String::from("Alaska"),
        name:  String::from("Alaska"),
        temperature: TemperatureGrade::Cold
    };

    let c2: City = City { 
        // No hace falta definir el tipo de name como String porque ya está definido anteriormente
        //name: String = String::from("Alaska"),
        name: String::from("Miami"),
        temperature: TemperatureGrade::Hot
    };

    println!("{:?}", c1);
    println!("{:?}", c2);



}
fn enums_exercise() {
    /*
        Matching Enums in Rust Exercise

        1. Write an Enum for three different types of shores: loafers, nike and vans.
        2. Using the match keyword, write a function that takes the enum as a value and can match
        each shoe with the following printed string values:
            nike: "Great for running"
            loafers: "Great for loafing around"
            vans: "Great for skateboarding"
        3. Call the function and print each string match by passing in each enum value and printing the matched result
    */

    enum Shoes {Loafer, Nike, Vans}

    fn shoes_msg (_shoe: Shoes) {
        match _shoe {
            Shoes::Loafer => {
                println!("Great for loafing around!");
            },

            Shoes::Nike => {
                println!("Great for running!");
            },

            Shoes::Vans => {
                println!("Great for skateboarding!");
            }
        }
    }

    let shoe: Shoes = Shoes::Nike;
    shoes_msg(shoe);
}
fn modules_example() {

    // Module definition
    pub mod songs {
        pub fn play(name: String) {
            println!("Track selection: {}", name);
        }
    }

    use songs::play;

    play("Kissed by a rose".to_string());

}
fn modules_exercice() {

    /*
    Create a module named tracks that contains a module named rock that contains another module
    indie which contains a function called select which takes a String object as a parameter and 
    prints the string

    Import select and in the main function call select three times printing the following song titles:
    "Serenade", "French Baguette" and "Pineablle Blues"
    */

    pub mod tracks {
        pub mod rock {
            pub mod indie {
                pub fn select (_name : String) {
                    println!("{}", _name);
                }
            }
        }
    }

    use tracks::rock::indie::select;

    select("Serenade".to_string());
    select("French Baguette".to_string());
    select("Pineapple Blues".to_string());

}
fn hashmaps_example() {

    use std::collections::HashMap;

    let mut account_info: HashMap<&str, &str> = HashMap::new();
    account_info.insert("Johnny", "Overdraft!");
    account_info.insert("Sally", "Good Standing!");
    account_info.insert("Superman", "Insufficient funds!");

    println!("the size of the map is {}", account_info.len());


}
fn hashmaps_exercice() {
    /*
    1. Create a new HashMap instance variable bar_drinks and using the insert method
    add three new k/v pairs to your table: vodka, beer and whiskey. Values -> the bar has some, while
    beer has all run out

    2. Remove whiskey from the local table
    */

    use std::collections::HashMap;

    let mut bar_drinks: HashMap<&str, bool> = HashMap::new();
    bar_drinks.insert("whiskey", true);
    bar_drinks.insert("vodka", true);
    bar_drinks.insert("beer", false);

    println!("Bar Drinks HashMap : {:?}", bar_drinks);
    println!("Bar Drinks Size: {}", bar_drinks.len());

    bar_drinks.remove(&"whiskey");

    println!("{:?}", bar_drinks);

}

// REDOS
fn tuples_exercice_redo() {

    fn user_data (x: (i32, bool, String)) {        
    
        //let age: i32 = x.0;
        //let active: bool = x.1;
        //let name: String = x.2;

        let (age, active, name) = x;

        println!("The age is {age}");
        println!("The active status is {active}");
        println!("The name is {name}");
    }

    let user2:(i32, bool, String) = (30, true, "Jack".to_string());
    
    user_data(user2);

}
fn arrays_exercice_redo() {

    let mut arr: [i32; 6] = [12,2,3,2,4,5];

    println!("Before : {:?}", arr);

    for i in 0..arr.len() {
        if arr[i] == 2 {
            arr[i] = 0;
        }
    }

    println!("After: {:?}", arr);
}
fn borrowing_exercise_redo() {

    fn display2(_car : &mut String) {
        _car.push_str("F8 Tributo");
    }

    let mut car: String = "Ferrari".to_string();

    println!("Car before: {}", car);
    display2(&mut car);
    println!("Car after: {}", car);
}
fn slices_exercise_redo() {

    fn slice_and_dice(_array: &mut[i32;5]) {
        // Length of the array
        println!("Length of the array : {}", _array.len());
        // Replace first element
        _array[0] = 200;
        println!("Array : {:?}", _array);
        // Slice values 2 and 3
        let sliced_values: &[i32];
        sliced_values = &_array[1..3];
        println!("Sliced values: {:?}", sliced_values);
    }

    let mut nums:[i32; 5] = [1,2,3,4,5];
    for i in 0..nums.len() {
        println!("Array value: {}", nums[i]);
    }
    slice_and_dice(&mut nums);
}
fn structs_exercice_redo() {

    struct Triangle {base: u32, height: u32}

    impl Triangle {
        fn area (&self) -> u32 {
            return (&self.base * &self.height)/2;
        }
    }

    let triangle: Triangle = Triangle {base: 10, height: 30};
    println!("Triangle is {} base and {} height", triangle.base, triangle.height);
    println!("The area is : {}", triangle.area());
}
fn enums_exercise_redo() {

    enum Shoes {Loafer, Nike, Vans}

    fn shoes_msg (_shoe: Shoes) {
        match _shoe {
            Shoes::Loafer => {
                println!("Great for loafing around!");
            },

            Shoes::Nike => {
                println!("Great for running!");
            },

            Shoes::Vans => {
                println!("Great for skateboarding!");
            }
        }
    }

    let shoe: Shoes = Shoes::Nike;
    shoes_msg(shoe);
}


fn main() {

    // BEGINNER -> INTERMEDIATE LESSONS
    // ================================

    //tuples_example();                     // 58
    //tuples_exercise();                    // 59
    //arrays_example();                     // 60
    //arrays_exercise();                    // 61
    //ownership_example();                  // 64
    //borrowing_example();                  // 65
    //borrowing_exercise();                 // 66
    //slices_example();                     // 67
    //slices_exercise();                    // 68
    //structs_example();                    // 71
    //structs_exercise();                   // 72
    //structs_exercise_myself();            // 72
    //enums_example();                      // 73
    //enums_exercise();                     // 75
    //modules_example();                    // 77
    //modules_exercice();                     // 78
    //hashmaps_example();                     // 80
    hashmaps_exercice();

    // REDO
    // ====

    //tuples_exercice_redo();
    //arrays_exercice_redo();
    //borrowing_exercise_myself();       
    //borrowing_exercise_redo();
    //slices_exercise_redo();
    //structs_exercice_redo();
    //enums_exercise_redo();
    
    print!("\n");
}
