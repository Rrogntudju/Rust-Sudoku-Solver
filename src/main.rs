// A translation of Peter Norvig’s Sudoku solver from Python to Rust     http://www.norvig.com/sudoku.html
use std::collections::{HashMap};

fn cross (rows : &[char], cols : &[char]) -> Vec<String> {
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

fn main() {
    let cols : Vec<char> = "123456789".chars().collect();
    let rows : Vec<char> = "ABCDEFGHI".chars().collect();
    let squares = cross(&rows, &cols);

    // A vector of units (a unit = a column, a row or a box of 9 squares)
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
        let unit_s : Vec<Vec<String>> = unitlist.clone().into_iter().filter(|u| u.contains(s)).collect();
        units.insert(s.clone(), unit_s);   
    }
  
    //  peers is a dictionary where each square s maps to the set of squares formed by the union of the squares in the units of s, but not s itself 
    let mut peers = HashMap::<String, Vec<String>>::new();
    for s in &squares {
        let mut peers_s : Vec<String> = units[s].concat().clone().into_iter().filter(|p| p != s).collect();
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
}
