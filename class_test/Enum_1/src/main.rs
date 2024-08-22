
fn main() {
    let mut x = vec![3,3,2,3,4];
    let mut nyah = x.clone();
    println!("{:?}",x);
    for j in x.iter_mut(){
        *j *= 2;
    } 
    for k in x.iter(){
        println!("{k}");
    }
    for i in &mut nyah {
        *i += 2;

    }
println!("{:?}",x);
}