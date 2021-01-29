use diamond::*;

#[test]
fn test_a() {
    assert_eq!(get_diamond('A'), vec!["A"]);
}

#[test]
#[ignore]
fn test_b() {
    #[rustfmt::skip]
    assert_eq!(
        get_diamond('B'),
        vec![
            " A ",
            "B B",
            " A ",
        ]
    );
}

#[test]
#[ignore]
fn test_c() {
    #[rustfmt::skip]
    assert_eq!(
        get_diamond('C'),
        vec![
            "  A  ",
            " B B ",
            "C   C",
            " B B ",
            "  A  ",
        ]
    );
}

#[test]
#[ignore]
fn test_d() {
    #[rustfmt::skip]
    assert_eq!(
        get_diamond('D'),
        vec![
            "   A   ",
            "  B B  ",
            " C   C ",
            "D     D",
            " C   C ",
            "  B B  ",
            "   A   ",
        ]
    );
}

#[test]
#[ignore]
fn test_e() {
    assert_eq!(
        get_diamond('Z'),
        vec![
            "                         A                         ",
            "                        B B                        ",
            "                       C   C                       ",
            "                      D     D                      ",
            "                     E       E                     ",
            "                    F         F                    ",
            "                   G           G                   ",
            "                  H             H                  ",
            "                 I               I                 ",
            "                J                 J                ",
            "               K                   K               ",
            "              L                     L              ",
            "             M                       M             ",
            "            N                         N            ",
            "           O                           O           ",
            "          P                             P          ",
            "         Q                               Q         ",
            "        R                                 R        ",
            "       S                                   S       ",
            "      T                                     T      ",
            "     U                                       U     ",
            "    V                                         V    ",
            "   W                                           W   ",
            "  X                                             X  ",
            " Y                                               Y ",
            "Z                                                 Z",
            " Y                                               Y ",
            "  X                                             X  ",
            "   W                                           W   ",
            "    V                                         V    ",
            "     U                                       U     ",
            "      T                                     T      ",
            "       S                                   S       ",
            "        R                                 R        ",
            "         Q                               Q         ",
            "          P                             P          ",
            "           O                           O           ",
            "            N                         N            ",
            "             M                       M             ",
            "              L                     L              ",
            "               K                   K               ",
            "                J                 J                ",
            "                 I               I                 ",
            "                  H             H                  ",
            "                   G           G                   ",
            "                    F         F                    ",
            "                     E       E                     ",
            "                      D     D                      ",
            "                       C   C                       ",
            "                        B B                        ",
            "                         A                         ",
        ]
    );
}
