use std::collections::HashMap;
use std::io::{BufferedReader, EndOfFile, File, IoResult};

#[deriving(Copy, Clone)]
struct TeamResult {
    wins: uint,
    draws: uint,
    losses: uint,
}

impl TeamResult {
    fn new(w: uint, t: uint, l: uint) -> TeamResult {
        TeamResult { wins: w, draws: t, losses: l }
    }
    fn add_win(&mut self) {
        self.wins += 1;
    }
    fn add_draw(&mut self) {
        self.draws += 1;
    }
    fn add_loss(&mut self) {
        self.losses += 1;
    }
}

pub fn tally(input_filename: &Path, output_filename: &Path) -> IoResult<uint> {
    let mut reader = BufferedReader::new(File::open(input_filename));
    let mut count = 0;
    let mut results: HashMap<String, TeamResult> = HashMap::new();
    loop {
        let line = match reader.read_line() {
            Err(ref e) if e.kind == EndOfFile => break,
            Err(e) => return Err(e),
            Ok(l) => l
        };
        match line.as_slice().trim_right().split(';').collect::<Vec<&str>>().as_slice() {
            [team1, team2, outcome] => {
                match outcome {
                    "win" => {
                        add_win(&mut results, team1.to_string());
                        add_loss(&mut results, team2.to_string());
                        count += 1;
                    },
                    "draw" => {
                        add_draw(&mut results, team1.to_string());
                        add_draw(&mut results, team2.to_string());
                        count += 1;
                    },
                    "loss" => {
                        add_loss(&mut results, team1.to_string());
                        add_win(&mut results, team2.to_string());
                        count += 1;
                    },
                    _ => () // Ignore bad lines
                }
            },
            _ => () // Ignore bad lines
        }
    }
    try!(write_tally(&results, output_filename));
    Ok(count)
}

fn write_tally(results: &HashMap<String, TeamResult>, output_filename: &Path) -> IoResult<()> {
    let mut v: Vec<(String, uint, TeamResult, uint)> = Vec::new();
    for (team, r) in results.iter() {
        let games = r.wins + r.draws + r.losses;
        let points = r.wins * 3 + r.draws;
        v.push((team.clone(), games, r.clone(), points));
    }
    // Sort by points, then games played, in reverse order.
    v.sort_by(|a, b| 
              match a.ref3().cmp(b.ref3()).reverse() {
                  Equal => a.ref1().cmp(b.ref1()).reverse(),
                  other => other
              });
    let mut f = File::create(output_filename);
    try!(writeln!(&mut f, "{:30s} | MP |  W |  D |  L |  P", "Team"));
    for &(ref team, games, r, points) in v.iter() {
        try!(writeln!(&mut f, "{:30s} | {:2u} | {:2u} | {:2u} | {:2u} | {:2u}",
                      team.as_slice(), games, r.wins, r.draws, r.losses, points));
    }
    Ok(())
}

fn add_win(results: &mut HashMap<String, TeamResult>, team: String) {
    results.insert_or_update_with(team, TeamResult::new(1, 0, 0), |_, tr| tr.add_win());
}

fn add_draw(results: &mut HashMap<String, TeamResult>, team: String) {
    results.insert_or_update_with(team, TeamResult::new(0, 1, 0), |_, tr| tr.add_draw());
}

fn add_loss(results: &mut HashMap<String, TeamResult>, team: String) {
    results.insert_or_update_with(team, TeamResult::new(0, 0, 1), |_, tr| tr.add_loss());
}
