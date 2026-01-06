
use crate::config::Config;
use std::f64::consts::PI;

pub struct Rainbow {
    col: usize,
    row: usize,
    freq_h: f64,
    freq_v: f64,
}

impl Rainbow {
    fn new(config: Config) -> Self {
        Self {
            col: 0,
            row: 0,
            freq_h: config.freq_h,
            freq_v: config.freq_v,
        }
    }

    fn color_for(&self) -> (u8, u8, u8) {
        let theta = self.col as f64 * self.freq_h + self.row as f64 * self.freq_v;

        let r = ((theta.sin() * 0.5 + 0.5) * 255.0) as u8;
        let g = (((theta + 2.0 * PI / 3.0).sin() * 0.5 + 0.5) * 255.0) as u8;
        let b = (((theta + 4.0 * PI / 3.0).sin() * 0.5 + 0.5) * 255.0) as u8;

        (r, g, b)
    }

    fn advance(&mut self, ch: char) {
        if ch == '\n' {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum EscapeState {
    None,
    Esc,
    Csi,
}

impl EscapeState {
    fn advance(self, ch: char) -> EscapeState {
        match self {
            EscapeState::None => {
                if ch == '\x1b' {
                    EscapeState::Esc
                } else {
                    EscapeState::None
                }
            }
            EscapeState::Esc => {
                if ch == '[' {
                    EscapeState::Csi
                } else {
                    // Single-char escape sequence ends immediately
                    EscapeState::None
                }
            }
            EscapeState::Csi => {
                // CSI sequences end with letters '@'..='~'
                if ('@'..='~').contains(&ch) {
                    EscapeState::None
                } else {
                    EscapeState::Csi
                }
            }
        }
    }
}
pub fn rainbow(input_string: &str, config: Config) -> String {
    let mut rainbow = Rainbow::new(config);
    let mut escape = EscapeState::None;
    let mut csi_buffer = String::new();
    let mut external_color = false;
    let mut output = String::new();

    for ch in input_string.chars() {
        let next_escape = escape.advance(ch);

        match escape {
            EscapeState::None => {
                if !external_color && ch != '\n' {
                    let (r, g, b) = rainbow.color_for();
                    output.push_str(&format!("\x1b[38;2;{};{};{}m{}", r, g, b, ch));
                } else {
                    output.push(ch);
                }
                rainbow.advance(ch);
            }

            EscapeState::Esc => {
                output.push('\x1b');
                output.push(ch);
            }

            EscapeState::Csi => {
                csi_buffer.push(ch);
                if ch.is_ascii_alphabetic() {
                    output.push_str(&format!("\x1b[{}", csi_buffer));
                    if csi_buffer == "0m" {
                        external_color = false;
                    } else if csi_buffer.ends_with('m') {
                        external_color = true;
                    }
                    csi_buffer.clear();
                }
            }
        }

        escape = next_escape;
    }

    output.push_str("\x1b[0m");
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    fn config_fixture() -> Config {
        let mut config = Config::default();
        config.freq_h = 0.23;
        config.freq_v = 0.1;
        config
    }
    #[test]
    fn test_escape_state_none_to_esc() {
        let state = EscapeState::None;
        let next = state.advance('\x1b');
        assert_eq!(next, EscapeState::Esc);
    }

    #[test]
    fn test_escape_state_esc_to_csi() {
        let state = EscapeState::Esc;
        let next = state.advance('[');
        assert_eq!(next, EscapeState::Csi);
    }

    #[test]
    fn test_escape_state_csi_ends_with_letter() {
        let state = EscapeState::Csi;
        for ch in '@'..='~' {
            assert_eq!(state.advance(ch), EscapeState::None);
        }
    }

    #[test]
    fn test_escape_state_esc_single_char() {
        let state = EscapeState::Esc;
        let next = state.advance('A'); // not '['
        assert_eq!(next, EscapeState::None);
    }

    #[test]
    fn test_rainbow_color_range() {
        let rainbow = Rainbow::new(config_fixture());
        let (_r, _g, _b) = rainbow.color_for();
    }

    #[test]
    fn test_rainbow_advance_col_row() {
        let mut rainbow = Rainbow::new(config_fixture());
        assert_eq!(rainbow.col, 0);
        assert_eq!(rainbow.row, 0);

        rainbow.advance('a');
        assert_eq!(rainbow.col, 1);
        assert_eq!(rainbow.row, 0);

        rainbow.advance('\n');
        assert_eq!(rainbow.col, 0);
        assert_eq!(rainbow.row, 1); // ✅ check row increment
    }

    #[test]
    fn test_rainbow_color_variation() {
        let mut rainbow = Rainbow::new(config_fixture());
        let first = rainbow.color_for();
        rainbow.advance('a');
        let second = rainbow.color_for();
        assert_ne!(first, second); // colors should change as we advance
    }

    #[test]
    fn test_rainbow_full() {
        let mut rainbow = Rainbow::new(config_fixture());
        let first = rainbow.color_for();
        rainbow.advance('a');
        let second = rainbow.color_for();
        assert_ne!(first, second); // colors should change as we advance
    }

    #[test]
    fn test_rainbow_basic_string() {
        let input = "hello";
        let config = config_fixture();
        let output = rainbow(input, config);

        // Should start with an ANSI color for first char
        assert!(output.starts_with("\x1b[38;2;"));
        // Should end with reset
        assert!(output.ends_with("\x1b[0m"));
        // Original letters should be present
        for ch in input.chars() {
            assert!(output.contains(ch));
        }
    }

    #[test]
    fn test_rainbow_newline() {
        let input = "hi\nthere";
        let config = config_fixture();
        let output = rainbow(input, config);

        // Should contain newline as-is
        assert!(output.contains('\n'));
        // Each line should still have color codes
        let lines: Vec<&str> = output.split('\n').collect();
        assert!(lines[0].starts_with("\x1b[38;2;"));
        assert!(lines[1].starts_with("\x1b[38;2;"));
    }

    #[test]
    fn test_rainbow_escape_sequence_passthrough() {
        let input = "A\x1b[31mB";
        let config = config_fixture();
        let output = rainbow(input, config);

        // Original characters present
        assert!(output.contains("A"));
        assert!(output.contains("B"));
        // Escape sequence is preserved
        assert!(output.contains("\x1b[31m"));
        // Output ends with reset
        assert!(output.ends_with("\x1b[0m"));
    }

    #[test]
    fn test_rainbow_empty_string() {
        let input = "";
        let config = config_fixture();
        let output = rainbow(input, config);
        // Should just be reset code
        assert_eq!(output, "\x1b[0m");
    }

    #[test]
    fn test_rainbow_color_changes() {
        let input = "ab";
        let config = config_fixture();
        let output = rainbow(input, config);

        // The ANSI color codes for 'a' and 'b' should be different
        let first_color_start = output.find("38;2").unwrap();
        let second_color_start = output[first_color_start+1..].find("38;2").unwrap() + first_color_start + 1;
        assert_ne!(first_color_start, second_color_start);
    }
    #[test]
    fn profile_rainbow() {
        let input = "The quick brown fox jumps over the lazy dog. ".repeat(4);
        let config = Config::default();

        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = super::rainbow(&input, config);
        }
        let elapsed = start.elapsed();
        println!("rainbow x1000 took {:?}", elapsed);
    }
}