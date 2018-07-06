use std::cmp::Ordering;

pub fn find<C, T>(elements: C, needle: T) -> Option<usize>
where
    C: AsRef<[T]>,
    T: Ord,
{
    let mut base = 0usize;
    let mut slice: &[T] = elements.as_ref();

    loop {
        let (head, tail) = slice.split_at(slice.len() >> 1);
        if let Some(middle_element) = tail.first() {
            match middle_element.cmp(&needle) {
                Ordering::Less => {
                    base += head.len() + 1;
                    slice = &tail[1..];
                }
                Ordering::Greater => slice = head,
                Ordering::Equal => {
                    return Some(base + head.len());
                }
            }
        } else {
            return None;
        }
    }
}
