use std::io;
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}



fn main() {
    let mut data:Vec<Person> = Vec::new();
    for i in 0..5{
        let mut age1: String = String::new();
        let mut name1 = String::new();
        println!("Enter name");
        io::stdin().read_line(&mut name1).expect("Failed to read");
        let name2 = name1.trim().to_string();
        println!("Enter age");
        io::stdin().read_line(&mut age1).expect("Failed to read");
        let age2 = age1.trim().parse().expect("Failed to convert");

        let  people: Person = Person{
            name: name2,
            age: age2,
        };
        data.push(people)
    }

    for i in data{
        println!("{:?}",i);

    }
}
