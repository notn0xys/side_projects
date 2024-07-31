use core::num;
use std::io;
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

impl Player {
    fn new() -> Player{
        Player{
            HP: 100,
            Stamina: 60,
            Power: 10,
            Gold: 0
        }
    }
    fn fight(&mut self, mut x: Enemy){
        loop {
            println!("{:?}", self);
            println!("{:?}", x);
            if self.HP <= 0 {
                println!("You've died");
                self.HP = 0;
                break;
            }
            else if x.HP <= 0 {
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
                4 => {
                    println!("You've killed Dragon");
                    self.Gold += 50;
                    break;
                }
                _ => {
                    println!("You've killed secrect last boss");
                    self.Gold += 99999999;
                    break;
                }
               }
                
            }
            let mut input = String::new();
            println!("(1)Fight or (2) Flight");
            io::stdin().read_line(&mut input).expect("Failed to read");
            let temp:i32 = match input.trim().parse(){
                Ok(1) => 1,
                Ok(2) => 2,
                Ok(_) => {
                    println!("Wrong option");
                    continue;
                },
                Err(_) => {
                    println!("wrong input type");
                    continue;
                }
            };
            match temp {
                1 => {
                    let playerrng = rand::thread_rng().gen_range(1..100);
                    let enemyrng = rand::thread_rng().gen_range(1..100);
                    if playerrng > x.Stamina{
                        println!("Player Hit");
                        if enemyrng > self.Stamina{
                            println!("Enemy Hit");
                            x.HP -= self.Power;
                            self.HP -= x.Power;

                        }
                        else {
                            x.HP -= self.Power;
                        }
                    }
                    else {
                        println!("Player Missed");
                        if enemyrng > self.Stamina {
                            println!("Enemy Hit");
                            self.HP -= x.Power
                        }
                        else {
                            println!("Enemy Missed");
                        }
                    }

                }
                _ => {
                    println!("Flee you coward");
                    break;
                }
            }
            
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
                println!("Found Bush Stamina -1");
                self.Stamina -=2;
            }
            Encounter::Iron_ore => {
                println!("Found IronOre Power + 10");
                self.Stamina -=1;
                self.Power +=10;
            }
            Encounter::Meat => {
                println!("Found meat HP + 10");
                self.Stamina -= 1;
                self.HP += 10;
            }
            Encounter::Water => {
                println!("Found Water Stamina +2");
                self.Stamina += 1;
            }
            Encounter::Nothing => {
                println!("Found Nothing");
                self.Stamina -= 1;
            }
            Encounter::Herb => {
                println!("Found Herb Power +2");
                self.Stamina -= 1;
                self.Power += 2;
            }
            Encounter::Enemy => {
                self.Stamina -= 1;
                let n = rand::thread_rng().gen_range(1..100);
                match n {
                    1..=45 => self.fight(y[0].clone()),
                    46..=70 => self.fight(y[1].clone()),
                    71..=85 => self.fight(y[2].clone()),
                    86..=95 => self.fight(y[3].clone()),
                    _ => self.fight(y[4].clone())
                }
            }
        }
    }
}

fn main() {
    let mut player = Player::new();
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
        Enemy{
            Name: "Trex".to_string(),
            HP: 100,
            Stamina: 70,
            Power: 50,
            ID:5
        }
    ];
    loop {
        if player.Gold >= 200 {
            let mut input = String::new();
            println!("Do you wish to fight secrect last boss? ");
            println!("(1) Yes (2) No");
            io::stdin().read_line(&mut input).expect("Failed to read");
            match input.trim().parse() {
                Ok(1) => {
                    player.fight(enemy1[5].clone());
                    
                },
                Ok(2) => {
                    println!("Thanks for playing");
                },
                Ok(_) => {
                    println!("Wrong number");
                    continue;
                },
                Err(_) => {
                    println!("wrong inout type");
                    continue;
                }
            };
            println!("You've won");
            break;
        }else if player.Stamina <= 0 || player.HP <= 0 {
            println!("You've lost");
            break;
        }
        let mut input = String::new();
        println!("(1) Move East (2) Move West (3) Move North (4) Move South (9) Exit Game");
        io::stdin().read_line(&mut input).expect("Failed to read");
        let temp = match input.trim().parse() {
            Ok(1) => Move::East,
            Ok(2) => Move::North,
            Ok(3) => Move::South,
            Ok(4) => Move::West,
            Ok(9) => {
                println!("Exiting game");
                println!("{:?}", player);
                break;
            }
            Ok(_) => {
                println!("Invalid input type");
                continue;
            }
            Err(_) => {
                println!("Enter an integer");
                continue;
            }
        };
        input.clear();
        temp.get_direction();
        let x = player.Encounter();
        player.result(x, enemy1.clone());
    }
    println!("{:?}",player);
    println!("Game end");

}