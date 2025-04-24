const CHILDREN: [&str; 12] = [
    "Alice", 
    "Bob", 
    "Charlie", 
    "David", 
    "Eve", 
    "Fred", 
    "Ginny", 
    "Harriet", 
    "Ileana", 
    "Joseph", 
    "Kincaid", 
    "Larry"
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    // let mut resres = Vec::new();

    let idx = CHILDREN.iter().position(|&child| child == student).unwrap() * 2;

    println!("{}", idx);

    diagram.lines().flat_map(|line| {
        println!("{}", line);
        line[idx..=idx+1].chars().map(|c| 
            match c {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                _ => "violets",
            }
        )
    })
    .collect()

    // resres.push("grass");

    // println!("{}", diagram);

    // resres

}
