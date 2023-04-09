#[test]
pub fn label_for_employee() {
    assert_eq!(
        "[17] - Ryder Herbert - MARKETING",
        tim_from_marketing::print_name_badge(Some(17), "Ryder Herbert", Some("Marketing"))
    );
}

#[test]
#[ignore]
pub fn label_for_new_employee() {
    assert_eq!(
        "Bogdan Rosario - MARKETING",
        tim_from_marketing::print_name_badge(None, "Bogdan Rosario", Some("Marketing"))
    );
}

#[test]
#[ignore]
pub fn label_for_owner() {
    assert_eq!(
        "[59] - Julie Sokato - OWNER",
        tim_from_marketing::print_name_badge(Some(59), "Julie Sokato", None)
    );
}

#[test]
#[ignore]
pub fn label_for_new_owner() {
    assert_eq!(
        "Amare Osei - OWNER",
        tim_from_marketing::print_name_badge(None, "Amare Osei", None)
    );
}
