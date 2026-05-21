pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

/// Checks if two Axis-Aligned Bounding Boxes intersect
pub fn aabb_vs_aabb(a: &Rect, b: &Rect) -> bool {
    a.x < b.x + b.w &&
    a.x + a.w > b.x &&
    a.y < b.y + b.h &&
    a.y + a.h > b.y
}

/// Checks if a rectangle and a circle intersect
pub fn aabb_vs_circle(rect: &Rect, circle: &Circle) -> bool {
    // Find the closest point to the circle within the rectangle
    let closest_x = circle.x.clamp(rect.x, rect.x + rect.w);
    let closest_y = circle.y.clamp(rect.y, rect.y + rect.h);

    // Calculate the distance between the circle's center and this closest point
    let dx = circle.x - closest_x;
    let dy = circle.y - closest_y;

    (dx * dx + dy * dy) < (circle.radius * circle.radius)
}

/// Checks if two circles intersect
pub fn circle_vs_circle(a: &Circle, b: &Circle) -> bool {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let distance_sq = dx * dx + dy * dy;
    let radius_sum = a.radius + b.radius;
    distance_sq < (radius_sum * radius_sum)
}

/// Reflects a velocity vector against a surface normal
pub fn reflect(velocity: [f32; 2], normal: [f32; 2]) -> [f32; 2] {
    let dot = velocity[0] * normal[0] + velocity[1] * normal[1];
    [
        velocity[0] - 2.0 * dot * normal[0],
        velocity[1] - 2.0 * dot * normal[1],
    ]
}
