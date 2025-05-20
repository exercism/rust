use std::collections::HashMap;

#[derive(Debug)]
struct Score {
    matches_played: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32
}

impl Score {

    fn new() -> Self {
        Self {
            matches_played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0 
        }
    }

    fn add_win(&mut self) {
        self.matches_played += 1;
        self.wins += 1;
        self.points += 3;
    }

    fn add_loss(&mut self) {
        self.matches_played += 1;
        self.losses += 1;
    }

    fn add_draw(&mut self) {
        self.matches_played += 1;
        self.draws += 1;
        self.points += 1;
    }
    
}

pub fn tally(match_results: &str) -> String {
    let mut ret = String::new();
    let mut teams_scores: HashMap<String, Score> = HashMap::new();

    // Process each line
    for result in match_results.lines() {
        // Parse results from line
        let mut parts =  result.split(";");
        let team1 = parts.next().unwrap();
        let team2 = parts.next().unwrap();
        let match_result = parts.next().unwrap();

        // Update `teams_scores` according to current result
        match match_result {
            "win" => {
                // Update score of `team1`
                teams_scores.entry(team1.to_string())
                .and_modify(|v| {
                    v.add_win();
                })
                .or_insert_with(|| {
                    let mut win_score = Score::new();
                    win_score.add_win();
                    win_score
                });
                // Update score of `team2`
                teams_scores.entry(team2.to_string())
                .and_modify(|v| {
                    v.add_loss();
                })
                .or_insert_with(|| {
                    let mut loss_score = Score::new();
                    loss_score.add_loss();
                    loss_score
                }); 
            },
            "draw" => {
                // Update score for both teams (they both got a draw)
                for team in [team1, team2] {
                    teams_scores.entry(team.to_string())
                    .and_modify(|v| {
                        v.add_draw();
                    })
                    .or_insert_with(|| {
                        let mut draw_score = Score::new();
                        draw_score.add_draw();
                        draw_score
                    });
                }
            },
            "loss" => {
                // Update score of `team1`
                teams_scores.entry(team1.to_string())
                .and_modify(|v| {
                    v.add_loss();
                })
                .or_insert_with(|| {
                    let mut loss_score = Score::new();
                    loss_score.add_loss();
                    loss_score
                });
                // Update score of `team2`
                teams_scores.entry(team2.to_string())
                .and_modify(|v| {
                    v.add_win();
                })
                .or_insert_with(|| {
                    let mut win_score = Score::new();
                    win_score.add_win();
                    win_score
                }); 
            },
            _ => {
                println!("Not supposed to happen")
            }
        }
    }

    // Convert to vector of key-value pairs (HashMap are unordered)
    let mut teams_scores_vec: Vec<_> = teams_scores.into_iter().collect();

    // Sort 
    teams_scores_vec.sort_by(|(team1, score1), (team2, score2)| {
        score2.points.cmp(&score1.points) // Descending total points
        .then_with(|| {
            team1.cmp(team2) // Alphabetical order
        })
    });

    // Generate output header
    ret.push_str(&format!("{:<31}| MP |  W |  D |  L |  P", "Team"));

    // If empty, return header
    if teams_scores_vec.is_empty() {
        return ret;
    }

    for (team, score) in teams_scores_vec {
        ret.push('\n');
        ret.push_str(&format!("{:<31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", // use padding
        team, 
        score.matches_played, 
        score.wins, 
        score.draws, 
        score.losses, 
        score.points)); 
    }
    
    ret
}
