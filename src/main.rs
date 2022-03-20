use std::fs::read_to_string;

fn main() {
  let input: String = read_to_string("lines.txt").unwrap();
  for line in input.lines(){
      if starts_with_capital_letter(line) {
          println!("-{}", line);
      }
  }
}

#[allow(unused)]
fn starts_with_capital_letter(s: &str)-> bool {
    matches!(s.chars().next(), Some(c) if c.is_uppercase())
}