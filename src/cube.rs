pub struct Cube {
    pub positions: [[f32; 3]; 8],
    pub indices: [u32; 6 * 2 * 3],
    pub colors: [[f32; 3]; 8],
}

/**
 * 1x1x1 cube.
 *      *-(v7)---------* (v5)
 *     /              /|
 *    /              / |
 *   /              /  |
 *  /              /   |
 * *--(v3)--------*(v2)|
 * |    * (v6)    |    * (v4)
 * |              |   /
 * |              |  /
 * |              | /
 * |              |/
 * *--------------*
 * (v0)           (v1)
 */
impl Cube {
    /**  Counter clock wise, Z+ is towards the camera (right handed).
     * Return a 1x1x1 cube.
     *
     */
    pub fn new() -> Self {
        Self {
            positions: [
                // v0 - v7
                [-0.5, -0.5, 0.5],
                [0.5, -0.5, 0.5],
                [0.5, 0.5, 0.5],
                [-0.5, 0.5, 0.5],
                [0.5, -0.5, -0.5],
                [0.5, 0.5, -0.5],
                [-0.5, -0.5, -0.5],
                [-0.5, 0.5, -0.5],
            ],
            indices: [
                0, 1, 2, 0, 2, 3, 1, 4, 5, 1, 5, 2, 3, 2, 5, 3, 5, 7, 6, 4, 1, 6, 1, 0, 4, 6, 7, 4,
                7, 5, 6, 0, 3, 6, 3, 7,
            ],
            colors: [
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.3, 0.6],
                [0.3, 0.3, 0.3],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
            ],
        }
    }

    pub fn interleaved(&self) -> [[f32; 3]; 8 * 2] {
        let result: [[f32; 3]; 8 * 2] = Default::default();
        self.positions
            .iter()
            .zip(self.colors.iter())
            .enumerate()
            .fold(result, |mut acc, (i, (a, b))| {
                acc[i * 2] = *a;
                acc[i * 2 + 1] = *b;
                acc
            })
    }
}
