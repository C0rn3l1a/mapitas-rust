struct Gradient {
    x: f64,
    y: f64,
    z: f64
}

fn dot(grad: Gradient, x: f64, y:f64) -> f64{
    let ret = (grad.x * x) + (grad.y * y);
    return ret;
}

pub fn noise_2D(xin: f64, yin: f64) -> f64{
    let grad3 = [ Gradient{x:1.0,y: 1.0,z: 0.0}, Gradient{x:-1.0,y: 1.0,z: 0.0}, Gradient{x:1.0,y: -1.0,z: 0.0}, Gradient{x:-1.0,y: -1.0,z: 0.0},
    Gradient{x:1.0,y: 0.0,z: 1.0}, Gradient{x:-1.0,y: 0.0,z: 1.0}, Gradient{x:1.0,y: 0.0,z: -1.0}, Gradient{x:-1.0,y: 0.0,z: -1.0},
    Gradient{x:0.0,y: 1.0,z: 1.0}, Gradient{x:0.0,y: -1.0,z: 1.0}, Gradient{x:0.0,y: 1.0,z: -1.0}, Gradient{x:0.0,y: -1.0,z: -1.0}];


    let p: [usize; 256] = [151,160,137,91,90,15,
        131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,
        190, 6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,
        88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,
        77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,
        102,143,54, 65,25,63,161, 1,216,80,73,209,76,132,187,208, 89,18,169,200,196,
        135,130,116,188,159,86,164,100,109,198,173,186, 3,64,52,217,226,250,124,123,
        5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,
        223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167, 43,172,9,
        129,22,39,253, 19,98,108,110,79,113,224,232,178,185, 112,104,218,246,97,228,
        251,34,242,193,238,210,144,12,191,179,162,241, 81,51,145,235,249,14,239,107,
        49,192,214, 31,181,199,106,157,184, 84,204,176,115,121,50,45,127, 4,150,254,
        138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180];
        
    let mut permMod12: [usize; 512] = [0; 512];
    let mut perm: [usize; 512] = [0; 512];

    for i in 0..512 {
        perm[i] = p[i & 255];
        permMod12[i] = perm[i] % 12;
    }

    let tres: f64 = 3.0;
    let F2: f64 = 0.5 * ( tres.sqrt() - 1.0 );
    let G2: f64 = ( 3.0 - tres.sqrt() ) / 6.0;

    let n0: f64;
    let n1: f64;
    let n2: f64;
    let s: f64 =  ( xin + yin ) * F2; // Hairy factor for 2D
    let mut ii = xin + s ;
    let mut jj = yin + s ;
    let i: usize = ii.floor() as usize;
    let j: usize = jj.floor() as usize;

    let t: f64 = (i as f64 + j as f64) * G2;
    let X0: f64 = i as f64 - t; // Unskew the cell origin back to (x,y) space
    let Y0: f64 = j as f64 - t;
    let x0: f64 = xin - X0; // The x distances from the cell origin
    let y0: f64 = yin - Y0; // The y distances from the cell origin

    let i1: f64;
    let j1: f64; // Offsets for second (middle) corner of simplex in (i,j) coords
    if x0 > y0 {
        i1 = 1.0; j1 = 0.0; // lower triangle, XY order: (0,0)->(1,0)->(1,1)
    } else {
        i1 = 0.0; j1 = 1.0;
    }      // upper triangle, YX order: (0,0)->(0,1)->(1,1)
    
    let x1: f64 = x0 - i1 + G2; // Offsets for middle corner in (x,y) unskewed coords
    let y1: f64 = y0 - j1 + G2;
    let x2: f64 = x0 - 1.0 + 2.0 * G2; // Offsets for last corner in (x,y) unskewed coords
    let y2: f64 = y0 - 1.0 + 2.0 * G2;

    let ii: usize = i & 255;
    let jj: usize = j & 255;
    let gi0: usize = permMod12[ii + perm[jj]];
    let gi1: usize = permMod12[ii + i1 as usize + perm[ jj + j1 as usize ]];
    let gi2: usize = permMod12[ii + 1 + perm[ jj + 1 ]];

    let mut t0 = 0.5 - x0 * x0 - y0 * y0;
    if t0 < 0.0 {
        n0 = 0.0
    } else {
      t0 *= t0;
      n0 = t0 * t0 * dot(Gradient{x: grad3[gi0].x, y: grad3[gi0].y, z: grad3[gi0].z}, x0, y0);  // (x,y) of grad3 used for 2D gradient
    }

    let mut t1 = 0.5 - x1 * x1 - y1 * y1;
    if t1 < 0.0 {
        n1 = 0.0
    } else {
      t1 *= t1;
      n1 = t1 * t1 * dot(Gradient{x: grad3[gi1].x, y: grad3[gi1].y, z: grad3[gi1].z}, x1, y1);
    }

    let mut t2 = 0.5 - x2 * x2 - y2 * y2;
    if t2 < 0.0 {
        n2 = 0.0
    } else {
      t2 *= t2;
      n2 = t2 * t2 * dot(Gradient{x: grad3[gi2].x, y: grad3[gi2].y, z: grad3[gi2].z}, x2, y2);
    }

    return 70.0 * (n0 + n1 + n2);
}