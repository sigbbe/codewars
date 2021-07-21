use std::str::FromStr;
use preload::FIGHTERS;
use preload::Direction;
use position::Position;

mod preload {
    #[allow(dead_code)]
    pub const FIGHTERS: [[&str; 6]; 2] = [
        ["Ryu", "E.Honda", "Blanka", "Guile", "Balrog", "Vega"],
        ["Ken", "Chun Li", "Zangief", "Dhalsim", "Sagat", "M.Bison"], 
    ];
    
    #[derive(Debug)]
    pub struct DirectionError(String);
    
    #[derive(Debug, Clone, Copy)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    
    use std::str::FromStr;
    impl FromStr for Direction {
        type Err = DirectionError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                u if u.to_ascii_lowercase().trim() == "up"    => Ok(Direction::Up), 
                d if d.to_ascii_lowercase().trim() == "down"  => Ok(Direction::Down), 
                r if r.to_ascii_lowercase().trim() == "right" => Ok(Direction::Right), 
                l if l.to_ascii_lowercase().trim() == "left"  => Ok(Direction::Left),

                _ => Err(DirectionError(String::from("Invalid string slice for Direction enum!")))
            }
        }
    }
}

pub mod position {
    use super::preload::Direction;
    use std::fmt::Debug;
    use std::fmt::Formatter;
    use std::fmt::Result;
    use std::cmp::Eq;
    use std::cmp::PartialEq;

    #[derive(Clone, Copy)]
    pub struct Position(i64, i64, usize, usize);

    impl Debug for Position {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "({},{})", self.get_x(), self.get_y())
        }
    }

    impl PartialEq for Position {
        fn eq(&self, other: &Position) -> bool {
            (self.get_x() == other.get_x()) & (self.get_y() == self.get_y())
        }
    }
    impl Eq for Position {}

    impl Position {
        pub fn new(x: i64, y: i64, board_dim: (usize, usize)) -> Self {
            let (width, height) = board_dim;
            Self(x, y, width, height)
        }
        pub fn move_direction(&mut self, d: Direction) -> Self {
            let old_pos = self.clone();
            match d {
                Direction::Up => {
                    if self.1 > 0 {
                        self.1 -= 1;
                    }
                },
                Direction::Down  => {
                    if (self.1 as usize) < self.get_height() - 1 {
                        self.1 += 1;
                    }
                }, 
                Direction::Right => {
                    if self.0 as usize >= self.get_width() - 1 {
                        self.0 = 0;
                    } else {
                        self.0 += 1;
                    }
                }, 
                Direction::Left  => {
                    if self.0 <= 0 {
                        self.0 = (self.get_width() - 1) as i64;
                    } else {
                        self.0 = self.0 - 1;
                    }

                }
            };
            old_pos
        }
        pub fn get_x(&self) -> usize {
            self.0 as usize
        }
        pub fn get_y(&self) -> usize {
            self.1 as usize
        }
        pub fn get_width(&self) -> usize {
            self.2
        }
        pub fn get_height(&self) -> usize {
            self.3
        }
    }
}
#[allow(dead_code)]
pub fn street_fighter_selection(fighters: &[[&str; 6]; 2], position: &[i64; 2], moves: &[Direction]) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut pos: Position = Position::new(position[0], position[1], (fighters[0].len(), fighters.len()));
    for m in moves {
        let _old_pos = pos.move_direction(*m);
        res.push(String::from(fighters[pos.get_y()][pos.get_x()]));
    }
    res
}

#[allow(dead_code)]
pub fn run() {
    let initial_position: &[i64; 2] = &[0, 0];
    let moves: &[Direction] = &["up", "left", "right", "left", "left"].iter().map(|i| Direction::from_str(i).unwrap()).collect::<Vec<Direction>>();
    let res = street_fighter_selection(&FIGHTERS, initial_position, moves);
    println!("{:?}", res);
}

#[cfg(test)]
mod tests {
    use super::street_fighter_selection;
    use super::preload::{Direction, FIGHTERS};

    #[test]
    fn few_moves() {
        use Direction::*;
        let moves = [Up, Left, Right, Left, Left];
        
        assert_eq!(
            street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
            ["Ryu", "Vega", "Ryu", "Vega", "Balrog"],
        );
    }
    #[test]
    fn no_moves() {
        let moves: [Direction; 0] = [];
        
        assert_eq!(street_fighter_selection(&FIGHTERS, &[0, 0], &moves), [] as [String; 0]);
    }
    
    #[test]
    fn moving_left() {
        use Direction::*;
        let moves = [Left, Left, Left, Left, Left, Left, Left, Left];
        
        assert_eq!(
            street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
            ["Vega", "Balrog", "Guile", "Blanka", "E.Honda", "Ryu", "Vega", "Balrog"],
        );
    }
    
    #[test]
    fn moving_right() {
        use Direction::*;
        let moves = [Right, Right, Right, Right, Right, Right, Right, Right];
        
        assert_eq!(
            street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
            ["E.Honda", "Blanka", "Guile", "Balrog", "Vega", "Ryu", "E.Honda", "Blanka"],
        );
    }
    
    #[test]
    fn uses_all_4_directions_clockwise_twice() {
        use Direction::*;
        let moves = [Up, Left, Down, Right, Up, Left, Down, Right];
        
        assert_eq!(
            street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
            ["Ryu", "Vega", "M.Bison", "Ken", "Ryu", "Vega", "M.Bison", "Ken"],
        );
    }
    
    #[test]
    fn always_moving_down() {
        use Direction::*;
        let moves = [Down, Down, Down, Down];
        
        assert_eq!(
            street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
            ["Ken", "Ken", "Ken", "Ken"],
        );        
    }
    
    #[test]
    fn always_moving_up() {
        use Direction::*;
        let moves = [Up, Up, Up, Up];
        
        assert_eq!(
            street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
            ["Ryu", "Ryu", "Ryu", "Ryu"],
        );        
    }
}