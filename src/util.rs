use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader.lines().collect()
}

pub fn read_chars<P>(filename: P) -> io::Result<impl Iterator<Item=char>>
    where
        P: AsRef<Path>,
{
    File::open(filename)
        .map(|f| io::BufReader::new(f))
        .map(|buf_reader| buf_reader.bytes()
            .map(|byte| byte.unwrap() as char))
}

pub fn read_string<P>(filename: P) -> io::Result<String>
    where
        P: AsRef<Path>,
{
    let mut content = String::new();
    let file = File::open(filename)?;
    let mut buf_reader = io::BufReader::new(file);
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

pub fn cartesian_product_flat_map<I, J, T>(iter1: I, iter2: J) -> impl Iterator<Item=(T, T)>
    where
        I: IntoIterator<Item=T>,
        J: Clone + IntoIterator<Item=T>,
        T: Clone,
{
    iter1.into_iter().flat_map(move |a| {
        let iter2_clone = iter2.clone();
        iter2_clone.into_iter().map(move |b| (a.clone(), b))
    })
}

/// 100x slower than [`cartesian_product_flat_map`] and 10_000x slower than [`cartesian_product_refs`]
/// This function is deprecated. Please use one of the faster options instead.
#[deprecated]
pub fn cartesian_product_mut_push<I, J, T>(iter1: I, iter2: J) -> impl Iterator<Item=(T, T)>
    where
        I: IntoIterator<Item=T>,
        J: IntoIterator<Item=T> + Clone,
        T: Clone,
{
    let vec1: Vec<T> = iter1.into_iter().collect();
    let vec2: Vec<T> = iter2.into_iter().collect();

    let total_size = vec1.len().saturating_mul(vec2.len());
    let mut result = Vec::with_capacity(total_size);

    for i in &vec1 {
        for j in &vec2 {
            result.push((i.clone(), j.clone()));
        }
    }

    result.into_iter()
}

/// constant runtime of 250pico-seconds - which is absolutely nuuuuuts
pub fn cartesian_product_refs<'a, T>(vec1: &'a [T], vec2: &'a [T])
                                     -> impl Iterator<Item=(&'a T, &'a T)>
{
    vec1.iter().flat_map(move |a| {
        vec2.iter().map(move |b| (a, b))
    })
}