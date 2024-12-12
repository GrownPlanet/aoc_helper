use std::{fmt::Debug, fs};

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim().to_string()
}

pub fn to_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn to_type_map<N: std::str::FromStr>(input: &str) -> Vec<Vec<N>>
where
    <N as std::str::FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<N>().unwrap())
                .collect::<Vec<N>>()
        })
        .collect()
}

pub fn calculate_limits<N>(map: &[Vec<N>]) -> (usize, usize) {
    (map.len() - 1, map[0].len() - 1)
}

pub fn get_offset_position(
    (x, y): (usize, usize),
    (dx, dy): (i32, i32),
    (xlim, ylim): (usize, usize),
) -> Option<(usize, usize)> {
    let x_with_offset = x as i32 + dx;
    let y_with_offset = y as i32 + dy;

    if x_with_offset < 0
        || y_with_offset < 0
        || x_with_offset > xlim as i32
        || y_with_offset > ylim as i32
    {
        return None;
    }

    Some((x_with_offset as usize, y_with_offset as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = read_file("test_input.txt");
        assert_eq!(
            input,
            "0123
4567
8901"
        );

        let map = to_map(&input);
        assert_eq!(
            map,
            vec![
                vec!['0', '1', '2', '3'],
                vec!['4', '5', '6', '7'],
                vec!['8', '9', '0', '1']
            ]
        );

        let num_map = to_type_map::<u8>(&input);
        assert_eq!(
            num_map,
            vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 0, 1]]
        );

        let lims = calculate_limits(&map);
        assert_eq!(lims, (2, 3));

        let offset_pos = get_offset_position((0, 0), (1, 4), lims);
        assert_eq!(offset_pos, None);

        let offset_pos = get_offset_position((0, 0), (1, 2), lims);
        assert_eq!(offset_pos, Some((1, 2)));
    }
}
