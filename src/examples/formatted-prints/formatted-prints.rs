fn main() {
  // normal print
  println!("print");
  // interpolation
  println!("{} days to go", 3);
  // positional interpolation
  println!("{0} and {1}", "this", "that");
  // variable interpolation
  println!("{this} and {that}", this = "what is that", that = "what is that");
  // prints base 10
  println!("Base 10: {}", 10);
  println!("{number:>5}", number=1);
  println!("{number:0<5}", number=1);
  println!("{number:0>width$}", number=1, width=5);

  struct Structure(i32);

  let number: f64 = 1.0;
  let width: usize = 5;
  println!("{number:>width$}");

  #[derive(Debug)]
  struct Person<'a> {
    name: &'a str,
    age: u8,
  }

  let name = "Prajwal";
  let age = 27;
  let person = Person { name, age };
  println!("{:#?}", person) 
}