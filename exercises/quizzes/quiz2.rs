enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        input.into_iter()
            .map(|(mut s, cmd)| {
                match cmd {
                    Command::Uppercase => s = s.to_uppercase(),
                    Command::Trim => s = s.trim().to_string(),
                    Command::Append(n) => s += &"bar".repeat(n),
                }
                s
            })
            .collect()
    }
}

fn main() {
    // 你可以在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            vec![
                "HELLO".to_string(),
                "all roads lead to rome!".to_string(),
                "foobar".to_string(),
                "barbarbarbarbarbar".to_string(),
            ]
        );
    }
}
