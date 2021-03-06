// A translation of Peter Norvig’s Sudoku solver from Python to Rust     http://www.norvig.com/sudoku.html
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Debug)]
struct Context {
    cols: Vec<char>,
    rows: Vec<char>,
    squares: Vec<String>,
    unitlist: Vec<Vec<String>>,
    units: HashMap<String, Vec<Vec<String>>>,
    peers: HashMap<String, Vec<String>>,
}

fn cross(rows: &[char], cols: &[char]) -> Vec<String> {
    let mut v = Vec::new();
    for ch in rows {
        for d in cols {
            let mut sq = String::new();
            sq.push(*ch);
            sq.push(*d);
            v.push(sq)
        }
    }
    v
}

fn test(ctx: &Context) -> () {
    // A set of unit tests.
    assert_eq!(ctx.squares.len(), 81);
    assert_eq!(ctx.unitlist.len(), 27);
    assert!(ctx.squares.iter().all(|s| ctx.units[s].len() == 3));
    assert!(ctx.squares.iter().all(|s| ctx.peers[s].len() == 20));
    assert_eq!(
        ctx.units.get("C2"),
        Some(&vec![
            vec![
                "A2".into(),
                "B2".into(),
                "C2".into(),
                "D2".into(),
                "E2".into(),
                "F2".into(),
                "G2".into(),
                "H2".into(),
                "I2".into()
            ],
            vec![
                "C1".into(),
                "C2".into(),
                "C3".into(),
                "C4".into(),
                "C5".into(),
                "C6".into(),
                "C7".into(),
                "C8".into(),
                "C9".into()
            ],
            vec![
                "A1".into(),
                "A2".into(),
                "A3".into(),
                "B1".into(),
                "B2".into(),
                "B3".into(),
                "C1".into(),
                "C2".into(),
                "C3".into()
            ]
        ])
    );

    let mut peers_c2 = vec![
        "A2".into(),
        "B2".into(),
        "D2".into(),
        "E2".into(),
        "F2".into(),
        "G2".into(),
        "H2".into(),
        "I2".into(),
        "C1".into(),
        "C3".into(),
        "C4".into(),
        "C5".into(),
        "C6".into(),
        "C7".into(),
        "C8".into(),
        "C9".into(),
        "A1".into(),
        "A3".into(),
        "B1".into(),
        "B3".into(),
    ];
    peers_c2.sort();
    assert_eq!(ctx.peers.get("C2"), Some(&peers_c2));
    println!("All tests pass.\n");
}

fn grid_values(grid: &str, ctx: &Context) -> HashMap<String, Vec<char>> {
    //  Convert grid into a dict of (square, char Vec) with '0' or '.' for empties.
    let grid_chars: Vec<Vec<char>> = grid
        .chars()
        .filter(|ch| ctx.cols.contains(ch) || ['0', '.'].contains(ch))
        .map(|ch| vec![ch])
        .collect();
    assert_eq!(grid_chars.len(), 81);
    let mut grid_values = HashMap::<String, Vec<char>>::new();
    grid_values.extend(ctx.squares.iter().cloned().zip(grid_chars.into_iter()));
    grid_values
}

fn parse_grid(grid: &str, ctx: &Context) -> Option<HashMap<String, Vec<char>>> {
    //  Convert grid to Some dict of possible values, [square, digits], or return None if a contradiction is detected.
    let mut values = HashMap::<String, Vec<char>>::with_capacity(81);
    for s in &ctx.squares {
        values.insert(s.clone(), ctx.cols.clone());
    }
    for (s, gvalues) in &grid_values(grid, ctx) {
        for d in gvalues {
            if ctx.cols.contains(d) && !assign(&mut values, s, d, ctx) {
                return None;
            }
        }
    }
    Some(values)
}

fn assign(values: &mut HashMap<String, Vec<char>>, s: &str, d: &char, ctx: &Context) -> bool {
    // Assign a value d by eliminating all the other values (except d) from values[s] and propagate. Return false if a contradiction is detected.
    let other_values: Vec<char> = values[s].iter().cloned().filter(|d2| d2 != d).collect();
    other_values.iter().all(|d2| eliminate(values, s, d2, ctx))
}

