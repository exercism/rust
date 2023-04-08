#[test]
fn display_single_line() {
    let expected: &str = "                  Lance Green ♡ Pat Riley                    ";
    let actual = high_school_sweethearts::display_single_line("Lance Green", "Pat Riley");
    assert_eq!(expected, actual,);
}

#[test]
#[ignore]
fn display_banner_whitespace_on_right() {
    let expected: &str = "
     ******       ******
   **      **   **      **
 **         ** **         **
**            *            **
**                         **
**     L. G.  +  P. R.     **
 **                       **
   **                   **
     **               **
       **           **
         **       **
           **   **
             ***
              *
";
    let actual = high_school_sweethearts::display_banner("L. G. ", "P. R. ");
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
fn display_banner_whitespace_on_both_sides() {
    let expected: &str = "
     ******       ******
   **      **   **      **
 **         ** **         **
**            *            **
**                         **
**     L. G.  +  P. R.     **
 **                       **
   **                   **
     **               **
       **           **
         **       **
           **   **
             ***
              *
";
    let actual = high_school_sweethearts::display_banner(" L. G. ", " P. R. ");
    assert_eq!(expected, actual);
}
