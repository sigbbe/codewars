mod solution {}

pub mod preloaded {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Direction {
        #[allow(dead_code)]
        Up,
        #[allow(dead_code)]
        Down,
        #[allow(dead_code)]
        Left,
        #[allow(dead_code)]
        Right,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Position {
        pub x: usize,
        pub y: usize,
    }

    impl Position {
        #[allow(dead_code)]
        pub fn new(x: usize, y: usize) -> Self {
            Self { x, y }
        }
    }
}

use preloaded::{Direction, Position};
use std::ops::Neg;

impl Neg for Direction {
    type Output = Direction;
    fn neg(self) -> Self::Output {
        match self {
            Direction::Down => Direction::Up, 
            Direction::Up => Direction::Down, 
            Direction::Left => Direction::Right, 
            Direction::Right => Direction::Left
        }
    }
}

impl Position {
    #[allow(dead_code)]
    pub fn move_direction(&self, d: Direction, width: usize, height: usize) -> Position {
        let mut res = Position::new(self.x, self.y);
        match d {
            Direction::Up => {
                if res.y > 0 {
                    res.y -= 1;
                }
            }, 
            Direction::Right => {
                if res.x < width - 1 {
                    res.x += 1;
                } else {
                    res.x = 0;
                }
            }, 
            Direction::Down => {
                if res.y < height - 1 {
                    res.y += 1;
                }
            }, 
            Direction::Left => {
                if res.x > 0 {
                    res.x -= 1;
                } else {
                    res.x = width - 1;
                }
            }
        };
        res
    }
}

#[allow(dead_code)]
fn super_street_fighter_selection(fighters: &[&[&str]], position: Position, moves: &[Direction]) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let width = fighters[0].len();
    let height = fighters.len();
    let mut p = position;
    // For each move
    for m in moves {
        // Make the move
        p = p.move_direction(*m, width, height);
        // If the spot is empty, move back if the move was vertical, 
        // and keep on moving in the same direction if the move was horizontal
        'l: while fighters[p.y][p.x] == "" {
            if *m == Direction::Up || *m == Direction::Down {
                // If a vertical move hits a blank space, go back 
                p = p.move_direction(-*m, width, height);
                break 'l;
            }
            p = p.move_direction(*m, width, height);
        }
        // Get the value of the new position
        let new_value = fighters[p.y][p.x];
        res.push(new_value.to_string());
    }
    res
}

