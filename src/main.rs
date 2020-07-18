mod simple_map;
mod noise;
use crate::simple_map::SimpleMap;
use crate::simple_map::Entity;

fn main() {
    println!("Noise?? \n");
    gen_one_map(1);
}

fn gen_one_map(seed: usize){
    let map = SimpleMap::from_noise(0 + seed , 100 + seed, 0 + seed, 50 + seed);

    for i in map.quadrants.as_slice(){
        let mut line = "".into();
        
        for u in i.as_slice(){
            match &u.entity {
                Entity::HighMountain => line = format!("{}{}",line,"\x1b[0;31m#\x1b[0m"),
                Entity::Mountain => line = format!("{}{}",line,"\x1b[0;32m#\x1b[0m"),
                Entity::Water => line = format!("{}{}",line,"\x1b[0;33m#\x1b[0m"),
                Entity::DeepWater => line = format!("{}{}",line,"\x1b[0;34m#\x1b[0m"),
                Entity::Plain => line = format!("{}{}",line,"\x1b[0;35m#\x1b[0m"),
                Entity::Forest => line = format!("{}{}",line,"\x1b[0;36m#\x1b[0m"),
                Entity::Coast => line = format!("{}{}",line,"\x1b[0;37m#\x1b[0m"),
            }
        }

        println!("{}",line)
    }
}