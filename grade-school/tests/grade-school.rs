extern crate school;

fn stringvec_to_strvec(v: &Vec<String>) -> Vec<&str> {
    v.iter().map(|s| s.as_ref()).collect()
}

#[test]
fn test_add_student() {
    let mut s = school::School::new();
    s.add(2, "Aimee");
    assert_eq!(s.grades(), vec!(2));
    assert_eq!(s.grade(2).map(|v| stringvec_to_strvec(v)),
                Some(vec!("Aimee")))
}

#[test]
#[ignore]
fn test_add_more_students_in_same_class() {
    let mut s = school::School::new();
    s.add(2, "James");
    s.add(2, "Blair");
    s.add(2, "Paul");
    assert_eq!(s.grades(), vec!(2));
    assert_eq!(s.grade(2).map(|v| stringvec_to_strvec(v)),
               Some(vec!("Blair", "James", "Paul")));
}

#[test]
#[ignore]
fn test_add_students_to_different_grades() {
    let mut s = school::School::new();
    s.add(3, "Chelsea");
    s.add(7, "Logan");
    assert_eq!(s.grades(), vec!(3, 7));
    assert_eq!(s.grade(3).map(|v| stringvec_to_strvec(v)),
               Some(vec!("Chelsea")));
    assert_eq!(s.grade(7).map(|v| stringvec_to_strvec(v)),
               Some(vec!("Logan")));
}

#[test]
#[ignore]
fn test_get_students_in_a_non_existent_grade() {
    let s = school::School::new();
    assert_eq!(s.grade(1), None);
}
