use std::ops::Rem;

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    box_index: usize,
    focal_length: usize,
}

impl Lens {
    fn from_str(s: &str) -> Lens {
        if s.contains("-") {
            let label = &s[..s.len() - 1];
            Lens {
                label: label.to_string(),
                box_index: hash(label) as usize,
                focal_length: 0, // denotes remove lens operation
            }
        } else if s.contains("=") {
            let (label, focal_length) = s.split_once("=").unwrap();
            Lens {
                label: label.to_string(),
                box_index: hash(label) as usize,
                focal_length: focal_length.parse::<usize>().unwrap(),
            }
        } else {
            panic!("Invalid step format!")
        }
    }
}

#[derive(Debug)]
struct LensBox {
    lenses: Vec<Lens>,
}

impl LensBox {
    fn new() -> LensBox {
        LensBox {
            lenses: Vec::<Lens>::new(),
        }
    }
}

fn hash(input: &str) -> u32 {
    let mut current = 0;
    for c in input.chars() {
        current += c as u32;
        current *= 17;
        current = current.rem(256);
    }
    current
}

fn focusing_power(lens_array: Vec<LensBox>) -> usize {
    lens_array
        .iter()
        .map(|lens_box| {
            lens_box
                .lenses
                .iter()
                .enumerate()
                .map(|(i, lens)| (1 + lens.box_index) * (i + 1) * lens.focal_length)
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    // Part 1
    let output_part_1: u32 = include_str!("day15.txt").split(",").map(hash).sum();
    println!("{}", output_part_1);

    // Part 2
    // Initalise lens array
    let mut lens_array = Vec::with_capacity(256);
    for _ in 0..256 {
        lens_array.push(LensBox::new());
    }

    // Initalise lenses
    let lenses = include_str!("day15.txt").split(",").map(Lens::from_str);

    // Apply operation for each lens step (focal_length == 0 denotes removal)
    for lens in lenses {
        if lens.focal_length == 0 {
            // Remove lens from lens box at lens.box_index
            match lens_array[lens.box_index]
                .lenses
                .iter()
                .position(|x| *x.label == lens.label)
            {
                Some(index) => {
                    lens_array[lens.box_index].lenses.remove(index);
                }
                None => {}
            };
        } else {
            // Insert lens into box (or replace if it already exists)
            match lens_array[lens.box_index]
                .lenses
                .iter()
                .position(|x| &x.label == &lens.label)
            {
                Some(index) => lens_array[lens.box_index].lenses[index] = lens.clone(),
                None => lens_array[lens.box_index].lenses.push(lens),
            };
        }
    }

    let output_part_2 = focusing_power(lens_array);
    println!("{:?}", output_part_2);
}

#[cfg(test)]
mod tests {
    use crate::hash;

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"));
    }
}
