fn main(){
    let mut a:[i32;5]= [1,2,3,4,5];
    let mut s = a[2];
    for i in &mut a{
        i += 2;
        println!("{i}");
    }
    println!("{:?}",a);

}