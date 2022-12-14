#[aoc_generator(day14)]
pub fn generator(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day14, part1)]
pub fn part1(count: &usize) -> i64 {
    let mut recipes = vec![3, 7];

    let mut first_elf = 0;
    let mut second_elf = 1;

    for i in 0.. {
        let new_recipe: i64 = recipes[first_elf] + recipes[second_elf];
        let new_recipe_count = (new_recipe as f32).log10() as i64 + 1;
        if i == count + 10 {
            return recipes[*count..*count + 10]
                .iter()
                .enumerate()
                .map(|(i, r)| *r * 10i64.pow(9 - i as u32))
                .sum::<i64>();
        }
        for j in (0u32..new_recipe_count as u32).rev() {
            recipes.push(new_recipe / (10i64.pow(j)) % 10);
        }
        first_elf = (first_elf + 1 + recipes[first_elf] as usize) % recipes.len();
        second_elf = (second_elf + 1 + recipes[second_elf] as usize) % recipes.len();
    }

    unreachable!()
}

#[aoc(day14, part2)]
pub fn part2(count: &usize) -> usize {
    let mut recipes = vec![3, 7];
    let count_len = (*count as f32).log10() as usize + 1;

    let mut first_elf = 0;
    let mut second_elf = 1;

    for _ in 0.. {
        let new_recipe: i64 = recipes[first_elf] + recipes[second_elf];
        let new_recipe_count = (new_recipe as f32).log10() as i64 + 1;
        if recipes.len() >= count_len {
            let last_five = recipes[recipes.len() - count_len..]
                .iter()
                .enumerate()
                .map(|(i, r)| *r as usize * 10usize.pow(count_len as u32 - 1 - i as u32))
                .sum::<usize>();
            if count == &last_five {
                return recipes.len() - count_len;
            }
        }

        for j in (0u32..new_recipe_count as u32).rev() {
            recipes.push(new_recipe / (10i64.pow(j)) % 10);
        }
        first_elf = (first_elf + 1 + recipes[first_elf] as usize) % recipes.len();
        second_elf = (second_elf + 1 + recipes[second_elf] as usize) % recipes.len();
    }

    unreachable!()
}

#[test]
fn test() {
    assert_eq!(5158916779, part1(&9));
    assert_eq!(124515891, part1(&5));
    assert_eq!(9251071085, part1(&18));
    assert_eq!(5941429882, part1(&2018));
}

#[test]
fn test1() {
    assert_eq!(9, part2(&51589));
    assert_eq!(18, part2(&92510));
    assert_eq!(2018, part2(&59414));
    assert_eq!(7, part2(&245158));
}
