use crate::noise::noise_2D;

type Color = (i32, i32, i32, i32);

pub enum Entity{
    HighMountain(Color),
    Mountain(Color),
    Water(Color),
    DeepWater(Color),
    Plain(Color),
    Forest(Color),
    Coast(Color)
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
                let color = (0,0,0,0);
                let mut entity: Entity;
                // -0.8 < weight < 0.8
                if weight > 0.58 {
                    entity = Entity::HighMountain(color);
                } else if weight > 0.36 {
                    entity = Entity::Mountain(color);
                } else if weight > 0.14 {
                    entity = Entity::Forest(color);
                } else if weight > -0.08 {
                    entity = Entity::Plain(color);
                } else if weight > -0.3 {
                    entity = Entity::Coast(color);
                } else if weight > -0.52 {
                    entity = Entity::Water(color);
                } else {
                    entity = Entity::DeepWater(color);
                }

                let quadrant: Quadrant = Quadrant{ weight, entity};
                row.push(quadrant)
            }
            map.push(row)
        }
        return SimpleMap{quadrants: map}
    }
    
}