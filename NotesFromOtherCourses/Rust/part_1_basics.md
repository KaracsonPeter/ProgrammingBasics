## Introduction
Firefox developed Rust because they were sick of C/C++.  
It provides the flexibility, concurrency and safety of script languages as Python,  
and it also provides the speed and concurrency of lower level languages like C/C++;    
Rust has a package manager called Cargo. (like pip for python)
[Installing Rust with Cargo](https://www.rust-lang.org/tools/install)  

Cargo's responsibilities:
 * Package manager
 * Build system
 * Documentation generator

Create a project called hello:
```bash
$ cargo new hello
    Created binary (application) `hello` package
```
Take look at what cargo created for us:
```bash
tree --noreport hello
hello
|___Cargo.toml
|___src
     |___main.rs
```
Config files use `.toml` format stands for toms obvious minimal language.
Rust source files use `.rs` extension.

Let's check what is in Cargo.taml:
```bash
vim .\hello\Cargo.taml
```
Cargo.taml is the config file for your project.
```text
[package]
name = "hello"
version = "0.1.0"  # [semantic versioning](https://semver.org/)
edition = "2021"  # if you do not have this line, you have to run rust update
```

Command to build and run your project in one step:
```bash
$ cargo run
Compiling hello v0.1.0 (C:\Users\u27j77\.cargo\bin\hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.55s
     Running `target\debug\hello.exe`
Hello, world!
```

Cargo runs your project in debug setting by default.  
Run in release:
```bash
cargo run --release
```

## Syntax
Lot of syntax element in Rust is inherited from C and Python.
### Variables
```rust
// You do not have to specify the type if it is obvious for the compiler.  
let bunnies = 2;
// If you want to specify the type:
let bunnies: i32 = 4;
// You can define multiple variables in one line according to a pattern defined in let() statement:
let(bunnies, carrot) = (8, 50);
```
Variables are immutable by default in Rust!!! (cannot change its value after defining)  
That is wierd. Most programming languages define variables mutable by default.  
In Rust it is immutable because Safety, Concurrently and Speed.  
If you want to assign a value to an immutable variable, you will get the following error:
```text
error[E0384]: cannot assign twice to immutable variable `bunnies`
 --> src\main.rs:4:5
  |
3 |     let bunnies = 3;
  |         -------
  |         |
  |         first assignment to `bunnies`
  |         help: consider making this binding mutable: `mut bunnies`
4 |     bunnies = 5;
  |     ^^^^^^^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
warning: `hello` (bin "hello") generated 2 warnings
error: could not compile `hello` due to previous error; 2 warnings emitted
```

How an error message builds up:
 * Summary of the error
 * Where it can be found in the code
 * What we are doing wrong and how we might want to fix it
 * If this is not enough to understand what the error is about, there is a recommended command can be run for more detailed explanation: `rust --explain E0384`

The solution for the error above:
```rust
let mut bunnies = 32;
bunnies = 2;
```

A more immutable variable definition:
```rust
// All const is UPPER_CASE and the type notation cannot be leaved out
// The value also must be a value can be known at compile time
const WARP_FACTOR: f64 = 9.9;  // Advantage: const can be global an really fast
```

### Scope
```rust
fn main() {
  let x = 5;
  {
    let y = 99;
    println!("{}, {}", x, y);
  }
  println!("{}, {}", x, y);  // Error: y is deleted because we run out of its scope (E0425: cannot find calue 'y' int this scope)
}
```
#### Variable shadowing
Variables are always local to their scope!
```rust
fn main() {
  let x = 5;
  {
    let x = 99;
    println!("{}", x);  // prints "99"
  }
  println!("{}", x);  // prints "5"
}
```

Shadowing from the same scope:
```rust
// Shadow mutable with immutable
let mut x = 5;  // x is mutable
let x = x;  // x is now immutable
```

Shadow a variable with a different type:
```rust
let meme = "More cowbell!";
let meme = make_image(meme);
```

### Memory safety
Rust provides memory safety at compile time!
What would result an undefined behavior, won't compile!
```rust
// This wont work:
let enigma: i32;
if true {
  enigma = 19;
}

// This works:
let enigma: i32;
if true {
  enigma = 19;
} else {
  enigma = 7;
}
```

[Exercise A](https://github.com/CleanCut/ultimate_rust_crash_course)  
```rust
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {
    let (ready, mut missiles) = (READY_AMOUNT, STARTING_MISSILES);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles -= ready;
    println!("{} missiles left", missiles);
}
```

### Functions
Definitions start with the `fn` (pronounced as fun) keyword.  
Parameters are separated by ',' in the round brackets.  
Return type is specified after an arrow '->'.  
```rust
fn mull(arg_1: f64, arg_2: f64) -> f64 {
  return arg_1 * arg_2;
}
```
Tail expression: The last line is a shorthand expression for return (I won't use it.)
```rust
fn mull(arg_1: f64, arg_2: f64) -> f64 {
  arg_1 * arg_2
}
```

Calling a function is similar to other languages function call. (Like C.)  
```rust
// You have to provide the arguments in the correct order!
let x = mull(2.1, 12.5);
```

`println!` is a so-called macro function which is marked by '!' signe.  
A single function des not accept different type of inputs as argument.  
A macro does. It's a bit complicated to make a macro. We will learn it later.  

### Module System

```bash
tree --noreport hello
hello
|___Cargo.toml
|___src
     |___lib.rs  # Special file that will be the root of the hello library
     |___main.rs  # Special file that will be the hello binary
```

`lib.rs`:
```rust
pub fn greet() {
    println!("Hi!");
}
```

`main.rs`:
```rust
fn main() {
  // call the function by specifying its absolute path 
    hello::greet(); // Won't work if function is not publit
}
```

Calling the function by specifying its absolute path can make your code look messy. So:
```rust
use hello::greet;
// Or you can include multiple functions by:
use hello::{func_1, func_2, func_3}

fn main () {
  greet();
}
```
It is like `import hello.greet as greet` in Python.  
  
The standard library is always available by default:
```rust
use std::collections::HashMap;
```
(You can google things in standard lib like: rust std vector.)  

What if you want something what is not present in the standard library?  
crates.io is Rust's package registry.  
You have to add the package you want to use in the `Cargo.taml` file as dependency.
```text
[dependencies]
name_of_the_package = "0.6.5" (version)
```

### Scalar types
#### Integer
unsigned: u8, u16, u32, u64, u128, usize,  
signed: i8, i16, i32, i64, i128, isize  
usize is the size of the platform's pointer type and can represent every memory address in the process. It is also to type you will usually use to index into an array or a vector.  
The maximum isize value is the upper bound of object and array size. This ensures that isize can be used to calculate differences between pointers and be able to address every byte within a value like a struct.  
If you do not specify an integer, it will be i32 by default, because it is usually the fastest type even on 64bit systems.  
Not every hardware supports all of these types. (e.g.: A 16bit microcontroller might only support these types: u8, u16, usize, i8, i16, isize)  
You can initialize an integer by in decimal, hexadecimal, octal, binary and byte format:  

| Decimal       | 10           |
|---------------|--------------|
| Hex           | 0x52         |
| Octal         | 0o111        |
| Binary        | 0b01101110   |
| Byte(only u8) | b'A' (rare)  |

This is also acceptable:
```rust
let alma = 1_000_000
// And this is also (cause underscores are ignored)
const ALMA = 1_0_0_0_0_0_0
```

#### Float
You have: `f32` and `f64`.  
f64 is the default because it is more precise, however it can be really slow on 32bit architectures!!!  

These definitions follow IEEE-754 standard:
 * You always have to specify a number before the '.'
 * You do not need to specify suffixes like `f`  

3.14159  

It is also fine to suffix the literal this way:
```rust
let x = 5u16;
const PI = 3.1415_f32;  // See! Underscores can improve readability.
```

#### Bool
It values `true` or `false`.  
Booleans are not integers so do not try to make integer operations on them unless you cast them into int:
```rust
true as u8
```

#### Char
A character is always 4 bites.  
Which makes an array of chars a UCS-4 / UTF-32 string.  
Strings are UTF8! and characters are not!

### Compound(or composite) types
Any data type which can be constructed in a program using the programming language's primitive data types / other compound types.

#### Tuple
```rust
let info = (1, 3.3, 999);
let info: (u8, f64, i32) = (1, 3.3, 999);
// Accessing members by '.' to signe that the members of the Tuple sometimes are not the same data type
let jets = info.0;
let fuel = info.1;
let ammo = info.2;
// Second way of access members of a Tuple
let (jets, fuel, ammo) = info;
// If the Arity of the Tuple is more than 12 it can only be used with limited functionality (Arity means how many items in the Tuple. e.g. (u8, f64, i32) -> Arity = 3)
```

#### Arrays
Store multiple values of the same type.
```rust 
let buffer = [1, 4, 8, 12];
// Or specify a [value; and how many do you want from this value]
let buff = [0; 4];
// The type annotation alwais use the semicolumn form:
let buff: [u8; 3] = [1, 2, 3];
// Index values in an array by sqare brackets
let value = buff[1];
// Array are lose most of their functionality above a size of 32!
```

### Control flow
The order in which individual statements, instructions, or function calls are executed or evaluated.

#### if
 - The condition must be boolean, cause rust has no automatic type conversion (called type coertion).

```rust
if num == 5 {
  msg = "five";
} else if num == 4 {
  msg = "four";
} else {
  msg = "other";
}
```
Statements do not have return values, expressions do. We can modify the above statement as:
```rust
// All the blocks must return the same type
let msg = if num == 5 {
  "five"
} else if num == 4 {
  "four"
} else {
  "other"
};
```
#### Unconditional loops
```rust
loop {
  break;
}
```
Break out of nested loop:
```rust
'bob: loop {  // Annotate the loop you wanna break out. This one is named bob.
  loop {
    break 'bob;  // Tell break which loop you want to break out of.
  }
}
```
Continue is very similar:
```bash
'bob: loop {
  loop {
    continue 'bob;  // It continues the annotated loop
  }
}
```

#### While loops
Break when their condition becomes false.
```rust
while dizzy() {
  // do stuff
}
// while is the same like:
loop {
  if !dizzy() { break }
  // do stuff
}
// do-while:
loop {
  // do stuff
  if !dizzy() { break }
}
```

#### for loop and iterating
You can iterate through a compound type by an iterator which you can get by the `.iter()` function.
```rust
for num in [7, 8, 9].iter() {
  // do stuff with num
}
```
A for loop can take a pattern to destructure the items it receives:
```rust
let array = [(1,2), (3,4)];

for (x, y) in array.iter() {
  // do stuff with x & y
}
```
Ranges with for loop:
```rust
for num in 0..50 {
  // do stuff with num
}
```
Two dots are separating your start and end point.  
The start is inclusive and the end is exclusive by default.  
If you write `0..=50` both will be inclusive, so this will count up to 50.

### Strings
There are your 6 different string types in rust, but we will cover 2 of them.
#### &str ((borrowed) string slice)
Literal string is a sequence of characters enclosed in the single quotes('') or double quotes("").  
Borrowed is refers to it stored elsewhere, and it does not have ownership of the data. So in this type the data cannot be modified!  
A literal string is always a borrowed string slice.  
A borrowed string slice is often referred to as a string.  
It is made up of 
* a pointer to some bytes containing the string 
* and a length.


#### String
In this, data can be modified.  
Converting borrowed string to String:
```rust
let msg = "abc".to_string();
// OR
let msg = String::from("abc");
```
"String" is made up of 
* a pointer to some bytes containing the string, 
* a length,
* and a capacity which might be larger than the current size of the string.

You can see that a borrowed string slice is a subset of a "String" in more ways than one, which is why they share a bunch of other characteristics:
 - Both string types are valid UTF-8
 - Strings cannot be indexed by character position. (cause there are more than 6900 languages on earth and the byte representation is not always that simple like in English)
  - So for English text, you can get exact characters by `word.bytes();`, which you can index into if you want.
  - You can use `word.chars();` method to get unicode scalars
  - or you can use a package like unicode segmentation, which provides helper functions which can provide iterators can handle graphemes. `graphemes(my_str, true)`
  - or you can use e.g. `nth(3)`

### Ownership
This makes those crazy safety guarantees possible.  
There are 3 rules to ownership:
 1. Each value has an owner. (There is no value in the memory which does not have a variable that owns it.)
 2. There is only one owner of a value. (No variables may share ownership. Other variables may borrow the value which we will talk about.)
 3. When the owner runs out of scope, the value gets dropped immediately.

```rust
let s1 = String::from("asd");
let s2 = s1;  // Here the value is not copied. The value from s1 is moved to s2, because only one variable can own the value.
println!("{}", s1);  // ERROR! If we try to use s1 after this print
```

Quick reminder:  

| Stack                    | Heap                          |
|--------------------------|-------------------------------|
| Ordered                  | Unordered                     |
| Fixed-size               | Variable-size                 |
| LIFO                     | Unordered                     |
| Fast (cause predictable) | Slow (u have to use cpu cash) |  

```rust
let s1 = String::from("asd");
// A pointer a length and a capacity gets pushed into the stack
// e.g. 0x0003ba42    3    3

let s2 = s1;  // move s1's value into s2, because we can only have one owner
// The pointer length and capacity all get copied from s1 and pushed as new values on the stack as part of s2. If we stopped here, then memory safety would be non-existent, which is why Rust immediately invalidates s1. It still exists on the stack, the compiler just consider s1 now uninitialized and won't let you use it after this point.
//(technically, if s1 was mutable, we could assigne it some new value and then use it again.)
```
What if we didn't want to move the value but copy it instead?  
To make a copy of this one we would call the `clone()` method.
```rust
let s1 = String::from("asd");
let s2 = s1.clone();
println!("{}", s1);
```

So, the stack and heap data (if there is heap data) together make a value:  
 - On stack, a pointer a length and a capacity can be found
 - The pointer points to the data stored in the heap

Rust reserves the term `copy` when only stack data is being copied.  
When there is heap data and pointer updates involved, then we use the term `.clone()`.  
(Basically, there are 2 types of value assignment:
  - The metadata (in stack) gets copied but the underlying data remains the same
  - Everything gets a clone (both metadata on stack and actual data on heap).

)

When a value is dropped:
 1. Destructor
 2. Free heap
 3. Pop stack

So, no leaks, no dangling pointers...

What about function calls with parameters?
```rust
fn main() {
  let s1 = String::from("asd");
  do_stuff(s1);
  println!("{}", s1); // ERROR!!!
}

fn do_stuff(s: String) {
  // do stuff
}
```
In the example above, `s1` gets moved into the local scope of the function, therefore we cannot access it after the function call.  
So, how do I get back its modified value?  
 1. One option is to move it back when it is done by defining it mutable and add a return type to the function:  

```rust
fn main() {
  let mut s1 = String::from("asd");
  do_stuff(s1);
  println!("{}", s1);
}

fn do_stuff(s: String) -> String {
  s  // tail expression
}
```
Although, this is usually not the case you want.
  2. For most other case you should use references:

### References:
Instead of moving our variable, let's use a reference!
```rust
fn main() {
  let s1 = String::from("asd");
  do_stuff(&s1);  // do_stuff borrows a refference of the value passed to it
  println!("{}", s1);
}

fn do_stuff(s: &String) {  
  // do stuff
}
```
The reference, not the value gets moved to the function.  
At the end of the function, the reference gets out of its scope and the reference gets dropped.  
Under the hood, rust creates a pointer to `s1`. 
Although, you almost never speak about pointer in rust, cause it automatically handles the creation and destruction for the most part and makes sure that they always valid using a concept called lifetimes.  
Lifetime can be summed up as a rule that references must always be valid, which means the compiler won't let you create a reference to outlives the data it is ultimately referencing, and you can never point to null!!!  
By default, references are immutable even if the value being referenced is by them is mutable.   
But if we make a mutable reference to a mutable value, then we can use the reference to change the value as well.  
The syntax for a mutable reference is a little special:
```rust
fn main() {
  let s1 = String::from("asd");
  do_stuff(&mut s1);
  println!("{}", s1);
}

fn do_stuff(s: &mut String) {  
  s.insert_str(0, "Hi, ");
}
```
Wait C++ fun! Why don't we have to dereference `s`? Cause the `.` "method" auto-dereferences down to the actual value!  
So, you DON'T HAVE TO WORRY about if a value is a value or reference or ref of ref...  
However you have a way to manually dereference a value:  
```rust
(*s).insert_str(0, "Hi, ");  // You sometimes need to use parentheses, cause the dereference '*' has really low precedence
```

With most other operators, like the assignment operator for example you will need to manually dereference your reference if you want to read from or to the actual value:
```rust
s.insert_str(0, "Hi, ");
*s = String::from("Replacement");  // replace original value
```

| Variable                 | x              |
|--------------------------|----------------|
| Immutable reference to x | &x             |
| Mutable reference to x   | &mut x         |

With type annotation:

| Type                        | i32              |
|-----------------------------|------------------|
| Immutable reference to type | &i32             |
| Mutable reference to type   | &mut i32         |

Example:
```rust
let x: &mut i32
*x // a mutable i32
```

Rust has a special rule to keep us safe:  
At ANY given time you can have exactly one mutable reference or any number of immutable reference. This rule applies across all threads.  
If you would have references on different threads for a variable allocated on another thread, it is obvious that you need some kind of lock on this variable. 
But if all the references are immutable then there is no problem. So, you can have lots of immutable references across multiple threads.  

All these rules are enforced by the compiler. This results in lots of compiler errors if you are not familiar with the language.  
But:
 - You do not have any mysterious seg faults anymore
 - You get some nice readable warnings and errors
 - And if your code compiles, it works!

### Structs
It is like classes in other languages. This is what you can perform OOP with.  
Structs can have 
 - class fields
 - methods
 - and associated functions

**Syntax**: (struct, name of struct, and fields and type annotations in a list separated by commas)  
Definition:
```rust
struct RedFox {
  enemy: bool,
  life: u8,
}
```

Instantiation:
```rust
let fox = RedFox {
  enemy: true,
  life: 70,
};
```

Constructor function for the struct:
```rust
impl RedFox {
  // This is an associated function cause it does not have an input parameter of Self
  fn new() -> Self {  // new is the conventional name for creating a new instance of a struct (you could call it something else but that would not be conventional)
    Self {
      enemy: true,
      life: 70,
    }
  }
}
```
You can see that Struct related methods are defined in an `impl` block(separated from the struct definition).  

Self is just an alias for the actual implementation name:
```rust
impl RedFox {
  fn new() -> RedFox {
    RedFox {
      enemy: true,
      life: 70,
    }
  }
}
```

Instantiation with constructor:
```rust
let fox = RedFox::new();  // '::' = scope operator. We already used it to access items inside of modules
// Once you have an instantiated value, you get and set field and call methods 
// with '.' syntax as you do in most languages.
let life_left = fox.life;
fox.enemy = false;
fox.some_method();
```

Methods are also defined in the implementation block.
```rust
impl RedFox {
  // associated function
  fn function ...
  // methods
  fn move(self) ...
  fn borrow(&self, frog) ...
  fn mut_borrow(&mut self) ...
}
```

#### Struct inheritance
Instead of classes Rust use Structs.  
There is no struct inheritance!  

Is Rust an OOP language or not?  
There is no universally-accepted definition of what an OOP language is or isn't.  
This is not the real question. The real question is:  
Why Rust does not provide inheritance?  
Cause it provides a better way to solve the problem we wish to solve with inheritance! --> Traits

### Traits
Traits are similar to interfaces in other languages.  
Rust takes the composition over inheritance approach. (Composition, is a mechanism that allows a class to use the properties and methods of other classes through object references, rather than inheriting them. This approach promotes loose coupling and greater flexibility.)  
Traits define required behavior. In other words, functions and methods that a struct must implement if it wants to have that trait.  

Here, the `Noisy` trait specifies that the struct must have a method named `get_noise()` that returns a borrowed string slice if the struct wants to be "noisy":
```rust
struct RedFox {
  enemy: bool,
  life: u32,
}

trait Noisy {
  fn get_noise(&self) -> &str;
}
// So let's add an implementation of the Noisy trait RedFox.
impl Niosy for RedFox {  // implement the Noisy state for the RedFox struct
  // and the implementation of our required get_noise() is Meow
  fn get_noise(&self) -> &str { "Meow?" }
}
```
Why don't we just implement simply the function?  
Cause now we can start writing generic functions that accept any value that implements the trait.  
```rust
// This function takes an item of type T 
// Which is defined to be anything that implements the Noise trait.
// The function can use any behavior on an item that the Noisy trait defines. (get_noise())
fn print_noise<t: Noisy>(iter: T) {
  println!("{}", item.get_noise());
}
```
So now we have a generic function that can take any type as long as it satisfies the Noisy trait.  
As long as one of either the trait or the struct is defined in your project you can implement any trait for any struct.  
That means you can implement your traits on any types from anywhere including built-ins or types you import from some other package. And on your struct you can implement any trait whether built-in or from some project.  
Here's how you can implement `get_noise()` for u8:
```rust
fn print_noise<T: Noise>(item: T) {
  println!("{}", item.get_noise());
}

impl Noise for u8 {
  fn get_noise(&self) -> &str { "BYTE!" }
}

fn main() {
  print_noise(5_u8);  // prints BYTE!
}
```

There is a special trait copy. If your type implements Copy, then it will be copied instead of moved in move situations. 
This makes sense for small values that fir entirely on the stack, which is why the simple primitive types like integers, floats and booleans implement Copy. 
If the type uses the heap at all then it cannot implement Copy. You can opt in to implementing Copy with your own type if your type only uses other Copy types.  
So, the trait can inherit from another trait.  
So, the struct which inherit traits, also have to implement the parent traits.  

Although, traits can have default behaviors, so if you design structs and traits carefully enough, you might not have to implement some of the trait methods at all.  
Implement default trait behavior:
```rust
trait Run {
  fn run(&self) {  // instead of: fn run(&self);
    println!("I am running!");
  }
}
```
Then, when you implement the trait for your struct just do not provide a new definition for the method whose default implementation you want to use. (The presence of an implementation will overwrite the default.)
```rust
struct Robot {}
impl Run for Robot {}  // Does not override the default Run method

fn main() {
  let robot = Robot {};
  robot.run();  // prints: "I am running!"
}
```

Unfortunately, data fields cannot be defined in traits yet. This can be done by implementing getters and setters in the traits.

### Exercise F - Structs & Traits
```rust
// 1. Define a trait named `Bite`
//
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.  Once this trait is defined, you should be able to run the program with
// `cargo run` without any errors.
//
trait Bite {
    fn bite(self: &mut Self);
}


// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// use a different field, though).
//
#[derive(Debug)] // include this line right before your struct definition
struct Grapes {
    amount_left: i32,
}

// 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// If you need a hint, look at how it was done for Carrot at the bottom of this file.
//
impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}


fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // 4. Uncomment and adjust the code below to match how you defined your
    // Grapes struct.
    //
    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    fn bunny_nibbles<T: Bite>(iter: &mut T) {  // "bunny_nibbles" will only accept a function that define trait "Bite"
        iter.bite();
    }
    
    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}
```

### Collections 
Collections are just like containers on C++. It is not mandatory to use, but it can help a lot in solving different problems.
Each has its own strengths and weaknesses, so you have to choose wisely according to your purposes.

#### Vec<T>
Vector is a generic collection that holds a bunch of the same type.  
```rust
let mut x: Vec<32> = Vec::new();
v.push(2);
v.push(4);
v.push(8);
let x = v.pop();      // x is 8
println!("{}", v[1]); // prints 4
```
Once you have the vector you can push values to its end and  
pop values from the end which returns the oldest value. (FIFO)  
Since vectors store known size object next to each-other in memory, you can index into it.  
If your index is out of bounds, Rust will panic, so you should check your length before you index.  
Making vector from literal(known) values:
```rust
let mut v = vec![2, 4, 8];
```
Vectors give you a lot of low-level control over their behavior, and they have a ton of methods to do just about anything you could possibly want like: `insert`, `remove`, `split`, `splice`, `sort`, `repeat`, `binary search`.

#### HashMap<K, V>
HashMap is a generic collection where you specify a type for the key and a type for the value, and you access the value by key. (In Python you would call it dictionary. (or std::map in c++?))  
The whole point of a hashmap is to be able to insert, look up and remove values by key in constant time.  
When you create a HashMap, you specify the type of the key, and the type of the value.
```rust
// In this case, the key is a byte and the value is a boolean.
let mut h: HashMap<u8, bool> = HashMap::new();
// You insert entries with the inert method:
h.insert(5, true);
h.insert6, false();
// Use the remove method to get a value
// remove actually returnes a enum called "option"
let have_five = h.remove(&5).unwrap();
// There are also methods to values and iterating through keys, values or keys and values, either as immutable or mutable references.
```
Other collections quickly:
#### VecDeque
Uses a ring buffer to implement a double ended queue, which can efficiently add or remove items from the front and the back, but everything else is a little less efficient than a regular Vector.

#### HashSet
Hashing implementation of a set that performs set operations really efficiently.

#### BTreeMap


#### LinkedList
Quick ad adding or removing items at an arbitrary point in the list but slow doing absolutely anything else.

#### BinaryHeap

#### BTreeSet

### Enums
Enums are more like algebraic data types in Haskell (I don't know this language) rather than C like enums.
```rust
enum Color {
    Red,
    Green,
    Blue,
}
// usage
let color = Color::Red;
```
The real power of Rust enum somes from associating data and methods with the variants.  
A variant can have a:
```rust
enum DispenserItem {
    Empty,      // variant with no data
    Ammo(u8),   // single type of data
    Things(String, i32),    // a tuple of data
    Place { x: i32, y: i32 },   // or an anonimus struct of data
}
```
So, enum in Rust is sort of like a union in C only so much better.  
If you create an enum, the value can be any one of the variants listed above:
```rust
use DisperserItem::*;

let item = Empty;
let item = Ammo(34);
let item = Things("hat".to_string(), 7);
let item = Place(x: 24, y: 48);
```
It can be any one of those but only one at a time.  
Even better, you can implement methods and functions for an enum:
```rust
impl DisperserItem {
    fn display(&self) {}
}
```
Use can also use enums with generics. (Peace of code that works with any type.)
```rust
enum Option<T> {  // You could use e.g. ANY_TYPE instead of T but the idiomatic thing to do in Rust is using T or some other letters.
    Some(T),
    None,
}
```
The Option enum represents when something is either absent or present. 
If you are trying to reach for a null or nil value like in other languages, you probably want to use an Option in Rust.  
Because enums can represent all sorts of data, you need to use patterns to examine them. 
If you want to check for a single variant, you use the "if let" expression.  
"if let" takes a pattern that will match one of the variants:
```rust
if let Some(x) = my_variable {
    println!("value is {}", x);
}
```
If the pattern does match, then the condition is true and the variables inside the pattern are created for the scope of the "if let" block. 
This is pretty handy if you care about one variant matching or not, but not as great if you need to handle all the variants at once.  
In that case you use the match expression:
```rust
match my_variable {
    Some(x) => {
        println!("value is {}", x);
    },
    None => {
        println!("no value");
    },
    _ => {
        println!("DEFAULT");
    },
}
```
(It is like switch an enum in C. '_' is default case)  
Match expressions require you to write a branch arm for every possible outcome.  
You can write any expression as branch arm not only blocks:
```rust
match my_variable {
    Some(x) => x.squared() + 1,
    None => 42,
};  // you have to put a semicolon after the closing brace cause match returns a value
```
Either all branch arms need to return nothing or the same type!  
These expressions are always brought in the standard prelude, which is the list of items from the standard library that are always brought into scope by default.  
Two special enums: (used all over the standard lib)
#### Option
Option is used whenever is something might be absent.  
Here's how you could create a None variant of an Option:
```rust
let mut x: Option<i32> = None;
```
If you ever use Option with a concreate type, then the compiler will infer the type, which means you can leave the type annotation off of the declaration most of the time:
```rust
let mut x = None;
x = Some(5);
// There is a handy helper method called is_some() that returnes true if y is the a defined variant,
x.is_some(); // true
x.is_none(); // false
// Option also implements the Intoiterator trait:
for i in x {
    println!("{}", i);  // prints 5
}
```
There are a lot of methods of Option you should definitely check out!

#### Result
Result is used whenever something might have a useful result, or might have an error. (This turns up especially often in the IO module.)  
Definition of the Result enum:  
```rust
#[must_use]
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
1. Ok and Err are both generic, but independent of each other!
2. The #[must_use] annotation makes it a compiler warning to silently drop a result. You must do something with it! 
Rust strongly encourages you to look at all possible errors and make a conscious choice what to do with each one. 
Any time you deals with I/O, failure is a possibility, so Results are used a lot there.

Let's see it in action!
```rust
// bring standard filesystem File into scope
use std::fs::File;

fn main() {
    // try to open a file
    File::open("foo");  // This returns a Result, because the file might not get opened successfully.
}
```
Since I dropped the Result without doing anything with it, I got the following warning:
```text
warning: unused `std::result::Result` that must be used
```

Simplest way to deal with the warning:
```rust
use std::fs::File;

fn main() {
    let res = File::open("foo");
    let f = res.unwrap(); 
}
```
If the Result is Ok, then this gives you the File struct you wanted. If not, the program crashes.  
Another way is to use `except` method:
```rust
use std::fs::File;

fn main() {
    let res = File::open("foo");
    let f = res.except("error message"); 
}
```
Same as `unwrap` except that the string you pass to it will be printed out.  
Just like in Option, there are helper methods related to Result.
```rust
use std::fs::File;

fn main() {
    let res = File::open("foo");
    if res.is_ok() {
        let f = res.unwrap();
    }
}
```

You could always to pattern matching as well:
```rust
use std::fs::File;

fn main() {
    let res = File::open("foo");
    match res {
        Ok(f) => { /* do stuff */ },
        Err(e) => { /* do stuff */ },
    }
}
```

### Exercise G - Collections and Enums
```rust
// Silence some warnings that could distract from the exercise
#![allow(unused_variables, unused_mut, dead_code)]

// Someone is shooting arrows at a target.  We need to classify the shots.
//
// 1a. Create an enum called `Shot` with variants:
// - `Bullseye`
// - `Hit`, containing the distance from the center (an f64)
// - `Miss`
//
// You will need to complete 1b as well before you will be able to run this program successfully.
enum Shot {
    Bullseye,
    Hit(f64),
    Miss,
}

impl Shot {
    // Here is a method for the `Shot` enum you just defined.
    fn points(self) -> i32 {
        // 1b. Implement this method to convert a Shot into points
        // - return 5 points if `self` is a `Shot::Bullseye`
        // - return 2 points if `self` is a `Shot::Hit(x)` where x < 3.0
        // - return 1 point if `self` is a `Shot::Hit(x)` where x >= 3.0
        // - return 0 points if `self` is a Miss
        match self {
            Shot::Bullseye => 5,
            Shot::Hit(x) if x < 3.0 => 2,
            Shot::Hit(x) => 1,
            Shot::Miss => 0,
            /*// My solution was
            Shot::Bullseye => {
                5
            },
            Shot::Hit(x) => {
                if x < 3.0 {
                    2
                } else {
                    1
                }
            },
            Shot::Miss => {
                0
            },*/
        }
    }
}

fn main() {
    // Simulate shooting a bunch of arrows and gathering their coordinates on the target.
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    // 2. For each coord in arrow_coords:
    //
    //   A. Call `coord.print_description()`
    //   B. Append the correct variant of `Shot` to the `shots` vector depending on the value of
    //   `coord.distance_from_center()`
    //      - Less than 1.0 -- `Shot::Bullseye`
    //      - Between 1.0 and 5.0 -- `Shot::Hit(value)`
    //      - Greater than 5.0 -- `Shot::Miss`
    for i in arrow_coords {
        i.print_description();
        let dis = i.distance_from_center();
        // could have use match instead...
        /*
        let shot = match i.distance_from_center() {
            x if x < 1.0 => Shot::Bullseye,
            x if x < 5.0 => Shot::Hit(x),
            _ => Shot::Miss,
        };
        */
        if dis < 1.0 {
            shots.push(Shot::Bullseye);
        } else if dis < 5.0 {
            shots.push(Shot::Hit(dis));
        } else {
            shots.push(Shot::Miss);
        }
    }


    let mut total = 0;
    // 3. Finally, loop through each shot in shots and add its points to total
    for i in shots {
        total += i.points();
    }
    println!("Final point total is: {}", total);
}

// A coordinate of where an Arrow hit
#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y);
    }

}

// Generate some random coordinates
fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}
```

### Closures
You will encounter closures when you want to spawn a thread, or when you want to do some functional programming with iterators, 
and in some other common pleaces in the standard lib.  
A closure is an anonymous function that can borrow or capture some data from the scope it is nested in. (A lambda?)  
The syntax is: (A parameter list between 2 pipes without type annotations followed by the closure block.)
```rust
|x, y| { x + y }  // This creates a function, called the closure, you can call later
```

```rust
let add = |x, y| { x + y };

add(1, 2);  // returns 3
```
The closure will borrow a reference to values in the enclosing scope:
```rust
let s = "s".to_string();
let f = || {
    println!("{}", s);
};

f();  // prints "s"
```
This is great if your closure never outlives the variable it is referencing, 
but the compiler won't let us send this over to another thread because another thread might live longer than this thread.  
Lucky for us, closures also supports move semantics!!!
So we can force the closere to move any variable it uses into itself and take ownership of them:
```rust
let s = "s".to_string();
let f = move || {
    println!("{}", s);
};

f();  // prints "s"
```
Now `s` is owned by the closure and will live until the closure itself runs out of scope and gets dropped.
So, we could send this closure over to another thread, or return it as the value of a function or whatever we want with it.  

If you want to do some functional-style programming, closures will be your close friends. 
Call `.iter()` on a vector to get an iterator and a whole bunch of methods that use closure will be available to you.  
Here is an example of using `.map()` and a closure to multiply each item in a vector by 3. 
Then, `.filter()` in a closure to discard any values that aren't greater than 10 and then `.fold()` with an initial value and a closure to sum the remaining values together:
```rust
let mut v = vec![2, 4, 8];

v.iter()
    .map(|x| x * 3)
    .filter(|x| *x > 10)
    .fold(0, |acc, x| acc + x);
```

### Threads
Rust threading is portable. (So this code should work across Linux, Windows and Mac.)  
Example:
```rust
use std::thread;

fn main() {
    let handle = thread::spawn(move || {
        // do stuff in child thread
    });
    
    // do stuff simultaneously in main thread
    
    // wait until thread has expired
    handle.join().unwrap(); 
}
```
Spawn returns a join handle. With that handle we can call `.join()` which will pause the thread that we are on until the thread we are joinning has compleated and executed.  
The thread that we spawn could have an error like panic, or it could return a value successfully back to the thread that joins it.
So what we get from the join call is a result thet wrap a possible seccess value returned from the thread, or an error if the thread panicked.  
Threading is a bit heavy weight. Creating a new thread allocated an operation-system-dependent amount of RAM for the thread's own stack. Often a couple of megabytes.
Whenever a CPU switches from running one thread to another, it has to do an expensive context switch!!!  
So the more threads you have trying to share the same CPU core, the more overhead you will have in context switching.  

In summary:  
Even so, threads are fantastic tool when you need to use CPU and memory concurrently, because they can run simultaneously on multiple cores,
ande actually accomplish more work!  
However, if you just want to continue doing some work while you are waiting for something like disk I/O or network I/O,
then I encourage you to look into async / await, which is much more efficient approach for concurrently waiting for things.

```rust
// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");
    // 1a. Between the .iter() and the .sum() add a .filter() with a closure to keep any even
    // number (`x % 2` will be 0 for even numbers).
    // 1b. Between the .filter() and the .sum() add a .map() with a closure to square the values
    // (multiply them by themselves)
    //
    // In the closures for both the .filter() and .map() the argument will be a reference, so you'll
    // either need to dereference the argument once in the parameter list like this: `|&x|` or you
    // will need to dereference it each time you use it in the expression like this: `*x`
    v.iter()
        .filter( |x| { (*x) % 2 == 0 } )  // or ( |&x| { x%2 == 0 )
        .map( |&x| { x * x } )  // why did it work without ref? :/
        .sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    // 2. Spawn a child thread and have it call `expensive_sum(my_vector)`.  Store the returned
    // join handle in a variable called `handle`. Once you've done this you should be able to run
    // the code and see the Child thread output in the middle of the main thread's letters
    //
        // a single expression does not need to be in {}
        // `move` is necessary cause we are passing the vector to another thread. (Since expensive_sum takes my_vector by value, the closure vill automatically become  a move closure and take ownership of my_vector.)
    let handle = thread::spawn( move || expensive_sum(my_vector) );

    // While the child thread is running, the main thread will also do some work
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", letter);
        pause_ms(200);
    }

    // 3. Let's retrieve the value returned by the child thread once it has exited.  Using the
    // `handle` variable you stored the join handle in earlier, call .join() to wait for the thread
    // to exit with a `Result<i32, Err>`.  Get the i32 out of the result and store it in a `sum`
    // variable.  Uncomment the println.  If you did 1a and 1b correctly, the sum should be 20.
    //
    /*  We will take our handle to the thread. We'll join on it, which will cause us to pause until the thread exits.
        That will return a result that will be an Error(if thread crashed) or a Value to unwrap is the thread exited succesfully.
        I'll go ahead and assume it not crashes. */
    let sum = handle.join().unwrap();
    /*match handle.join() {
        Ok(result) => { result },
        Err(e) => { 0 },
    };*/
    println!("The child thread's expensive sum is {}", sum);
    /*  Note, that the main thread did not join the child thred until the main thread
        had finished printing out the letters. */

    // Time for some fun with threads and channels!  Though there is a primitive type of channel
    // in the std::sync::mpsc module, I recommend always using channels from the crossbeam crate,
    // which is what we will use here.
    //
    // 4. Uncomment the block comment below (Find and remove the `/*` and `*/`).  Examine how the
    // flow of execution works.  Once you understand it, alter the values passed to the `pause_ms()`
    // calls so that both "Thread B" outputs occur before "Thread A" outputs.

    
    let (tx, rx) = channel::unbounded();
    /* unbounded means that the channel will buffer as many values as it can until it runs out of memory. */
    // Cloning a channel makes another variable connected to that end of the channel so that you can
    // send it to another thread.
    let tx2 = tx.clone();

    let handle_a = thread::spawn(move || {
        pause_ms(400);  // have to shift even 200 ms compared to B cause it is a non-raltime OS
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });

    pause_ms(100); // Make sure Thread A has time to get going before we spawn Thread B

    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    // Using a Receiver channel as an iterator is a convenient way to get values until the channel
    // gets closed.  A Receiver channel is automatically closed once all Sender channels have been
    // closed.  Both our threads automatically close their Sender channels when they exit and the
    // destructors for the channels get automatically called.
    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    // Join the child threads for good hygiene.
    handle_a.join().unwrap();
    /*  technically these are not necessary, cause the main thread would never get pass 
        the receiving loop until all of the transmitting sides had been closed. */
    handle_b.join().unwrap();
    
    pause_ms(1000);
    
    // Challenge: Make two child threads and give them each a receiving end to a channel.  From the
    // main thread loop through several values and print each out and then send it to the channel.
    // On the child threads print out the values you receive. Close the sending side in the main
    // thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`).  Join
    // the child threads.

    /*  This would not work with mpsc channels.
        That is the primary reason we use crossbeam channels. */
    let (tx, rx) = channel::unbounded();
    let rx_c = rx.clone();
    let rx_d = rx.clone();
    
    let handle_c = thread::spawn(move || {
        for msg in rx_c {
            println!("Thread c: Received {}", msg);
        }
    });
    
    pause_ms(100);
    
    let handle_d = thread::spawn(move || {
        for msg in rx_d {
            println!("Thread d: Received {}", msg);
        }
    });
    
    for num in 0..10 {
        tx.send(num.to_string()).unwrap();
        pause_ms(100);
    }
    drop(tx);
    
    handle_c.join().unwrap();
    handle_d.join().unwrap();
    
    println!("Main thread: Exiting.")
}
```
