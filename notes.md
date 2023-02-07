# Notes about Complete Rust Course - Zero to Advanced 2023

Personal notes for the course "Complete Rust Course - Zero To Advanced 2023" on Udemy by Clarian North.

Udemy Course: https://www.udemy.com/course/complete-rust-course-zero-to-master/

## Static
Static variable -> Persist over all program
```rust
weapon: &'static str //Lifetime value -> Static
```

## Loops
```rust
for i in [0..10] {
    ...
}
```

## Functions
```rust
// General functions
fn area (x: &mut i32 ) -> u32 {
            return (&self.base * &self.height)/2;
        }
// Struct methods (impl) -> &self
fn area (&self) -> u32 {
            return (&self.base * &self.height)/2;
        }
```

## Match
```rust
    match var {
        Value1 => {
            ...
        },

        Value2 => {
            ...
        },

        _ => {

        }
    }

```


## Tuples

```rust
// Definition
let tuple:(i8, f32, i32) = (2, 3.4, 100);
// Element access
println!("The first value is {:?}", tuple.0);
// Full asignment
let (age, active, name) = x;
```

## Arrays

```rust
// Definition - name [type; size]
let arr: [i32; 6] = [12,2,3,2,4,5];
// Iterate the array by position
for i in 0..arr.len() {
    // arr[i] = ...
    }
}
// Iterate the array by iterator
for value in arr.iter() {
    println!("value = {}", value);
}
```

## Borrowing

```rust
// Variable reference (&)
fn fn_example(_example: &mut String){
    _car.push_str("more");
}
let mut example: String = "Example".to_string();
fn_example(&mut example);
``` 

## Slicing on arrays

Slices are pointers to data. They are used with references (&).

```rust
let mut ejemplo:[i32; 5] = [1,2,3,4,5];
let sliced_values: &[i32];
sliced_values = &ejemplo[1..3]; // Valores 2 y 3
```

## Structs
```rust
struct Triangle {base: u32, height: u32}
impl Triangle {
    fn example (&self) -> return_value {
        return ("...");
    }
}
// Create struct
let triangle: Triangle = Triangle {base: 10, height: 30};
// Call data and methods
println!("{}", triangle.base);
println!("{}", triangle.example());
```

## Enums

```rust
// Definition
enum Shoes {Loafer, Nike, Vans}
// Variable creation (::)
let shoe: Shoes = Shoes::Nike;
```

## Iterators
Iterators iterate over a different collection of values from types such as arrays, vectors, maps, etc.


### Iterator trait (iter)
The Iterator trait is invoked from iterators defined in the Rust standard library.

- iter() -> returns an iterator object of the collection of items.
- next() -> traverses through items and returns none once it reachs the end of items.

```rust
let x: [i32; 3] = [1,2,3];
let iter = x.iter();

for items in iter {
    println!("{}\t", items)
```

Note the borrowing concept here.

### into_iter

The ```into_iter``` method moves values in the collection into an inter object via ownership.

```rust
let values: Vec<&str> = vec!["a", "b", "c"];
for value in values.into_iter() {
    match value {
        "c" => println!("c is a good time"),
        _ => println!("iteration: {}", value)
    }
}
//println!("{}", values)  <- This cannot work, because values doens't have the ownership!!
```

### iter_mut()

The iter_mut it's used for mutable values.

```rust
// iter_mut -> mutable iteration 
let mut pets: Vec<&str> = vec!["cat", "dog", "goldfish"];

for pet in pets.iter_mut() {
    match pet {
        &mut "dog" => println!("cute doggy!"),
        _ => println!("hello! {}", pet)
    }
}
println!("{:?}", pets);         // <= Borrowed, it works
```

## Modules
Colection of multiple elements (functions, structs, traits, impl... or even other modules).
Can be used to manage visibility (public/private)

Multiple modules -> Crate

- Crate -> Compiling unit in Rust to binary/library
- Cargo -> Management tool for crates
- Module -> Collection of items
- crates.io -> Rust packages registry

