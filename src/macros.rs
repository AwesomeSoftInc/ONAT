#[macro_export]
macro_rules! texture_rect {
    ($textures:tt.$thing:tt) => {
        Rectangle::new(0.0,0.0,$textures.$thing.width as f32,$textures.$thing.height as f32)
    };
    ($textures:tt.$thing:tt, $x:tt, $y:tt) => {
        Rectangle::new($x as f32,$y as f32,$textures.$thing.width as f32,$textures.$thing.height as f32)
    };
    ($textures:tt.$thing:tt + $num:tt) => {
        Rectangle::new(0.0,0.0,$textures.$thing.width as f32 + $num as f32,$textures.$thing.height as f32 + $num as f32)
    };
    ($textures:tt.$thing:tt + $num:tt, $x:tt, $y:tt) => {
        Rectangle::new($x as f32,$y as f32,$textures.$thing.width as f32 + $num as f32,$textures.$thing.height as f32 + $num as f32)
    };
    ($textures:tt.$thing:tt - $num:tt) => {
        Rectangle::new(0.0,0.0,$textures.$thing.width as f32 - $num as f32,$textures.$thing.height as f32 - $num as f32)
    };
    ($textures:tt.$thing:tt - $num:tt, $x:tt, $y:tt) => {
        Rectangle::new($x as f32,$y as f32,$textures.$thing.width as f32 - $num as f32,$textures.$thing.height as f32 - $num as f32)
    };
    ($textures:tt.$thing:tt * $num:tt) => {
        Rectangle::new(0.0,0.0,$textures.$thing.width as f32 * $num as f32,$textures.$thing.height as f32 * $num as f32)
    };
    ($textures:tt.$thing:tt * $num:tt, $x:tt, $y:tt) => {
        Rectangle::new($x as f32,$y as f32,$textures.$thing.width as f32 * $num as f32,$textures.$thing.height as f32 * $num as f32)
    };
    ($textures:tt.$thing:tt / $num:tt) => {
        Rectangle::new(0.0,0.0,$textures.$thing.width as f32 / $num as f32,$textures.$thing.height as f32 / $num as f32)
    };
    ($textures:tt.$thing:tt / $num:tt, $x:tt, $y:tt) => {
        Rectangle::new($x as f32,$y as f32,$textures.$thing.width as f32 / $num as f32,$textures.$thing.height as f32 / $num as f32)
    };
}