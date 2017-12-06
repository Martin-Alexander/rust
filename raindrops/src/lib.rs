fn has_as_factor(number :usize, factor: usize) -> bool {
  return number % factor == 0;
}

pub fn raindrops(n: usize) -> String {
  let mut result_string: String = String::new();

  if has_as_factor(n, 3) {
    result_string.push_str("Pling");
  }

  if has_as_factor(n, 5) {
    result_string.push_str("Plang");;
  }

  if has_as_factor(n, 7) {
    result_string.push_str("Plong");
  } 
  
  if result_string.is_empty() {
    return n.to_string();  
  } else {
    return result_string;
  }
}
