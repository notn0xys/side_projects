fn plus_one(x:&mut i32){
    *x += 1;
}
fn main() {
    let mut x = vec![3,3,2,3,4];
    let mut nyah = &mut x;
    let a = x[0];
    for i in &mut *nyah {
        *i += 2;
    }
println!("{:?}",x);
let mut meow = 2;
plus_one(&mut meow);
println!("{meow}")
}