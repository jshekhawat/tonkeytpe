use crate::{
    theme::Theme,
    typing::{TypingTest, GameMode},
};

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Menu,
    Typing,
    Results,
}

pub struct MenuMode {
    pub label: &'static str,
    pub mode: GameMode,
}

pub const MENU_MODES: &[MenuMode] = &[
    MenuMode { label: "15 seconds", mode: GameMode::Timed(15) },
    MenuMode { label: "30 seconds", mode: GameMode::Timed(30) },
    MenuMode { label: "60 seconds", mode: GameMode::Timed(60) },
    MenuMode { label: "120 seconds", mode: GameMode::Timed(120) },
    MenuMode { label: "25 words", mode: GameMode::WordCount(25) },
    MenuMode { label: "50 words", mode: GameMode::WordCount(50) },
    MenuMode { label: "100 words", mode: GameMode::WordCount(100) },
];

pub struct App {
    pub screen: Screen,
    pub typing_test: Option<TypingTest>,
    pub theme: Theme,
    pub quit: bool,
    pub menu_selection: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Menu,
            typing_test: None,
            theme: Theme::default(),
            quit: false,
            menu_selection: 1,
        }
    }

    pub fn start_test(&mut self, mode: GameMode) {
        self.typing_test = Some(TypingTest::new(mode));
        self.screen = Screen::Typing;
    }

    pub fn end_test(&mut self) {
        self.screen = Screen::Results;
    }

    pub fn restart(&mut self) {
        if let Some(test) = &self.typing_test {
            let mode = test.mode;
            self.typing_test = Some(TypingTest::new(mode));
            self.screen = Screen::Typing;
        }
    }

    pub fn back_to_menu(&mut self) {
        self.typing_test = None;
        self.screen = Screen::Menu;
    }

    pub fn menu_up(&mut self) {
        self.menu_selection = self.menu_selection.saturating_sub(1);
    }

    pub fn menu_down(&mut self) {
        if self.menu_selection + 1 < MENU_MODES.len() {
            self.menu_selection += 1;
        }
    }

    pub fn next_theme(&mut self) {
        self.theme = self.theme.next();
    }

    pub fn selected_mode(&self) -> GameMode {
        MENU_MODES[self.menu_selection].mode
    }

    pub fn tick(&mut self) {
        if let Some(ref mut test) = self.typing_test {
            test.tick();
            if test.is_finished() {
                self.end_test();
            }
        }
    }
}
