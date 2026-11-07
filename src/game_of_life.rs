// Game of Life, Week 12: grid, display, neighbor counts, the rule for one cell.
//
// TODO: remove the next line once every function below is used.
#![allow(dead_code)]

const ROWS: usize = 10;
const COLS: usize = 20;
type Grid = [[bool; COLS]; ROWS];

// All cells dead.
pub fn new_grid() -> Grid {
    [[false; COLS]; ROWS]
}

// '#' for a live cell, '.' for a dead cell, one line per row.
pub fn print_grid(grid: &Grid) {
    for row in grid {
	for &cell in row {
		if cell {
			print!("#");
		}
		else {
			print!(".");
		}
	}
	println!();
    }
}

// Live neighbors of (row, col): the eight surrounding cells, excluding the cell itself.
// Cells beyond the edge count as dead. Careful: row - 1 on a usize of 0 panics.
pub fn count_neighbors(grid: &Grid, row: usize, col: usize) -> u8 {
    let mut count = 0;
    let r_i = row as i32;
    let c_i = col as i32;

    for dr in -1..=1 {
	for dc in -1..=1 {
	    if dr == 0 && dc == 0 {
		continue;
	    }
	    let nr = r_i + dr;
	    let nc = c_i + dc;

	    if nr >= 0 && nr < ROWS as i32 && nc >= 0 && nc < COLS as i32 {
		if grid[nr as usize][nc as usize] {
		    count += 1;
		}
	    }
	}
    }
    count
}


// The rule for one cell: does it live in the next generation?
pub fn next_state(alive: bool, neighbors: u8) -> bool {
    match (alive, neighbors) {
	(true, 2) | (true, 3) => true,
	(false, 3)	      => true,
	_		      => false,
    }
}

// Debug view: each cell's neighbor count as a digit, one line per row.
pub fn print_neighbor_counts(grid: &Grid) {
    for r in 0..ROWS {
	for c in 0..COLS {
	    let count = count_neighbors(grid, r, c);
	    print!("{}", count);
	}
	println!()
    }
}

// Set a few cells alive (hardcoded, or entered as `row col` pairs until a blank line),
// print the grid, then the neighbor counts.
pub fn play() {
    let mut grid = new_grid();
    
    grid[5][4] = true;
    grid[5][5] = true;
    grid[5][6] = true;

    println!("Grid Display:");
    print_grid(&grid);

    println!("\nNeighbor Counts:");
    print_neighbor_counts(&grid);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Supplied by the instructor.
    #[test]
    fn live_1_dies() {
        assert!(!next_state(true, 1));
    }

    #[test]
    fn live_2_survives() {
        assert!(next_state(true, 2));
    }

    #[test]
    fn dead_3_born() {
        assert!(next_state(false, 3));
    }

    // TODO (student): write the remaining cases from your truth table before you
    // write next_state. At minimum: a live cell with 4 neighbors, a dead cell with 2.

    fn live_4_overpopulation_dies() {
	assert!(!next_state(true, 4));
    }

    fn dead_2_stays_dead() {
	assert!(!next_state(false, 2));
    }
}
