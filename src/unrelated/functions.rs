
pub fn simpson_integration(f: &dyn Fn(i32), a:f32, b:f32, n:i32) -> f32 {
    // Width of each segment
    let dx:f32 = (b - a) / n;

    // lambdas
    let a1 = |i: i32| -> i32 {2 * i - 2};
    let a2 = |i: i32| -> i32 {2 * i - 1};
    // sum of heights
    let mut height:f32 = 0;
    for i in (1..(n/2).round()) {
        height += f(a + a1(i) * dx) + 4 * f(a + a2(i) * dx) + f(a + 2 * i * dx);
    }

    // Aproximate integral of f
    return (dx / 3) * height;
}