Also:
- The ```use``` keyword import a module
- Functions are default to ```private```

```rust
// Module declaration
pub mod example {
    pub fn public_function(name: String) {}
    fn _private_function(name: String) {}
}
// Import module
use example:public_function
// Public function use
public_function("Example".to_string());
```

## HashMaps

HashMap<K, V>
Hashmaps are key/value pairs ordered by a hashing function (fast).

```std::collections::HashMap```

Methods:
- insert()
- remove(&)
- len()
- get()
- iter()
- contains_key()

```rust
use std::collections::HashMap;

let mut account_info: HashMap<&str, &str> = HashMap::new();

account_info.insert("Johnny", "Overdraft!");
account_info.remove(&"Johnny");
```

## Error handling

```Enum: OK, Err```

Result is a type that represents either success (Ok) or failure (Err). It's used on return values of functions with Ok (bool), Err (String)

```rust
// Data types for Result -> bool (for Ok) and String (for Err)

fn check_error(...) -> Result <bool, String> {
    if (..) {
        return Ok(true);
    }
    else {
        return Err("Error".to_string());
    }
}

let variable: bool = check_error(..).unwrap();
```

### panic!() macro
```rust
// Panic
panic!("This will cause the program to abruptly end");
```
```
thread 'main' panicked at 'This will cause the program to abruptly end', src/main.rs:9:5
```

### unwrap(self): T

Check if it's ok or error.
Expects self to be Ok/Some and returns the value contained within. If it is Err or None instead, it raises a panic with the contents.

```rust
let f = File::open("doestnotexist.txt").unwrap();
let file = File::open(file_name)?; // ? is the same as ?
```
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:17:45
```

### expect(self, msg: &str): T
Like unwrap except that it outputs a custom message before panicking in addition to the contents of the error.

```rust
let f = File::open("doestnotexist.txt").expect("No such thing!");
```
```
thread 'main' panicked at 'No such thing!: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:14:45
```


## Generic Data Types

Generics -> Definitions for items like function signatures or structs, which we can then use with many different concrete data types.

```<T>``` -> Type parameter, it's used to declare a generic construct (any data type)

```rust
// Generic struct
struct GenericStruct <T> {value: T}

// i32
let t1: GenericStruct<i32> = GenericStruct {value: 100};
// String
let t2: GenericStruct<String> = GenericStruct {value: "String Data Type".to_string()};
```

## Traits
Like interfaces in OOP.
Traits defines functionality a particular type has and can share with other types.

- Abstract -> Shared behavior in an abstract way (overriding functions)
- Concretely -> Bounds to specify that a generic type can be any type that has certain behavior

They are often used to implement a standard set of actions (methods) across multiple structures.

```rust
// Definition of abstract trait (with one method, character_stats)
trait Stats {
    fn character_stats(&self);
}
// Implementation of the trait for one struct (Game)
impl Stats for Game {
    // Method implementation for Game
    fn character_stats(&self) {
        //We can access struct fields now from here
    }
}

//Create a instance for the struct and call the trait implementation for this struct
g1.character_stats();

```

## File operations

```use std::fs::File;```

### File object
```rust
{ 
  fd: 3, 
  path: "/home/kr0n0/Proyectos/rust/advanced/cat.txt", 
  read: false, 
  write: true 
}
```

## Create file

```rust
match File::create(file_name) {
    // 1 - Ok
    Ok(file) => println!("Writing {:?}", file),
    // 2 - Err
    Err(_) => println!("Unable to create the file {}", file_name)
}
```

## Open file

```rust
match File::open(file_name) {
    // 1 - Ok
    Ok(file) => println!("Reading {:?}", file),
    // 2 - Err
    Err(_) => println!("Unable to open the file {}", file_name)
}
```

## Reading contents of file
```use std::io::{Error, Read, BufReader}```

BuffReader provides a buffering component for reading

```rust
let file_name = "cat.txt";
let file = File::open(file_name)?; // Open the file
let mut reader = BufReader::new(file); // Open the BufReader pointer from file
let mut contents: String = String::new();   // String object for the content

