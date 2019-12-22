use polygons::io;
use polygons::structures::Edge;
use polygons::stuff;

fn floats_are_same(f1: f64, f2: f64) -> bool {
    let d = f1 - f2;
    return d.abs() < std::f64::EPSILON;
}

#[test]
fn rectangle() {
    let mut polygons: Vec<Vec<Edge>> = Vec::new();

    let (xs, ys) = io::read_points("tests/rectangle.txt");
    let num_points = xs.len();
    let polygon = stuff::create_polygon(num_points, &xs, 0.0, &ys, 0.0, 0);
    polygons.push(polygon);

    let mut nodes = Vec::new();
    for p in polygons.iter() {
        // group edges to nodes, 4 at the time
        nodes.append(&mut stuff::group_edges(4, p.clone()));
    }

    // we group nodes into a tree
    while nodes.len() > 1 {
        nodes = stuff::group_nodes(4, nodes);
    }

    let pxs: [f64; 2] = [0.6, 0.5];
    let pys: [f64; 2] = [0.6, -0.5];

    let mut distances: [f64; 2] = [0.0; 2];
    stuff::get_distances_edge(&nodes, 2, &pxs, &pys, &mut distances);
    assert_eq!(distances, [0.4, 0.5]);

    distances = [0.0; 2];
    stuff::get_distances_vertex(&nodes, 2, &pxs, &pys, &mut distances);
    assert!(floats_are_same(distances[0], 0.5656854249492381));
    assert!(floats_are_same(distances[1], 0.7071067811865476));

    let mut indices: [usize; 2] = [0; 2];
    stuff::get_closest_vertices(&nodes, 2, &pxs, &pys, &mut indices);
    assert_eq!(indices, [2, 0]);

    let mut contains: [bool; 2] = [false; 2];
    stuff::contains_points(&nodes, 2, &pxs, &pys, &mut contains);
    assert_eq!(contains, [true, false]);
}

#[test]
fn polygon() {
    let mut polygons: Vec<Vec<Edge>> = Vec::new();

    let (xs, ys) = io::read_points("tests/polygon.txt");
    let num_points = xs.len();
    let polygon = stuff::create_polygon(num_points, &xs, 0.0, &ys, 0.0, 0);
    polygons.push(polygon);
    let polygon = stuff::create_polygon(num_points, &xs, 5.0, &ys, 0.0, num_points);
    polygons.push(polygon);
    let polygon = stuff::create_polygon(num_points, &xs, 10.0, &ys, 0.0, 2 * num_points);
    polygons.push(polygon);
    let polygon = stuff::create_polygon(num_points, &xs, 15.0, &ys, 0.0, 3 * num_points);
    polygons.push(polygon);
    let polygon = stuff::create_polygon(num_points, &xs, 20.0, &ys, 0.0, 4 * num_points);
    polygons.push(polygon);

    let mut nodes = Vec::new();
    for p in polygons.iter() {
        // group edges to nodes, 4 at the time
        nodes.append(&mut stuff::group_edges(4, p.clone()));
    }

    // we group nodes into a tree
    while nodes.len() > 1 {
        nodes = stuff::group_nodes(4, nodes);
    }

    const NUM_REFERENCE_POINTS: usize = 5000;

    let (pxs, pys) = io::read_points("tests/reference/reference_points.txt");

    let mut distances: [f64; NUM_REFERENCE_POINTS] = [0.0; NUM_REFERENCE_POINTS];
    stuff::get_distances_edge(&nodes, NUM_REFERENCE_POINTS, &pxs, &pys, &mut distances);
    let reference_distances = io::read_vector("tests/reference/distances_edge.txt");
    for (i, &reference_distance) in reference_distances.iter().enumerate() {
        assert!(floats_are_same(distances[i], reference_distance));
    }

    let mut distances = [0.0; NUM_REFERENCE_POINTS];
    stuff::get_distances_vertex(&nodes, NUM_REFERENCE_POINTS, &pxs, &pys, &mut distances);
    let reference_distances = io::read_vector("tests/reference/distances_vertex.txt");
    for (i, &reference_distance) in reference_distances.iter().enumerate() {
        assert!(floats_are_same(distances[i], reference_distance));
    }

    let mut indices: [usize; NUM_REFERENCE_POINTS] = [0; NUM_REFERENCE_POINTS];
    stuff::get_closest_vertices(&nodes, NUM_REFERENCE_POINTS, &pxs, &pys, &mut indices);
    let reference_indices = io::read_vector("tests/reference/closest_indices.txt");
    for (i, &reference_index) in reference_indices.iter().enumerate() {
        assert_eq!(indices[i], reference_index);
    }

    let mut contains: [bool; NUM_REFERENCE_POINTS] = [false; NUM_REFERENCE_POINTS];
    stuff::contains_points(&nodes, NUM_REFERENCE_POINTS, &pxs, &pys, &mut contains);
    let reference_bools = io::read_vector("tests/reference/contains_points.txt");
    for (i, &reference_bool) in reference_bools.iter().enumerate() {
        assert_eq!(contains[i], reference_bool);
    }
}
