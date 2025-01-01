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
        (
            vec![
                $ui.push_style_color(::imgui::StyleColor::Button, [0.25, 0.25, 0.25, 1.0]),
                $ui.push_style_color(::imgui::StyleColor::ButtonHovered, [0.15, 0.15, 0.15, 1.0]),
                $ui.push_style_color(::imgui::StyleColor::ButtonActive, [0.05, 0.05, 0.05, 1.0]),
                $ui.push_style_color(::imgui::StyleColor::Separator, [0.0, 0.0, 0.0, 0.0]),
                $ui.push_style_color(::imgui::StyleColor::Border, [1.0, 0.25, 0.25, 1.0]),
                $ui.push_style_color(::imgui::StyleColor::BorderShadow, [1.0, 0.25, 0.25, 1.0]),
            ],
            vec![
                $ui.push_style_var(::imgui::StyleVar::WindowBorderSize(0.0)),
                $ui.push_style_var(::imgui::StyleVar::ChildBorderSize(0.0)),
                $ui.push_style_var(::imgui::StyleVar::PopupBorderSize(0.0)),
                $ui.push_style_var(::imgui::StyleVar::FrameBorderSize(0.0)),
            ],
        )
    };
}

/// Pop the styles in questions.
#[macro_export]
macro_rules! style_pop {
    ($styles:tt) => {
        for style in $styles.0 {
            style.pop();
        }
        for style in $styles.1 {
            style.pop();
        }
    };
}
