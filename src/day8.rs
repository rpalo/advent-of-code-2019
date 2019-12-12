/// Day 8: Space Image Format
/// 
/// Parse a layered image format being STREAMED THROUGH SPACE

use std::fs;
use std::convert::TryInto;

/// Expects a single line of 0's 1's and 2's
fn parse_input() -> Vec<Vec<usize>> {
    let text = fs::read_to_string("data/day8.txt").unwrap();

    let mut results: Vec<Vec<usize>> = vec![];
    let mut current: Vec<usize> = vec![];
    let mut counter = 0;

    for c in text.chars() {
        current.push(c.to_digit(10).unwrap().try_into().unwrap());
        counter += 1;
        if counter == 25 * 6 {
            counter = 0;
            results.push(current);
            current = vec![];
        }
    }

    results
}

/// To verify the parsing, find the layer with the fewest zeros and return the
/// number of 1's in that layer times the number of 2's.
fn part1(layers: &Vec<Vec<usize>>) -> usize {
    let target_layer: &Vec<usize> = layers.iter().min_by_key(|layer| {
        layer.iter().filter(|x| **x == 0).count()
    }).unwrap();
    let ones = target_layer.iter().filter(|x| **x == 1).count();
    let twos = target_layer.iter().filter(|x| **x == 2).count();

    ones * twos
}

/// Stack the layers top to bottom.  If a layer contains a 2, it's transparent
/// and layers below can be seen.  1's are black.  0's are white.
/// 
/// Show the final compressed image.
fn part2(layers: Vec<Vec<usize>>) {
    let start: Vec<usize> = vec![2; 25*6];
    let result = layers.into_iter().fold(start, |acc, layer| {
        acc.into_iter().zip(layer.into_iter()).map(|(current, new)| {
            if current == 2 {
                new
            } else {
                current
            }
        }).collect()
    });

    for row in 0..6 {
        for col in 0..25 {
            print!("{}", if result[row * 25 + col] == 1 {"#"} else {" "});
        }
        print!("\n");
    }
}

pub fn run() {
    let layers = parse_input();
    println!("Part 1 checksum: {}", part1(&layers));
    part2(layers);
}