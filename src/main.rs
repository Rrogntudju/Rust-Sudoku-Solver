// A translation of Peter Norvig’s Sudoku solver from Python to Rust     http://www.norvig.com/sudoku.html
use sudoku::solver::Sudoku;

fn from_file(filename: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::prelude::*;
    let mut f = File::open(filename).unwrap_or_else(|_| panic!("Unable to open {}", filename));
    let mut lines = String::new();
    f.read_to_string(&mut lines).unwrap_or_else(|_| panic!("Error reading {}", filename));
    lines.split('\n').map(str::to_string).collect()
}

fn solve_all(grids: &[String], name: &str, showif: Option<f64>, solver: &Sudoku) {
    /*  Attempt to solve a sequence of grids. Report results.
    When showif is a number of seconds, display puzzles that take longer.
    When showif is None, don't display any puzzles. */
    use std::f64;
    use std::time::Instant;
    let time_solve = |grid: &String| {
        let start = Instant::now();
        let values = solver.solve(grid);
        let t = start.elapsed().as_secs_f64();
        if let Some(show_time) = showif {
            if t > show_time {
                if let Ok(v) = &values {
                    Sudoku::display(grid).unwrap().iter().for_each(|s| println!("{}", s));
                    println!();
                    Sudoku::display(v).unwrap().iter().for_each(|s| println!("{}", s));
                }
                println!("{:.5} seconds\n", t);
            }
        }
        (t, values.is_ok())
    };

    let (times, results): (Vec<_>, Vec<_>) = grids.iter().map(time_solve).unzip();
    let nb = grids.len() as f64;
    if nb > 1.0 {
        println!(
            "Solved {0} of {1} {2} puzzles (avg {3:.5} secs ({4:.0} Hz), max {5:.5} secs).",
            results.iter().fold(0, |acc, r| acc + *r as usize),
            nb,
            name,
            times.iter().sum::<f64>() / nb,
            nb / times.iter().sum::<f64>(),
            times.iter().fold(f64::NAN, |f1, &f2| { f1.max(f2) })
        );
    }
}

fn main() {
    let solver = Sudoku::new();
    solver.test();

    solve_all(&from_file("easy50.txt"), "easy", Some(0.02), &solver);
    solve_all(&from_file("top95.txt"), "hard", Some(0.02), &solver);
    solve_all(&from_file("hardest.txt"), "hardest", Some(0.02), &solver);
    solve_all(
        &(0..99).map(|_| solver.random_puzzle(17)).collect::<Vec<String>>(),
        "random",
        Some(0.02),
        &solver,
    );
}
