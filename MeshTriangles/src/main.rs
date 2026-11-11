use std::fs::File;
use std::io::{BufWriter, Write};


struct Point {
    x: f32,
    y: f32,
    z: f32,
}

type Triangle = [Point; 3];


struct Mesh {
    triangles: Vec<Triangle>,
}

fn distance_pp(p1: Point, p2: Point) -> f32 {
    ((p1.x - p2.x).powi(2)
        + (p1.y - p2.y).powi(2)
        + (p1.z - p2.z).powi(2))
        .sqrt()
}

fn distance_pt(p: Point, t: Triangle) -> f32 {
    let d0 = distance_pp(p, t[0]);
    let d1 = distance_pp(p, t[1]);
    let d2 = distance_pp(p, t[2]);
    d0.min(d1).min(d2)
}

fn load_mesh(filename: &str) -> Mesh {
    let contents = std::fs::read_to_string(filename)
        .unwrap_or_else(|error| panic!("failed to read mesh file '{filename}': {error}"));
    let mut values = contents.split_whitespace();
    let triangle_count = values
        .next()
        .expect("mesh file is missing its triangle count")
        .parse::<usize>()
        .unwrap_or_else(|error| panic!("invalid triangle count in '{filename}': {error}"));

    let mut mesh = Mesh {
        triangles: Vec::with_capacity(triangle_count),
    };

    for _ in 0..triangle_count {
        let mut triangle: Triangle = [
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 0.0, y: 0.0, z: 0.0 },
        ];
        for p in 0..3 {
            let x = values
                .next()
                .expect("mesh file is missing a vertex coordinate")
                .parse::<f32>()
                .expect("mesh vertex coordinate is not a valid number");
            let y = values
                .next()
                .expect("mesh file is missing a vertex coordinate")
                .parse::<f32>()
                .expect("mesh vertex coordinate is not a valid number");
            let z = values
                .next()
                .expect("mesh file is missing a vertex coordinate")
                .parse::<f32>()
                .expect("mesh vertex coordinate is not a valid number");  
            triangle[p] = Point { x, y, z}
        }

        mesh.triangles.push(triangle);
    }
    mesh
}

fn free_mesh(mesh: &mut Mesh) {
    // TODO
    mesh.triangles.clear();
}

fn save_to_file(mesh: &Mesh, filename: &str) {
    let file = File::create(filename)
        .unwrap_or_else(|error| panic!("failed to create mesh file '{filename}': {error}"));
    let mut writer = BufWriter::new(file);
    let _ = mesh;
    writeln!(writer, "{}", 1).expect("failed to write triangle count");
    writeln!(writer, "{} {} {}", 1.0, 2.0, 3.0).expect("failed to write vertex coordinates");
}

fn closest_triangles(mesh: &Mesh, p: Point, count: usize) -> Mesh {
    // TODO
    let _ = (mesh, p, count);
    todo!()
}

fn main() {
    let mesh = load_mesh("bunny.txt");
    save_to_file(&mesh, "test_bunny_copy.txt");

    let players = [
        Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        Point {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        },
    ];
    let counts = [1, 5, 10, 100, 1000, 10000, 100000];

    for (player_index, player) in players.iter().enumerate() {
        for count in counts {
            println!("Player {}, Count {}", player_index + 1, count);
            let mut closest = closest_triangles(&mesh, *player, count);
            let filename = format!("player_{}_count_{}.mesh", player_index + 1, count);
            save_to_file(&closest, &filename);
            free_mesh(&mut closest);
        }
    }
}
