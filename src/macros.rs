#[macro_export]
macro_rules! texture_rect {
    ($($textures:tt).+) => {
        Rectangle::new(
            0.0,
            0.0,
            $($textures).*.width as f32,
            $($textures).*.height as f32,
        )
    };
    ($($textures:tt).+, $x:tt, $y:tt) => {
        Rectangle::new(
            $x as f32,
            $y as f32,
            $($textures).*.width as f32,
            $($textures).*.height as f32,
        )
    };
    ($($textures:tt).+ + $num:tt) => {
        Rectangle::new(
            0.0,
            0.0,
            $($textures).*.width as f32 + $num as f32,
            $($textures).*.height as f32 + $num as f32,
        )
    };
    ($($textures:tt).+ + $num:tt, $x:tt, $y:tt) => {
        Rectangle::new(
            $x as f32,
            $y as f32,
            $($textures).*.width as f32 + $num as f32,
            $($textures).*.height as f32 + $num as f32,
        )
    };
    ($($textures:tt).+ - $num:tt) => {
        Rectangle::new(
            0.0,
            0.0,
            $($textures).*.width as f32 - $num as f32,
            $($textures).*.height as f32 - $num as f32,
        )
    };
    ($($textures:tt).+ - $num:tt, $x:tt, $y:tt) => {
        Rectangle::new(
            $x as f32,
            $y as f32,
            $($textures).*.width as f32 - $num as f32,
            $($textures).*.height as f32 - $num as f32,
        )
    };
    ($($textures:tt).+ * $num:tt) => {
        Rectangle::new(
            0.0,
            0.0,
            $($textures).*.width as f32 * $num as f32,
            $($textures).*.height as f32 * $num as f32,
        )
    };
    ($($textures:tt).+ * $num:tt, $x:tt, $y:tt) => {
        Rectangle::new(
            $x as f32,
            $y as f32,
            $($textures).*.width as f32 * $num as f32,
            $($textures).*.height as f32 * $num as f32,
        )
    };
    ($($textures:tt).+ / $num:tt) => {
        Rectangle::new(
            0.0,
            0.0,
            $($textures).*.width as f32 / $num as f32,
            $($textures).*.height as f32 / $num as f32,
        )
    };
    ($($textures:tt).+ / $num:tt, $x:tt, $y:tt) => {
        Rectangle::new(
            $x as f32,
            $y as f32,
            $($textures).*.width as f32 / $num as f32,
            $($textures).*.height as f32 / $num as f32,
        )
    };
}
