// struct Monster {
//     health: int,
//     attack: int
// }

// fn main() {
//     let m = Monster { health: 10, attack: 20 };

//     println!("{}", m.health);
//     println!("{}", m.attack);
// }



struct Monster {
    health: int,
    attack: int
}

impl Monster {
    fn attack(&self) {
        println!("The monster attacks for {} damage.", self.attack);
    }
}

fn main() {
    let m = Monster { health: 10, attack: 20 };

    m.attack();
}
