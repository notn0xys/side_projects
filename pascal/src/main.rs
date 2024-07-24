use std::io;

fn pascal(i:i32, j:i32) -> i32{
    if j == 0{
        1
    }else if i == j {
        1
    }else{
        pascal(i-1, j-1) + pascal(i-1, j)
    }
} 

fn main(){
    let mut input = String::new();
    println!("Input amount of Row");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let row = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };
    for i in 0..row{
        for space in 0..(row - (i+1)){
            print!(" ");
        }
        for j in 0..i + 1{
            let a = pascal(i, j);
            print!("{} ",a);

        }
        println!("")
    }
}