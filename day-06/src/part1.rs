use std::path::Path;
use std::fs::read_to_string;


trait Problem { 
    fn solve(&self) -> usize;

    fn create(vec: Vec<usize>) -> impl Problem;
}
pub struct MultiplicationProblem {
    vals: Vec<usize>,
}

impl Problem for MultiplicationProblem {
    fn solve(&self) -> usize {
        let mut res = 1;
        self.vals.iter().for_each(|v| res = res * v);
        res
    }

    fn create(vec: Vec<usize>) -> impl Problem {
        MultiplicationProblem { vals: vec }
    }
}

pub struct AdditionProblem {
    vals: Vec<usize>,
}

impl Problem for AdditionProblem {
    fn solve(&self) -> usize {
        self.vals.iter().sum()
    }

    fn create(vec: Vec<usize>) -> impl Problem {
        AdditionProblem { vals: vec }
    }
}

pub enum Operator {
    Multiply,
    Add,
}

pub fn read_txt(path: &Path) -> Vec<Vec<String>> {
    let file_text = read_to_string(path).expect("Failed to read file");

    let mut problems = vec![];

    let mut lines = file_text.lines();

    while let Some(line) = lines.next() {
        problems.push(line.split_whitespace().map(|v| v.to_owned()).collect::<Vec<String>>());
    }

    problems
}

pub fn solve_problems(data: Vec<Vec<String>>) -> Vec<usize> {
    let mut problems = vec![];
    let n_vals = data.len() - 1;
    for j in 0..data[0].len() {
        let vals: Vec<usize> = (0..n_vals).map(|i| data[i][j].parse::<usize>().unwrap()).collect();

        let value = match data[n_vals][j].as_str() {
            "*" => MultiplicationProblem::create(vals).solve(),
            "+" => AdditionProblem::create(vals).solve(),
            _ => unreachable!()
        };

        problems.push(value)
    }

    problems
}

#[tracing::instrument]
pub fn process(input: &Path) -> anyhow::Result<usize> {
    let data = read_txt(input);
    let solutions = solve_problems(data);

    Ok(solutions.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = Path::new("../inputs/day6_sample.txt");

        assert_eq!(4277556, process(&input)?);
        Ok(())
    }

    #[test]
    fn test_process_full() -> anyhow::Result<()> {
        let input = Path::new("../inputs/day6.txt");

        assert_eq!(6605396225322, process(&input)?);
        Ok(())
    }
}
