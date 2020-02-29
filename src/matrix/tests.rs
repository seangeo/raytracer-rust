use super::*;
use crate::point::Point;
use crate::vector::Vector;

#[test]
fn can_create_a_4x4_matrix() {
    let m = Matrix4x4::from_elements(
        [
        [1.0, 2.0, 3.0, 4.0],
        [1.1, 2.1, 3.1, 4.1],
        [1.2, 2.2, 3.2, 4.2],
        [1.3, 2.3, 3.3, 4.3]
        ]);
    assert_eq!(1.0, m.get(0, 0));
    assert_eq!(2.0, m.get(0, 1));
    assert_eq!(3.0, m.get(0, 2));
    assert_eq!(4.0, m.get(0, 3));
    assert_eq!(1.1, m.get(1, 0));
    assert_eq!(2.1, m.get(1, 1));
    assert_eq!(3.1, m.get(1, 2));
    assert_eq!(4.1, m.get(1, 3));
    assert_eq!(1.2, m.get(2, 0));
    assert_eq!(2.2, m.get(2, 1));
    assert_eq!(3.2, m.get(2, 2));
    assert_eq!(4.2, m.get(2, 3));
    assert_eq!(1.3, m.get(3, 0));
    assert_eq!(2.3, m.get(3, 1));
    assert_eq!(3.3, m.get(3, 2));
    assert_eq!(4.3, m.get(3, 3));
}

#[test]
fn comparing_equal_matrices() {
    let m1 = Matrix4x4::from_elements([
        [1.0, 2.0, 3.0, 4.0],
        [1.1, 2.1, 3.1, 4.1],
        [1.2, 2.2, 3.2, 4.2],
        [1.3, 2.3, 3.3, 4.3]
    ]);
    let m2 = Matrix4x4::from_elements([
        [1.0, 2.0, 3.0, 4.0],
        [1.1, 2.1, 3.1, 4.1],
        [1.2, 2.2, 3.2, 4.2],
        [1.3, 2.3, 3.3, 4.3]
    ]);
    assert_eq!(m1, m2)
}

#[test]
fn comparing_nonequal_matrices() {
    let m1 = Matrix4x4::from_elements([
        [1.0, 2.0, 3.0, 4.0],
        [1.1, 2.1, 3.1, 4.1],
        [1.2, 2.2, 3.2, 4.2],
        [1.3, 2.3, 3.3, 4.3]
    ]);
    let m2 = Matrix4x4::from_elements([
        [1.0, 2.0, 3.0, 4.0],
        [1.1, 2.1, 3.1, 4.1],
        [1.2, 2.2, 3.2, 4.2],
        [1.3, 2.3, 3.3, 5.3]
    ]);
    assert_ne!(m1, m2)
}

#[test]
fn comparing_nearly_equal_matrices() {
    let m1 = Matrix4x4::from_elements([
        [1.0, 2.0, 3.0, 4.0],
        [1.1, 2.1, 3.1, 4.1],
        [1.2, 2.2, 3.2, 4.2],
        [1.3, 2.3, 3.3, 4.3]
    ]);
    let m2 = Matrix4x4::from_elements([
        [1.0, 2.0, 3.0, 4.0],
        [1.1, 2.1, 3.1, 4.1],
        [1.2, 2.2, 3.2, 4.2],
        [1.3, 2.3, 3.3, 4.30003]
    ]);
    assert_eq!(m1, m2)
}

#[test]
fn can_multiply_two_matrixes() {
    let m1v = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 8, 7, 6],
        [5, 4, 3, 2]
    ];
        let m2v = [
            [-2, 1, 2, 3],
            [3, 2, 1, -1],
            [4, 3, 6,  5],
            [1, 2, 7, 8]
        ];
            let resultv = [
                [20, 22, 50, 48],
                [44, 54, 114, 108],
                [40, 58, 110, 102],
                [16, 26, 46, 42]
            ];
                let m1 = Matrix4x4::from_elementsi(m1v);
                let m2 = Matrix4x4::from_elementsi(m2v);
                let result = Matrix4x4::from_elementsi(resultv);

                assert_eq!(result, m1 * m2);
}

