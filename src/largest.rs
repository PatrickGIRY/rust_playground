pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn stats() {
        let v = vec![0];
        assert_that!(*largest(&v)).is_equal_to(0);
    }

    #[test]
    fn stats2() {
        let v = vec![0,10];
        assert_that!(*largest(&v)).is_equal_to(10);
    }

    #[test]
    fn stats2_2() {
        let v = vec![10,10];
        assert_that!(*largest(&v)).is_equal_to(10);
    }
}
