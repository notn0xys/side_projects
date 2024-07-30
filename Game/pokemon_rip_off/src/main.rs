use rand::Rng;
use std::io;

const enemy1:[enemy;5] = [
    enemy{
        Name: "Rat",
        HP: 10,
        Stamina: 20,
        Power: 2,
        ID: 0
    },
    enemy{
        Name: "Wolf",
        HP: 20,
        Stamina: 20,
        Power: 10,
        ID: 1

    },
    enemy{
        Name: "Boar",
        HP: 30,
        Stamina: 40,
        Power: 20,
        ID: 2
    },
    enemy{
        Name: "Tiger",
        HP: 40,
        Stamina: 50,
        Power: 30,
        ID: 3
    },
    enemy{
        Name: "Dragon",
        HP: 60,
        Stamina: 60,
        Power: 40,
        ID: 4
    }, 
    ];       


enum Move {
    North,
    East,
    South,
    West   
}
#[derive(Debug)]
struct Player{
    HP: i32,
    Stamina: i32,
    Power: i32,
    Gold: i32,
}
#[derive(Debug, Clone)]
struct enemy<'a> {
    
    Name: &'a str,
    HP: i32,
    Stamina: i32,
    Power: i32,
    ID: i32

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
impl Move{
    fn change(&self){
        match self {
            Move::East => println!("You have moved east"),
            Move::West => println!("You have moved west"),
            Move::North => println!("You have moved North"),
            Move::South => println!("You have moved South")
        }
    }
}
impl Player {
    fn fight_enemy(&mut self, mut x:enemy, ){
        let mut input = String::new();
        loop {
            println!("{:?}", self);
            println!("{:?}", x);
            if self.HP < 0 {
                println!("You've died");
                break;
            } else if x.HP < 0 {
                match x.ID {
                    0 => {
                        println!("You've killed Rat");
                        self.Gold += 5;
                        break;
                    }
                    1 => {
                        println!("You've killed wolf");
                        self.Gold += 10;
                        break;
                    }
                    2 => {
                        println!("You've killed Boar");
                        self.Gold += 15;
                        break;
                    }
                    3 => {
                        println!("You've killed tiger");
                        self.Gold += 25;
                        break;
                    }
                    _ => {
                        println!("You've killed Dragon");
                        self.Gold += 50;
                        break;
                    }
                }
            }
            println!("(1)Fight or (2)Flee");
            io::stdin().read_line(&mut input).expect("Failed to read");
            let outcome = match input.trim().parse() {
                Ok(num) => {
                    num
                },
                Err(_) =>{
                    println!("Wrong input type");
                    continue;
                }
            };
            input.clear();
            match outcome {
                1 => {
                    let rand = rand::thread_rng().gen_range(0..100);
                    let enemy = rand::thread_rng().gen_range(0..100);
                    if rand > x.Stamina {
                        println!("Player hit");
                        if enemy > self.Stamina{
                            println!("Enemy hit");
                            x.HP -= self.Power;
                            self.HP -= x.Power;
                        }
                        else{
                            println!("Enemy Missed");
                            x.HP -= self.Power;
                        }
                    }
                    else {
                        println!("Player Missed");
                        if enemy > self.Stamina{
                            println!("Enemy hit");
                            self.HP -= x.Power;
                        }
                        else{
                            println!("Enemy Missed");
                        }
                    }

                }

                2 => {
                    println!("Run you coward");
                    break;
                }
                
                _ => {
                    println!("Invalid number");
                    continue;
                }
            }
        }
    }
    fn new() -> Self{
        Player {
            HP: 100,
            Stamina: 60,
            Gold: 0,
            Power: 10,
        }

    }
    fn get_encounter(&mut self) -> Encounter{
        let mut rng = rand::thread_rng();
        let n:i32 = rng.gen_range(0..100);
        let result = match n {
            0..=16 => Encounter::Nothing,
            17..=26 => Encounter::Meat,
            27..=39 => Encounter::Bush,
            40..=54 => Encounter::Water,
            55..=69 => Encounter::Herb,
            70..=74 => Encounter::Iron_ore,
            75..=99 => Encounter::Enemy,
            _ => Encounter:: Bush
        };
        result
        


    }

    fn Encounter(&mut self, data:Encounter){
        match data {
            Encounter::Bush => {
                println!("Found Bush Stamina -2");
                self.Stamina -= 2;
            }
            Encounter::Herb =>{
                println!("Found Herb Power + 1");
                self.Power += 1;
                self.Stamina -= 1;
            }
            Encounter::Iron_ore => {
                println!("Found Iron Power + 10");
                self.Power += 10;
                self.Stamina -= 1;
            }
            Encounter::Water => {
                println!("Found Water Stamina +2");
                self.Stamina += 2;
            }
            Encounter::Meat => {
                println!("Found Meat HP + 10");
                self.HP += 10;
                self.Stamina -= 1;

            }
            Encounter::Nothing => {
                println!("Found Nothing");
                self.Stamina -=1

            }
            Encounter::Enemy => {
                self.Stamina -= 1;
                let mut rng = rand::thread_rng();
                let n:i32 = rng.gen_range(0..100);
                match n {
                    0..=44 => self.fight_enemy(enemy1[0].clone()),
                    45..=69 => self.fight_enemy(enemy1[1].clone()),
                    70..=84 => self.fight_enemy(enemy1[2].clone()),
                    85..=94 => self.fight_enemy(enemy1[3].clone()),
                    95..=100 => self.fight_enemy(enemy1[4].clone()),
                    _ => println!("Hi"),
                
                
            }
        }
    }
}
}
fn main() {
    let mut player = Player::new();       
    loop {
        if player.Stamina < 0 || player.Gold > 200{
            break;
        }
        let mut input = String::new();
        println!("(1) Move East (2) Move West (3) Move North (4) Move South (9) Exit Game");
        io::stdin().read_line(&mut input).expect("failed to read");
        let value = match input.trim().parse(){
            Ok(1) => Move::East,
            Ok(2) => Move::West,
            Ok(3) => Move::North,
            Ok(4) => Move::South,
            Ok(9) => {
                println!("{:?}", player);
                println!("Exit game");
                break;
            }
            Ok(_) => {
                println!("Wrong input Number");
                continue;
            }
            Err(_) => {
                println!("Wrong Input Type");
                continue;
            }
        };
        value.change();
        let x = player.get_encounter();
        player.Encounter(x);

    }
    println!("Game finished");

}
