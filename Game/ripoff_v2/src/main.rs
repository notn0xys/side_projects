use std::io::{self, Read};
use rand::Rng;
#[derive(Debug,Clone)]

struct Enemy {
    Name: String,
    HP: i32,
    Stamina: i32,
    Power: i32,
    ID: i32
}

#[derive(Debug)]
struct  Player {
    HP: i32,
    Stamina: i32,
    Power: i32,
    Gold: i32,
}

enum Move {
    North,
    East,
    South,
    West
}

enum Encounter {
    Meat,
    Nothing,
    Bush,
    Water,
    Herb,
    Iron_ore,
    Enemy
}

impl Move {
    fn get_direction(&self){
        match self {
            Move::East => println!("You've moved east"),
            Move::North => println!("You've moved north"),
            Move::South => println!("You've moved south"),
            Move::West => println!("You've moved west")
        }
    }
}


fn get_enemy (x:Vec<Enemy>, y:i32) -> Enemy{
    match y {
        0 => x[0].clone(),
        1 => x[1].clone(),
        2 => x[2].clone(),
        3 => x[3].clone(),
        _ => x[4].clone()
    }
}
impl Player {
    fn fight(&self, x: Enemy){
        loop {
            
        }

    }
    fn Encounter(&self) -> Encounter{
        let n = rand::thread_rng().gen_range(1..100);
        match n {
            1..=17 => Encounter::Nothing,
            18..=25 => Encounter::Bush,
            26..=40 => Encounter::Meat,
            41..=55 => Encounter::Water,
            56..=70 => Encounter::Herb,
            71..=75 => Encounter::Iron_ore,
            _  => Encounter::Enemy
        }
    }

    fn result(&mut self, x:Encounter, y:Vec<Enemy>){
        match x {
            Encounter::Bush => {
                self.Stamina -=2;
            }
            Encounter::Iron_ore => {
                self.Stamina -=1;
                self.Power +=10;
            }
            Encounter::Meat => {
                self.Stamina -= 1;
                self.HP += 5
            }
            Encounter::Water => {
                self.Stamina += 1;
            }
            Encounter::Nothing => {
                self.Stamina -= 1;
            }
            Encounter::Herb => {
                self.Stamina -= 1;
                self.Power += 2;
            }
            Encounter::Enemy => {
                self.Stamina -= 1;
                let n = rand::thread_rng().gen_range(1..100);
                match n {
                    1..=45 => self.fight(get_enemy(y,0)),
                    46..=70 => self.fight(get_enemy(y, 1)),
                    71..=85 => self.fight(get_enemy(y, 2)),
                    86..=95 => self.fight(get_enemy(y, 3)),
                    _ => self.fight(get_enemy(y, 4))
                }
            }
        }
    }
}

fn main() {
    let mut enemy1:Vec<Enemy> = vec![
        Enemy{
            Name: "Rat".to_string(),
            HP: 10,
            Stamina: 20,
            Power: 2,
            ID: 0
        },
        Enemy{
            Name: "Wolf".to_string(),
            HP: 20,
            Stamina: 20,
            Power: 10,
            ID: 1
        },
        Enemy{
            Name: "Boar".to_string(),
            HP: 30,
            Stamina: 40,
            Power: 20,
            ID: 2
        },
        Enemy{
            Name: "Tiger".to_string(),
            HP: 40,
            Stamina: 50,
            Power: 30,
            ID:3
        },
        Enemy{
            Name: "Dragon".to_string(),
            HP: 60,
            Stamina: 60,
            Power: 40,
            ID: 4
        },        
    ];
    loop {
        let mut input =String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let temp = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter an integer");
                continue;
            }
        }
    }

}