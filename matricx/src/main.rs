

fn matrix_multiply(x:[[i32; 3]; 3], y: [[i32; 3]; 3]) -> Vec<Vec<i32>>{
    let mut fianl_result:Vec<Vec<i32>> = vec![];
    let mut value = 0;
    for i in 0..x.len(){
        let mut added_matrix:Vec<i32> = vec![];
        for k in 0..y[0].len(){
            for z in 0..y.len(){
                let val = x[i][z] * y[z][k];
                value += val;
            }
            added_matrix.push(value);
        }
        fianl_result.push(added_matrix);
    }
    
    fianl_result
}


fn main() {
    let x = [
        [1,5,6],
        [2,4,3],
        [4,6,7]
        
    ];
    let y= [
        [1,5,5],
        [2,4,7],
        [2,4,7]
    ];
    let x = matrix_multiply(x, y);
    for i in &x{
        println!("{:?}", i);
    }
    
}
