type Value = Option<i64>;
type Message = Box<str>;

fn unary(value: Value, f: fn(i64) -> i64) -> Result<Value, Message> {
    match value {
        None => Err(Box::from("missing operand")),
        Some(x) => Ok(Some(f(x)))
    }
}

fn parse_int(word: &str) -> Result<Value, Message> {
    Ok(Some(word.parse()
        .map_err(|_| {
            // we want it to survive our return
            Box::from(format!("{}?", word))
        })?))
}

fn interpret(result: Result<Value, Message>, current_word: &&str) -> Result<Value, Message> {
    match *current_word {
        "neg" => unary(result?, |x| -x),
        "abs" => unary(result?, |x| x.abs()),
        word => parse_int(word)
    }
}

fn eval(expression: &str) -> Result<i64, Message> {
    let s: Vec<&str> = expression.split(" ").collect();

    let result = s.iter().fold(Ok(None), interpret);

    result.map(|o| o.unwrap())
}

#[cfg(test)]
mod should {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn parse_number() {
        assert_that!(eval("42")).is_equal_to(Ok(42));
        assert_that!(eval("-42")).is_equal_to(Ok(-42));
        assert_that!(eval("4807")).is_equal_to(Ok(4807));
    }

    #[test]
    fn return_error_not_given_a_number() {
        assert_that!(eval("")).is_err_containing(&Box::from("?"));
        assert_that!(eval("foo")).is_err_containing(&Box::from("foo?"));
    }

    #[test]
    fn handle_negate() {
        assert_that!(eval("42 neg")).is_equal_to(Ok(-42));
        assert_that!(eval("4807 neg")).is_equal_to(Ok(-4807));
    }

    #[test]
    fn handle_absolute_value() {
        assert_that!(eval("42 abs")).is_equal_to(Ok(42));
        assert_that!(eval("-42 abs")).is_equal_to(Ok(42));
    }


    #[test]
    fn return_error_when_missing_operand() {
        assert_that!(eval("neg")).is_err_containing(&Box::from("missing operand"));
        assert_that!(eval("abs")).is_err_containing(&Box::from("missing operand"));
    }
}
