# Rust-Sudoku-Solver
An implementation of Peter Norvig’s Sudoku solver in Rust 

# Results (slowest to fastest) for various implementations of Peter Norvig’s Sudoku solver (running on same hardware)
**[Python version](http://www.norvig.com/sudoku.html) on Windows 10 :**  
```
Solved 50 of 50 easy puzzles (avg 0.01 secs (182 Hz), max 0.01 secs).
Solved 95 of 95 hard puzzles (avg 0.02 secs (51 Hz), max 0.09 secs).
Solved 11 of 11 hardest puzzles (avg 0.01 secs (131 Hz), max 0.01 secs).
Solved 99 of 99 random puzzles (avg 0.01 secs (167 Hz), max 0.01 secs).
```
**[F# version](https://github.com/Rrogntudju/FSharp-Sudoku-Solver) on Windows 10 :**
```
Solved 50 of 50 easy puzzles (avg 0.01 secs (187 Hz), max 0.02 secs). 
Solved 95 of 95 hard puzzles (avg 0.02 secs (61 Hz), max 0.08 secs).
Solved 11 of 11 hardest puzzles (avg 0.01 secs (155 Hz), max 0.01 secs).
Solved 99 of 99 random puzzles (avg 0.01 secs (184 Hz), max 0.01 secs).
```
**[Python version](http://www.norvig.com/sudoku.html) on Ubuntu Neon (Hyper-V) :**
```
Solved 50 of 50 easy puzzles (avg 0.00 secs (217 Hz), max 0.01 secs).
Solved 95 of 95 hard puzzles (avg 0.02 secs (64 Hz), max 0.08 secs).
Solved 11 of 11 hardest puzzles (avg 0.01 secs (147 Hz), max 0.01 secs).
Solved 99 of 99 random puzzles (avg 0.00 secs (219 Hz), max 0.01 secs).
```
**Rust version on Windows 10 :**
```
Solved 50 of 50 easy puzzles (avg 0.002 secs (446 Hz), max 0.004 secs).
Solved 95 of 95 hard puzzles (avg 0.008 secs (130 Hz), max 0.039 secs).
Solved 11 of 11 hardest puzzles (avg 0.003 secs (314 Hz), max 0.006 secs).
Solved 99 of 99 random puzzles (avg 0.003 secs (372 Hz), max 0.004 secs).
```
**Rust version on Ubuntu Neon (Hyper-V) :**
```
Solved 50 of 50 easy puzzles (avg 0.001 secs (877 Hz), max 0.003 secs).
Solved 95 of 95 hard puzzles (avg 0.005 secs (211 Hz), max 0.031 secs).
Solved 11 of 11 hardest puzzles (avg 0.001 secs (688 Hz), max 0.002 secs).
Solved 99 of 99 random puzzles (avg 0.001 secs (884 Hz), max 0.003 secs).
```
**Rust version on Ubuntu (Windows Subsystem for Linux) :**
```
Solved 50 of 50 easy puzzles (avg 0.001 secs (943 Hz), max 0.002 secs).
Solved 95 of 95 hard puzzles (avg 0.004 secs (222 Hz), max 0.023 secs).
Solved 11 of 11 hardest puzzles (avg 0.001 secs (786 Hz), max 0.002 secs).
Solved 99 of 99 random puzzles (avg 0.001 secs (917 Hz), max 0.002 secs).
```