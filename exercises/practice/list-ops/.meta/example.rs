pub fn append(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
    let mut output = vec![];

    for item in a {
        output.push(item);
    }

    for item in b {
        output.push(item);
    }

    output
}

pub fn concat<T>(list: Vec<Vec<T>>) -> Vec<T> {
    let mut output = vec![];

    for sublist in list {
        for item in sublist {
            output.push(item);
        }
    }

    output
}

pub fn filter(list: Vec<u32>, function: impl Fn(u32) -> bool) -> Vec<u32> {
    let mut output = vec![];

    for item in list {
        if function(item) {
            output.push(item);
        }
    }

    output
}

pub fn length(list: Vec<u32>) -> usize {
    let mut count = 0;

    for _ in list {
        count += 1;
    }

    count
}

pub fn map(list: Vec<u32>, function: impl Fn(u32) -> u32) -> Vec<u32> {
    let mut output = vec![];

    for item in list {
        output.push(function(item));
    }

    output
}

pub fn foldl(list: Vec<f64>, initial: f64, function: impl Fn(f64, f64) -> f64) -> f64 {
    let mut acc = initial;

    for item in list {
        acc = function(acc, item);
    }

    acc
}

pub fn foldr(list: Vec<f64>, initial: f64, function: impl Fn(f64, f64) -> f64) -> f64 {
    let mut acc = initial;

    for item in reverse(list) {
        acc = function(acc, item);
    }

    acc
}

pub fn reverse<T>(mut list: Vec<T>) -> Vec<T> {
    list.reverse();

    list
}
