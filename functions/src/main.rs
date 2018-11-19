fn main() {
  println!("Hello, world!");
  snake_case_function(5, -10);
  function_definition();
  expression_inside_function();
  let num = 4;
  let double = double_it(num);
  println!("The double of {} is {}", num, double);
}

fn snake_case_function(x: i32, y: i32) {
  println!("The value of x is: {} and the value of y is: {}", x, y);
}

// Functions are made up of Statements and optionally end in Expressions
fn function_definition() {
  let _statement: &'static str = "I am a statement my function too :)";
}
// Expressions evaluate to something while Statements don't
// We must explicitly write `return` or avoid ending our statement with a semicolon to return values
fn expression_inside_function() {
  let _x = 5;
  let y = {
    let _x = 3;
    _x + 1
  };

  println!("The value of y is: {}", y);
}

// Specifying return value's data type
fn double_it(num: i32) -> i32 {
    num * 2
}

