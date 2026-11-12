use std::fs::File;
use std::io::{BufWriter, Write};


#[derive(Clone, Copy)]
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
    
    let mut triangles: Vec<Triangle> = Vec::with_capacity(triangle_count);

    for _ in 0..triangle_count {
        let mut triangle = [Point { x: 0.0, y: 0.0, z: 0.0 }; 3];
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
            triangle[p] = Point { x, y, z };
        }
        triangles.push(triangle);
    }
    Mesh { triangles }
}

fn free_mesh(mesh: &mut Mesh) {
    mesh.triangles.clear();
    mesh.triangles.shrink_to_fit();
}

fn save_to_file(mesh: &Mesh, filename: &str) {
    let file = File::create(filename)
        .unwrap_or_else(|error| panic!("failed to create mesh file '{filename}': {error}"));
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", mesh.triangles.len()).expect("failed to write triangle count");
    for triangle in &mesh.triangles {
        write!(writer, "{:.6} {:.6} {:.6} ", triangle[0].x, triangle[0].y, triangle[0].z).expect("failed to write vertex coordinates");
        write!(writer, "{:.6} {:.6} {:.6} ", triangle[1].x, triangle[1].y, triangle[1].z).expect("failed to write vertex coordinates");
        write!(writer, "{:.6} {:.6} {:.6} ", triangle[2].x, triangle[2].y, triangle[2].z).expect("failed to write vertex coordinates");
        writeln!(writer).expect("failed to write newline");
    }
}

fn partition(items: &mut [(Triangle, f32)]) -> usize{
    let pivot = items.len() / 2;
    let last = items.len() - 1;
    items.swap(pivot, last);
    let pivot_distance = items[last].1;
    let mut first_larger = 0;
    for i in 0..last{
        if items[i].1 < pivot_distance {
            items.swap(i, first_larger);
            first_larger += 1;
        }
    }
    items.swap(first_larger, last);
    first_larger
}

fn quick_sort_left_k(items: &mut [(Triangle, f32)], k: usize) {
    let pivot = partition(items);
    if items.len() <= 1 || k == 0 || k >= items.len() {
        return;
    }
    if pivot < k - 1 {
        quick_sort_left_k(&mut items[pivot + 1..], k - pivot - 1);
    } else if pivot > k {
        quick_sort_left_k(&mut items[..pivot], k);
    }
}

fn closest_triangles(mesh: &Mesh, p: Point, count: usize) -> Mesh {
    let mut triangle_distances: Vec<(Triangle, f32)> = Vec::with_capacity(mesh.triangles.len());

    for triangle in &mesh.triangles {
        let distance = distance_pt(p, *triangle);
        triangle_distances.push((*triangle, distance));
    }

    quick_sort_left_k(&mut triangle_distances, count);

    let mut closest_triangles: Vec<Triangle> = Vec::with_capacity(count);
    for (triangle, _) in triangle_distances.iter().take(count) {
        closest_triangles.push(*triangle);
    }

    Mesh { triangles: closest_triangles }

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
