use crate::noise::noise_2D;

pub type Color = (f32, f32, f32, f32);

pub const RED: Color = (255.0/255.0 ,0.0/255.0 ,0.0/255.0 ,255.0/255.0 );
pub const GREEN: Color = (0.0,255.0/255.0 ,0.0,255.0/255.0 );
pub const BLUE: Color = (0.0,0.0,255.0/255.0 ,255.0/255.0 );
pub const BROWN: Color = (150.0/255.0 ,100.0/255.0 , 50.0/255.0 , 255.0/255.0 );
pub const YELLOW: Color = (255.0/255.0 ,255.0/255.0 ,0.0,255.0);
pub const WHITE: Color = (255.0/255.0 ,255.0/255.0 ,255.0/255.0 ,255.0/255.0 );
pub const LIGHT_BLUE: Color = (0.0, 220.0/255.0 , 220.0/255.0 ,255.0/255.0 );
pub const DARK_BLUE: Color = (0.0,0.0,122.0/255.0 ,255.0/255.0 );
pub const DARK_GREEN: Color = (0.0,122.0/255.0 ,0.0,255.0/255.0 );
pub const LIGHT_BROWN: Color = (200.0/255.0 ,100.0/255.0 ,50.0/255.0 ,255.0/255.0 );
pub const LIGHT_YELLOW: Color = (255.0/255.0 ,255.0/255.0 ,122.0/255.0 ,255.0/255.0 );

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