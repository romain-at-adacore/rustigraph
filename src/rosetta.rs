use std::f64::consts::PI;

// A 2D coordinate in cartesian space.
#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

// A mathematical description of a rosetta (specifically, a hypotrochoid),
// formed by tracing a point attached to a circle rolling inside another circle.
#[derive(Debug, Clone, Copy)]
pub struct Hypotrochoid {
    pub outer_radius: f64, // Radius of the fixed outer circle.
    pub inner_radius: f64, // Radius of the rolling inner circle.
    pub pen_offset: f64,   // From the center of the inner circle to the drawing point.
    pub steps: usize,      // Number of steps (points) used to approximate the curve.
}

impl Hypotrochoid {
    /// Computes a single point on the hypotrochoid curve for a given angle theta.
    /// Uses the standard parametric equation of a hypotrochoid. 
    fn generate_point(&self, theta: f64) -> Coordinate {
        let r_diff = self.outer_radius - self.inner_radius;
        let ratio = r_diff / self.inner_radius;

        Coordinate {
            x: r_diff * theta.cos() + self.pen_offset * (ratio * theta).cos(),
            y: r_diff * theta.sin() - self.pen_offset * (ratio * theta).sin(),
        }
    }

    /// Computes all the points of the hypotrochoid curve and recenters them.
    /// The result is an array of coordinates centered around the origin.
    pub fn compute_points(&self) -> Vec<Coordinate> {
        let mut points = Vec::with_capacity(self.steps + 1);
        let (mut max_x, mut min_x) = (f64::MIN, f64::MAX);
        let (mut max_y, mut min_y) = (f64::MIN, f64::MAX);

        // The number of revolutions is hard-coded for simplicity's sake.
        // It can be calculated from the large (R) and small (r) radius to "close" the curve.
        // Formula: `revolutions = r / gcd(R, r)` (using integer radius).
        const REVOLUTIONS: f64 = 16.0;
        
        // Computes raw points and updates the bounding box extents.
        for j in 0..=self.steps {
            let theta = 2.0 * PI * (j as f64) / (self.steps as f64) * REVOLUTIONS;
            let p = self.generate_point(theta);
            points.push(p);

            max_x = max_x.max(p.x);
            min_x = min_x.min(p.x);
            max_y = max_y.max(p.y);
            min_y = min_y.min(p.y);
        }

        // Computes the center offset based on the bounding box.
        let offset_x = (max_x + min_x) / 2.0;
        let offset_y = (max_y + min_y) / 2.0;

        // Recenters all points by substracting the center offset.
        for p in &mut points {
            p.x -= offset_x;
            p.y -= offset_y;
        }

        points
    }
}