#[test]
fn multiplying_a_matrix_by_the_identity_matrix_returns_itself() {
    let m = Matrix4x4::from_elementsi( [
        [0, 1, 2, 4],
        [1, 2, 4, 8],
        [2, 4, 8, 16],
        [4, 8, 16, 32]
    ]);
    assert_eq!(m, m * Matrix4x4::identity())
}

#[test]
fn multiplying_point_by_identity_returns_the_point() {
    let p = Point::new(1.0, 2.0, 3.0);
    assert_eq!(p, Matrix4x4::identity() * p)
}

#[test]
fn transposing_a_matrix() {
    let m = Matrix4x4::from_elementsi([
        [0,9,3,0],
        [9,8,0,8],
        [1,8,5,3],
        [0,0,5,8]
    ]);
    let transposed = Matrix4x4::from_elementsi([
        [0,9,1,0],
        [9,8,8,0],
        [3,0,5,5],
        [0,8,3,8]
    ]);

    assert_eq!(transposed, m.transpose());
}

#[test]
fn transposing_identity() {
    assert_eq!(Matrix4x4::identity(), Matrix4x4::identity().transpose())
}

#[test]
fn can_multiply_a_matrix_by_a_point() {
    let p = Point::new(1.0, 2.0, 3.0);
    let m = Matrix4x4::from_elementsi([
        [1, 2, 3, 4],
        [2, 4, 4, 2],
        [8, 6, 4, 1],
        [0, 0, 0, 1]
    ]);
    let result = Point::new(18.0, 24.0, 33.0);
    assert_eq!(result, m * p);
}

#[test]
fn can_multiply_a_matrix_by_a_vector() {
    let p = Vector::new(1.0, 2.0, 3.0);
    let m = Matrix4x4::from_elementsi([
        [1, 2, 3, 4],
        [2, 4, 4, 2],
        [8, 6, 4, 1],
        [0, 0, 0, 1]
    ]);
    let result = Vector::new(14.0, 22.0, 32.0);
    assert_eq!(result, m * p);
}

#[test]
fn determinant_of_2x2() {
    let m = Matrix2x2{elements: [[1.0, 5.0], [-3.0, 2.0]]};
    assert_eq!(17.0, m.determinant())
}

#[test]
fn submatrix_of_4x4_matrix() {
    let m4 = Matrix4x4::from_elementsi([
        [-6, 1, 1, 6],
        [-8, 5, 8, 6],
        [-1, 0, 8, 2],
        [-7, 1,-1, 1]
    ]);
    let m3 = Matrix3x3::from_elementsi([
        [-6, 1, 6],
        [-8, 8, 6],
        [-7, -1, 1]
    ]);

    assert_eq!(m3, m4.submatrix(2, 1))
}

#[test]
fn submatrix_of_3x3_mmatrix() {
    let m3 = Matrix3x3::from_elementsi([
        [1,  5, 0],
        [-3, 2, 7],
        [0,  6,-3]
    ]);
    let m2 = Matrix2x2{elements: [[-3.0, 2.0], [0.0, 6.0]]};
    assert_eq!(m2, m3.submatrix(0, 2))
}

#[test]
fn minor_of_3x3_matrix() {
    let m3 = Matrix3x3::from_elementsi([
        [3, 5, 0],
        [2,-1,-7],
        [6,-1, 5]
    ]);

    assert_eq!(25.0, m3.minor(1, 0));
}

#[test]
fn cofactor_of_3x3_matrix() {
    let m3 = Matrix3x3::from_elementsi([
        [3, 5, 0],
        [2,-1,-7],
        [6,-1, 5]
    ]);
    assert_eq!(-12.0, m3.minor(0, 0));
    assert_eq!(-12.0, m3.cofactor(0, 0));
    assert_eq!(25.0, m3.minor(1, 0));
    assert_eq!(-25.0, m3.cofactor(1, 0));
}

