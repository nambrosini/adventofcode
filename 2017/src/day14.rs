#[aoc_generator(day14)]
pub fn generator(input: &str) -> Vec<Vec<u8>> {
    let mut hasher = KnotHasher::new();

    (0..128)
        .map(|x| {
            let new_input = format!("{}-{}", input, x);
            hasher.do_knot_for_input_second(&new_input);
            hash_to_binary(hasher.get_dense_hash())
        })
        .map(|x| {
            let mut line: Vec<u8> = Vec::new();
            x.chars().for_each(|b| {
                if b == '1' {
                    line.push(1);
                } else {
                    line.push(0)
                }
            });
            line
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn part1(input: &[Vec<u8>]) -> usize {
    input.iter().flatten().filter(|&&x| x == 1).count()
}

#[aoc(day14, part2)]
pub fn part2(input: &[Vec<u8>]) -> usize {
    let mut input = input.to_vec();
    count_groups(&mut input)
}

pub fn hash_to_binary(input: String) -> String {
    input
        .chars()
        .map(|a| format!("{:04b}", a.to_digit(16).unwrap()))
        .collect()
}

pub struct KnotHasher {
    pub list: Vec<u16>,
    pub current_position: usize,
    pub skip_size: usize,
}

impl Default for KnotHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl KnotHasher {
    pub fn new() -> Self {
        KnotHasher {
            list: (0..256).collect(),
            current_position: 0,
            skip_size: 0,
        }
    }

    pub fn do_knot_for_input_first(&mut self, input: &str) {
        input.split(',').for_each(|a| {
            self.do_knot(a.parse::<usize>().unwrap());
        });
    }

    pub fn do_knot_for_input_second(&mut self, input: &str) {
        // suffixes the input stream
        let mut input_suffixed: Vec<u8> = input.chars().map(|a| a as u8).collect();

        input_suffixed.push(17);
        input_suffixed.push(31);
        input_suffixed.push(73);
        input_suffixed.push(47);
        input_suffixed.push(23);

        // Resets the list, current_position and skip_size
        self.current_position = 0;
        self.skip_size = 0;
        self.list = (0..256).collect();

        // Do the knotting on the suffixed input stream. 64 rounds.
        for _ in 1..65 {
            input_suffixed.iter().for_each(|a| {
                self.do_knot(*a as usize);
            });
        }
    }

    pub fn do_knot(&mut self, knot_length: usize) {
        let mut curr_pos: usize = self.current_position;
        let mut reversed_list: Vec<u8> = Vec::new();
        for _ in 0..knot_length {
            reversed_list.push(self.list[curr_pos] as u8);
            curr_pos += 1;
            if curr_pos >= self.list.len() {
                curr_pos = 0;
            }
        }

        let mut iter_reversed_list = reversed_list.into_iter().rev();

        curr_pos = self.current_position;
        for _ in 0..knot_length {
            let val: u8 = iter_reversed_list.next().unwrap();
            self.list[curr_pos] = val as u16;
            curr_pos += 1;
            if curr_pos >= self.list.len() {
                curr_pos = 0;
            }
        }

        self.current_position += knot_length + self.skip_size;
        self.current_position %= self.list.len();
        self.skip_size += 1;
    }

    pub fn get_dense_hash(&mut self) -> String {
        let mut hash: String = String::new();
        for i in 0..16 {
            let val: u8 = self
                .list
                .iter()
                .skip(i * 16)
                .take(16)
                .fold(0, |acc, x| acc ^ x) as u8;

            let mut val_to_add: String = "".into();
            if val <= 15 {
                val_to_add.push('0');
            }
            val_to_add.push_str(&format!("{:x}", val));
            hash.push_str(&val_to_add);
        }

        hash
    }

    pub fn get_result(&mut self) -> i32 {
        (self.list[0] * self.list[1]) as i32
    }
}

pub fn count_groups(grid: &mut Vec<Vec<u8>>) -> usize {
    let mut nb_groups: usize = 0;
    for i in 0..128 {
        for j in 0..128 {
            if grid[i][j] == 1 {
                recursive_check_neighbors(grid, i, j);
                nb_groups += 1;
            }
        }
    }
    nb_groups
}

fn recursive_check_neighbors(grid: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    grid[x][y] = 0;
    if x <= 126 && grid[x + 1][y] == 1 {
        recursive_check_neighbors(grid, x + 1, y);
    }
    if y <= 126 && grid[x][y + 1] == 1 {
        recursive_check_neighbors(grid, x, y + 1);
    }
    if x >= 1 && grid[x - 1][y] == 1 {
        recursive_check_neighbors(grid, x - 1, y);
    }
    if y >= 1 && grid[x][y - 1] == 1 {
        recursive_check_neighbors(grid, x, y - 1);
    }
    if x <= 126 {
        grid[x + 1][y] = 0;
    }
    if y <= 126 {
        grid[x][y + 1] = 0;
    }
    if x >= 1 {
        grid[x - 1][y] = 0;
    }
    if y >= 1 {
        grid[x][y - 1] = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let s = generator("flqrgnkx");

        assert_eq!(part1(&s), 8108);
    }

    #[test]
    pub fn test2() {
        let s = generator("flqrgnkx");

        assert_eq!(part2(&s), 1242);
    }
}
