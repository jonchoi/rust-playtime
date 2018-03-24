// Source: https://learnxinyminutes.com/docs/rust/

// [ ] get syntastic

// Comment.

/// Documentation
/// # Examples
///
/// ```
/// let five = 5
/// ```

///////////////
// 1. Basics //
///////////////

// Functions
// `i32` is 32-bit signed integer.
// https://en.wikipedia.org/wiki/32-bit#Range_for_storing_integers
fn add2(x: i32, y:i32) -> i32 {
    // Implicit return (no semicolon)
    x + y
}

// Main function
fn main() {
    // Numbers //

    // Immutable bindings
    let x: i32 = 1;

    // Integer/float suffixes
    let y: i32 = 13i32;
    let f: f64 = 1.3f64;

    // Type reference
    let implicit_x = 1;
    let implicit_f = 1.13;

    // Arithmetic
    let sum = x + y + 13;

    // Mutable variable
    let mut mutable = 1;
    mutable = 4;
    mutable += 2;

    // Strings //

    // String literals
    let x: &str = "hello world!";

    println!("{} {}", f, x);

    // `String` – heap-allocated string
    let s: String = "hello world".to_string();

    // string slice
    let s_slice: &str = &s;
    println!("{} {}", s, s_slice);

    // Vectors/arrays //
    
    // A fixed-size array
    let four_ints: [i32; 4] = [1, 2, 3, 4];

    // A dynamic array (vector)
    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    vector.push(5);

    // A slice – an immutable view into a vector or array
    // Like string slice but for vectors
    let slice: &[i32] = &vector;

    println!("{:?} {:?}", vector, slice);

    // Tuples //
    let x: (i32, &str, f64) = (1, "hello", 3.4);

    let (a, b, c) = x;
    println!("{} {} {}", a, b, c);

    //////////////
    // 2. Types //
    //////////////

    // Struct
    struct Point {
        x: i32,
        y: i32,
    }

    let origin: Point = Point { x: 0, y: 0 };

    // 'tuple struct'
    struct Point2(i32, i32);

    let origin2 = Point2(0, 0);

    // C-style enum
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    let up = Direction::Up;

    // Enum with fields
    enum OptionalI32 {
        AnI32(i32),
        Nothing,
    }

    let two: OptionalI32 = OptionalI32::AnI32(2);
    let nothing = OptionalI32::Nothing;

    // Generics //
    
    struct Foo<T> { bar: T }

    // Option
    enum Optional<T> {
        SomeVal(T),
        NoVal,
    }

    // Methods //
    
    // Cool!
    impl<T> Foo<T> {
        // explicit `self`
        fn get_bar(self) -> T {
            self.bar
        }
    }

    let a_foo = Foo { bar: 1 };
    println!("{}", a_foo.get_bar());

    // Traits (interfaces/typeclasses)

    trait Frobnicate<T> {
        fn frobnicate(self) -> Option<T>;
    }

    impl<T> Frobnicate<T> for Foo<T> {
        fn frobnicate(self) -> Option<T> {
            // TODO: What is `Some`?
            Some(self.bar)
        }
    }

    let another_foo = Foo { bar: 2 };
    println!("{:?}", another_foo.frobnicate());

    /////////////////////////
    // 3. Pattern matching //
    /////////////////////////

    let foo = OptionalI32::AnI32(1);
    
    // similar to switch / case ?
    match foo {
        OptionalI32::AnI32(n) => println!("it's an i32: {}", n),
        OptionalI32::Nothing => println!("it's nothing"),
    }

    // Advaned pattern matching
    struct FooBar { x: i32, y: OptionalI32 }
    let bar = FooBar { x: 15, y: OptionalI32::AnI32(32) };

    match bar {
        FooBar { x: 0, y: OptionalI32::AnI32(0) } => 
            println!("The numbers are zero!"),
        FooBar { x: n, y: OptionalI32::AnI32(m) } if n==m =>
            println!("The nubmers are the same!"),
        FooBar { x: n, y: OptionalI32::AnI32(m) } =>
            println!("Different numbers: {}, {}", n, m),
        FooBar { x: _, y: OptionalI32::Nothing } =>
            println!("The second number is nothing"),
    }

    /////////////////////
    // 4. Control Flow //
    /////////////////////
   
    // `for` loop
    let array = [1, 2, 3];
    // have to call `.iter()`
    for i in array.iter() {
        println!("{}", i);
    }

    // Ranges
    for i in 0u32..10 { // unsigned int
        print!("{} ", i);
    }
    println!("");
    for i in 0i32..10 { // signed int
        print!("{} ", i);
    }
    println!("");

    // `if`
    if 1 == 1 {
        println!("Math be true!")
    } else {
        println!("hmmmm... what?!")
    }

    // `if` as expression
    let value = if 2 == 3 {
        "good"
    } else {
        "bad"
    };
    println!("{}", value);

    // `while` loop
    while 1 == 2 {
        println!("The universe works... jk!");
    }

    // Infinite loop
    // loop {
    //      something;
    // }


    /////////////////////////////////
    // 5. Memory safety & pointers //
    /////////////////////////////////

    // Owned pointer
    let mut mine: Box<i32> = Box::new(3);
    *mine = 5; // dereference
    let mut now_its_mine = mine;
    *now_its_mine += 2;

    println!("{}", now_its_mine); // 7
    // println!("{}", mine); // wouldn't compile because doesn't own it now.
    
    // Reference – an immutable pointer that refers to other data
    let mut var = 4;
    var = 3;
    let ref_var: &i32 = &var;

    println!("{}", var); // this *can* be used
    println!("{}", *ref_var);
    // var = 5; // this would not compile because var is borrowed
    // *ref_var = 6; // this would not because `ref_var` is an immutable reference
    
    // Mutable Reference
    // can't be used at all
    let mut var2 = 4;
    let ref_var2: &mut i32 = &mut var2;
    *ref_var2 += 2;
    println!("{}", *ref_var2); // var2 wouldn't compile
    // ref_var2 stores reference to i32 not the value
    // var2 = 2; woudln't compile because var2 is borrowed.
    
    // TODO: review.
}


