use rigorstarter_rust_tauri_webgpu::stdlib::linear_algebra::{Vec2, Vec3, Mat2, Mat3, Mat4};

fn main() {
    println!("Linear Algebra Demo");
    println!("===================");
    
    // Vector operations
    println!("\nVector Operations:");
    let v1 = Vec2::new(3.0, 4.0);
    let v2 = Vec2::new(1.0, 2.0);
    
    println!("v1 = ({}, {})", v1.x, v1.y);
    println!("v2 = ({}, {})", v2.x, v2.y);
    println!("v1 + v2 = ({}, {})", (v1 + v2).x, (v1 + v2).y);
    println!("v1 - v2 = ({}, {})", (v1 - v2).x, (v1 - v2).y);
    println!("v1 * 2.0 = ({}, {})", (v1 * 2.0).x, (v1 * 2.0).y);
    println!("v1.length() = {}", v1.length());
    println!("v1.normalize() = ({}, {})", v1.normalize().x, v1.normalize().y);
    println!("v1.dot(v2) = {}", v1.dot(v2));
    println!("v1.lerp(v2, 0.5) = ({}, {})", v1.lerp(v2, 0.5).x, v1.lerp(v2, 0.5).y);
    
    // 3D Vector operations
    println!("\n3D Vector Operations:");
    let v3 = Vec3::new(1.0, 2.0, 3.0);
    let v4 = Vec3::new(4.0, 5.0, 6.0);
    
    println!("v3 = ({}, {}, {})", v3.x, v3.y, v3.z);
    println!("v4 = ({}, {}, {})", v4.x, v4.y, v4.z);
    println!("v3 + v4 = ({}, {}, {})", (v3 + v4).x, (v3 + v4).y, (v3 + v4).z);
    println!("v3.cross(v4) = ({}, {}, {})", v3.cross(v4).x, v3.cross(v4).y, v3.cross(v4).z);
    println!("v3.length() = {}", v3.length());
    println!("v3.normalize() = ({}, {}, {})", v3.normalize().x, v3.normalize().y, v3.normalize().z);
    println!("v3.dot(v4) = {}", v3.dot(v4));
    
    // Matrix operations
    println!("\nMatrix Operations:");
    let m1 = Mat2::new(1.0, 2.0, 3.0, 4.0);
    let m2 = Mat2::new(2.0, 0.0, 1.0, 3.0);
    
    println!("m1 = [{} {}]", m1.data[0], m1.data[1]);
    println!("     [{} {}]", m1.data[2], m1.data[3]);
    println!("m2 = [{} {}]", m2.data[0], m2.data[1]);
    println!("     [{} {}]", m2.data[2], m2.data[3]);
    println!("m1 + m2 = [{} {}]", (m1 + m2).data[0], (m1 + m2).data[1]);
    println!("          [{} {}]", (m1 + m2).data[2], (m1 + m2).data[3]);
    println!("m1 * m2 = [{} {}]", (m1 * m2).data[0], (m1 * m2).data[1]);
    println!("          [{} {}]", (m1 * m2).data[2], (m1 * m2).data[3]);
    println!("det(m1) = {}", m1.determinant());
    
    if let Some(inv) = m1.inverse() {
        println!("m1^-1 = [{} {}]", inv.data[0], inv.data[1]);
        println!("        [{} {}]", inv.data[2], inv.data[3]);
    } else {
        println!("m1 is not invertible");
    }
    
    // Matrix-vector multiplication
    println!("\nMatrix-Vector Multiplication:");
    let vec = Vec2::new(2.0, 3.0);
    let result = m1 * vec;
    println!("m1 * [{}, {}] = [{}, {}]", vec.x, vec.y, result.x, result.y);
    
    // 3D Transformation matrices
    println!("\n3D Transformation Matrices:");
    let translation = Mat3::translation(5.0, 3.0);
    let scaling = Mat3::scaling(2.0, 1.5);
    let rotation = Mat3::rotation(std::f32::consts::PI / 4.0); // 45 degrees
    
    println!("Translation Matrix:");
    print_mat3(&translation);
    
    println!("Scaling Matrix:");
    print_mat3(&scaling);
    
    println!("Rotation Matrix (45°):");
    print_mat3(&rotation);
    
    // Combined transformation
    let combined = translation * rotation * scaling;
    println!("Combined Matrix (T * R * S):");
    print_mat3(&combined);
    
    // Transform a point
    let point = Vec2::new(1.0, 1.0);
    let transformed = combined * point;
    println!("Transforming point ({}, {}):", point.x, point.y);
    println!("Result: ({}, {})", transformed.x, transformed.y);
    
    // 4x4 matrices for 3D graphics
    println!("\n4x4 Matrices for 3D Graphics:");
    let projection = Mat4::perspective(std::f32::consts::FRAC_PI_4, 16.0/9.0, 0.1, 100.0);
    println!("Perspective Matrix (45° FOV, 16:9 aspect, 0.1-100 range):");
    print_mat4(&projection);
    
    let view = Mat4::look_at(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );
    println!("View Matrix (camera at (0,0,5) looking at origin):");
    print_mat4(&view);
    
    let model = Mat4::scaling(2.0, 2.0, 2.0);
    println!("Model Matrix (uniform scale of 2x):");
    print_mat4(&model);
    
    let mvp = projection * view * model;
    println!("Model-View-Projection Matrix:");
    print_mat4(&mvp);
}

fn print_mat2(m: &Mat2) {
    println!("  [{} {}]", m.data[0], m.data[1]);
    println!("  [{} {}]", m.data[2], m.data[3]);
}

fn print_mat3(m: &Mat3) {
    println!("  [{} {} {}]", m.data[0], m.data[1], m.data[2]);
    println!("  [{} {} {}]", m.data[3], m.data[4], m.data[5]);
    println!("  [{} {} {}]", m.data[6], m.data[7], m.data[8]);
}

fn print_mat4(m: &Mat4) {
    println!("  [{} {} {} {}]", m.data[0], m.data[1], m.data[2], m.data[3]);
    println!("  [{} {} {} {}]", m.data[4], m.data[5], m.data[6], m.data[7]);
    println!("  [{} {} {} {}]", m.data[8], m.data[9], m.data[10], m.data[11]);
    println!("  [{} {} {} {}]", m.data[12], m.data[13], m.data[14], m.data[15]);
}