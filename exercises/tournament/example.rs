use std::cmp::Ordering::Equal;
use std::collections::HashMap;

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

pub fn tally(input: &str) -> String {
    let mut results: HashMap<String, TeamResult> = HashMap::new();
    for line in input.to_string().lines() {
        let parts: Vec<&str> = line.trim_end().split(';').collect();
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
            }
            "draw" => {
                add_game_result(&mut results, team1.to_string(), GameResult::Draw);
                add_game_result(&mut results, team2.to_string(), GameResult::Draw);
            }
            "loss" => {
                add_game_result(&mut results, team1.to_string(), GameResult::Loss);
                add_game_result(&mut results, team2.to_string(), GameResult::Win);
            }
            _ => (), // Ignore bad lines
        }
    }
    write_tally(&results)
}

fn write_tally(results: &HashMap<String, TeamResult>) -> String {
    let mut v: Vec<_> = results
        .iter()
        .map(|(team, r)| {
            let games = r.wins + r.draws + r.losses;
            let points = r.wins * 3 + r.draws;
            (team, games, r, points)
        })
        .collect();
    // Sort by points descending, then name A-Z.
    v.sort_by(|a, b| match a.3.cmp(&(b.3)).reverse() {
        Equal => a.0.cmp(&(b.0)),
        other => other,
    });
    let mut lines = vec![format!("{:30} | MP |  W |  D |  L |  P", "Team")];
    lines.extend(v.iter().map(|&(ref team, games, r, points)| {
        format!(
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            team, games, r.wins, r.draws, r.losses, points
        )
    }));
    lines.join("\n")
}

fn add_game_result(results: &mut HashMap<String, TeamResult>, team: String, result: GameResult) {
    results
        .entry(team)
        .or_insert_with(TeamResult::new)
        .add_game_result(result);
}
