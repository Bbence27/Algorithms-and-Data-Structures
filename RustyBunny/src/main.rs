use std::println;


#[derive(Clone, Copy, Debug)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

type Triangle = [Point; 3];


struct Mesh {
    triangles: Vec<Triangle>,
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

fn point_less(a: Point, b: Point) -> bool {
    a.x < b.x || (a.x == b.x && a.y < b.y) || (a.x == b.x && a.y == b.y && a.z < b.z)
}

fn point_eq(a: Point, b: Point) -> bool {
    a.x == b.x && a.y == b.y && a.z == b.z
}

fn edge_less(a: &(Point, Point, u32), b: &(Point, Point, u32)) -> bool {
    if !point_eq(a.0, b.0) {
        point_less(a.0, b.0)
    } else {
        point_less(a.1, b.1)
    }
}

fn make_edge(a: Point, b: Point, id: u32) -> (Point, Point, u32) {
    if point_less(a, b) {
        (a, b, id)
    } else {
        (b, a, id)
    }
}

fn partition_edges(edges: &mut [(Point, Point, u32)]) -> usize {
    let pivot = edges.len() / 2;
    let last = edges.len() - 1;
    edges.swap(pivot, last);
    let pivot_edge = edges[last]; // Copy the pivot tuple so we can compare against it
    
    let mut first_larger = 0;
    for i in 0..last {
        if edge_less(&edges[i], &pivot_edge) {
            edges.swap(i, first_larger);
            first_larger += 1;
        }
    }
    edges.swap(first_larger, last);
    first_larger
}

fn sort_edges(edges: &mut [(Point, Point, u32)]) {
    if edges.len() <= 1 {
        return;
    }
    let pivot = partition_edges(edges);
    sort_edges(&mut edges[..pivot]);
    sort_edges(&mut edges[pivot + 1..]);
}

fn collect_edges(mesh: &Mesh) -> Vec<(Point, Point, u32)>{
    let mut edges: Vec<(Point, Point, u32)> = Vec::with_capacity(3 * mesh.triangles.len());
    for i in 0..mesh.triangles.len(){
        let edge1= make_edge(mesh.triangles[i][0], mesh.triangles[i][1], i as u32);
        let edge2 = make_edge(mesh.triangles[i][1], mesh.triangles[i][2], i as u32);
        let edge3 = make_edge(mesh.triangles[i][0], mesh.triangles[i][2], i as u32);
        edges.push(edge1);
        edges.push(edge2);
        edges.push(edge3);
    }
    sort_edges(&mut edges);
    edges
}

fn build_adjacency_list(mesh: &Mesh,edges: &mut [(Point, Point, u32)]) -> Vec<Vec<u32>>{
    let mut adjacency_list: Vec<Vec<u32>> = vec![Vec::new(); mesh.triangles.len()];
    let mut i = 0;
    while i < edges.len() - 1 {
        if point_eq(edges[i].0, edges[i + 1].0) && point_eq(edges[i].1, edges[i + 1].1) {
            adjacency_list[edges[i].2 as usize].push(edges[i + 1].2);
            adjacency_list[edges[i + 1].2 as usize].push(edges[i].2);
            i += 2;
        }else{
            i += 1;
        }
    }
    adjacency_list
}

fn simulate_rust(adjacency_list: &Vec<Vec<u32>>, start: u32){
    let mut queue: Vec<u32> = Vec::new();
    let mut visited: Vec<bool> = vec![false; adjacency_list.len()];

    let mut day = 1;
    let mut total_rusted = 0;

    queue.push(start);
    visited[start as usize] = true;

    while !queue.is_empty() {
        let new_rust     = queue.len();
        println!("Day {}: {} new rusted triangles, {} total rusted triangles", day, new_rust, total_rusted + new_rust);

        total_rusted += new_rust;   

        for _ in 0..new_rust {
            let current = queue.remove(0);
            for &neighbor in &adjacency_list[current as usize] {
                if !visited[neighbor as usize] {
                    visited[neighbor as usize] = true;
                    queue.push(neighbor);
                }
            }
        }
        day += 1;
    }
    let total_days = day - 1;
    println!("Total days to rust all reachable triangles: {}", total_days);

    if total_rusted < adjacency_list.len() {
        let unreachable = adjacency_list.len() - total_rusted;
        println!("{} triangles were unreachable and did not rust.", unreachable);
    }

}
fn main() {
    let mesh = load_mesh("bunny.txt");
    let mut edges = collect_edges(&mesh);
    let adjacency_list = build_adjacency_list(&mesh, &mut edges);
    println!("From which triangle do you want to start rusting? (0-{})", mesh.triangles.len() - 1);
    
    let mut start_triangle = String::new();

    std::io::stdin().read_line(&mut start_triangle).expect("failed to readline");
    simulate_rust(&adjacency_list, start_triangle.trim().parse::<u32>().expect("please enter a valid triangle index"));
}
