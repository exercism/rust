use std::collections::{HashMap, HashSet};

// We first look for corners that are connected by a straight uninterrupted line.
// At the same time we make a note for each corner in which direction it's connected.
//
// Later we simply scan over the corners, pairing each top-left with each right-bottom corner
// (provided it's to the right and below the top-left corner) and look if all four lines
// of the rectangle exist.
//
// To simplify scanning for lines and points we run the horizontal lines algorithm on a
// seemingly transposed version of the input and later transpose the results again.

// Note: values of these constants are used in transposition, they're not random.
const CONN_LEFT: u8 = 0x1;
const CONN_RIGHT: u8 = 0x2;
const CONN_UP: u8 = 0x4;
const CONN_DOWN: u8 = 0x8;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Hash)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Symbol {
    Corner,  // '+'
    Connect, // '|' or '-' depending on direction
    Other,   // ' ', or anything really
}

// The input area.
struct RealArea {
    width: usize,
    height: usize,
    chars: Vec<Vec<char>>,
}

trait Area {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn symbol_at(&self, x: usize, y: usize) -> Symbol;
}

// For horizontal scanning
impl Area for RealArea {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn symbol_at(&self, x: usize, y: usize) -> Symbol {
        match self.chars[y][x] {
            '+' => Symbol::Corner,
            '-' => Symbol::Connect,
            _ => Symbol::Other,
        }
    }
}

struct TransposedArea<'a>(&'a RealArea);

// For vertical scanning
impl<'a> Area for TransposedArea<'a> {
    fn width(&self) -> usize {
        self.0.height
    }
    fn height(&self) -> usize {
        self.0.width
    }
    fn symbol_at(&self, x: usize, y: usize) -> Symbol {
        match self.0.chars[x][y] {
            '+' => Symbol::Corner,
            '|' => Symbol::Connect,
            _ => Symbol::Other,
        }
    }
}

// Information about connections.
struct Connections {
    lines: HashSet<Line>,
    points: HashMap<Point, u8>,
}

pub fn count(lines: &[&str]) -> usize {
    if lines.is_empty() || lines[0].is_empty() {
        return 0;
    }
    let area = RealArea {
        width: lines[0].len(),
        height: lines.len(),
        chars: lines.iter().map(|line| line.chars().collect()).collect(),
    };
    let area_transposed = TransposedArea(&area);
    let mut conns = scan_connected(&area);
    let conns_transposed = scan_connected(&area_transposed);

    // The transposed connections have their coordinate system wrong,
    // correct this.
    for l in conns_transposed.lines {
        conns.lines.insert(Line {
            x1: l.y1,
            y1: l.x1,
            x2: l.y2,
            y2: l.x2,
        });
    }
    for (p, tcf) in conns_transposed.points {
        let cf = conns.points.entry(Point { x: p.y, y: p.x }).or_insert(0);
        *cf |= tcf << 2
    }

    let mut total = 0;
    for (tl_p, tl_cf) in conns.points.iter() {
        // top left point and connection flags
        if tl_cf & (CONN_RIGHT | CONN_DOWN) == CONN_RIGHT | CONN_DOWN {
            for (br_p, br_cf) in conns.points.iter() {
                // left, right, top, bottom of potential rectangle
                let l = tl_p.x;
                let r = br_p.x;
                let t = tl_p.y;
                let b = br_p.y;
                let is_rect = br_cf & (CONN_LEFT | CONN_UP) == CONN_LEFT | CONN_UP
                    && r > l
                    && b > t
                    && conns.lines.contains(&Line {
                        x1: l,
                        y1: t,
                        x2: l,
                        y2: b,
                    })
                    && conns.lines.contains(&Line {
                        x1: l,
                        y1: t,
                        x2: r,
                        y2: t,
                    })
                    && conns.lines.contains(&Line {
                        x1: l,
                        y1: b,
                        x2: r,
                        y2: b,
                    })
                    && conns.lines.contains(&Line {
                        x1: r,
                        y1: t,
                        x2: r,
                        y2: b,
                    });
                if is_rect {
                    total += 1
                }
            }
        }
    }
    total
}

fn scan_connected(area: &dyn Area) -> Connections {
    let mut conns = Connections {
        lines: HashSet::new(),
        points: HashMap::new(),
    };
    let mut connected: Vec<usize> = vec![];
    for y in 0..area.height() {
        connected.clear();
        for x in 0..area.width() {
            let sym = area.symbol_at(x, y);
            if sym == Symbol::Corner {
                for prev in connected.iter() {
                    conns.lines.insert(Line {
                        x1: *prev,
                        y1: y,
                        x2: x,
                        y2: y,
                    });
                }
                if let Some(last) = connected.last() {
                    let cf = conns.points.get_mut(&Point { x: *last, y }).unwrap();
                    *cf |= CONN_RIGHT;
                }
                let cf = conns.points.entry(Point { x, y }).or_insert(0);
                if !connected.is_empty() {
                    *cf |= CONN_LEFT;
                }
                connected.push(x);
            } else if sym != Symbol::Connect {
                connected.clear(); // End of connected bit.
            }
        }
    }
    conns
}