#[test]
fn determinant_of_3x3_matrix() {
    let m3 = Matrix3x3::from_elementsi([
        [ 1, 2, 6],
        [-5, 8,-4],
        [ 2, 6, 4]
    ]);
    assert_eq!(56.0, m3.cofactor(0, 0));
    assert_eq!(12.0, m3.cofactor(0, 1));
    assert_eq!(-46.0, m3.cofactor(0, 2));
    assert_eq!(-196.0, m3.determinant());
}

#[test]
fn determinant_of_4x4_matrix() {
    let m = Matrix4x4::from_elementsi([
        [-2,-8, 3, 5],
        [-3, 1, 7, 3],
        [1, 2,-9, 6],
        [-6, 7, 7,-9]
    ]);
    assert_eq!(690.0, m.cofactor(0, 0));
    assert_eq!(447.0, m.cofactor(0, 1));
    assert_eq!(210.0, m.cofactor(0, 2));
    assert_eq!(51.0, m.cofactor(0, 3));
    assert_eq!(-4071.0, m.determinant());
}

#[test]
fn inverse_of_non_invertible_matrix_is_none() {
    let m = Matrix4x4::from_elementsi([
        [-4, 2,-2,-3],
        [ 9, 6, 2, 6],
        [ 0,-5, 1,-5],
        [ 0, 0, 0, 0]
    ]);

    assert_eq!(None, m.inverse());
}

#[test]
fn inverse_of_invertible_matrix() {
    let m = Matrix4x4::from_elementsi([
        [-5, 2, 6,-8],
        [1,-5, 1, 8],
        [7, 7,-6,-7],
        [1,-3, 7, 4]
    ]);
    let result = Matrix4x4::from_elements([
        [ 0.21805 , 0.45113 , 0.24060 , -0.04511 ],
        [-0.80827 , -1.45677 , -0.44361 , 0.52068 ],
        [-0.07895 , -0.22368 , -0.05263 , 0.19737 ],
        [-0.52256 , -0.81391 , -0.30075 , 0.30639 ]
    ]);

    assert_eq!(160.0, m.minor(2, 3));
    assert_eq!(-160.0, m.cofactor(2, 3));
    assert_eq!(-24.0, m.cofactor(3, 0));
    assert_eq!(result, m.inverse().unwrap());
}

#[test]
fn matrix_inversion1() {
    let m = Matrix4x4::from_elementsi([
        [8,-5, 9, 2],
        [7,5,6,1],
        [-6, 0, 9, 6],
        [-3, 0,-9,-4]
    ]);

    let result = Matrix4x4::from_elements([
        [-0.15385 , -0.15385 , -0.28205 , -0.53846 ],
        [-0.07692 , 0.12308 , 0.02564 , 0.03077 ],
        [0.35897 , 0.35897 , 0.43590 , 0.92308 ],
        [-0.69231 , -0.69231 , -0.76923 , -1.92308]
    ]);

    assert_eq!(result, m.inverse().unwrap());
}

#[test]
fn matrix_inversion2() {
    let m = Matrix4x4::from_elementsi([
        [9,3,0,9],
        [-5 , -2 , -6 , -3 ],
        [-4, 9, 6, 4],
        [-7, 6, 6, 2]
    ]);

    let result = Matrix4x4::from_elements([
        [-0.04074 , -0.07778 , 0.14444 , -0.22222 ],
        [-0.07778 , 0.03333 , 0.36667 , -0.33333 ],
        [-0.02901 , -0.14630 , -0.10926 , 0.12963 ],
        [0.17778 , 0.06667 , -0.26667 , 0.33333]
    ]);

    assert_eq!(result, m.inverse().unwrap());
}

#[test]
fn multiply_product_by_inverse() {
    let a = Matrix4x4::from_elementsi([
        [3,-9, 7, 3],
        [3,-8, 2,-9],
        [-4, 4, 4, 1],
        [-6, 5,-1, 1]
    ]);
    let b = Matrix4x4::from_elementsi([
        [8,2,2,2],
        [3,-1, 7, 0],
        [7,0,5,4],
        [6,-2, 0, 5]
    ]);

    let c = a * b;
    assert_eq!(a, c * b.inverse().unwrap());
}

