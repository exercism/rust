//! This exercise is about finding a [Eulerian path]. The "dots" are the
//! vertices of the graph, while the dominoes are the edges.
//!
//! [Eulerian path]: https://en.wikipedia.org/wiki/Eulerian_path

type Domino = (u8, u8);

pub fn chain(input: &[Domino]) -> Option<Vec<Domino>> {
    let mut bag = DominoBag::default();
    for domino in input.iter().copied() {
        bag.insert(domino);
    }

    let mut chain = Vec::with_capacity(input.len());
    let mut tail = vec![]; // used for temporary storage

    // start with any domino. (default will cause empty chain to be returned)
    let (mut first_dots, mut current_dots) = (0..7)
        .find_map(|i| bag.take_neighbor(i))
        .inspect(|&d| chain.push(d))
        .unwrap_or_default();

    loop {
        while let Some(next_domoino) = bag.take_neighbor(current_dots) {
            chain.push(next_domoino);
            current_dots = next_domoino.1;
        }
        if current_dots != first_dots {
            return None; // unbalanced
        }
        // reintegrate second chain half from previous loop iteration
        chain.append(&mut tail);

        if bag.is_empty() {
            return Some(chain);
        }
        // We have found a path that ends where it started, but not all dominoes
        // are used up. We must find a location in the current chain where we
        // could've taken a different path.
        let (fork_point, next_domino) = chain
            .iter()
            .enumerate()
            .find_map(|(i, &(x, _))| bag.take_neighbor(x).map(|d| (i, d)))?;

        // put aside second half of first chain
        tail.extend(chain.drain(fork_point..));

        chain.push(next_domino);
        // Treat the domino after the fork point as the first domino, to search
        // for a path that ends up back at the fork point.
        (first_dots, current_dots) = next_domino;
    }
}

/// The domino bag stores all "untraversed edges" of the graph, using an
/// adjacency matrix.
#[derive(Default)]
struct DominoBag([[u8; 7]; 7]);

impl DominoBag {
    fn is_empty(&self) -> bool {
        self.0.iter().flatten().all(|d| *d == 0)
    }

    fn insert(&mut self, d: Domino) {
        let (i, j) = (d.0 as usize, d.1 as usize);
        self.0[i][j] += 1;
        self.0[j][i] += 1;
    }

    /// Takes a domino connecting `i` to any neighbor.
    fn take_neighbor(&mut self, i: u8) -> Option<Domino> {
        (0..7).map(|j| (i, j)).find(|d| {
            let (i, j) = (d.0 as usize, d.1 as usize);
            if self.0[i][j] == 0 {
                return false;
            };
            self.0[i][j] -= 1;
            self.0[j][i] -= 1;
            true
        })
    }
}
