use csv_builder::*;

#[test]
fn test_no_escaping() {
    let mut builder = CSVRecordBuilder::new();

    builder.add("ant");
    builder.add("bat");
    builder.add("cat");

    let list = builder.build();
    //Note that from now on we can do nothing now with builder.
    assert_eq!("ant,bat,cat", &list);
}

#[test]
#[ignore]
fn test_quote() {
    let mut builder = CSVRecordBuilder::new();

    builder.add("ant");
    builder.add("ba\"t");
    builder.add("cat");

    let list = builder.build();
    //Note that from now on we can do nothing now with builder.
    assert_eq!(r#"ant,"ba""t",cat"#, &list);
}

#[test]
#[ignore]
fn test_new_line() {
    let mut builder = CSVRecordBuilder::new();

    builder.add("ant");
    builder.add("ba\nt");

    let list = builder.build();
    assert_eq!("ant,\"ba\nt\"", &list);
}
#[test]
#[ignore]
fn test_comma() {
    let mut builder = CSVRecordBuilder::new();

    builder.add("ant");
    builder.add("ba,t");

    let list = builder.build();
    assert_eq!("ant,\"ba,t\"", &list);
}

#[test]
#[ignore]
fn test_empty() {
    let builder = CSVRecordBuilder::new();
    let list = builder.build();
    assert!(list.is_empty());
}
