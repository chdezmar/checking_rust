// https://doc.rust-lang.org/book/second-edition/ch03-00-common-programming-concepts.html
fn main() {
  // 3.1. Variables and Mutability
  // constant
  const MAX_POINTS: u32 = 100_000;
  println!("The value of MAX_POINTS is: {}", MAX_POINTS);

  // mutable variable
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);

  // shadowing variables
  // we can reassign type, variable stays immutable after new assignment with let
  let x = 5;
  let x = x + 1;
  let x = x * 2;
  println!("The value of x is: {}", x);

  // 3.2. Data types
  let _guess: u32 = "42".parse().expect("message if Err");

  // __Scalar types__
  // Integer
  let integer: i32 = 32;
  println!("Signed 32-bit: {}", integer);

  // Floating-point
  // https://stackoverflow.com/a/5098598/6224895
  let floating_double_precision = 64.0; // f64
  let floating_single_precision: f32 = 32.0;
  println!("Floating-point types: {} {}", floating_double_precision, floating_single_precision);

  // Numeric operations
  let sum = integer + 10;
  println!("Sum {}", sum);
  let difference = floating_double_precision - 4.3;
  println!("Substraction {}", difference);
  let product = integer * 30;
  println!("multiplication {}", product);
  let quotient = integer / integer;
  println!("division {}", quotient);
  let remainder = floating_double_precision % 5.0;
  println!("modulo {}", remainder);

  // Boolean
  let _t = true;
  let _f: bool = false; // with explicit type annotation

  // Character
  let _c = 'z'; // single quotes, strings use double quotes
  let _z = 'ℤ'; // unicode encoding

  // __Compound types__
  // Can group multiple values into one type.
  // Tuples (different types)
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, _y, _z) = tup; // Destructuring!
  println!("The value of x is: {}", x);
  let one = tup.2; // access by index
  println!("The value of z is: {}", one);

  // Arrays (same type)
  // For a fixed collection of values of same type to allocate in the stack. Vectors can change in size
  let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
  let june = months[5];
  println!("The sixth month is: {}", june);
  // Dealing with out-of-range access
  // println!("Can't compile if trying to access > array lenght: {}", months[12]);
  // let a = [1, 2, 3, 4, 5];
  // let index = 10;
  // let _element = a[index];
  // println!("In this case compilation works but fails at runtime: {}", element);
}
