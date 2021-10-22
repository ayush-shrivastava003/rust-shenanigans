// rust-by-example is ayush's code
// https://doc.rust-lang.org/rust-by-example/index.html
fn main() {
  // single line comment
  /*
  multi
  line
  comment
  */
  let x = 5 + /* 90 + */ 5;
  println!("Hello, World!");
  println!("x is: {}", x);

  // formatted strings
  println!("Hello, {}!", "ayush");

  // formatted strings: positional args
  println!("{0} -> this is the first provided argument. {1} -> this is the second provided argument", "hello", "hi");

  // formatted strings: keyword args
  println!("{name} is doing {activity}!", activity="coding", name="ayush");

  // special formatting. :b means binary
  println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

  



}