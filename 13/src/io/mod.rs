use std::{collections::HashSet, fs};

use crate::solver::Fold;

pub fn read_input(filename: &str) -> (HashSet<(usize, usize)>, Vec<Fold>) {
    let raw_input: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let mut input_iter = raw_input.into_iter();

    let (raw_points, raw_folds) = (input_iter.next().unwrap(), input_iter.next().unwrap());

    let points = raw_points
        .lines()
        .map(|s| {
            let mut split_iter = s.split(',').into_iter();
            (
                split_iter.next().unwrap().parse::<usize>().unwrap(),
                split_iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<HashSet<(usize, usize)>>();

    let folds = raw_folds
        .lines()
        .map(|s| {
            let mut split_iter = s.split(' ').nth(2).unwrap().split('=').into_iter();
            let fold_dir = split_iter.next().unwrap();
            let fold_coord = split_iter.next().unwrap().parse::<usize>().unwrap();

            match fold_dir {
                "y" => Fold::Horizontal(fold_coord),
                "x" => Fold::Vertical(fold_coord),
                _ => panic!("parsing error"),
            }
        })
        .collect();

    (points, folds)
}
