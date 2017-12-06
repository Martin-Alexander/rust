pub fn reverse(string: &str) -> String {

  let mut character_vector = Vec::new();
  let initial_string = String::from(string);

  for i in 0..initial_string.len() {
    character_vector.push(initial_string.chars().nth(initial_string.len() - i - 1).unwrap());
  }

  let reversed_string :String = character_vector.into_iter().collect();

  return reversed_string;
}
