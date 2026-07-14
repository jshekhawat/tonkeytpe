use crate::typing::TypingTest;

pub struct Stats {
    pub wpm: f64,
    pub raw_wpm: f64,
    pub accuracy: f64,
    pub elapsed: f64,
    pub total_chars: usize,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
}

impl Stats {
    pub fn calculate(test: &TypingTest) -> Self {
        let total = test.total_typed();
        let correct = test.correct_up_to;
        let incorrect = total.saturating_sub(correct);
        let elapsed = if test.is_finished() {
            test.end_time
                .and_then(|e| test.start_time.map(|s| e.duration_since(s)))
                .map(|d| d.as_secs_f64())
                .unwrap_or(1.0)
        } else {
            test.elapsed.max(1.0)
        };
        let minutes = elapsed / 60.0;

        let raw_wpm = (total as f64 / 5.0) / minutes;
        let wpm = (correct as f64 / 5.0) / minutes;
        let accuracy = if total > 0 {
            (correct as f64 / total as f64) * 100.0
        } else {
            100.0
        };

        Self {
            wpm,
            raw_wpm,
            accuracy,
            elapsed,
            total_chars: total,
            correct_chars: correct,
            incorrect_chars: incorrect,
        }
    }
}
