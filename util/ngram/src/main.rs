use ngrammatic::{CorpusBuilder, Pad};

fn main() {
    let mut args = std::env::args();
    let exercises = args.nth(1).expect("Missing exercises argument");
    let slug = args.nth(0).expect("Missing slug argument");
    let exercises: Vec<&str> = exercises
        .split(|c: char| c.is_whitespace() || c == '\n')
        .collect();
    let mut corpus = CorpusBuilder::new().arity(2).pad_full(Pad::Auto).finish();

    for exercise in exercises.iter() {
        corpus.add_text(exercise);
    }

    if let Some(top_result) = corpus.search(&slug, 0.25).first() {
        if top_result.similarity > 0.99 {
            println!("{}", top_result.text);
        } else {
            println!(
                "{} - There is an exercise with a similar name: '{}' [{:.0}% match]",
                slug,
                top_result.text,
                top_result.similarity * 100.0
            );
        }
    } else {
        println!("Couldn't find any exercise similar to this: {}", slug);
    }
}