reader.read_to_string(&mut contents)?;
println!("The contents of the file are: {}", contents);
```

## Closures

Closures are functions within functions that are nameless (inline functions or anonymous functions).

We can assign closures to variables and then pass a function as a parameter to other functions.

These are functions you can save in a variable or pass as arguments to other functions. 
You can create the closure in one place and then call the closure to evaluate it in a differente context.

Unlike functions, closures can capture values from the scope in which they're defined, for code reuse and behaviour customization.

```rust
let closer_function = |parameter| {
    //pass some logic
}
```
```rust
// Closure
let is_even = |n| {n%2==0};

// Program
let num = 11;
println!("{} is even? {}", num, is_even(num));
```

## Smart Pointers

Pointers -> Variables that contains an address in memory.
Reference (&) -> borrow the value they point to. They refer to data.

Smart Pointers are data structures that act like a pointer but also have additional metadata and capabilities (Originated in C++)

Provides more functionality that standard references.

* -> Dereference unary operator -> For a pointer, the pointed-to location.

https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/reference/expressions/operator-expr.html

### Traits for Smart Pointers
- Deref

```std::ops::Deref```

Immutable dereferencis operations

- Drop

```std::ops::Drop```

When a value goes out of scope you can run some code also known as a destructor.

### Box
Allows to store memory on the heap explicity rather than the stack.
Stack contains the pointer to the heap data.

```rust
let greeting: &str = "hello";
let greeting_heap: Box <&str> = Box::new(greeting);     // Points to a new variable in the heap

println!("{}","hello"==greeting);       // Print the return value of the conditional
println!("{}","hello"==*greeting_heap);       // Access the value in the heap
```

### Custom Smart Pointers

For creating Custom Smart Pointers we have to create the traits for Deref, Drop and New.

- New -> Create a new object in the Heap and returns the reference pointer to the stack
- Deref -> Accessing the values pointed in the stack for the heap using *
- Drop -> Free the memory when it's out of scope

```rust
struct CustomSmartPointer<T> (T);

// New
impl <T> CustomSmartPointer<T> {
    fn new (value:T) -> CustomSmartPointer<T> {
        CustomSmartPointer(value)
    }
}

// Deref
impl <T> Deref for CustomSmartPointer<T> {
    type Target = T; // in traits, type is used to declare an associated type
    fn deref(&self) -> &Self::Target {
        &self.0        // syntax for taking the first argument which is 0
    }
}

// Drop
impl <T> Drop for CustomSmartPointer<T> {
    fn drop(&mut self) {
        println!("Memory freed!")
    }
}

let color = "green";
let ref_color = CustomSmartPointer::new(color);
println!("green value is the same as color which is {}", "green"==*ref_color);  //Heap

```

## Concurrency (Multi-threading)

- Threads -> Run code at the same time. OS manages multiple processes at once and programs execute code in a process.

They are used when you have independant parts of your code that run together -> Multi thread programming.

We use ```std::thread::spawn```

The parameter is the closure wich defines the code.

For low-level blocking:

- std::thread::park -> Blocks the current thread
- std::thread::unpark -> Unblocks/resumes the thread (callable from another thread)

This code creates a new threads using the thread::Builder API. The new thread runs an anonymous function that performs two actions, separated by a call to thread::park(). The main thread then sleep for 10 milliseconds, unpark the created thread, and finally join it to get the final result, with the unwrap function the code will panic if something goes wrong during the process.

```rust
use std::thread;

let example_thread = thread::Builder::new().spawn(
        ||
            {
                // Do something
                thread::park();
                // Do something after it's unparked
            }
    ).unwrap();     // Gimme the results and if something happens, panic!

thread::sleep(Duration::from_millis(10)); // Sleeps the thread for 10 milisecs
example_thread.thread().unpark();      // Unparks the thread
example_thread.join().unwrap();        // Waits to finish
```

