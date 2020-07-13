type Color = (i32, i32, i32, i32);

pub enum Entity{
    HighMountain(Color),
    Mountain(Color),
    Water(Color),
    DeepWater(Color)
    Plain(Color),
    Forest(Color),
    Coast(Color)
}

pub struct Quadrant {
    weight: f64 // valor de la funcion simplex
    entity: Entity // entidad
}

pub struct SimpleMap {
    quadrants: Vec<Vec<Quadrant>>
}

impl SimpleMap{
    fn from_noise() -> SimpleMap{

    }
}