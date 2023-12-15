pub trait One {
    const ONE: Self;
}

impl One for i32 {
    const ONE: Self = 1;
}

impl One for f32 {
    const ONE: Self = 1.0;
}

impl One for f64 {
    const ONE: Self = 1.0;
}