#[allow(dead_code)]
pub fn run() {
    let fighters: [&[&'static str]; 3] = [
        &[       "",    "Ryu",  "E.Honda",  "Blanka",   "Guile", ""       ],
        &[ "Balrog",    "Ken",  "Chun Li", "Zangief", "Dhalsim", "Sagat"  ],
        &[   "Vega", "T.Hawk", "Fei Long",  "Deejay",   "Cammy", "M.Bison"],
    ];
    let initial_position: Position = Position::new(0, 1);
    let moves: &[Direction] = &[Direction::Down, Direction::Down, Direction::Down];
    let res = super_street_fighter_selection(&fighters, initial_position, moves);
    println!("{:?}", res);    
}

#[cfg(test)]
mod tests {
    use super::super_street_fighter_selection;
    use super::preloaded::{Direction, Position};

    #[rustfmt::skip]
    const FIGHTERS_A: [&[&'static str]; 3] = [
        &[       "",    "Ryu",  "E.Honda",  "Blanka",   "Guile", ""       ],
        &[ "Balrog",    "Ken",  "Chun Li", "Zangief", "Dhalsim", "Sagat"  ],
        &[   "Vega", "T.Hawk", "Fei Long",  "Deejay",   "Cammy", "M.Bison"],
    ];
    
    #[rustfmt::skip]
    const FIGHTERS_B: [&[&'static str]; 3] = [
        &[       "",    "Ryu",  "E.Honda",  "Cammy",  "Blanka",   "Guile",        "", "Chun Li" ],
        &[ "Balrog",    "Ken",  "Chun Li",       "", "M.Bison", "Zangief", "Dhalsim", "Sagat"   ],
        &[   "Vega",       "", "Fei Long", "Balrog",  "Deejay",   "Cammy",        "", "T.Hawk"  ],
    ];
    
    #[rustfmt::skip]
    const FIGHTERS_C: [&[&'static str]; 6] = [
        &[        "",     "Ryu",  "E.Honda",  "Cammy" ],
        &[  "Balrog",     "Ken",  "Chun Li",       "" ],
        &[    "Vega",        "", "Fei Long", "Balrog",],
        &[  "Blanka",   "Guile",         "", "Chun Li"],
        &[ "M.Bison", "Zangief",  "Dhalsim", "Sagat"  ],
        &[  "Deejay",   "Cammy",         "", "T.Hawk" ],
    ];    
    
    #[test]
    fn no_selection() {
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 0), &[] as &[Direction]),
            vec![] as Vec<String>,
            "it should work with no selection cursor moves",
        );
    }
    
    #[test]
    fn empty_vertical_spaces_single_move() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 1), &[Up]),
            vec!["Balrog"],
            "it should stop on empty spaces vertically",
        );
    }
    
    #[test]
    fn empty_vertical_spaces_multiple_moves() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 1), &[Up, Up, Up, Up]),
            vec!["Balrog", "Balrog", "Balrog", "Balrog"],
            "it should stop on empty spaces vertically (up)",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 1), &[Down, Down, Down, Down]),
            vec!["Vega", "Vega", "Vega", "Vega"],
            "it should stop on empty spaces vertically (down)",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(5, 1), &[Up, Up, Up, Up]),
            vec!["Sagat", "Sagat", "Sagat", "Sagat"],
            "it should stop on empty spaces vertically (up)",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(5, 1), &[Down, Down, Down, Down]),
            vec!["M.Bison", "M.Bison", "M.Bison", "M.Bison"],
            "it should stop on empty spaces vertically (down)",
        );        
    }
    
    #[test]
    fn rotate_horizontally() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(2, 0), &[Left, Left, Left, Left, Left, Left, Left, Left]),
            vec!["Ryu", "Guile", "Blanka", "E.Honda", "Ryu", "Guile", "Blanka", "E.Honda"],
            "it should rotate horizontally (left)",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(3, 1), &[Left, Left, Left, Left, Left, Left, Left, Left]),
            vec!["Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "Chun Li", "Ken"],
            "it should rotate horizontally (left)",
        );        
    }
    
    #[test]
    fn rotate_horizontally_with_empty_spaces() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(2, 0), &[Right, Right, Right, Right, Right, Right, Right, Right]),
            vec!["Blanka", "Guile", "Ryu", "E.Honda", "Blanka", "Guile", "Ryu", "E.Honda"],
            "it should rotate horizontally with empty spaces",
        );        
    }
    
    #[test]
    fn rotate_on_all_rows() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(2, 0), &[Right, Right, Right, Right, Right, Right, Down, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Down, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right]),
            vec!["Blanka", "Guile", "Ryu", "E.Honda", "Blanka", "Guile", "Dhalsim", "Zangief", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Cammy", "M.Bison", "Vega", "T.Hawk", "Fei Long", "Deejay", "Cammy", "M.Bison", "Vega", "T.Hawk", "Fei Long", "Deejay", "Cammy"],
            "it should rotate on all rows",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_B[..], Position::new(2, 0), &[Right, Right, Right, Right, Right, Right, Down, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Down, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right]),
            vec!["Cammy", "Blanka", "Guile", "Chun Li", "Ryu", "E.Honda", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "M.Bison", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "Cammy", "T.Hawk", "Vega", "Fei Long", "Balrog", "Deejay", "Cammy", "T.Hawk", "Vega", "Fei Long", "Balrog", "Deejay", "Cammy"],
            "it should rotate on all rows",
        );        
    }
    
    #[test]
    fn should_rotate_on_all_rows() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_B[..], Position::new(3, 0), &[Down, Right, Right, Right, Down, Left, Left, Down, Right, Right, Right, Up]),
            vec!["Cammy", "Blanka", "Guile", "Chun Li", "Sagat", "Dhalsim", "Zangief", "Cammy", "T.Hawk", "Vega", "Fei Long", "Chun Li"],
            "it should rotate on all rows",
        );
    }
    
    #[test]
    fn should_work_with_longer_grid() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_C[..], Position::new(3, 0), &[Left, Left, Down, Right, Right, Right, Right, Down, Left, Left, Left, Left, Down, Right, Right, Down, Right, Right, Right, Down, Left, Left, Left, Down, Left, Left, Left]),
            vec!["E.Honda", "Ryu", "Ken", "Chun Li", "Balrog", "Ken", "Chun Li", "Fei Long", "Vega", "Balrog", "Fei Long", "Vega", "Blanka", "Guile", "Chun Li", "Sagat", "M.Bison", "Zangief", "Dhalsim", "Dhalsim", "Zangief", "M.Bison", "Sagat", "T.Hawk", "Cammy", "Deejay", "T.Hawk"],
            "it should work with longer grid",
        );        
    }
    
    #[test]
    fn should_work_with_odd_initial_position() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_C[..], Position::new(3, 3), &[Left, Left, Down, Right, Right, Right, Right, Down, Left, Left, Left, Left, Up, Right, Right,  Up, Right, Right, Right]),
            vec!["Guile", "Blanka", "M.Bison", "Zangief", "Dhalsim", "Sagat", "M.Bison", "Deejay", "T.Hawk", "Cammy", "Deejay", "T.Hawk", "Sagat", "M.Bison", "Zangief", "Guile", "Chun Li", "Blanka", "Guile"],
            "it should work with odd initial position",
        );        
    }
}

