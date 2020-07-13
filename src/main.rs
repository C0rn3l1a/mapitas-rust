mod noise;
use crate::noise::noise_2D;
fn main() {
    println!("Noise?? \n");
    let mut out: String = "".into();
    for x in 50..100{

        for y in 50..300{
            let mut val = "_";
            if noise_2D(x  as f64 , y as f64) > -1.0 { val = "_" }
            //if noise_2D(x  as f64 , y as f64) > -0.8 { val = "2" }
            if noise_2D(x  as f64 , y as f64) > -0.6 { val = "-" }
            //if noise_2D(x  as f64 , y as f64) > -0.4 { val = "4" }
            //if noise_2D(x  as f64 , y as f64) > -0.2 { val = "5" }
            //if noise_2D(x  as f64 , y as f64) > 0.0 { val = "1" }
            if noise_2D(x  as f64 , y as f64) > 0.2 { val = "7" }
            //if noise_2D(x  as f64 , y as f64) > 0.4 { val = "8" }
            if noise_2D(x  as f64 , y as f64) > 0.6 { val = "0" }
            //if noise_2D(x  as f64 , y as f64) > 0.8 { val = "0" }
            out = format!("{}{}", out, val);
        }
        out = format!("{}\n", out);
    }
    println!("{}",out)
}