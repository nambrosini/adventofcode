pub fn calculate_weight(input: &[i32]) -> i32 {
    let mut res = 0;

    for i in input {
        res += i / 3 - 2;
    }

    res
}

pub fn calculate_fuel_weight(input: &[i32]) -> i32 {
    let mut res = 0;

    for i in input {
        let mut n: i32 = i / 3 - 2;
        while n > 0 {
            res += n;
            n = n / 3 - 2;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_one() {
        let weight = vec![12];

        assert_eq!(calculate_weight(&weight), 2);
    }

    #[test]
    fn test_one_two() {
        let weight = vec![14];

        assert_eq!(calculate_weight(&weight), 2);
    }

    #[test]
    fn test_one_three() {
        let weight = vec![1_969];

        assert_eq!(calculate_weight(&weight), 654);
    }

    #[test]
    fn test_one_four() {
        let weight = vec![100_756];

        assert_eq!(calculate_weight(&weight), 33_583);
    }

    #[test]
    fn test_one_all() {
        let weight = vec![12, 14, 1_969, 100_756];

        assert_eq!(calculate_weight(&weight), 34_241);
    }

    #[test]
    fn test_two_one() {
        let weight = vec![14];

        assert_eq!(calculate_fuel_weight(&weight), 2);
    }

    #[test]
    fn test_two_two() {
        let weight = vec![1_969];

        assert_eq!(calculate_fuel_weight(&weight), 966);
    }

    #[test]
    fn test_two_three() {
        let weight = vec![100_756];

        assert_eq!(calculate_fuel_weight(&weight), 50_346);
    }

    #[test]
    fn test_two_all() {
        let weight = vec![14, 1_969, 100_756];

        assert_eq!(calculate_fuel_weight(&weight), 51_314);
    }
}
