use {
    crate::hal::{Grid, Point},
    core::ops::Not,
};

/// A cell within the Game of Life.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Alive,
    Dead,
}

impl Not for Cell {
    type Output = Cell;

    fn not(self) -> Self::Output {
        match self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }
}

/// The Game of Life.
pub struct Life {
    universe: [[Cell; Grid::size() as usize]; 2], // double buffered universe
    active_buffer_index: usize,
}

impl Life {
    /// Construct a new Game of Life.
    pub const fn new() -> Self {
        Life {
            universe: [[Cell::Dead; Grid::size() as usize]; 2],
            active_buffer_index: 0,
        }
    }

    /// Returns the state of the cell at the given point.
    pub fn get(&self, point: Point) -> Cell {
        self.universe[self.active_buffer_index][point.to_index() as usize]
    }

    /// Set the state of a cell at the given point.
    pub fn set(&mut self, point: Point, cell: Cell) {
        self.set_buffer(self.active_buffer_index, point, cell);
    }

    fn set_buffer(&mut self, buffer: usize, point: Point, cell: Cell) {
        self.universe[buffer][point.to_index() as usize] = cell;
    }

    /// Progress the simulation by one tick.
    pub fn tick(&mut self) {
        let next_buffer_index: usize = if self.active_buffer_index == 0 { 1 } else { 0 };
        for point in Grid::points() {
            self.set_buffer(next_buffer_index, point, self.next_state(point));
        }
        self.active_buffer_index = next_buffer_index;
    }

    /// Returns the next state of a cell at a point in the universe.
    fn next_state(&self, point: Point) -> Cell {
        match (self.get(point), self.live_neighbours(point)) {
            // rule 1 - underpopulation
            (Cell::Alive, n) if n < 2 => Cell::Dead,
            // rule 2 - stable population
            (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
            // rule 3 - overpopulation
            (Cell::Alive, n) if n > 3 => Cell::Dead,
            // rule 4 - reproduction
            (Cell::Dead, 3) => Cell::Alive,
            // anything else is stable
            (current, _) => current,
        }
    }

    /// Returns the number of live neighbours for the cell at the given point.
    fn live_neighbours(&self, point: Point) -> u8 {
        [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
        .iter()
        .map(|&(x, y)| point + Point::new(x, y))
        .filter(|&p| self.get(p) == Cell::Alive)
        .count() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_set_cells() {
        let mut life = Life::new();
        assert_eq!(life.get(Point::new(0, 0)), Cell::Dead);

        life.set(Point::new(0, 0), Cell::Alive);
        assert_eq!(life.get(Point::new(0, 0)), Cell::Alive);
    }

    #[test]
    fn counting_neighbours() {
        let mut life = Life::new();
        let origin = Point::new(0, 0);

        life.set(origin, Cell::Alive);
        assert_eq!(life.live_neighbours(origin), 0);

        life.set(origin + Point::new(1, 0), Cell::Alive);
        assert_eq!(life.live_neighbours(origin), 1);

        life.set(origin + Point::new(-1, 0), Cell::Alive);
        assert_eq!(life.live_neighbours(origin), 2);

        life.set(origin + Point::new(0, 1), Cell::Alive);
        assert_eq!(life.live_neighbours(origin), 3);

        life.set(origin + Point::new(0, -1), Cell::Alive);
        assert_eq!(life.live_neighbours(origin), 4);
    }

    #[test]
    fn underpopulation_rule() {
        let mut life = Life::new();
        life.set(Point::new(2, 2), Cell::Alive);
        life.tick();
        assert_eq!(life.get(Point::new(2, 2)), Cell::Dead);
    }

    #[test]
    fn stable_rule() {
        let mut life = Life::new();
        life.set(Point::new(2, 2), Cell::Alive);
        life.set(Point::new(3, 2), Cell::Alive);
        life.set(Point::new(1, 2), Cell::Alive);

        life.tick();
        assert_eq!(life.get(Point::new(2, 2)), Cell::Alive);

        life.set(Point::new(2, 3), Cell::Alive);
        assert_eq!(life.get(Point::new(2, 2)), Cell::Alive);
    }

    #[test]
    fn overpopulation_rule() {
        let mut life = Life::new();
        life.set(Point::new(2, 2), Cell::Alive);
        life.set(Point::new(3, 2), Cell::Alive);
        life.set(Point::new(1, 2), Cell::Alive);
        life.set(Point::new(2, 3), Cell::Alive);
        life.set(Point::new(2, 1), Cell::Alive);
        life.tick();
        assert_eq!(life.get(Point::new(2, 2)), Cell::Dead);
    }

    #[test]
    fn reproduction_rule() {
        let mut life = Life::new();
        life.set(Point::new(3, 2), Cell::Alive);
        life.set(Point::new(1, 2), Cell::Alive);
        life.set(Point::new(2, 3), Cell::Alive);
        life.tick();
        assert_eq!(life.get(Point::new(2, 2)), Cell::Alive);
    }
}
