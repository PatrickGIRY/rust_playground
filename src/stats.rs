type Mean = f32;
type Median = i32;
type Mode = i32;

use std::collections::HashMap;

fn stats_of(values: &Vec<i32>) -> (Mean, Median, Mode) {
    if values.is_empty() { return (0 as Mean, 0 as Median, 0 as Mode); }
    let mean = mean(values);
    let median = median(values);
    let mode = mode(values);
    (mean, median, mode)
}

fn mean(values: &Vec<i32>) -> Mean {
    let sum = values.iter().sum::<i32>();
    sum as Mean / values.len() as Mean
}

fn mode(values: &Vec<i32>) -> Mode {
    let mut counts = HashMap::new();
    for value in values {
        let count = counts.entry(*value).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    let mut mode = values[0];
    for (&value, &count) in counts.iter() {
        if count > max_count || (count == max_count && value < mode) {
            mode = value;
            max_count = count;
        }
    }
    mode
}

fn median(values: &Vec<i32>) -> Median {
    let median_index = values.len() / 2;
    let mut sorted = values.clone();
    sorted.sort();
    sorted[median_index]
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn stats() {
        let v = vec![];
        assert_that!(stats_of(&v)).is_equal_to((0f32, 0, 0));
    }

    #[test]
    fn stats1() {
        let v = vec![0];
        assert_that!(stats_of(&v)).is_equal_to((0f32, 0, 0));
    }

    #[test]
    fn stats1_2() {
        let v = vec![1];
        assert_that!(stats_of(&v)).is_equal_to((1f32, 1, 1));
    }

    #[test]
    fn stats2() {
        let v = vec![2, 2];
        assert_that!(stats_of(&v)).is_equal_to((2f32, 2, 2));
    }

    #[test]
    fn stats2_2() {
        let v = vec![1, 2];
        assert_that!(stats_of(&v)).is_equal_to((1.5, 2, 1));
    }

    #[test]
    fn stats3() {
        let v = vec![1, 2, 3];
        assert_that!(stats_of(&v)).is_equal_to((2 as Mean, 2, 1));
    }

    #[test]
    fn stats3_2() {
        let v = vec![2, 3, 1];
        assert_that!(stats_of(&v)).is_equal_to((2 as Mean, 2, 1));
    }

    #[test]
    fn stats3_3() {
        let v = vec![2, 5, 5];
        assert_that!(stats_of(&v)).is_equal_to((4 as Mean, 5, 5));
    }
}