fn eliminate(values: &mut HashMap<String, Vec<char>>, s: &str, d: &char, ctx: &Context) -> bool {
    if !values[s].contains(d) {
        return true; // already eliminated
    }
    let i = values[s].iter().position(|d2| d2 == d).unwrap();
    values.get_mut(s).unwrap().remove(i);
    // (rule 1) If a square s is reduced to one value d2, then eliminate d2 from the peers.
    let d2 = values[s].clone();
    if d2.is_empty() {
        return false; // Contradiction: removed last value
    }
    if d2.len() == 1
        && !ctx.peers[s]
            .iter()
            .all(|s2| eliminate(values, s2, &d2[0], ctx))
    {
        return false;
    }
    // (rule 2) If a unit u is reduced to only one place for a value d, then put it there.
    for u in &ctx.units[s] {
        let dplaces: Vec<String> = u
            .iter()
            .cloned()
            .filter(|s2| values[s2].contains(d))
            .collect();
        if dplaces.is_empty() {
            return false; // Contradiction: no place for this value
        }
        // if d can only be in one place in unit assign it there
        if dplaces.len() == 1 && !assign(values, &dplaces[0], d, ctx) {
            return false;
        }
    }
    true
}

fn display(values: &HashMap<String, Vec<char>>, ctx: &Context) -> () {
    let width = 1 + (values.iter().map(|v| v.1.len()).max().unwrap());
    let line = ["-"; 3]
        .iter()
        .map(|c| c.repeat(3 * width))
        .collect::<Vec<String>>()
        .join("+");
    for r in &ctx.rows {
        println!(
            "{}",
            ctx.cols
                .iter()
                .map(|c| {
                    let s = [*r, *c].iter().collect::<String>();
                    format!("{0: ^1$}", values[&s].iter().collect::<String>(), width)
                        + (if ['3', '6'].contains(c) { "|" } else { "" })
                })
                .collect::<String>()
        );
        if ['C', 'F'].contains(r) {
            println!("{}", line)
        }
    }
    println!("");
}

fn search(values: HashMap<String, Vec<char>>, ctx: &Context) -> Option<HashMap<String, Vec<char>>> {
    // Using depth-first search and propagation, try all possible values
    if values.iter().all(|(_, v)| v.len() == 1) {
        return Some(values); // Solved!
    }
    // Choose the unfilled square s with the fewest possibilities
    let (_, s) = values
        .iter()
        .filter(|&(_, v)| v.len() > 1)
        .map(|(s, v)| (v.len(), s))
        .min()
        .unwrap();
    for d in &values[s] {
        let mut cloned_values = values.clone();
        if assign(&mut cloned_values, s, d, ctx) {
            if let Some(svalues) = search(cloned_values, ctx) {
                return Some(svalues);
            }
        }
    }
    None
}

fn solve(grid: &str, ctx: &Context) -> Option<HashMap<String, Vec<char>>> {
    parse_grid(grid, ctx).and_then(|v| search(v, ctx))
}

fn solved(values: &HashMap<String, Vec<char>>, ctx: &Context) -> bool {
    //  A puzzle is solved if each unit is a permutation of the digits 1 to 9.
    let unitsolved = |unit: &Vec<String>| {
        let mut digits_values = unit
            .iter()
            .map(|s| values[s].iter().collect::<String>())
            .collect::<Vec<String>>();
        digits_values.sort();
        digits_values
            == ctx
                .cols
                .iter()
                .map(char::to_string)
                .collect::<Vec<String>>()
    };
    ctx.unitlist.iter().all(unitsolved)
}

