/// Rectangle that encompasses the dimensions of the texture.
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

/// Set up the styles used for any imgui buttons that are actually meant to be present in game.
#[macro_export]
macro_rules! style_push {
    ($ui:tt) => {
        vec![
            $ui.push_style_color(StyleColor::Button, [0.25, 0.25, 0.25, 1.0]),
            $ui.push_style_color(StyleColor::ButtonHovered, [0.15, 0.15, 0.15, 1.0]),
            $ui.push_style_color(StyleColor::ButtonActive, [0.05, 0.05, 0.05, 1.0]),
            $ui.push_style_color(StyleColor::Separator, [0.0, 0.0, 0.0, 0.0]),
        ]
    };
}

/// Pop the styles in questions.
#[macro_export]
macro_rules! style_pop {
    ($styles:tt) => {
        for style in $styles {
            style.pop();
        }
    };
}
