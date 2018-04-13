#[macro_use]
extern crate criterion;
extern crate ndarray;
extern crate rand;

use criterion::Criterion;
use ndarray::{Array, ArrayBase, ArrayD, ArrayViewD, Axis};
use rand::{Rand, Rng};

/// Returns an array view with `n_axis` axes with length 1 at the start of the shape.
fn insert_axes_first<S>(array: &ArrayD<S>, n_axis: usize) -> ArrayViewD<S> {
    let mut a = array.view();
    for _ in 0..n_axis {
        a = a.insert_axis(Axis(0))
    }
    a
}

/// Returns an array view with `n_axis` axes with length 1 at the end of the shape.
fn insert_axes_end<S>(array: &ArrayD<S>, n_axis: usize) -> ArrayViewD<S> {
    let mut a = array.view();
    for _ in 0..n_axis {
        let axis = Axis(a.ndim());
        a = a.insert_axis(axis);
    }
    a
}

/// Generates a random array with the given shape.
fn random_array<S: Rand>(shape: Vec<usize>) -> ArrayD<S> {
    let n_elements = shape.iter().fold(1, |accum, &elem| accum * elem);
    let mut rng = rand::thread_rng();
    let numbers: Vec<S> = (0..n_elements).map(|_| rng.gen()).collect();

    ArrayD::from_shape_vec(shape, numbers).unwrap()
}

fn product(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "product",
        |b, &size| {
            // Number of axes to add on each matrix.
            let new_axes = size / 2;
            let shape = vec![2; size];
            let left = random_array::<f64>(shape.clone());
            let right = random_array::<f64>(shape);
            // Expected cardinality of the product of the two arrays.
            let expected_card = vec![2; size + new_axes];
            b.iter_with_setup(
                || {
                    let left_view = insert_axes_end(&left, new_axes);
                    let right_view = insert_axes_first(&right, new_axes);
                    (left_view, right_view, expected_card.clone())
                },
                |(left_view, right_view, expected_card)| {
                    // Broadcasting the left hand side first.
                    let left_view = left_view.broadcast(expected_card).unwrap();
                    let _array = &left_view * &right_view;
                },
            )
        },
        vec![2usize, 4, 6, 8, 10, 12],
    );
}

criterion_group!(product_group, product);
criterion_main!(product_group);
