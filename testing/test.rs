fn main() {
  let mut immu = 0;
  much(&mut immu);
  println!("{}", immu);
}

fn much(num: &mut i64) {
  *num = 30
}