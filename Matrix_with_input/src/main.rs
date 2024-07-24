use std::io;
fn print_matrix(x:Vec<Vec<i32>>){
    println!("");
    for i in &x{
        println!("{:?}", i)
    }
    println!("");
}
fn get_matrix() -> Vec<Vec<i32>>{
    let mut input = String::new();
    let mut final_matrix:Vec<Vec<i32>> = vec![];
    println!("Type the amount of Rows in the matrix: ");
    io::stdin().read_line(&mut input).expect("Error");
    let amount_row = input.trim().parse().expect("Failed to convert to int");
    input.clear();
    println!("Type the amount of Collums in the matrix: ");
    io::stdin().read_line(&mut input).expect("Error");
    let collums = input.trim().parse().expect("Failed to convert to int");
    input.clear();
    for i in 0..amount_row{
        println!("rows {}", i+1);
        let mut temp:Vec<i32> = vec![];
        for k in 0..collums{
            println!("for index {}", k+1);
            io::stdin().read_line(&mut input).expect("Error");
            let num = input.trim().parse().expect("Failed to convert to int");
            input.clear();
            temp.push(num)
        }
        final_matrix.push(temp)
    }

    final_matrix
}


fn matrix_mult(x:Vec<Vec<i32>> ,y: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut final_matrix:Vec<Vec<i32>> = vec![];
    for i in 0..x.len(){
        let mut temp: Vec<i32> = vec![];
        for j in 0..y[0].len(){
            let mut result = 0;
            for k in 0..y.len(){
                let var = x[i][k] * y[k][j];
                result += var

            }
            temp.push(result)
        }
        final_matrix.push(temp)
    }
    final_matrix
}

fn main() {
    let matrix1 = get_matrix();
    let matrix2 = get_matrix();
    
    
    if matrix1[0].len() != matrix2.len(){
        println!("Cant multiply this matrix");
    }
    else {
        print_matrix(matrix1.clone());
        print_matrix(matrix2.clone());
        let answer = matrix_mult(matrix1, matrix2);
        print_matrix(answer);
    }
}