#[test]
fn translation() {
    let t = translate(5.0, -3.0, 2.0);
    let p = Point::new(-3.0, 4.0, 5.0);
    let r = Point::new(2.0, 1.0, 7.0);
    assert_eq!(r, t * p);
}

#[test]
fn translation_does_not_affect_vectors() {
    let t = translate(5.0, -3.0, 2.0);
    let v = Vector::new(1.0, 2.0, 3.0);
    assert_eq!(v, t * v);
}

#[test]
fn scaling_applied_to_a_point() {
    let t = scale(2.0, 3.0, 4.0);
    let p = Point::new(-4.0, 6.0, 8.0);
    let r = Point::new(-8.0, 18.0, 32.0);
    assert_eq!(r, t *  p);
}

#[test]
fn scaling_applied_to_a_vector() {
    let t = scale(2.0, 3.0, 4.0);
    let v = Vector::new(-4.0, 6.0, 8.0);
    let r = Vector::new(-8.0, 18.0, 32.0);
    assert_eq!(r, t *  v);
}

#[test]
fn point_rotation_around_x() {
    let p = Point::new(0.0, 1.0, 0.0);
    let half_quarter = rotation_x(std::f64::consts::PI / 4.0);
    let full_quarter = rotation_x(std::f64::consts::PI / 2.0);
    let halfq_result = Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);
    let fullq_result = Point::new(0.0, 0.0, 1.0);

    assert_eq!(halfq_result, half_quarter * p);
    assert_eq!(fullq_result, full_quarter * p);
}

#[test]
fn inverse_rotation_around_x() {
    let p = Point::new(0.0, 1.0, 0.0);
    let half_quarter = rotation_x(std::f64::consts::PI / 4.0);
    let inv = half_quarter.inverse().unwrap();
    let result = Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);

    assert_eq!(result, inv * p);
}

#[test]
fn point_rotation_around_y() {
    let p = Point::new(0.0, 0.0, 1.0);
    let half_quarter = rotation_y(std::f64::consts::PI / 4.0);
    let full_quarter = rotation_y(std::f64::consts::PI / 2.0);
    let halfq_result = Point::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0);
    let fullq_result = Point::new(1.0, 0.0, 0.0);

    assert_eq!(halfq_result, half_quarter * p);
    assert_eq!(fullq_result, full_quarter * p);
}

#[test]
fn point_rotation_around_z() {
    let p = Point::new(0.0, 1.0, 0.0);
    let half_quarter = rotation_z(std::f64::consts::PI / 4.0);
    let full_quarter = rotation_z(std::f64::consts::PI / 2.0);
    let halfq_result = Point::new(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
    let fullq_result = Point::new(-1.0, 0.0, 0.0);

    assert_eq!(halfq_result, half_quarter * p);
    assert_eq!(fullq_result, full_quarter * p);
}

#[test]
fn shear_x_in_y() {
    let t = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let p = Point::new(2.0, 3.0, 4.0);
    let r = Point::new(5.0, 3.0, 4.0);

    assert_eq!(r, t * p);
}

#[test]
fn shear_x_in_z() {
    let t = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let p = Point::new(2.0, 3.0, 4.0);
    let r = Point::new(6.0, 3.0, 4.0);

    assert_eq!(r, t * p);
}

#[test]
fn shear_y_in_x() {
    let t = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let p = Point::new(2.0, 3.0, 4.0);
    let r = Point::new(2.0, 5.0, 4.0);

    assert_eq!(r, t * p);
}

#[test]
fn shear_y_in_z() {
    let t = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let p = Point::new(2.0, 3.0, 4.0);
    let r = Point::new(2.0, 7.0, 4.0);

    assert_eq!(r, t * p);
}

#[test]
fn shear_z_in_x() {
    let t = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let p = Point::new(2.0, 3.0, 4.0);
    let r = Point::new(2.0, 3.0, 6.0);

    assert_eq!(r, t * p);
}

#[test]
fn shear_z_in_y() {
    let t = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let p = Point::new(2.0, 3.0, 4.0);
    let r = Point::new(2.0, 3.0, 7.0);

    assert_eq!(r, t * p);
}
