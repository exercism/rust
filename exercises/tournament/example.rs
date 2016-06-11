use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, Result};
use std::path::Path;

enum GameResult {
    Win,
    Draw,
    Loss,
}

struct TeamResult {
    wins: u32,
    draws: u32,
    losses: u32,
}

impl TeamResult {
    fn new() -> TeamResult {
        TeamResult {
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }
    fn add_game_result(&mut self, result: GameResult) {
        match result {
            GameResult::Win => self.wins += 1,
            GameResult::Draw => self.draws += 1,
            GameResult::Loss => self.losses += 1,
        }
    }
}

pub fn tally(input_filename: &Path, output_filename: &Path) -> Result<u32> {
    let reader = BufReader::with_capacity(2048, File::open(input_filename).unwrap());
    let mut count = 0;
    let mut results: HashMap<String, TeamResult> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.trim_right().split(';').collect();
        if parts.len() != 3 {
            continue;
        }
        let team1 = parts[0];
        let team2 = parts[1];
        let outcome = parts[2];
        match outcome {
            "win" => {
                add_game_result(&mut results, team1.to_string(), GameResult::Win);
                add_game_result(&mut results, team2.to_string(), GameResult::Loss);
                count += 1;
            }
            "draw" => {
                add_game_result(&mut results, team1.to_string(), GameResult::Draw);
                add_game_result(&mut results, team2.to_string(), GameResult::Draw);
                count += 1;
            }
            "loss" => {
                add_game_result(&mut results, team1.to_string(), GameResult::Loss);
                add_game_result(&mut results, team2.to_string(), GameResult::Win);
                count += 1;
            }
            _ => (), // Ignore bad lines
        }
    }
    try!(write_tally(&results, output_filename));
    Ok(count)
}

fn write_tally(results: &HashMap<String, TeamResult>, output_filename: &Path) -> Result<()> {
    let mut v: Vec<(&String, u32, &TeamResult, u32)> = Vec::new();
    for (team, r) in results.iter() {
        let games = r.wins + r.draws + r.losses;
        let points = r.wins * 3 + r.draws;
        v.push((team, games, r, points));
    }
    // Sort by points, then games played, in reverse order.
    v.sort_by(|a, b| {
        match a.3.cmp(&(b.3)).reverse() {
            Equal => a.1.cmp(&(b.1)).reverse(),
            other => other,
        }
    });
    let mut f = try!(File::create(output_filename));
    try!(writeln!(&mut f, "{:30} | MP |  W |  D |  L |  P", "Team"));
    for &(ref team, games, r, points) in v.iter() {
        try!(writeln!(&mut f,
                      "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
                      team,
                      games,
                      r.wins,
                      r.draws,
                      r.losses,
                      points));
    }
    Ok(())
}

fn add_game_result(results: &mut HashMap<String, TeamResult>, team: String, result: GameResult) {
    match results.entry(team) {
        Entry::Vacant(entry) => {
            let mut tr = TeamResult::new();
            tr.add_game_result(result);
            entry.insert(tr);
        }
        Entry::Occupied(mut entry) => {
            (*entry.get_mut()).add_game_result(result);
        }
    };
}
