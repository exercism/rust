# Instructions

In this exercise you'll be writing code to print name badges for factory employees.

## 1. Print a badge for an employee

Employees have an ID, name and department name. Employee badge labels are formatted as follows: `"[id] - name - DEPARTMENT"`. Implement the `print_name_badge()` function to return an employee's badge label:

```rust
print_name_badge(734, "Ernest Johnny Payne", "Strategic Communication");
// => "[734] - Ernest Johnny Payne - STRATEGIC COMMUNICATION"
```

Note that the department should be uppercased on the label.

## 2. Print a badge for a new employee

Due to a quirk in the computer system, new employees occasionally don't yet have an ID when they start working at the factory. As badges are required, they will receive a temporary badge without the ID prefix. Modify the `print_name_badge()` function to support new employees that don't yet have an ID:

```rust
print_name_badge(None, "Jane Johnson", "Procurement");
// => "Jane Johnson - PROCUREMENT"
```

## 3. Print a badge for the owner

Even the factory's owner has to wear a badge at all times. However, an owner does not have a department. In this case, the label should print `"OWNER"` instead of the department name. Modify the `print_name_badge()` function to print a label for the owner:

```rust
print_name_badge(254, "Charlotte Hale", None);
// => "[254] - Charlotte Hale - OWNER"
```

Note that it is possible for the owner to also be a new employee:

```rust
print_name_badge(None, "Charlotte Hale", None);
// => "Charlotte Hale - OWNER"
```
