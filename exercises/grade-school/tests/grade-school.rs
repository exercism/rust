use grade_school as school;

fn some_strings(v: &[&str]) -> Option<Vec<String>> {
    Some(v.iter().map(|s| s.to_string()).collect())
}

#[test]
fn test_grades_for_empty_school() {
    let s = school::School::new();
    assert_eq!(vec![], s.grades());
}

#[test]
#[ignore]
fn test_grades_for_one_student() {
    let mut s = school::School::new();
    s.add(2, "Aimee");
    assert_eq!(vec![2], s.grades());
}

#[test]
#[ignore]
fn test_grades_for_several_students_are_sorted() {
    let mut s = school::School::new();
    s.add(2, "Aimee");
    s.add(7, "Logan");
    s.add(4, "Blair");
    assert_eq!(vec![2, 4, 7], s.grades());
}

#[test]
#[ignore]
fn test_grades_when_several_students_have_the_same_grade() {
    let mut s = school::School::new();
    s.add(2, "Aimee");
    s.add(2, "Logan");
    s.add(2, "Blair");
    assert_eq!(vec![2], s.grades());
}

#[test]
#[ignore]
fn test_grade_for_empty_school() {
    let s = school::School::new();
    assert_eq!(None, s.grade(1));
}

#[test]
#[ignore]
fn test_grade_when_no_students_have_that_grade() {
    let mut s = school::School::new();
    s.add(7, "Logan");
    assert_eq!(None, s.grade(1));
}

#[test]
#[ignore]
fn test_grade_for_one_student() {
    let mut s = school::School::new();
    s.add(2, "Aimee");
    assert_eq!(some_strings(&["Aimee"]), s.grade(2));
}

#[test]
#[ignore]
fn test_grade_returns_students_sorted_by_name() {
    let mut s = school::School::new();
    s.add(2, "James");
    s.add(2, "Blair");
    s.add(2, "Paul");
    assert_eq!(some_strings(&["Blair", "James", "Paul"]), s.grade(2));
}

#[test]
#[ignore]
fn test_add_students_to_different_grades() {
    let mut s = school::School::new();
    s.add(3, "Chelsea");
    s.add(7, "Logan");
    assert_eq!(vec![3, 7], s.grades());
    assert_eq!(some_strings(&["Chelsea"]), s.grade(3));
    assert_eq!(some_strings(&["Logan"]), s.grade(7));
}
