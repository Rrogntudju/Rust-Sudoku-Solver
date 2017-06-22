// A translation of Peter Norvig’s Sudoku solver from Python to Rust     http://www.norvig.com/sudoku.html
use std::collections::{HashMap, HashSet};

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
    for r in [&rows[0..2], &rows[3..5], &rows[6..8]].iter() {
        for c in [&cols[0..2], &cols[3..5], &cols[6..8]].iter() {
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
        let peers_set : HashSet<String> = units[s].concat().clone().into_iter().filter(|p| p != s).collect();
        let mut peers_vec = Vec::<String>::new(); 
        peers_vec.extend(peers_set.into_iter());
        peers.insert(s.clone(), peers_vec);   
    }

    //  A set of unit tests.
    assert_eq!(squares.len(), 81);
    assert_eq!(unitlist.len(), 27);
    assert!(squares.iter().all(|s| units[s].len() == 3));
    assert!(squares.iter().all(|s| peers[s].len() == 20));
    assert_eq!(units.get("C2"), Some(vec![vec!["A2", "B2", "C2", "D2", "E2", "F2", "G2", "H2", "I2"],
                                          vec!["C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9"],
                                          vec!["A1", "A2", "A3", "B1", "B2", "B3", "C1", "C2", "C3"]]));
    assert_eq!(peers.get("C2"), Some(vec!["A2", "B2", "D2", "E2", "F2", "G2", "H2", "I2",
                                          "C1", "C3", "C4", "C5", "C6", "C7", "C8", "C9",
                                          "A1", "A3", "B1", "B3"]));
    println!("All tests pass.");
}
