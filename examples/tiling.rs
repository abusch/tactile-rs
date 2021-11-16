use tactile::{get_tiling_type, IsohedralTiling};

fn main() {
    let type_num = std::env::args()
        .nth(1)
        .and_then(|n| n.parse::<usize>().ok())
        .unwrap_or(21);
    // Get the 21st valid type
    let r#type = get_tiling_type(type_num);
    let tiling = IsohedralTiling::new(r#type);

    println!("This is tiling type {}. It has:", r#type);
    println!(
        "  - {} vertices, which can be controlled with {} parameters.",
        tiling.num_vertices(),
        tiling.num_params()
    );
    for (i, v) in tiling.vertices().iter().enumerate() {
        println!("    - vertex {}: {}", i, v);
    }
    println!("  - {} distinct edge shapes", tiling.num_edge_shapes());
    println!("  - the edges of the prototile are:");
    for (i, shape) in tiling.shapes().enumerate() {
        println!(
            "    - edge {} has shape {} (of type {:?})",
            i,
            shape.id(),
            shape.shape(),
        );
    }
    let (xmin, ymin, xmax, ymax) = (0.0, 0.0, 100.0, 100.0);
    let num_tiles = tiling.fill_region(xmin, ymin, xmax, ymax).iter().count();
    println!(
        "  - To fill the region [{}, {}]-[{}, {}] you would need to draw about {} tiles",
        xmin, ymin, xmax, ymax, num_tiles
    );
}
