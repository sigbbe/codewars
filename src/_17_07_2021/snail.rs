
#[allow(dead_code)]
fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let matrix_len: usize = matrix.as_ref().iter().map(|v| v.len()).sum();
    if matrix_len == 0 {
        return res;
    }
    if matrix_len != matrix[0].len() * matrix[0].len() {
        panic!("Argument matrix must be n x n array");
    }
    let get_col = |i: usize| matrix.iter().map(|v| {
        v[i]
    }).collect::<Vec<i32>>();
    let mut x_start = 0;
    let mut x_end = matrix[x_start].len() - 1;
    let mut y_start = 0;
    let mut y_end = matrix.len() - 1;
    while res.len() <= matrix_len {
        if x_start > x_end || y_start > y_end {
            break;
        }
        let mut row: Vec<i32> = matrix[y_start][x_start..=x_end].to_vec();
        for i in row {
            res.push(i);
        }
        y_start += 1;
        // println!("[{start}..{end}]", start=y_start, end=y_end);
        if x_start > x_end || y_start > y_end {
            break;
        }
        let mut col: Vec<i32> = get_col(x_end)[y_start..=y_end].to_vec();
        for i in col {
            res.push(i);
        }
        x_end -= 1;
        if x_start > x_end || y_start > y_end {
            break;
        }
        row = matrix[y_end][x_start..=x_end].iter().rev().cloned().collect();
        for i in row {
            res.push(i);
        }
        y_end -= 1;
        if x_start > x_end || y_start > y_end {
            break;
        }
        col = get_col(x_start)[y_start..=y_end].iter().rev().cloned().collect();
        for i in col {
            res.push(i);
        }
        x_start += 1;
    };
    res
}

#[allow(dead_code)]
pub fn run() {
    let array_0 = &[vec![1,2,3], vec![8,9,4], vec![7,6,5]];
    let array_1 = &[vec![1,2,3,1], vec![4,5,6,4], vec![7,8,9,7], vec![7,8,9,7]];
    println!("{:?}", snail(array_0));
    println!("{:?}", snail(array_1));
    println!("{:?}", snail(&[vec![1]]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snail() {
        let input: &[Vec<i32>] = &[vec![1,2,3], vec![4,5,6], vec![7,8,9]];
        snail(input);
    }
}