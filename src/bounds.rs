pub trait MinBound {
    const MIN_BOUND: Self;
}

impl MinBound for i32 {
    const MIN_BOUND: Self = i32::MIN;
}

impl MinBound for f32 {
    const MIN_BOUND: Self = f32::NEG_INFINITY;
}

impl MinBound for f64 {
    const MIN_BOUND: Self = f64::NEG_INFINITY;
}

pub trait MaxBound {
    const MAX_BOUND: Self;
}

impl MaxBound for i32 {
    const MAX_BOUND: Self = i32::MAX;
}

impl MaxBound for f32 {
    const MAX_BOUND: Self = f32::INFINITY;
}

impl MaxBound for f64 {
    const MAX_BOUND: Self = f64::INFINITY;
}
