mod simple_map;
mod noise;
use crate::simple_map::SimpleMap;
fn main() {
    println!("Noise?? \n");
    let map = SimpleMap::from_noise(0, 100, 0, 100);
    
    for i in map.quadrants.iter(){
        for u in i.iter(){
            println!("{}",u.entity);
        }
    }
}