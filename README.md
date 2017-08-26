# Rust-Sudoku-Solver
An implementation of Peter Norvig’s Sudoku solver in Rust 

# Results (slowest to fastest) for various implementations of Peter Norvig’s Sudoku solver (running on same hardware)
[Python version](http://www.norvig.com/sudoku.html) on Windows 10 : 
```
Solved 50 of 50 easy puzzles (avg 0.01 secs (182 Hz), max 0.01 secs).
Solved 95 of 95 hard puzzles (avg 0.02 secs (51 Hz), max 0.09 secs).
Solved 11 of 11 hardest puzzles (avg 0.01 secs (131 Hz), max 0.01 secs).
Solved 99 of 99 random puzzles (avg 0.01 secs (167 Hz), max 0.01 secs).
```
[F# version](https://github.com/Rrogntudju/FSharp-Sudoku-Solver) on Windows 10 :
```
Solved 50 of 50 easy puzzles (avg 0.01 secs (187 Hz), max 0.02 secs). 
Solved 95 of 95 hard puzzles (avg 0.02 secs (61 Hz), max 0.08 secs).
Solved 11 of 11 hardest puzzles (avg 0.01 secs (155 Hz), max 0.01 secs).
Solved 99 of 99 random puzzles (avg 0.01 secs (184 Hz), max 0.01 secs).
```
[Python version](http://www.norvig.com/sudoku.html) on Ubuntu Neon (Hyper-V) :
```
Solved 50 of 50 easy puzzles (avg 0.00 secs (217 Hz), max 0.01 secs).
Solved 95 of 95 hard puzzles (avg 0.02 secs (64 Hz), max 0.08 secs).
Solved 11 of 11 hardest puzzles (avg 0.01 secs (147 Hz), max 0.01 secs).
Solved 99 of 99 random puzzles (avg 0.00 secs (219 Hz), max 0.01 secs).
```
Rust version on Windows 10 :
```
Solved 50 of 50 easy puzzles (avg 0.0023 secs (442 Hz), max 0.0150 secs).
Solved 95 of 95 hard puzzles (avg 0.0084 secs (119 Hz), max 0.0560 secs).
Solved 11 of 11 hardest puzzles (avg 0.0037 secs (268 Hz), max 0.0160 secs).
Solved 99 of 99 random puzzles (avg 0.0028 secs (351 Hz), max 0.0160 secs).
```
Rust version on Ubuntu Neon (Hyper-V) :
```
Solved 50 of 50 easy puzzles (avg 0.0019 secs (526 Hz), max 0.0140 secs).
Solved 95 of 95 hard puzzles (avg 0.0060 secs (167 Hz), max 0.0380 secs).
Solved 11 of 11 hardest puzzles (avg 0.0019 secs (524 Hz), max 0.0030 secs).
Solved 99 of 99 random puzzles (avg 0.0019 secs (538 Hz), max 0.0030 secs).
```
Rust version on Ubuntu (Windows Subsystem for Linux) :
```
Solved 50 of 50 easy puzzles (avg 0.0013 secs (794 Hz), max 0.0020 secs).
Solved 95 of 95 hard puzzles (avg 0.0052 secs (193 Hz), max 0.0260 secs).
Solved 11 of 11 hardest puzzles (avg 0.0018 secs (550 Hz), max 0.0030 secs).
Solved 99 of 99 random puzzles (avg 0.0015 secs (651 Hz), max 0.0020 secs).
```
