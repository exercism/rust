pub fn transpose(in_lines: String) -> String {
    let in_lines: Vec<&str> = in_lines.lines().collect();
    let out_lines_count = in_lines.iter().map(|l| l.len()).max().unwrap();
    let mut out_lines: Vec<String> = vec![String::new(); out_lines_count]; 

    for (out_line_n, out_line) in out_lines.iter_mut().enumerate() {
        for in_line in &in_lines {
            let c = (*in_line).chars().nth(out_line_n).unwrap_or(' ');
            out_line.push(c);
        }
    }

    let mut out = String::new();
    for line in &out_lines {
        out.push_str(line.trim_right());
        out.push('\n');
    }
    out
}

