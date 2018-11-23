fn main() {
  string_literal();
  // s is out of scope. Not valid anymore. Popped off the stack
  string_type();
  data_interaction();
  clone_variables();

  let s = String::from("Ownership and functions");  // s comes into scope
  takes_ownership(s);             // s's value moves into the function...
                                  // ... and so is no longer valid here
  let x = 5;                      // x comes into scope
  makes_copy(x);                  // x would move into the function,
                                  // but i32 is Copy, so it’s okay to still
                                  // use x afterward
  return_values_and_scope();
  references();
  borrowing();
  no_dangle();
  let string = String::from("hola que tal");
  println!("{}", first_word(&string));
  string_slices_as_parameters();
  let a = [1, 2, 3, 4, 5];
  let _slice = &a[1..3];
}


// 4.1. OWNERSHIP
fn string_literal() {
  let _s = "hello"; // _s is valid until it goes out of scope. Pushed onto the stack.
}

fn string_type() {
  let mut s = String::from("hello"); // String type

  s.push_str(", world!"); // push_str() appends a literal to a String
  println!("{}", s); // This will print `hello, world!`
}

fn data_interaction() {
  let _x = 5; // Pushed onto the stack
  let _y = _x; // Copy of x pushed onto the stack

  let s1 = String::from("hello string type"); //
  // A String is made of a pointer, length and capacity.
  // This is saved in the stack and the pointer references data stored on the heap.
  let s2 = s1; // We make a copy of s1 so the reference stays the same.
  // When s1 and s2 go out of scope, the heap memory will be freed for those variables, but since they both point to the same
  // memory location, it could be a problem - known as 'double free' error.
  // But Rust considers s1 to be no longer valid and won't try to free anything when s1 goes out of scope.
  // println!("{}, world!", s1); <- This wouldn't compile because s1 was moved to s2.
  println!("{}, world!", s2);
  // So now when s2 goes out of scope the memory will be freed with no issues.
}

fn clone_variables() {
  // To copy stack data and heap data
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn return_values_and_scope() {
  let _s1 = gives_ownership();         // gives_ownership moves its return
                                      // value into s1

  let s2 = String::from("hello");     // s2 comes into scope

  let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                      // takes_and_gives_back, which also
                                      // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope
    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}


// 4.2. REFERENCES AND BORROWING
fn references() {
  let s1 = String::from("references");
  let len = calculate_length(&s1); // calculate_length borrows s1

  println!("The length of '{}' is {}.", s1, len);
}

// Takes reference of the paramater instead of ownership.
fn calculate_length(s: &String) -> usize { // &: referencing operator, *:dereferencing operator
    s.len()
}
// s goes out of scope but it doesn't have ownership of the value it refers to, so nothing happens.


fn borrowing() {
  let mut car = String::from("Car");
  //paint(&car); // You can't mutate a borrowed value
  paint_mutable(&mut car); // mutable borrow
  println!("{}", car);
}
// You can not borrow the same value more than one as a mutable reference on the same scope.
// Also combining mutable and immutable references won't compile.

// fn paint(car: &String) {
//   car.push_str(" painted");
// }

fn paint_mutable(car: &mut String) {
  car.push_str(" painted")
}

// fn dangle() -> &String { // dangle returns a reference to a String
//   let s = String::from("hello"); // s is a new String
//   &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//      Danger! It does not compile.
// It's trying to return a reference to a variable that will be out of scope, so non existant.

fn no_dangle() -> String {
  let s = String::from("hello");
  s
} // Ownership is moved out, and no memory is deallocated


// 4.3. SLICES
// String slice
fn first_word(s: &str) -> &str { // returns string slice
  let bytes = s.as_bytes(); // String to Array of bytes

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }
  &s[..]
}

// let s = "Hello, world!"; <- type is &str
// String literals are slices. &str is an immutable reference

fn string_slices_as_parameters() {
  let _my_string = String::from("hello world");

  // first_word works on slices of `String`s
  let _word = first_word(&_my_string[..]);
  let _my_string_literal = "hello world";

  // first_word works on slices of string literals
  let _word = first_word(&_my_string_literal[..]);

  // Because string literals *are* string slices already,
  // this works too, without the slice syntax!
  let _word = first_word(_my_string_literal);
}