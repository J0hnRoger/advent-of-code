fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    return "todo!()".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = process("");
        assert_eq!(result, "a");
    }
}
