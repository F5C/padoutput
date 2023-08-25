use std::env;

fn format_string(input: &str, format_spec: char, width: usize) -> String {
    let formatted = match format_spec {
        'L' => format!("{:<width$}", input, width = width),
        'R' => format!("{:>width$}", input, width = width),
        'Z' => {
            if let Some(c) = input.chars().next() {
                if c.is_digit(10) {
                    format!("{:0>width$}", input, width = width)
                } else {
                    format!("{:<width$}", input, width = width)
                }
            } else {
                format!("{:<width$}", input, width = width)
            }
        }
        _ => format!("{}", input),
    };

    if formatted.len() > width {
        match format_spec {
            'L' => formatted.chars().take(width).collect(),
            'R' | 'Z' => formatted.chars().rev().take(width).collect::<Vec<char>>().into_iter().rev().collect(),
            _ => formatted,
        }
    } else {
        formatted
    }
}

fn main_with_args(args: Vec<String>) -> String {
    if args.len() < 3 {
        return format!("Usage: {} <format> <strings...>", args[0]);
    }

    let format_str = &args[1];
    let format_chars: Vec<char> = format_str.chars().collect();

    let mut formatted_output = String::new();
    let mut format_index = 0;

    for arg in args.iter().skip(2) {
        if format_index >= format_chars.len() {
            break;
        }

        let format_spec = format_chars[format_index];
        format_index += 1;

        let mut width_str = String::new();
        while format_index < format_chars.len() && format_chars[format_index].is_digit(10) {
            width_str.push(format_chars[format_index]);
            format_index += 1;
        }

        let width: usize = if width_str.is_empty() {
            0
        } else {
            width_str.parse().unwrap()
        };

        formatted_output.push_str(&format_string(arg, format_spec, width));
    }

    formatted_output
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let formatted_output = main_with_args(args);
    print!("{}", formatted_output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_string() {
        assert_eq!(format_string("Arf", 'L', 4), "Arf ");
        assert_eq!(format_string("12", 'Z', 3), "012");
    }

    #[test]
    fn test_main_with_args() {
        assert_eq!(main_with_args(vec![
            String::from("program_name"),
            String::from("L4Z3"),
            String::from("Arf"),
            String::from("12"),
        ]), "Arf 012");
        assert_eq!(main_with_args(vec![
            String::from("program_name"),
            String::from("L10L6Z2"),
            String::from("0123456789A"),
            String::from("Arf"),
            String::from("122"),
        ]), "0123456789Arf   22");
        assert_eq!(main_with_args(vec![
            String::from("program_name"),
            String::from("R10R6Z3"),
            String::from("0123456789X"),
            String::from("Arf"),
            String::from("2"),
        ]), "123456789X   Arf002");
    }
}
