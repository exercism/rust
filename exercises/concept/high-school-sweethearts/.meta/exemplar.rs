pub fn display_single_line(student_a: &str, student_b: &str) -> String {
    let line_length: u8 = 61;
    let line_part_length: usize = ((line_length - 1) / 2 - 1).into();
    return format!(
        "{student_a:>line_part_length$} ♡ {student_b:<line_part_length$}",
        student_a = student_a,
        student_b = student_b,
        line_part_length = line_part_length,
    );
}

pub fn display_banner(student_a: &str, student_b: &str) -> String {
    return format!(
        "
     ******       ******
   **      **   **      **
 **         ** **         **
**            *            **
**                         **
**{0:>2$}  +  {1:<2$}**
 **                       **
   **                   **
     **               **
       **           **
         **       **
           **   **
             ***
              *
",
        student_a.trim(),
        student_b.trim(),
        10
    );
}
