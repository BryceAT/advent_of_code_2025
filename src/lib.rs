use pyo3::prelude::*;
#[allow(unused_imports)]
#[allow(dead_code)]
/// A Python module implemented in Rust.
#[pymodule]
mod advent_of_code_2025 {
    #[allow(unused_imports)]
    #[allow(dead_code)]
    use pyo3::prelude::*;
    use std::char::EscapeDebug;
    use std::collections::{HashSet, HashMap, VecDeque};
    use std::fmt::{LowerHex, format};
    use std::fmt;
    use std::process::Child;
    use std::{fs,env,iter};
    use std::error::Error;
    //use reqwest;
    use soup::prelude::*;
    use soup::Soup;
    use std::time::Instant;
    use regex::Regex;
    use rayon::prelude::*;
    use std::sync::mpsc::channel;
    use std::cmp::Reverse;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{stdin, Write};
    use rand::prelude::*;
    use pyo3::types::PyList;
    fn get_text(day: i32,sample:bool,part:usize) -> Result<String, Box<dyn Error>> {
        let aoc_session = fs::read_to_string("aoc_session.txt").unwrap();
        let path = format!("data/day{day}.txt");
        let sample_path = format!("data/day{day}sample{part}.txt");
        let year = 2025;
        match sample {
            false => {
                if let Ok(text) = fs::read_to_string(path.clone()) { return Ok(text)}
                let url = format!("https://adventofcode.com/{year}/day/{day}/input");
                let text = reqwest::blocking::Client::new().get(url).header("cookie",format!("session={}",aoc_session)).send()?.text()?.trim().to_string();
                fs::write(path, text.clone())?;
                Ok(text)
            },
            true => {
                if let Ok(text) = fs::read_to_string(sample_path.clone()) { return Ok(text) }
                let url = format!("https://adventofcode.com/{year}/day/{day}");
                let html_text = reqwest::blocking::Client::new().get(url).header("cookie",format!("session={}",aoc_session)).send()?.text()?;
                let text = &Soup::new(html_text.as_str()).tag("pre").find_all().map(|tag| {tag.text().trim().to_string()}).nth(part - 1).unwrap();
                fs::write(sample_path, text.clone())?;
                Ok(text.to_string())
            }
        }
    }
    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn get_text_py(day: i32, sample:bool, part:usize) -> PyResult<String> {
        Ok(get_text(day, sample, part).unwrap())
    }
    #[pyfunction]
    fn get_lines_py(py: Python<'_>, day: i32, sample:bool, part:usize) -> Bound<'_,PyList> {
        let text = get_text(day, sample, part).unwrap();
        let rust_vec:Vec<String> = text.lines().map(|s| s.to_string()).collect();
        PyList::new(py, &rust_vec).expect("could not make python list from rust vec")
    }
    #[pyfunction]
    fn get_csv_py(py: Python<'_>, day: i32, sample:bool, part:usize) -> Bound<'_,PyList> {
        let text = get_text(day, sample, part).unwrap();
        let rust_vec:Vec<String> = text.split(',').map(|s| s.trim().to_string()).collect();
        PyList::new(py, &rust_vec).expect("could not make python list from rust vec")
    }
    #[pyfunction]
    fn get_map_py(py: Python<'_>, day: i32, sample:bool, part:usize) -> Bound<'_,PyList> {
        //return Vec<Vec<i32>> 
        let text = get_text(day, sample, part).unwrap();
        let rust_vec:Vec<Vec<i32>> = text.lines().map(|line| line.bytes().map(|b| (b - b'0') as i32).collect::<Vec<_>>()).collect();
        PyList::new(py, &rust_vec).expect("could not make python list from rust vec")
    }
    #[pyfunction]
    fn get_map_char_py(py: Python<'_>, day: i32, sample:bool, part:usize) -> Bound<'_,PyList> {
        //return Vec<Vec<i32>> 
        let text = get_text(day, sample, part).unwrap();
        let rust_vec:Vec<Vec<char>> = text.lines().map(|line| line.chars().collect::<Vec<_>>()).collect();
        PyList::new(py, &rust_vec).expect("could not make python list from rust vec")
    }
    #[pymodule_export]
    use super::CharGrid;
}

#[pyclass]
struct CharGrid {
    grid: Vec<Vec<char>>,
}
#[pymethods]
impl CharGrid {
    #[new]
    fn new(grid:Vec<Vec<char>>) -> Self {
        CharGrid{grid}
    }
    fn get(&self,i: usize, j: usize) -> PyResult<char> {
        Ok(self.grid[i][j])
    }
    fn set(&mut self, i:usize, j:usize, val:char) -> PyResult<()> {
        self.grid[i][j] = val;
        Ok(())
    }
    fn count_neighbors(&self, i: usize, j: usize) -> i32 {
        let mut cnt = 0;
        for (x,y) in [(i.wrapping_sub(1),j.wrapping_sub(1)),(i,j.wrapping_sub(1)),(i+1,j.wrapping_sub(1)),
                        (i.wrapping_sub(1),j),(i+1,j),
                        (i.wrapping_sub(1),j+1),(i,j+1),(i+1,j+1),] {
            if x < self.grid.len() && y < self.grid[0].len() {
                if self.grid[x][y] == '@' {cnt += 1}
            }
        }
        cnt
    }
}

