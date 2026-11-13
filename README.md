# Algorithms and Data Structures in Rust 
A collection of data structures and algorithms implemented from scratch in **Rust** to explore memory management, time/space complexity, and core computer science fundamentals.

## Repository Structure

Each folder is a standalone Cargo project containing the implementation and test cases for a specific topic:

| [`GapBuffer/`](./GapBuffer) | **Gap Buffer** | Dynamic array for efficient text editor cursor insertions and deletions (`O(1)` amortized at cursor) |<br />
| [`MeshTriangles/`](./MeshTriangles) | 3D Mesh Triangle Culling | Filters the $k$ closest 3D triangles to a camera viewpoint using sorting / Top-$k$ selection ($O(n \log n)$ / $O(n)$) |
| [`RustyBunny/`](./RustyyBunny) | **Mesh Rust Simulation** | Simulates rust spreading day-by-day across a 3D mesh using a Dual Graph adjacency list and Level-Order Breadth-First Search (BFS) ($O(V + E)$). |
