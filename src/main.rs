mod simple_map;
mod noise;
use crate::simple_map::SimpleMap;
use crate::simple_map::Entity;
use ggez;
use ggez::{Context, GameResult};

struct AppState {
    map: SimpleMap
}

impl ggez::event::EventHandler for AppState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
       ggez::graphics::clear(ctx, ggez::graphics::BLACK);
       for (x, x_quadrants) in self.map.quadrants.iter().enumerate() {
         for (y, quadrant) in x_quadrants.into_iter().enumerate() {
             let color = match quadrant.entity {
                 simple_map::Entity::HighMountain => simple_map::BROWN,
                 simple_map::Entity::Mountain => simple_map::LIGHT_BROWN,
                 simple_map::Entity::Plain => simple_map::GREEN,
                 simple_map::Entity::Forest => simple_map::DARK_GREEN,
                 simple_map::Entity::Water => simple_map::BLUE,
                 simple_map::Entity::DeepWater => simple_map::DARK_BLUE,
                 simple_map::Entity::Coast => simple_map::LIGHT_YELLOW,
             };
             let q = ggez::graphics::Mesh::new_rectangle(
                 ctx, 
                 ggez::graphics::DrawMode::fill(), 
                 ggez::graphics::Rect::new((x * 8) as f32, (y * 8) as f32, 8.0, 8.0), 
                 color.into()).unwrap();
             ggez::graphics::draw(ctx, &q, ggez::graphics::DrawParam::default()).unwrap();
          }
       }
       ggez::graphics::present(ctx).unwrap();
       Ok(()) 
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}
fn main() {
    let cb = ggez::ContextBuilder::new("mapitas", "boios");
    let (ctx, events_loop) = &mut cb.build().unwrap();
    ggez::event::run(ctx, events_loop, &mut AppState { map: return_one_map(1)}).unwrap();
    println!("Noise?? \n");
    gen_one_map(1);
}

fn return_one_map(seed: usize) -> SimpleMap {
    SimpleMap::from_noise(0 + seed , 100 + seed, 0 + seed, 50 + seed)
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