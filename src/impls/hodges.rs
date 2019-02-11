use super::super::naive_estimator::Naive;
use super::super::sources::libhodges::*;

#[flame]
pub fn dr_ia(filename: String, estimator: &mut Naive) -> f32 {
    let state: State<f32> =
        State::from_file(filename.clone()).expect("Failed to open file with libhodges");
    estimator.analyse(state)
}

#[flame]
pub fn dr_ib_ia(filename: String, estimator: &mut Naive) -> f32 {
    let mut vec: Vec<f32> = Vec::with_capacity(1024 * 1024 * 1024);
    let state: State<f32> =
        State::from_file(filename.clone()).expect("Failed to open file with libhodges");

    while let Ok(buffer) = state.get_buffer() {
        vec.extend_from_slice(buffer);
    }

    estimator.analyse(vec.into_iter())
}

#[flame]
pub fn dr_ib_va(filename: String, estimator: &mut Naive) -> f32 {
    let mut vec: Vec<f32> = Vec::with_capacity(1024 * 1024 * 1024);
    let state: State<f32> =
        State::from_file(filename.clone()).expect("Failed to open file with libhodges");

    while let Ok(buffer) = state.get_buffer() {
        vec.extend_from_slice(buffer);
    }

    estimator.analyse_vec(&vec)
}

#[flame]
pub fn br_ib_ia(filename: String, estimator: &mut Naive) -> f32 {
    let mut vec: Vec<f32> = Vec::with_capacity(1024 * 1024 * 1024);
    let state: State<f32> =
        State::from_file(filename.clone()).expect("Failed to open file with libhodges");

    while let Ok(buffer) = state.get_buffer() {
        vec.extend_from_slice(buffer);
    }

    estimator.analyse(vec.into_iter())
}

#[flame]
pub fn br_ib_va(filename: String, estimator: &mut Naive) -> f32 {
    let mut vec: Vec<f32> = Vec::with_capacity(1024 * 1024 * 1024);
    let state: State<f32> =
        State::from_file(filename.clone()).expect("Failed to open file with libhodges");

    while let Ok(buffer) = state.get_buffer() {
        vec.extend_from_slice(buffer);
    }

    estimator.analyse_vec(&vec)
}
