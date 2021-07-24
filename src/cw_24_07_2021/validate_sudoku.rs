mod solution {}

/// Rules for validation
/// - [x] Data structure dimension: NxN where N > 0 and √N == integer
/// - [x] Rows may only contain integers: 1..N (N included)
/// - [x] Columns may only contain integers: 1..N (N included)
/// - [ ] 'Little squares' (3x3 in example above) may also only contain integers: 1..N (N included)


struct Sudoku {
    data: Vec<Vec<u32>>
}

#[allow(dead_code)]
impl Sudoku {
    fn is_valid(&self) -> bool {
        let n = self.data.len();
        let n_root = (n as f64).sqrt();
        println!("n: {}\nsqrt(n): {}", n, n_root);
        if n <= 0 || !(n_root.fract() == 0.0) {
            println!("Invalid n: {}", n);
            false
        } else {
            for row in &self.data {
                let k = row.len();
                if k != n {
                    println!("Invalid row len: {}, n={}", k, n);
                    return false;
                }
                for col in row {
                    if *col < 1 || *col > n as u32 {
                        println!("Invalid cell value: {}", col);
                        return false;
                    }
                }
            }
            
            return true;
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    let s = Sudoku{
        data: vec![
                vec![1,2,3,  4,5,6,  7,8,9],
                vec![1,2,3,  4,5,6,  7,8,9],
                vec![1,2,3,  4,5,6,  7,8,9],
  
                vec![1,2,3,  4,5,6,  7,8,9],
                vec![1,2,3,  4,5,6,  7,8,9],
                vec![1,2,3,  4,5,6,  7,8,9],
                  
                vec![1,2,3,  4,5,6,  7,8,9],
                vec![1,2,3,  4,5,6,  7,8,9],
                vec![1,2,3,  4,5,6,  7,8,9],
            ]
    };

    let v = s.is_valid();
    println!("{}", v);
}

#[test]
fn good_sudoku() {
    let good_sudoku_1 = Sudoku{
        data: vec![
                vec![7,8,4, 1,5,9, 3,2,6],
                vec![5,3,9, 6,7,2, 8,4,1],
                vec![6,1,2, 4,3,8, 7,5,9],

                vec![9,2,8, 7,1,5, 4,6,3],
                vec![3,5,7, 8,4,6, 1,9,2],
                vec![4,6,1, 9,2,3, 5,8,7],
                
                vec![8,7,6, 3,9,4, 2,1,5],
                vec![2,4,3, 5,6,1, 9,7,8],
                vec![1,9,5, 2,8,7, 6,3,4]
            ]
    };
    
    let good_sudoku_2 = Sudoku{
        data: vec![
                vec![1, 4,  2, 3],
                vec![3, 2,  4, 1],
        
                vec![4, 1,  3, 2],
                vec![2, 3,  1, 4],
            ]
    };
    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
}

#[test]
fn bad_sudoku() {
    let bad_sudoku_1 = Sudoku{
        data: vec![
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],

                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
            ]
    };
    
    let bad_sudoku_2 = Sudoku{
        data: vec![
                vec![1,2,3,4,5],
                vec![1,2,3,4],
                vec![1,2,3,4],
                vec![1],
            ]
    };
    assert!(!bad_sudoku_1.is_valid());
    assert!(!bad_sudoku_2.is_valid());
}