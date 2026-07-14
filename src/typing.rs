use std::time::Instant;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::words::WORDS;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameMode {
    Timed(u64),
    WordCount(usize),
}

pub struct TypingTest {
    pub mode: GameMode,
    pub words: Vec<String>,
    pub expected: Vec<char>,
    pub correct_up_to: usize,
    pub pending_errors: Vec<char>,
    pub total_keystrokes: usize,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub elapsed: f64,
    pub finished: bool,
}

impl TypingTest {
    pub fn new(mode: GameMode) -> Self {
        let word_count = match mode {
            GameMode::Timed(_) => 200,
            GameMode::WordCount(n) => n,
        };

        let mut rng = thread_rng();
        let words: Vec<String> = WORDS
            .choose_multiple(&mut rng, word_count)
            .map(|&s| s.to_string())
            .collect();

        let expected: Vec<char> = words.join(" ").chars().collect();

        Self {
            mode,
            words,
            expected,
            correct_up_to: 0,
            pending_errors: Vec::new(),
            total_keystrokes: 0,
            start_time: None,
            end_time: None,
            elapsed: 0.0,
            finished: false,
        }
    }

    pub fn type_char(&mut self, c: char) {
        if self.finished {
            return;
        }
        if c == '\n' || c == '\t' {
            return;
        }

        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }

        self.total_keystrokes += 1;

        let expected_char = self.expected.get(self.correct_up_to).copied();

        if expected_char.is_some_and(|exp| exp == c) {
            self.pending_errors.clear();
            self.correct_up_to += 1;
            if self.correct_up_to >= self.expected.len() {
                self.finished = true;
                self.end_time = Some(Instant::now());
            }
        } else {
            self.pending_errors.push(c);
        }
    }

    pub fn backspace(&mut self) {
        if self.finished {
            return;
        }

        if let Some(_) = self.pending_errors.pop() {
            return;
        }

        if self.correct_up_to > 0 {
            self.correct_up_to -= 1;
        }
    }

    pub fn tick(&mut self) {
        if self.finished {
            return;
        }

        if let Some(start) = self.start_time {
            self.elapsed = start.elapsed().as_secs_f64();
            if let GameMode::Timed(seconds) = self.mode {
                if self.elapsed >= seconds as f64 {
                    self.finished = true;
                    self.end_time = Some(Instant::now());
                }
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn is_started(&self) -> bool {
        self.start_time.is_some()
    }

    pub fn time_remaining(&self) -> f64 {
        match self.mode {
            GameMode::Timed(seconds) => (seconds as f64 - self.elapsed).max(0.0),
            GameMode::WordCount(_) => f64::MAX,
        }
    }

    pub fn word_progress(&self) -> (usize, usize) {
        (self.word_index(), self.words.len())
    }

    pub fn total_typed(&self) -> usize {
        self.total_keystrokes
    }

    fn word_index(&self) -> usize {
        let mut word_idx = 0;
        let mut pos = 0;
        for w in &self.words {
            let end = pos + w.chars().count() + 1;
            if self.correct_up_to < end {
                return word_idx;
            }
            pos = end;
            word_idx += 1;
        }
        word_idx.min(self.words.len().saturating_sub(1))
    }
}
