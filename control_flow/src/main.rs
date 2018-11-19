fn main() {
  println!("Hello, Control Flow!");
  if_statement();
  fizz_buzz();
  different_types_in_variable();
  for_loop();
  while_loop();
  iterate_through_collection()
}

fn if_statement() {
  let number = 3;

  // Always provide if with a Boolean as its condition
  if number < 5 {
      println!("condition was true");
  } else {
      println!("condition was false");
  }
}
fn different_types_in_variable() {
  let condition = true;
  let number = if condition {
      5
  } else {
    // "six" <- returning this would not compile
    // The compiler needs to know the type of `number`
    0
  };

  println!("The value of number is: {}", number);
}

// LOOPS
// Loop
fn for_loop() {
  loop {
    println!("Break stops loop");
    break;
  }
  for number in (1..4).rev() {
        println!("{}!", number);
  }
  println!("LIFTOFF!!!");
}
// For loop
fn fizz_buzz() {
  for x in 0..100 {
    if x % 15 == 0 {
      println!("FizzBuzz");
    } else if x % 5 == 0 {
      println!("Buzz");
    } else if x % 3 == 0 {
      println!("Fizz");
    } else {
      println!("{}", x);
    }
  }
}
// While loop
fn while_loop() {
  let mut countdown = 3;
  while countdown != 0 {
    println!("{}", countdown);
    countdown -= 1;
  }
  println!("GO!");
}

fn iterate_through_collection() {
  let a = [10, 20, 30, 40, 50];

  for element in a.iter() {
    println!("the value is: {}", element);
  }
}



