// https://blog.codeship.com/coming-rust-ruby/

pub struct Rectangle {
  pub width: i32,
  pub height: i32,
}

impl Rectangle {
  fn area(&self) -> i32 {
    self.width * self.height
  }
}

pub struct Circle {
  pub radius: f64,
}

/// For evaluating squareness
pub trait Squareness {
  /// Reveals the truth of ones squareness
  ///
  /// # Example
  /// ```
  /// use structs_traits::{Circle,Squareness};
  ///
  /// Circle {radius: 2.0}.is_square();
  /// ```
  fn is_square(&self) -> bool;
}

impl Squareness for Circle {
  fn is_square(&self) -> bool {
    false
  }
}

impl Squareness for Rectangle {
  fn is_square(&self) -> bool {
    self.width == self.height
  }
}

#[test]
fn it_is_square(){
  assert_eq!(Circle {radius: 2.0}.is_square(), false);
  assert_eq!(Rectangle {width: 4, height: 5}.is_square(), false);
  assert!(Rectangle {width: 4, height: 4}.is_square());
}
