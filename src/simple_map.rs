use crate::noise::noise_2D;

type Color = (i32, i32, i32, i32);

const RED: Color = (255,0,0,255);
const GREEN: Color = (0,255,0,255);
const BLUE: Color = (0,0,255,255);
const BROWN: Color = (150,100, 50, 255);
const YELLOW: Color = (255,255,0,255);
const WHITE: Color = (255,255,255,255);
const LIGHT_BLUE: Color = (0, 220, 220,255);
const DARK_BLUE: Color = (0,0,122,255);
const DARK_GREEN: Color = (0,122,0,255);
const LIGHT_BROWN: Color = (200,100,50,255);
const LIGHT_YELLOW: Color = (255,255,122,255);

pub enum Entity{
    HighMountain,
    Mountain,
    Water,
    DeepWater,
    Plain,
    Forest,
    Coast
}

pub struct Quadrant {
    pub weight: f64, // valor de la funcion simplex
    pub entity: Entity // entidad
}

pub struct SimpleMap {
    pub quadrants: Vec<Vec<Quadrant>>
}

impl SimpleMap{
    pub fn from_noise(start_x: usize, end_x: usize, start_y: usize, end_y: usize) -> SimpleMap{
        let mut map: Vec<Vec<Quadrant>> = vec![];
        for y in start_y..end_y {
            let mut row: Vec<Quadrant> = vec![];
            for x in start_x..end_x {
                let weight = noise_2D(x as f64, y as f64);
                let entity: Entity;
                // -0.8 < weight < 0.8
                if weight > 0.58 {
                    entity = Entity::HighMountain;
                } else if weight > 0.36 {
                    entity = Entity::Mountain;
                } else if weight > 0.14 {
                    entity = Entity::Forest;
                } else if weight > -0.08 {
                    entity = Entity::Plain;
                } else if weight > -0.3 {
                    entity = Entity::Coast;
                } else if weight > -0.52 {
                    entity = Entity::Water;
                } else {
                    entity = Entity::DeepWater;
                }

                let quadrant: Quadrant = Quadrant{ weight, entity};
                row.push(quadrant)
            }
            map.push(row)
        }
        return SimpleMap{quadrants: map}
    }
    
}