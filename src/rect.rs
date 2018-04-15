#[derive(Debug)]
struct Rectangle(u32, u32);


impl Rectangle {
    fn area(&self) -> u32 {
        self.0 * self.1
    }
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.0 >= other.0 && self.1 >= other.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area() {
        let rectangle = Rectangle(30, 50);
        assert_eq!(rectangle.area(), 1500);
        println!("{:?}", rectangle);
    }

    #[test]
    fn fit() {
        let rectangle1 = Rectangle(30, 50);
        let rectangle2 = Rectangle(10, 10);
        assert_eq!(rectangle1.can_hold(&rectangle2), true);
        assert_eq!(rectangle2.can_hold(&rectangle1), false);
    }
}