fn random_puzzle(n: usize, rng: &mut ThreadRng, ctx: &Context) -> String {
    /*  Make a random puzzle with N or more assignments. Restart on contradictions.
    Note the resulting puzzle is not guaranteed to be solvable, but empirically
    about 99.8% of them are solvable. Some have multiple solutions. */
    let mut values = HashMap::<String, Vec<char>>::with_capacity(81);
    for s in &ctx.squares {
        values.insert(s.clone(), ctx.cols.clone());
    }
    let mut squares = ctx.squares.clone();
    squares.shuffle(rng);
    for s in &squares {
        let d2 = values[s].clone();
        if !assign(&mut values, s, d2.choose(rng).unwrap(), ctx) {
            break;
        }
        let mut ds: Vec<Vec<char>> = values
            .iter()
            .filter(|&(_, v)| v.len() == 1)
            .map(|(_, v)| v.clone())
            .collect();
        if ds.len() >= n {
            ds.sort();
            ds.dedup();
            if ds.len() >= 8 {
                return ctx
                    .squares
                    .iter()
                    .map(|s| {
                        if values[s].len() == 1 {
                            values[s][0]
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>();
            }
        }
    }
    random_puzzle(17, rng, ctx) // Give up and make a new puzzle
}

fn solve_all(grids: &[String], name: &str, showif: Option<f64>, ctx: &Context) -> () {
    /*  Attempt to solve a sequence of grids. Report results.
    When showif is a number of seconds, display puzzles that take longer.
    When showif is None, don't display any puzzles. */
    use std::f64;
    use std::time::Instant;
    let time_solve = |grid: &String| {
        let start = Instant::now();
        let values = solve(grid, ctx);
        let t = start.elapsed().as_secs_f64();
        if let Some(show_time) = showif {
            if t > show_time {
                display(&grid_values(grid, ctx), ctx);
                if let Some(v) = values.as_ref() {
                    display(v, ctx);
                }
                println!("{:.4} seconds\n", t);
            }
        }
        (t, values.map_or(false, |v| solved(&v, ctx)))
    };
    let (times, results): (Vec<_>, Vec<_>) = grids.iter().map(time_solve).unzip();
    let nb = grids.len() as f64;
    if nb > 1.0 {
        println!(
            "Solved {0} of {1} {2} puzzles (avg {3:.4} secs ({4:.0} Hz), max {5:.4} secs).",
            results.iter().fold(0, |acc, r| acc + *r as usize),
            nb,
            name,
            times.iter().sum::<f64>() / nb,
            nb / times.iter().sum::<f64>(),
            times.iter().fold(f64::NAN, |f1, &f2| { f1.max(f2) })
        );
    }
}

fn from_file(filename: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::prelude::*;
    let mut f = File::open(filename).expect(&format!("Unable to open {}", filename));
    let mut lines = String::new();
    f.read_to_string(&mut lines)
        .expect(&format!("Error reading {}", filename));
    lines.split('\n').map(str::to_string).collect()
}

fn main() {
    let cols: Vec<char> = "123456789".chars().collect();
    let rows: Vec<char> = "ABCDEFGHI".chars().collect();
    let squares = cross(&rows, &cols);
    // A vector of units (a unit is a column or a row or a box of 9 squares)
    let mut unitlist = Vec::<Vec<String>>::with_capacity(27);
    // columns
    for d in &cols {
        unitlist.push(cross(&rows, &[*d]));
    }
    // rows
    for ch in &rows {
        unitlist.push(cross(&[*ch], &cols));
    }
    // boxes
    for r in &[&rows[0..3], &rows[3..6], &rows[6..9]] {
        for c in &[&cols[0..3], &cols[3..6], &cols[6..9]] {
            unitlist.push(cross(*r, *c));
        }
    }
    //  units is a dictionary where each square maps to the list of units that contain the square
    let mut units = HashMap::<String, Vec<Vec<String>>>::with_capacity(81);
    for s in &squares {
        let unit_s: Vec<Vec<String>> = unitlist.iter().cloned().filter(|u| u.contains(s)).collect();
        units.insert(s.clone(), unit_s);
    }
    //  peers is a dictionary where each square s maps to the set of squares formed by the union of the squares in the units of s, but not s itself
    let mut peers = HashMap::<String, Vec<String>>::with_capacity(81);
    for s in &squares {
        let mut peers_s: Vec<String> = units[s]
            .concat()
            .iter()
            .cloned()
            .filter(|p| p != s)
            .collect();
        peers_s.sort();
        peers_s.dedup();
        peers.insert(s.clone(), peers_s);
    }
    let context = Context {
        cols: cols,
        rows: rows,
        squares: squares,
        unitlist: unitlist,
        units: units,
        peers: peers,
    };
    test(&context);
    solve_all(&from_file("easy50.txt"), "easy", Some(0.5), &context);
    solve_all(&from_file("top95.txt"), "hard", Some(0.5), &context);
    solve_all(&from_file("hardest.txt"), "hardest", Some(0.5), &context);
    let mut rng = rand::thread_rng();
    solve_all(
        &(0..99)
            .map(|_| random_puzzle(17, &mut rng, &context))
            .collect::<Vec<String>>(),
        "random",
        Some(0.5),
        &context,
    );
}
