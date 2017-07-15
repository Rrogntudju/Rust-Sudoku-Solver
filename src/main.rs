// A translation of Peter Norvig’s Sudoku solver from Python to Rust     http://www.norvig.com/sudoku.html
use std::collections::{HashMap};

#[derive(Debug)]
struct Context {
    cols: Vec<char>,
    rows: Vec<char>,
    squares: Vec<String>,
    unitlist: Vec<Vec<String>>,
    units: HashMap<String, Vec<Vec<String>>>,
    peers: HashMap<String, Vec<String>>
}

fn cross (rows: &[char], cols: &[char]) -> Vec<String> {
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

fn grid_values (grid: &str, ctx: &Context) -> HashMap<String, Vec<char>> {
    //  Convert grid into a dict of (square, char Vec) with '0' or '.' for empties.
    let grid_chars: Vec<Vec<char>> = grid.chars().filter(|ch| ctx.cols.contains(ch) || ['0', '.'].contains(ch)).map(|ch| vec![ch]).collect();
    assert_eq!(grid_chars.len(), 81);
    let mut grid_values = HashMap::<String, Vec<char>>::new();
    grid_values.extend(ctx.squares.iter().cloned().zip(grid_chars.into_iter()));
    grid_values
}

fn parse_grid (grid: &str, ctx: &Context) -> Option<HashMap<String, Vec<char>>> {
    //  Convert grid to Some dict of possible values, [square, digits], or return None if a contradiction is detected.
    let mut values = HashMap::<String, Vec<char>>::new();
    for s in &ctx.squares { 
        values.insert(s.clone(), ctx.cols.clone());
    }
    for (s, gvalues) in grid_values(&grid, ctx).iter() {
        for d in gvalues {
            if ctx.cols.contains(d) && !assign(&mut values, s, d, ctx) {
                    return None;
            }
        }
    }
    Some(values)
}

fn assign (values: &mut HashMap<String, Vec<char>>, s: &String, d: &char, ctx: &Context) -> bool {
    // Assign a value d by eliminating all the other values (except d) from values[s] and propagate. Return false if a contradiction is detected.  
    let other_values: Vec<char> = values[s].iter().cloned().filter(|d2| d2 != d).collect();
    for d2 in &other_values {
        if !eliminate(values, s, d2, ctx) {
            return false;
        }
    }
    true
}

fn eliminate (values: &mut HashMap<String, Vec<char>>, s: &String, d: &char, ctx: &Context) -> bool {
    if !values[s].contains(d) {
        return true    // already eliminated
    }
    let i  = values[s].iter().position(|d2| d2 == d).unwrap();
    values.get_mut(s).unwrap().remove(i);
    // (rule 1) If a square s is reduced to one value d2, then eliminate d2 from the peers.
    let d2 = values[s].clone();
    if d2.len() == 0 {
        return false; // Contradiction: removed last value
    } else if d2.len() == 1 {
        for s2 in &ctx.peers[s] {
            if !eliminate(values, s2, &d2[0], ctx) {
                return false;
            }
        }
    }
    // (rule 2) If a unit u is reduced to only one place for a value d, then put it there.
    for u in &ctx.units[s] {
        let dplaces: Vec<String> = u.iter().cloned().filter(|s2| values[s2].contains(d)).collect();
        if dplaces.len() == 0 {
            return false;   // Contradiction: no place for this value
        } else if dplaces.len() == 1 {
            // d can only be in one place in unit; assign it there
            if !assign(values, &dplaces[0], d, ctx) {
                return false;
            }
        }
    }
    true
}

fn display (values: &HashMap<String, Vec<char>>, ctx: &Context) -> () {
   ()
}

fn solve (grid: &str, ctx: &Context) -> Option<HashMap<String, Vec<char>>> {
    if let Some(values) = parse_grid(grid, ctx) {
        display(&grid_values(grid, ctx), ctx);
        display(&values, ctx);
        return Some(values);
    }
    None
}

fn main() {
    let cols: Vec<char> = "123456789".chars().collect();
    let rows: Vec<char> = "ABCDEFGHI".chars().collect();
    let squares = cross(&rows, &cols);
    // A vector of units (a unit = a column or a row or a box of 9 squares)
    let mut unitlist = Vec::<Vec<String>>::new();
    // columns
    for d in &cols {
        unitlist.push(cross(&rows, &[*d]));
    }
    // rows
    for ch in &rows {
        unitlist.push(cross(&[*ch], &cols));
    }
    // boxes
    for r in [&rows[0..3], &rows[3..6], &rows[6..9]].iter() {
        for c in [&cols[0..3], &cols[3..6], &cols[6..9]].iter() {
            unitlist.push(cross(*r, *c));
        }
    }
    //  units is a dictionary where each square maps to the list of units that contain the square  
    let mut units = HashMap::<String, Vec<Vec<String>>>::new();
    for s in &squares {
        let unit_s : Vec<Vec<String>> = unitlist.iter().cloned().filter(|u| u.contains(s)).collect();
        units.insert(s.clone(), unit_s);   
    }
    //  peers is a dictionary where each square s maps to the set of squares formed by the union of the squares in the units of s, but not s itself 
    let mut peers = HashMap::<String, Vec<String>>::new();
    for s in &squares {
        let mut peers_s : Vec<String> = units[s].concat().iter().cloned().filter(|p| p != s).collect();
        peers_s.sort();
        peers_s.dedup();
        peers.insert(s.clone(), peers_s);   
    }
    //  A set of unit tests.
    assert_eq!(squares.len(), 81);
    assert_eq!(unitlist.len(), 27);
    assert!(squares.iter().all(|s| units[s].len() == 3));
    assert!(squares.iter().all(|s| peers[s].len() == 20));
    assert_eq!(units.get("C2"), Some(&vec![vec!["A2".to_string(), "B2".to_string(), "C2".to_string(), "D2".to_string(), "E2".to_string(), "F2".to_string(), "G2".to_string(), "H2".to_string(), "I2".to_string()],
                                           vec!["C1".to_string(), "C2".to_string(), "C3".to_string(), "C4".to_string(), "C5".to_string(), "C6".to_string(), "C7".to_string(), "C8".to_string(), "C9".to_string()],
                                           vec!["A1".to_string(), "A2".to_string(), "A3".to_string(), "B1".to_string(), "B2".to_string(), "B3".to_string(), "C1".to_string(), "C2".to_string(), "C3".to_string()]]));

    let mut peers_c2 = vec!["A2".to_string(), "B2".to_string(), "D2".to_string(), "E2".to_string(), "F2".to_string(), "G2".to_string(), "H2".to_string(), "I2".to_string(),
                            "C1".to_string(), "C3".to_string(), "C4".to_string(), "C5".to_string(), "C6".to_string(), "C7".to_string(), "C8".to_string(), "C9".to_string(),
                            "A1".to_string(), "A3".to_string(), "B1".to_string(), "B3".to_string()];
    peers_c2.sort();
    assert_eq!(peers.get("C2"), Some(&peers_c2));
    println!("All tests pass.");
    let context = Context {cols: cols, rows: rows, squares: squares, unitlist: unitlist, units: units, peers: peers};
    solve("003020600900305001001806400008102900700000008006708200002609500800203009005010300", &context);

}
