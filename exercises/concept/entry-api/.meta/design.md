# Design

## Learning objectives

Students who complete this exercise should be able to...

- introduce the student to the [Entry API](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html) on `HashMap`s/`BTreeMap`s
- spot appropriate use-cases/contexts in which usage of the entry API is appropriate
- use the entry API methods `or_default` or `or_insert` to ensure an `Entry` is not `Vacant` when fetching it

This exercise may review, but is not expected to include, other usages of `HashMap`s.

## Out of scope

Students who complete this exercise should not necessarily be able to...

- use the entry API `insert` method, as it's currently only in nightly

## Concepts

- Using `HashMap::entry` to fetch and mutate HashMap entries
- Using `Entry::or_default` to handle vacant entries
- Using the above methods in conjunction with one another

## Prerequisites

- Completion of the `HashMap`s concept exercise(s).
- string-slices
- slices

## Resources to refer to

- The [HashMap::Entry](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html) docs are a great resource to refer to for more information on the entry API.

### Hints

- The [HashMap::Entry](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html) docs.

### After

- The [HashMap::Entry](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html) docs detail additional `Entry` methods such as `insert` and `and_modify` that students can dig into further.

### Representer

No specific changes expected.

### Analyzer

No specific changes expected.
