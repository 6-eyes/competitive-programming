use std::io::{Read, Write};

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).expect("unable to parse input");

    let mut line_iter = s.lines();
    let tests = line_iter.next().expect("unable to read number of elements for test").parse::<usize>().expect("unable to parse number of tests");

    for _ in 0..tests {
        let num_elements = line_iter.next().expect("unable to read number of elements for test").parse::<usize>().expect("unable to parse number of elements");
        let mut elements = {
            let mut v = Vec::with_capacity(num_elements);
            line_iter.next().expect("unable to read data").split_whitespace().for_each(|s| {
                let e = s.parse::<usize>().expect("unable to parse element");
                v.push(e);
            });

            assert_eq!(v.len(), num_elements, "invalid number of elements received, expected: {}, received: {}", num_elements, v.len());
            v
        };

        level_cake(&mut elements);

        let mut stdout = std::io::stdout();
        for (i, x) in elements.iter().enumerate() {
            if i > 0 { write!(stdout, " ").expect("unable to write to stdout"); }
            write!(stdout, "{x}").expect("unable to write to stdout");
        }
        writeln!(stdout).expect("unable to write new line");
    }
}

fn level_cake(c: &mut Vec<usize>) {
    if c.len() < 2 { return; }

    let mut sum = c[0];
    (1..c.len()).for_each(|i| {
        sum += c[i];
        c[i] = c[i - 1].min(sum / (i + 1));
    });
}

#[cfg(test)]
mod tests {
    use crate::level_cake;

    #[test]
    fn test_1() {
        let mut cake = vec!{ 4, 2, 3 };
        level_cake(&mut cake);
        assert_eq!(cake, vec!{ 4, 3, 3 })
    }

    #[test]
    fn test_2() {
        let mut cake = vec!{ 2, 3, 4, 3, 2 };
        level_cake(&mut cake);
        assert_eq!(cake, vec!{ 2, 2, 2, 2, 2 })
    }

    #[test]
    fn test_3() {
        let mut cake = vec!{ 3, 3, 3, 1, 1 };
        level_cake(&mut cake);
        assert_eq!(cake, vec!{ 3, 3, 3, 2, 2 })
    }

    #[test]
    fn test_4() {
        let mut cake = vec!{ 913764826, 346182673, 764382516 };
        level_cake(&mut cake);
        assert_eq!(cake, vec!{ 913764826, 629973749, 629973749 })
    }

    #[test]
    fn test_5() {
        let mut cake = vec!{ 6, 7, 6, 7, 6, 7, 6, 7 };
        level_cake(&mut cake);
        assert_eq!(cake, vec!{ 6, 6, 6, 6, 6, 6, 6, 6 })
    }
}
