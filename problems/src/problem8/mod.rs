use image;

use failure::Error;
use utils::{result, split_by, ProblemResult, Ret};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

const BLACK: usize = 0;
const WHITE: usize = 1;
const TRANSP: usize = 2;

fn first_star(input: &[usize]) -> ProblemResult<usize> {
    let area = WIDTH * HEIGHT;

    let mut zeros = std::usize::MAX;
    let mut prev_zeros = std::usize::MAX;

    let mut ones = 0;
    let mut twos = 0;
    let mut res = 0;

    for (idx, e) in input.iter().enumerate() {
        if (idx % area) == 0 {
            if prev_zeros > zeros {
                res = ones * twos;
                prev_zeros = zeros;
            }

            zeros = 0;
            ones = 0;
            twos = 0;
        }

        if *e == 0 {
            zeros += 1;
        } else if *e == 1 {
            ones += 1;
        } else if *e == 2 {
            twos += 1;
        }
    }

    Ok(res)
}

fn get_pixel(input: &[usize], idx: usize) -> usize {
    match input
        .iter()
        .skip(idx)
        .step_by(WIDTH * HEIGHT)
        .skip_while(|e| **e == TRANSP)
        .next()
    {
        Some(e) => *e,
        None => TRANSP,
    }
}

fn second_star(input: &[usize]) -> ProblemResult<String> {
    let filename = "day8-2.png";
    let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    for (idx, pixel) in imgbuf.pixels_mut().enumerate() {
        match get_pixel(input, idx) {
            BLACK => *pixel = image::Rgb([0, 0, 0]),
            WHITE => *pixel = image::Rgb([255, 255, 255]),
            _ => {}
        };
    }

    imgbuf.save(filename).unwrap();

    Ok(filename.to_owned())
}

pub(crate) fn solve() -> Result<Ret<usize, String>, Error> {
    let input_raw = include_str!("./input");
    let input = split_by(input_raw, "", &|e: &str| Ok(e.parse::<usize>()?))?;

    Ok(result(first_star(&input), second_star(&input)))
}
