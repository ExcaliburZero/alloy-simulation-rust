extern crate image;
extern crate rand;

use image::{ImageBuffer, Rgb};
use rand::Rng;

fn main() {
    let path = "img/";

    let materials_def = create_materials_def(0.75, 1.0, 1.25);
    let mut alloy = create_alloy(50, 50, materials_def);

    initialize_materials(&mut alloy);
    initialize_points(&mut alloy);

    print_materials(&alloy);

    //println!("");

    let iterations = 100;

    for i in 0..iterations {
        alloy = update_alloy(i, alloy);
        write_alloy_png(&alloy, i, &path);
    }
}

struct MaterialsDef {
    const1: f64,
    const2: f64,
    const3: f64,
    ratio1: f64,
    ratio2: f64,
    ratio3: f64,
}

struct Alloy {
    width: i32,
    height: i32,
    materials_def: MaterialsDef,
    points_a: Vec<f64>,
    points_b: Vec<f64>,
    materials: Vec<f64>,
}

fn create_materials_def(const1: f64, const2: f64, const3: f64) -> MaterialsDef {
    MaterialsDef {
        const1: const1,
        const2: const2,
        const3: const3,
        ratio1: 1.0 / 3.0,
        ratio2: 1.0 / 3.0,
        ratio3: 1.0 / 3.0,
    }
}

fn create_alloy(width: i32, height: i32, materials_def: MaterialsDef) -> Alloy {
    Alloy {
        width: width,
        height: height,
        materials_def: materials_def,
        points_a: vec![0.0; (width * height) as usize],
        points_b: vec![0.0; (width * height) as usize],
        materials: vec![0.0; (width * height * 3) as usize],
    }
}

fn initialize_materials(alloy: &mut Alloy) {
    let mut rng = rand::thread_rng();
    for i in 0..alloy.width {
        for j in 0..alloy.height {
            let var = 12;

            let r1 = ((rng.gen::<i32>() % (2 * var)) - var) as f64;
            let r2 = ((rng.gen::<i32>() % (2 * var)) - var) as f64;
            let r3 = ((rng.gen::<i32>() % (2 * var)) - var) as f64;

            let mut p1 = alloy.materials_def.ratio1 * 100.0 + r1;
            let mut p2 = alloy.materials_def.ratio2 * 100.0 + r2;
            let mut p3 = alloy.materials_def.ratio3 * 100.0 + r3;

            if p1 < 0.0 {
                p1 = 0.0;
            }
            if p2 < 0.0 {
                p2 = 0.0;
            }
            if p3 < 0.0 {
                p3 = 0.0;
            }

            let total = p1 + p2 + p3;

            alloy.materials[offset_3d(alloy.width, alloy.height, i, j, 0) as usize] = p1 / total;
            alloy.materials[offset_3d(alloy.width, alloy.height, i, j, 1) as usize] = p2 / total;
            alloy.materials[offset_3d(alloy.width, alloy.height, i, j, 2) as usize] = p3 / total;
        }
    }
}

fn initialize_points(alloy: &mut Alloy) {
    // TODO: implement
    alloy.points_a[0] = 1000000.0;
}

fn print_materials(alloy: &Alloy) {
    for i in 0..alloy.width {
        for j in 0..alloy.height {
            for m in 0..3 {
                let index = offset_3d(alloy.width, alloy.height, i, j, m);
                let value: f64 = *&(alloy.materials)[index as usize];
                print!("{} ", value);
            }
            println!("");
        }
    }
}

fn print_points(alloy: &Alloy, points: &Vec<f64>) {
    for i in 0..alloy.width {
        for j in 0..alloy.height {
            let index = offset_2d(alloy.width, i, j);
            let value: f64 = *&(points)[index as usize];
            print!("{} ", value);
        }
        println!("");
    }
}

fn offset_2d(width: i32, x: i32, y: i32) -> i32 {
    y * width + x
}

fn offset_3d(width: i32, height: i32, x: i32, y: i32, z: i32) -> i32 {
    z * width * height + y * width + x
}

fn update_alloy(turn: i32, alloy: Alloy) -> Alloy {
    let mut alloy = alloy;
    {
        let (read, write) = if turn % 2 == 0 {
            (&alloy.points_a, &mut alloy.points_b)
        } else {
            (&alloy.points_b, &mut alloy.points_a)
        };

        for i in 0..alloy.width {
            for j in 0..alloy.height {
                let index = offset_2d(alloy.width, i, j) as usize;
                let value = next_position_temp(&alloy.materials_def, &alloy.materials, alloy.width, alloy.height, &read, i, j);
                write[index] = value;
            }
        }
    }

    alloy
}

fn next_position_temp(materials_def: &MaterialsDef, materials: &Vec<f64>, width: i32, height: i32, read: &Vec<f64>, x: i32, y: i32) -> f64 {
    let mut num_neighbors: f64 = 0.0;

    let mut total_temp: f64 = 0.0;
    for m in 0..3 {
        let mut temp_mat: f64 = 0.0;
        for i in (x - 1)..(x + 2) {
            let mut temp_per: f64 = 0.0;
            for j in (y - 1)..(y + 2) {
                if i >= 0 && j >= 0 &&
                    i < width && j < height {
                    let index = offset_2d(width, i, j) as usize;
                    let mat_index = offset_3d(width, height, i, j, m) as usize;
                    let value: f64 = read[index];
                    let mat_percent: f64 = materials[mat_index];

                    temp_per += value * mat_percent;

                    if m == 0 {
                        num_neighbors += 1.0;
                    }
                }
            }


            temp_mat += temp_per;
        }



        match m {
            0 => total_temp += temp_mat * materials_def.const1,
            1 => total_temp += temp_mat * materials_def.const2,
            _ => total_temp += temp_mat * materials_def.const3,
        }
    }





    return total_temp / num_neighbors;
}

fn pix(value: f64, max: f64) -> u8 {
    if value < 0.0 {
        return value as u8;
    }

    let val = (256.0 * (value / max)) as i32;
    if val > 255 {
        return 255;
    }

    val as u8
}

fn write_alloy_png(alloy: &Alloy, i: i32, path: &str) {
    let width = alloy.width as u32;
    let height = alloy.height as u32;
    let mut image = ImageBuffer::<Rgb<u8>, _>::new(width, height);

    for i in 0..alloy.width {
        for j in 0..alloy.height {
            // TODO: fix to use correct points Vec
            let index = offset_2d(alloy.width, i, j) as usize;
            image.get_pixel_mut(i as u32, j as u32).data = [pix(alloy.points_a[index], 1000.0), 0, 0];
        }
    }

    let mut file = path.clone().to_owned();
    let num_string = i.to_string();
    file.push_str(&num_string);
    file.push_str(".png");

    println!("{}", file);
    image.save(file).unwrap();
}
