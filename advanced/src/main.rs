#![allow(dead_code)]

// INTERMEDIATE TO ADVANCED
// ========================

fn error_handling_example() {

    // panic!
    //panic!("This will cause the program to abruptly end");
    
    // expect with message
    //use std::fs::File;
    //let f = File::open("doestnotexist.txt").expect("No such thing!");

    // unwrap
    //use std::fs::File;
    //let f = File::open("doestnotexist.txt").unwrap();
    //println!("The program is working");
}
fn error_handling_exercise() {

    /*
        1. Write a function is_seven which checks whether or not the input is the number 7 and returns true if so
        and an error ie (return Err("...")) if it is not true. If it is true return Ok(?) - these variants Err and Ok 
        come from the Enum Result. The function can return multiple datatypes.

        2. Create a variable 'solution' in the main function which is assigned calling the is_seven function testing
        various inputs.

        3. Print the solution variable in the program.
    */

    fn is_seven(number: u32) -> Result <bool, String> {
        if number == 7 {
            return Ok(true);
        }
        else {
            return Err("The number is not 7!".to_string());
        }
    }

    let solution: bool = is_seven(9).unwrap();
    println!("Solution #1 - {}", solution);
    
}
fn generic_data_types_example() {

    // Generic struct with <T>

    struct GenericStruct <T> {
        value: T
    }

    let t1: GenericStruct<i32> = GenericStruct {value: 100};
    println!("The value is {}", t1.value);
    let t2: GenericStruct<String> = GenericStruct {value: "String Data Type".to_string()};
    println!("The value is {}", t2.value);
}
fn traits_example () {

    // Basic structure
    struct Game {
        weapon: &'static str,   // Lifetime value -> Static
        power_level: u32
    }

    // Declaracion del trait (abstracto)
    trait Stats {
        fn character_stats(&self);
    }
    // Implementación para Game (concreto)
    impl Stats for Game {
        fn character_stats(&self) {
            println!("Printing stats of power level: {}, weapon: {}", 
                    self.power_level, self.weapon);
        }
    }

    // Main
    let g1: Game = Game {
        weapon: "Sword of Fire",
        power_level: 100
    };

    g1.character_stats();


}
fn reader_writer_io_example_1 () {
    use std::fs::File;

    let file_name = "cat.txt";
    
    // Create the file -> std::fs::File::create(path)
    match File::create(file_name) {
        // 1 - Ok
        Ok(file) => println!("Writing {:?}", file),
        // 2 - Err
        Err(_) => println!("Unable to create the file {}", file_name)
    }

    // Open the file -> std::fs::File::open(path)
    match File::open(file_name) {
        // 1 - Ok
        Ok(file) => println!("Reading {:?}", file),
        // 2 - Err
        Err(_) => println!("Unable to open the file {}", file_name)
    }

}
fn reader_writer_io_example_2 () -> Result <(), std::io::Error> {

    // Read the file using BufReader
    use std::fs::File;
    use std::io::{Read, BufReader};

    let file_name = "cat.txt";
    let file = File::open(file_name)?; // Open the file
    let mut reader = BufReader::new(file); // Open the BufReader pointer from file
    let mut contents: String = String::new();   // String object for the content

    reader.read_to_string(&mut contents)?;
    println!("The contents of the file are: {}", contents);

    // Returns Ok
    Ok(())

}
fn iterators_example() {

    let x: [i32; 3] = [1,2,3];
    let iter = x.iter();
    
    // iter
    for items in iter {
        println!("{}\t", items)
    }

    // into_iter()
    // Moving the collection into an iterative object (borrowing)
    let values: Vec<&str> = vec!["a", "b", "c"];
    // Moving values into value -> Borrowing
    for value in values.into_iter() {
        match value {
            "c" => println!("c is a good time"),
            _ => println!("iteration: {}", value)
        }
    }
    //println!("{}", values) // <- This cannot work, because values doens't have the ownership!!


}
fn iterators_exercise() {

    /*
    Iterate through a Vector

    Write a vector called pets which contains the 3 string items cat, dog and goldfish.
    Use the for with iter_mut() signature to match dog so that the program prints "cute doggy!""
    The default should print "hello!" and earch pet name for each iteration
    */

    // iter_mut -> mutable iteration 
    let mut pets: Vec<&str> = vec!["cat", "dog", "goldfish"];

    for pet in pets.iter_mut() {
        match pet {
            &mut "dog" => println!("cute doggy!"),
            _ => println!("hello! {}", pet)
        }
    }
    println!("{:?}", pets);         // <= Borrowed, it works
}
fn closures_example() {

    let is_even = |n| {n%2==0};

    let num = 11;
    println!("{} is even? {}", num, is_even(num));

}
fn smart_pointers_example() {

    // Box and dereferencing with the unary * symbol

    let greeting: &str = "hello";
    let greeting_heap: Box <&str> = Box::new(greeting);     // Points to a new variable in the heap

    println!("{}","hello"==greeting);       // Print the return value of the conditional
    println!("{}","hello"==*greeting_heap);       // Access the value in the heap

}
fn custom_smart_pointer_example() {

    use std::ops::Deref;

    // Custom Struct -> T porque puede ser cualquier tipo de datos
    struct CustomSmartPointer<T> (T);

    // HEAP ALLOCATION (NEW)
    impl <T> CustomSmartPointer<T> {
        // generic structure with static method

        // same as new from Box
        fn heap_allocation(value:T) -> CustomSmartPointer<T> {
            CustomSmartPointer(value)
        }
    }

    // DEREFERENCING (SHOW CONTAINED ELEMENTS IN HEAP)
    impl <T> Deref for CustomSmartPointer<T> {
        type Target = T; // in traits, type is used to declare an associated type
        fn deref(&self) -> &Self::Target {
            &self.0        // syntax for taking the first argument which is 0
        }
    }

    let color = "green";
    let ref_color = CustomSmartPointer::heap_allocation(color);
    // Call static method

    println!("green value is the same as color which is {}", "green"==color);       //Stack
    println!("green value is the same as color which is {}", "green"==*ref_color);  //Heap

}
fn custom_smart_pointer_exercise() {

    /*
        Customize your own Smart Pointer

        You can achieve automatic memory deallocation using Drop trait

        1. Implement the Drop trait for our Custom Pointer.
        It should include a function called specifically drop and print in the function a completion run of the 
        drop object from memory.

        std::ops::Drop
        when a value gets out of scope you can rum some code also known as a destructor

    */

    use std::ops::Deref;

    // Custom Struct -> T porque puede ser cualquier tipo de datos
    struct CustomSmartPointer<T> (T);

    // HEAP ALLOCATION (NEW)
    impl <T> CustomSmartPointer<T> {
        // generic structure with static method

        /*
        The "heap_allocation" method, which acts like the "new" operator in other languages, 
        returns a new instance of the custom smart pointer by allocating memory on the heap.
         */
        // same as new from Box
        fn new(value:T) -> CustomSmartPointer<T> {
            /* Returns New instance of CustomSmartPointer with the value
            The value is stored inside the custom smart pointer, which is created on the 
            heap using Rust's standard heap allocation mechanism.
            */
            CustomSmartPointer(value)
        }
    }

    // DEREFERENCING (SHOW CONTAINED ELEMENTS IN HEAP)
    /*
    The "Deref" trait enables the custom smart pointer to be dereferenced (i.e., access its 
    underlying value) using the * operator.
     */
    impl <T> Deref for CustomSmartPointer<T> {
        type Target = T; // in traits, type is used to declare an associated type
        fn deref(&self) -> &Self::Target {
            &self.0        // syntax for taking the first argument which is 0
        }
    }

    // DROP -> EXERCISE
    /*
    The "Drop" trait allows the custom smart pointer to free up memory when it goes out of scope by running 
    code in its "drop" method, which in this case just prints "Memory freed!".
    */
    impl <T> Drop for CustomSmartPointer<T> {
        fn drop(&mut self) {
            println!("Memory freed!")
        }
    }

    let color = "green";
    /* When the function is called, a string "green" is allocated on the stack and then passed to the 
    "heap_allocation" method, which returns a custom smart pointer that holds a reference to the string.
     */
    let ref_color = CustomSmartPointer::new(color);
    // Call static method
    println!("green value is the same as color which is {}", "green"==*ref_color);  //Heap
    println!("green value is the same as color which is {}", "green"==color);       //Stack
    /* When the custom smart pointer goes out of scope, its "drop" method is called, freeing up 
    the memory that it was using.
     */


}
fn multithread_example() {

    use std::thread;
    use std::time::Duration;

    let park_thread = thread::Builder::new().spawn(
        ||
            {
                println!("Parking Thread!");
                thread::park();
                println!("Unpark Thread!");
            }
    ).unwrap();     // Gimme the results and if something happens, panic!

    thread::sleep(Duration::from_millis(10)); // Sleeps for 10 milisecs
    println!("Unpark the Thread");
    park_thread.thread().unpark();
    park_thread.join().unwrap(); // Waits for the associated thread to finish

}
fn multithread_exercise() {

    /*
        1. Create two definite loops in the main function.

        2. One loop should be set to a spawn thread x which runs from 1 to 20 with a sleep duration
        of half the time of the main thread loop. The spawn thread should also print the index values
        of each iteration as it loops til completion as should the main thread loop.

        3. The other loop should run in the main thread with a sleep duration of twice the time of the spawn
        threaded loop.
        The loop should run only from 1 to 5 and should also print each index valued tagged as the main thread.

        4. Ensure the spawn thread has a full completion of its run even if the main thread finishes
    */

    use std::thread;
    use std::time::Duration;

    let x = thread::Builder::new().spawn( || 
        {
            let _thread_time: Duration = Duration::from_millis(100/2);

            // Loop from 1 to 20 with sleep 1/2 time of the main loop
            for i in 1..20 {
                println!("x - number {:?}", i);
                thread::sleep(_thread_time);
            }
        }).unwrap();

    let thread_time: Duration = Duration::from_millis(100);

    for i in 1..5 {
        println!("main - number {:?}", i);
        thread::sleep(thread_time);
    }
    x.join().unwrap();
}

fn main() {
    //error_handling_example();             // 83
    //error_handling_exercise();            // 84
    //generic_data_types_example();         // 86
    //traits_example();                     // 87
    //reader_writer_io_example_1();           // 88
    //let return_value: Result<(), std::io::Error>  = reader_writer_io_example_2();           // 89
    //iterators_example();                    // 92
    //iterators_exercise();
    //closures_example();                     // 96
    //smart_pointers_example();                   //97
    //custom_smart_pointer_example();             // 98
    //custom_smart_pointer_exercise();            // 99
    //multithread_example();                      //101
    //multithread_exercise();
}
