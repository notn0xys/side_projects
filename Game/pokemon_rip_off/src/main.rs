use rand::Rng;
use std::io;

const enemy1:[enemy;5] = [
    enemy{
        Name: "Rat",
        HP: 10,
        Stamina: 20,
        Power: 2,
        new_hp: 10
    },
    enemy{
        Name: "Wolf",
        HP: 20,
        Stamina: 20,
        Power: 10,
        new_hp: 20
    },
    enemy{
        Name: "Boar",
        HP: 30,
        Stamina: 40,
        Power: 20,
        new_hp: 30
    },
    enemy{
        Name: "Tiger",
        HP: 40,
        Stamina: 50,
        Power: 30,
        new_hp: 40
    },
    enemy{
        Name: "Dragon",
        HP: 60,
        Stamina: 60,
        Power: 40,
        new_hp: 60
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
    new_hp: i32
}
#[derive(Debug)]
struct enemy{
    
    Name: &str,
    HP: i32,
    Stamina: i32,
    Power: i32,
    new_hp: i32

}
enum Enemies{
    Rat,
    Wolf,
    Boar,
    Tiger,
    Dragon
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

impl Player {
    fn fight_enemy(&mut self, x:enemy, ){
        let mut input = String::new();
        println!("{:?}", self);
        println!("{:?}", x);
        println!("(1)Fight or (2)Flee");
        io::stdin().read_line(&mut input).expect("Failed to read");
        let outcome = match input.trim().parse() {
            Ok(num) => num,
            Err(_) =>{
                println!("Error");
                0
            }
        };


    }
    fn new() -> Self{
        Player {
            HP: 100,
            Stamina: 60,
            Gold: 0,
            Power: 10,
            new_hp: 0
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
                self.Stamina -= 2;
            }
            Encounter::Herb =>{
                self.Power += 1;
                self.Stamina -= 1;
            }
            Encounter::Iron_ore => {
                self.Power += 10;
                self.Stamina -= 1;
            }
            Encounter::Water => {
                self.Stamina += 2;
            }
            Encounter::Meat => {
                self.HP += 10;
                self.Stamina -= 1;

            }
            Encounter::Nothing => {
                self.Stamina -=1

            }
            Encounter::Enemy => {
                self.Stamina -= 1;
                let mut rng = rand::thread_rng();
                let n:i32 = rng.gen_range(0..100);
                match n {
                    0..=44 => self.fight_enemy(enemy1[0]),
                    45..=69 => self.fight_enemy(enemy1[1]),
                    70..=84 => self.fight_enemy(enemy1[2]),
                    85..=94 => self.fight_enemy(enemy1[3]),
                    95..=100 => self.fight_enemy(enemy1[4]),

                    _ => println!("hi");
                };
                
                
            }
        }
    }
}




fn main() {
    let mut player = Player::new();

    // let mut enemy1:Vec<enemy> = vec![
    //     enemy{
    //         Name: "Rat".to_string(),
    //         HP: 10,
    //         Stamina: 20,
    //         Power: 2,
    //         new_hp: 10
    //     },
    //     enemy{
    //         Name: "Wolf".to_string(),
    //         HP: 20,
    //         Stamina: 20,
    //         Power: 10,
    //         new_hp: 20
    //     },
    //     enemy{
    //         Name: "Boar".to_string(),
    //         HP: 30,
    //         Stamina: 40,
    //         Power: 20,
    //         new_hp: 30
    //     },
    //     enemy{
    //         Name: "Tiger".to_string(),
    //         HP: 40,
    //         Stamina: 50,
    //         Power: 30,
    //         new_hp: 40
    //     },
    //     enemy{
    //         Name: "Dragon".to_string(),
    //         HP: 60,
    //         Stamina: 60,
    //         Power: 40,
    //         new_hp: 60
    //     },        
    // ];
    
       
    loop {
        let x = player.get_encounter();
        player.Encounter(x);

    }

}
