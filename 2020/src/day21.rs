use itertools::Itertools as _;
use std::collections::HashMap;

#[aoc_generator(day21)]
pub fn generator(input: &str) -> HashMap<Vec<String>, Vec<String>> {
    let mut map = HashMap::new();

    for l in input.lines() {
        let mut split = l.split(" (");
        let ingredients: Vec<String> = split
            .next()
            .unwrap()
            .split(' ')
            .map(|x| x.to_owned())
            .collect();

        let allergenes = split.next().unwrap();
        let allergenes = &allergenes[9..allergenes.len() - 1];
        let allergenes: Vec<String> = if allergenes.contains(',') {
            allergenes.split(", ").map(|x| x.to_owned()).collect()
        } else {
            vec![allergenes.to_owned()]
        };

        map.insert(ingredients, allergenes);
    }

    map
}

#[aoc(day21, part1)]
pub fn part1(input: &HashMap<Vec<String>, Vec<String>>) -> usize {
    let ingredients = input.iter().flat_map(|(k, _)| k.clone()).collect_vec();

    let unique_ingredients = ingredients.iter().unique().map(String::from).collect_vec();
    let safe_ingredients = input
        .iter()
        .flat_map(|(_, v)| v)
        .unique()
        .map(|allergen| unsafe_ingredients(allergen, input))
        .fold(unique_ingredients, |i, s_i| {
            i.into_iter().filter(|i| !s_i.contains(i)).collect_vec()
        });
    ingredients
        .iter()
        .filter(|i| safe_ingredients.contains(i))
        .count()
}

#[aoc(day21, part2)]
pub fn part2(input: &HashMap<Vec<String>, Vec<String>>) -> String {
    let mut unsafe_ingredients = input
        .iter()
        .flat_map(|(_, v)| v)
        .unique()
        .map(|allergen| (allergen, unsafe_ingredients(allergen, input)))
        .collect_vec();
    let mut res = Vec::new();
    while !unsafe_ingredients.is_empty() {
        let i = unsafe_ingredients
            .iter()
            .enumerate()
            .find(|(_, (_, is))| is.len() == 1)
            .unwrap()
            .0;
        let (allergen, ingredient) = unsafe_ingredients[i].clone();
        res.push((allergen, ingredient[0].clone()));
        unsafe_ingredients.remove(i);
        unsafe_ingredients = unsafe_ingredients
            .into_iter()
            .map(|(a, is)| {
                let is = is.into_iter().filter(|i| ingredient[0] != *i).collect_vec();
                (a, is)
            })
            .collect_vec();
    }
    res.into_iter()
        .sorted_by_key(|&(a, _)| a)
        .map(|(_, i)| i)
        .join(",")
}

fn unsafe_ingredients(allergen: &str, foods: &HashMap<Vec<String>, Vec<String>>) -> Vec<String> {
    let has_allergen = foods
        .iter()
        .filter(|(_, v)| v.contains(&allergen.to_string()))
        .collect_vec();
    let shared_ingredients = has_allergen.iter().fold(HashMap::new(), |mut r, (k, _)| {
        k.iter().map(String::from).for_each(|i| {
            *r.entry(i).or_insert(0) += 1;
        });
        r
    });
    shared_ingredients
        .iter()
        .filter(|(_, v)| **v == has_allergen.len())
        .map(|(k, _)| k.to_string())
        .collect()
}

#[test]
fn sample1() {
    let s = generator(
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)",
    );

    assert_eq!(part1(&s), 5);
}

#[test]
fn sample1_test2() {
    let s = generator(
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)",
    );

    assert_eq!(part2(&s), "mxmxvkd,sqjhc,fvjkl");
}
