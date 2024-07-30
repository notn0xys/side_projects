enum Season{
    Summer,
    Winter,
    Rainy
}
impl Season{
    fn discribe(&self) -> String{
        match self{
            Season::Summer => return "This is summer".to_string(),
            Season::Rainy => return  "This is Rainy".to_string(),
            Season::Winter => return  "this is snowy".to_string()

        }
    }
    fn action(&self) {
        match self {
            Season::Rainy => println!("It is raining"),
            Season::Summer => println!("It is Sunny"),
            Season::Winter => println!("IT is snowing")
        }
    }
}
fn main() {
    let s = Season::Summer;
    let r = Season::Rainy;
    let w = Season::Winter;

    r.action();
    let meow = r.discribe();
    println!("{}", meow);
}
