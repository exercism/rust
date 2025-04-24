use std::cmp::Ordering;

// pub fn find(array: &[i32], key: i32) -> Option<usize> {

//     println!("{:?}", array);
//     println!("{}", 1 /2 );

//     if array.len() == 0 {
//         return None;
//     }

//     let index_value_middle = array.len() / 2;
//     let value_middle = array.get(index_value_middle).unwrap();
//     match key.cmp(value_middle) {
//         Ordering::Less => {
//             return find(&array[0..index_value_middle], key);
//         },
//         Ordering::Greater => {
//             return find(&array[index_value_middle+1..], key).map(|i| i + index_value_middle + 1);
//         },
//         Ordering::Equal => {
//             return Some(index_value_middle);
//         }
//     }

// }

pub fn find<R: AsRef<[T]>, T: Ord>(array: R, key: T) -> Option<usize> {

    let array = array.as_ref();

    if array.is_empty() {
        return None;
    }

    let index_value_middle = array.len() / 2;
    let value_middle = array.get(index_value_middle).unwrap();
    match key.cmp(value_middle) {
        Ordering::Less => {
            find(&array[0..index_value_middle], key)
        },
        Ordering::Greater => {
            find(&array[index_value_middle+1..], key).map(|i| i + index_value_middle + 1)
        },
        Ordering::Equal => {
            Some(index_value_middle)
        }
    }

}