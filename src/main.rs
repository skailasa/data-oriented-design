use std::time::Instant;
use data_oriented_design::*;
use std::hint::black_box;


fn main() {
    let num_shapes = 10000;
    let num_iterations = 10000;
    let mut shapes = Vec::new();

    for _i in 0..num_shapes {
        shapes.push(Shape::Square { side: 5f32 });
        shapes.push(Shape::Rectangle {
            width: 4f32,
            height: 6f32,
        });
        shapes.push(Shape::Triangle {
            base: 3f32,
            height: 7f32,
        });
    }

    let s = Instant::now();

    for _i in 0..num_iterations {
        // let area =  total_area(&shapes);
        let area =  total_vertices_area(&shapes);
        black_box(area); // Prevent optimization of the total_area calculation

    }

    let elapsed = s.elapsed();
    let avg_time_per_iteration = elapsed / num_iterations;
    let avg_time_per_shape = avg_time_per_iteration / num_shapes;

    let cpu_clock_speed = 3.2; // GhZ
    let cycles_per_shape = avg_time_per_shape.as_secs_f32() * cpu_clock_speed * 1e9;
    println!("(Rust/Traits) Average cycles per shape: {:?}", cycles_per_shape);

}