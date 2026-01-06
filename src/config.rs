
#[derive(Debug, Default, Clone, Copy)]
pub struct Config {
    pub freq_h: f64,
    pub freq_v: f64,
}


impl Config {
    pub fn from_args() -> Result<Self, String> {
        Self::from_args_slice(&std::env::args().collect::<Vec<_>>())
    }

    pub fn from_args_slice(args: &[String]) -> Result<Self, String> {
        let mut freq_h = 0.23;
        let mut freq_v = 0.1;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-h" | "--horizontal-frequency" => {
                    i += 1;
                    freq_h = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(freq_h);
                }
                "-v" | "--vertical-frequency" => {
                    i += 1;
                    freq_v = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(freq_v);
                }
                unknown => {
                    return Err(format!("Unknown argument: {}", unknown));
                }
            }
            i += 1;
        }

        Ok(Config { freq_h, freq_v })
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_args_default() {
        let args = vec!["program".to_string()];
        let config = Config::from_args_slice(&args).unwrap();
        assert_eq!(config.freq_h, 0.23);
        assert_eq!(config.freq_v, 0.1);
    }

    #[test]
    fn test_from_args_horizontal_vertical() {
        let args = vec![
            "program".to_string(),
            "-h".to_string(),
            "0.5".to_string(),
            "--vertical-frequency".to_string(),
            "0.9".to_string(),
        ];
        let config = Config::from_args_slice(&args).unwrap();
        assert_eq!(config.freq_h, 0.5);
        assert_eq!(config.freq_v, 0.9);
    }

    #[test]
    fn test_from_args_invalid_number() {
        let args = vec!["program".to_string(), "-h".to_string(), "foo".to_string()];
        let config = Config::from_args_slice(&args).unwrap();
        // Should fallback to default
        assert_eq!(config.freq_h, 0.23);
    }

    #[test]
    fn test_from_args_unknown_flag() {
        let args = vec!["program".to_string(), "--unknown".to_string()];
        let err = Config::from_args_slice(&args).unwrap_err();
        assert!(err.contains("Unknown argument"));
    }
}
