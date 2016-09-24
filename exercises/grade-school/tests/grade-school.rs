extern crate grade_school as school;

fn stringvec_to_strvec(v: &Vec<String>) -> Vec<&str> {
    v.iter().map(|s| s.as_ref()).collect()
}

#[test]
fn test_grades_for_empty_school() {
    let s = school::School::new();
    assert_eq!(s.grades(), vec![]);
}

#[test]
#[ignore]
fn test_grades_for_one_student() {
    let mut s = school::School::new();
    s.add(2, "Aimee");
    assert_eq!(s.grades(), vec![2]);
}

#[test]
#[ignore]
fn test_grades_for_several_students_are_sorted() {
    let mut s = school::School::new();
    s.add(2, "Aimee");
    s.add(7, "Logan");
    s.add(4, "Blair");
    assert_eq!(s.grades(), vec![2, 4, 7]);
}

#[test]
#[ignore]
fn test_grades_when_several_students_have_the_same_grade() {
    let mut s = school::School::new();
    s.add(2, "Aimee");
    s.add(2, "Logan");
    s.add(2, "Blair");
    assert_eq!(s.grades(), vec![2]);
}

#[test]
#[ignore]
fn test_grade_for_empty_school() {
    let s = school::School::new();
    assert_eq!(s.grade(1), None);
}

#[test]
#[ignore]
fn test_grade_when_no_students_have_that_grade() {
    let mut s = school::School::new();
    s.add(7, "Logan");
    assert_eq!(s.grade(1), None);
}

#[test]
#[ignore]
fn test_grade_for_one_student() {
    let mut s = school::School::new();
    s.add(2, "Aimee");
    assert_eq!(s.grade(2).map(stringvec_to_strvec),
                Some(vec!["Aimee"]))
}

#[test]
#[ignore]
fn test_grade_returns_students_sorted_by_name() {
    let mut s = school::School::new();
    s.add(2, "James");
    s.add(2, "Blair");
    s.add(2, "Paul");
    assert_eq!(s.grade(2).map(stringvec_to_strvec),
               Some(vec!["Blair", "James", "Paul"]));
}

#[test]
#[ignore]
fn test_add_students_to_different_grades() {
    let mut s = school::School::new();
    s.add(3, "Chelsea");
    s.add(7, "Logan");
    assert_eq!(s.grades(), vec!(3, 7));
    assert_eq!(s.grade(3).map(stringvec_to_strvec),
               Some(vec!["Chelsea"]));
    assert_eq!(s.grade(7).map(stringvec_to_strvec),
               Some(vec!["Logan"]));
}
