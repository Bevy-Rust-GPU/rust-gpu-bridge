pub trait Two {
    const TWO: Self;
}

impl Two for i32 {
    const TWO: Self = 2;
}

impl Two for f32 {
    const TWO: Self = 2.0;
}

impl Two for f64 {
    const TWO: Self = 2.0;
}
