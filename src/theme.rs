use ratatui::style::Color;

#[derive(Clone, Copy)]
pub struct Theme {
    pub name: &'static str,
    pub bg: Color,
    pub fg: Color,
    pub correct: Color,
    pub incorrect: Color,
    pub cursor: Color,
    pub accent: Color,
    pub dim: Color,
}

impl Default for Theme {
    fn default() -> Self {
        THEMES[0]
    }
}

impl Theme {
    pub fn next(&self) -> Self {
        let idx = THEMES.iter().position(|t| t.name == self.name).unwrap_or(0);
        THEMES[(idx + 1) % THEMES.len()]
    }
}

pub const THEMES: [Theme; 4] = [
    Theme {
        name: "dark",
        bg: Color::Rgb(30, 30, 30),
        fg: Color::Rgb(220, 220, 220),
        correct: Color::Rgb(100, 200, 100),
        incorrect: Color::Rgb(220, 80, 80),
        cursor: Color::Rgb(255, 200, 50),
        accent: Color::Rgb(100, 150, 255),
        dim: Color::Rgb(100, 100, 100),
    },
    Theme {
        name: "light",
        bg: Color::Rgb(245, 245, 245),
        fg: Color::Rgb(30, 30, 30),
        correct: Color::Rgb(40, 150, 40),
        incorrect: Color::Rgb(200, 50, 50),
        cursor: Color::Rgb(200, 150, 0),
        accent: Color::Rgb(50, 100, 200),
        dim: Color::Rgb(160, 160, 160),
    },
    Theme {
        name: "nord",
        bg: Color::Rgb(46, 52, 64),
        fg: Color::Rgb(216, 222, 233),
        correct: Color::Rgb(163, 190, 140),
        incorrect: Color::Rgb(191, 97, 106),
        cursor: Color::Rgb(235, 203, 139),
        accent: Color::Rgb(129, 161, 193),
        dim: Color::Rgb(76, 86, 106),
    },
    Theme {
        name: "catppuccin",
        bg: Color::Rgb(30, 30, 46),
        fg: Color::Rgb(205, 214, 244),
        correct: Color::Rgb(166, 227, 161),
        incorrect: Color::Rgb(243, 139, 168),
        cursor: Color::Rgb(249, 226, 175),
        accent: Color::Rgb(137, 180, 250),
        dim: Color::Rgb(88, 91, 112),
    },
];
