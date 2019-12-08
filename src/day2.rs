use std::io;

fn read_inputs() -> Vec<usize> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();

    buffer
        .trim()
        .split_terminator(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn run_program(input: &mut Vec<usize>) {
    let mut idx = 0;
    loop {
        let opcode = input[idx];
        if opcode == 99 {
            return;
        }

        let pos1 = input[idx + 1];
        let pos2 = input[idx + 2];
        let pos3 = input[idx + 3];

        input[pos3] = match opcode {
            1 => input[pos1] + input[pos2],
            2 => input[pos1] * input[pos2],
            n => panic!("Unexpected opcode {}", n),
        };

        idx += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &mut Vec<usize>, expected: Vec<usize>) {
        run_program(input);
        assert_eq!(*input, expected);
    }

    #[test]
    fn all() {
        test(
            &mut vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );

        test(&mut vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
        test(&mut vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]);
        test(&mut vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]);
        test(
            &mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }
}

pub fn part1() {
    let mut program = read_inputs();

    program[1] = 12;
    program[2] = 2;
    run_program(&mut program);

    println!("{}", program[0]);
}
