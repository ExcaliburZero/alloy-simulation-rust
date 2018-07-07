fn main() {
    let materials_def = create_materials_def(0.75, 1.0, 1.25);
    let mut alloy = create_alloy(5, 5, materials_def);

    initialize_materials(&mut alloy);
    initialize_points(&mut alloy);

    print_materials(&alloy);

    println!("");

    print_points(&alloy, &(alloy.points_a));
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
    // TODO: change to actual implementation
    for i in (0..alloy.width) {
        for j in (0..alloy.height) {
            for m in (0..3) {
                let index = offset_3d(alloy.width, alloy.height, i, j, m);
                alloy.materials[index as usize] = 0.33333333;
            }
        }
    }
}

fn initialize_points(alloy: &mut Alloy) {
    // TODO: implement
}

fn print_materials(alloy: &Alloy) {
    for i in (0..alloy.width) {
        for j in (0..alloy.height) {
            for m in (0..3) {
                let index = offset_3d(alloy.width, alloy.height, i, j, m);
                let value: f64 = *&(alloy.materials)[index as usize];
                print!("{} ", value);
            }
            println!("");
        }
    }
}

fn print_points(alloy: &Alloy, points: &Vec<f64>) {
    for i in (0..alloy.width) {
        for j in (0..alloy.height) {
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
