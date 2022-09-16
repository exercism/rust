# Instructions

Search files for lines matching a search string and return all matching lines.

The Unix [`grep`](http://pubs.opengroup.org/onlinepubs/9699919799/utilities/grep.html) command searches for lines in files that match a regular expression. Your task is to implement a simplified `grep` function, which supports searching for literal strings.

The `grep` function takes three arguments:

1. The string to search for.
2. Zero or more flags to customize its behavior.
3. One or more files to search in.

It then reads the contents of the specified files (in the order specified),
finds the lines that contain the search string, 
and finally returns those lines (in the order in which they were found).

## Flags

The `grep` function should support the following flags:

- `-n` Prepend the line number to each line in the output.
- `-l` Instead of lines, only output the names of the files that contain at least one matching line.
- `-i` Match using a case-insensitive comparison.
- `-v` Invert the program -- collect all lines that fail to match the pattern.
- `-x` Search instead for lines matching the search string entirely, instead of merely containing it.
