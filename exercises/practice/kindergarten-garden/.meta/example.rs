pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
   let student_names = [
       "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
       "Kincaid", "Larry",
   ];
   let index = student_names
       .iter()
       .position(|&name| name == student)
       .unwrap();

   let mut lines = diagram
       .lines()
       .map(|line| line.chars().map(plant_from_char));

   let start = index * 2;
   let mut first_row = lines.next().unwrap();
   let mut second_row = lines.next().unwrap();

   vec![
       first_row.nth(start).unwrap(),
       first_row.next().unwrap(),
       second_row.nth(start).unwrap(),
       second_row.next().unwrap(),
   ]
}

fn plant_from_char(c: char) -> &'static str {
   match c {
       'R' => "radishes",
       'C' => "clover",
       'G' => "grass",
       'V' => "violets",
       _ => "No such plant",
   }
}
