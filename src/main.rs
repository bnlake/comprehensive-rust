type Matrix = [[i32; 3]; 3];

fn transpose(matrix: Matrix) -> Matrix {
    let mut result: Matrix = [[0; 3]; 3];
    for (y, row) in matrix.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            result[x][y] = *val; 
        }
    }
    result
}

fn pretty_print(matrix: Matrix) {
    for line in matrix {
        let x: Vec<String> = line.iter().map(|c| c.to_string()).collect();

        let mut joined = String::new();
        joined.push('[');
        joined.push_str(&x.join(", "));
        joined.push(']');
        println!("{joined}");
    }
}

fn main() {
    let matrix: Matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];

    println!("Matrix:");
    pretty_print(matrix);

    let transposed = transpose(matrix);
    println!("Transposed Matrix:");
    pretty_print(transposed);
}
