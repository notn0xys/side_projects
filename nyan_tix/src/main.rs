//Use rand Crate pls import in to cargo.toml
use std::{io};
use rand::Rng;

fn get_matrix() -> Vec<Vec<i32>>{
    let mut input = String::new();
    let mut final_matrix:Vec<Vec<i32>> = vec![];
    println!("Type the matrix size ");
    io::stdin().read_line(&mut input).expect("Error");
    let n = input.trim().parse().expect("Failed to convert to int");
    input.clear();
    for i in 0..n{
        let mut temp:Vec<i32> = vec![];
        for k in 0..n{
            let num = rand::thread_rng().gen_range(0..=10);
            temp.push(num)
        }
        final_matrix.push(temp)
    }

    final_matrix
}
fn print_matrix(matrix:&Vec<Vec<i32>>){
    for i in matrix{
        println!("{:?}",i);
    }
}
fn sum_row(matrix:&Vec<Vec<i32>>,row:usize) -> i32{
    let mut sum = 0;
    for i in 0..matrix[row - 1].len(){
        sum += matrix[row - 1][i];
    }
    sum
}
fn sum_collum(matrix:&Vec<Vec<i32>>,colllum:usize) -> i32{
    let mut sum = 0;
    for i in 0..matrix.len(){
        sum += matrix[i][colllum - 1];
    }
    sum
}
fn sum_diagonal(matrix:&Vec<Vec<i32>>) -> i32{
    let mut sum = 0;
    for i in 0..matrix.len(){
        sum += matrix[i][i];
    }
    sum
}
fn sum_anti_diagonal(matrix:&Vec<Vec<i32>>) -> i32{
    let mut sum = 0;
    for i in 0..matrix.len(){
        sum += matrix[i][matrix.len()-(i + 1)];
    }
    sum
}

fn main() {
let matrix1 = get_matrix();
print_matrix(&matrix1);
for i in 0..matrix1.len(){
    println!("Sum Row: {} {}",i + 1, sum_row(&matrix1, i + 1));
}
for i in 0..matrix1.len(){
    println!("Sum Collum: {} {}",i + 1, sum_collum(&matrix1, i + 1));
}
println!("Sum Diagonal: {}", sum_diagonal(&matrix1));
println!("Sum Anti Diagonal: {}", sum_anti_diagonal(&matrix1));
}
