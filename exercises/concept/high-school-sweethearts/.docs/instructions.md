# Instructions

In this exercise, you are going to help high school sweethearts profess their love on social media.

## 1. Display the couple's name separated by a heart

Please implement the `display_single_line()` method to take two names and display them separated by a heart.
The formatted text should be 61 characters wide, with the heart in the center of the string.

All names are guaranteed to fit well within the width of the line.

```rust
high_school_sweethearts::display_single_line("Lance Green", "Pat Riley");
// => "                  Lance Green ♡ Pat Riley                    "
```

## 2. Display the couple's initials in an ascii art heart

Implement the `display_banner()` method which displays the two sets of initials separated with a plus sign.

```rust
high_school_sweethearts::display_banner("L. G.", "P. R.");
// see the ascii art below
```

```
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
```
