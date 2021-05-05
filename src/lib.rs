use pyo3::prelude::*;

#[pymodule]
fn rust_helpers(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "get_neighbors")]
    fn get_neighbors(_py: Python, raw_point: [f32; 2]) -> Vec<(i32, i32)> {
        // Return the 8 neighboring coordinates of point
        let mut neighbors: Vec<(i32, i32)> = Vec::with_capacity(8);
        /* Uses 0 to 3 and -1 because I'm not sure if Rust allows
        signed ints as iteration values */
        let point = [raw_point[0] as i32, raw_point[1] as i32];
        for i in 0..3 {
            for j in 0..3 {
                // skips what would be +0, +0 (the initial point)
                if i == 1 && j == 1 {
                    continue;
                }
                neighbors.push((point[0] + i - 1, point[1] + j - 1));
            }
        }
        return neighbors;
    }

    #[pyfn(m, "index_of_closest")]
    fn index_of_closest(
        _py: Python,
        unit_positions: Vec<[f32; 2]>,
        target_position: [f32; 2],
    ) -> usize {
        let mut closest_index: usize = 0;
        let mut closest_distance: f32 = 9999.9;
        for i in 0..unit_positions.len() {
            let dist: f32 = get_squared_distance(unit_positions[i], target_position);
            if dist < closest_distance {
                closest_index = i;
                closest_distance = dist;
            }
        }
        return closest_index;
    }

    fn get_squared_distance(p1: [f32; 2], p2: [f32; 2]) -> f32 {
        return f32::powf(p1[0] - p2[0], 2.0) + f32::powf(p1[1] - p2[1], 2.0);
    }

    Ok(())
}
