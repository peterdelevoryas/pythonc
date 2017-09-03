extern crate lalrpop_util;
extern crate regex;

// dev-dependency for testing

#[cfg(test)]
extern crate rand;

pub mod p0;
pub mod lexer;
pub mod ast;
pub mod ir;

#[cfg(test)]
mod test {
    use lexer;
    use p0;
    use ast::*;
    use std::fmt::Debug;

    fn test<'input, F, E, T, R>(parse: F, s: &'input str, test: T) -> R
    where
        F: FnOnce(&'input str, lexer::Lexer<'input>) -> E,
        E: Debug + PartialEq,
        T: FnOnce(E) -> R,
    {
        let lexer = lexer::Lexer::new(s);
        let parsed = parse(s, lexer);
        test(parsed)
    }

    macro_rules! test {
        (
            parse: $func:expr,
            input: $input:expr,
            test: $test:expr
        ) => ({
            test($func, $input, $test)
        })
    }

    macro_rules! parsed_is_err {
        () => ({
            |parsed| {
                assert!(parsed.is_err(), "parsed is not err: {:#?}", parsed);
            }
        })
    }

    macro_rules! parsed_equals {
        ($expected:expr) => ({
            |parsed| {
                let parsed = parsed.unwrap();
                let expected = $expected;
                assert_eq!(parsed, expected, "parsed: {:#?}, expected: {:#?}", parsed, expected);
            }
        })
    }

    macro_rules! expected_ne {
        ($parsed:expr) => ({
            let parsed = $parsed;
            assert!(false, "parsed {:#?}, expected NOT equal", parsed);
        })
    }

    #[test]
    fn parse_assign_statement() {
        test! {
            parse: p0::parse_statement,
            input: "x = 1",
            test: parsed_equals!(
                Statement::Assign(
                    Name::new(b"x").unwrap(),
                    Expression::DecimalI32(DecimalI32(1))
                )
            )
        }

        test! {
            parse: p0::parse_statement,
            input: "x = (-1 + x + (-y + 4))",
            test: parsed_equals!(
                Statement::Assign(
                    Name::new(b"x").unwrap(),
                    Expression::Add(
                        // - 1 + x
                        Expression::Add(
                            Expression::DecimalI32(DecimalI32(-1)).into(),
                            Expression::Name(Name::new(b"x").unwrap()).into()
                        ).into(),
                        // -y + 4
                        Expression::Add(
                            Expression::UnaryNeg(Expression::Name(Name::new(b"y").unwrap()).into()).into(),
                            Expression::DecimalI32(DecimalI32(4)).into()
                        ).into()
                    )
                )
            )
        }
    }

    #[test]
    fn parse_print_statement() {
        test! {
            parse: p0::parse_statement,
            input: "print 1 + x",
            test: parsed_equals!(
                Statement::Print(
                    Expression::Add(
                        Expression::DecimalI32(DecimalI32(1)).into(),
                        Expression::Name(Name::new(b"x").unwrap()).into()
                    )
                )
            )
        }
        test! {
            parse: p0::parse_statement,
            input: "print1",
            test: |parsed| {
                match parsed {
                    Ok(Statement::Print(_)) => expected_ne!(parsed),
                    _ => {}
                }
            }
        }
        test! {
            parse: p0::parse_statement,
            input: "printx+2",
            test: |parsed| {
                match parsed {
                    Ok(Statement::Print(_)) => expected_ne!(parsed),
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn parse_input_call() {
        test! {
            parse: p0::parse_term,
            input: "input()",
            test: parsed_equals!(Expression::Input(Input))
        }
    }

    #[test]
    fn parse_parens_term() {
        test! {
            parse: p0::parse_term,
            input: "(rust_python)",
            test: parsed_equals!(Expression::Name(Name::new(b"rust_python").unwrap()))
        }
        test! {
            parse: p0::parse_term,
            input: "( (( rust_python + 222 )) )",
            test: parsed_equals!(
                Expression::Add(
                    Expression::Name(Name::new(b"rust_python").unwrap()).into(),
                    Expression::DecimalI32(DecimalI32(222)).into()
                )
            )
        }
        test! {
            parse: p0::parse_term,
            input: "( 1829102 + 291 )",
            test: parsed_equals!(
                Expression::Add(
                    Expression::DecimalI32(DecimalI32(1829102)).into(),
                    Expression::DecimalI32(DecimalI32(291)).into()
                )
            )
        }
        test! {
            parse: p0::parse_term,
            input: "-(_ + 2)",
            test: parsed_equals!(
                Expression::UnaryNeg(
                    Expression::Add(
                        Expression::Name(Name::new(b"_").unwrap()).into(),
                        Expression::DecimalI32(DecimalI32(2)).into()
                    ).into()
                )
            )
        }

    }

    #[test]
    fn parse_name_with_uppercase_letters() {
        let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        for c in uppercase.chars() {
            let name = format!("rust{c}python", c = c);
            test! {
                parse: p0::parse_name,
                input: name.as_str(),
                test: parsed_equals!(Name::new(name.as_bytes()).unwrap())
            }
        }
    }

    #[test]
    fn parse_name_with_lowercase_letters() {
        let lowercase = "abcdefghijklmnopqrstuvwxyz";
        for c in lowercase.chars() {
            let name = format!("rust{c}python", c = c);
            test! {
                parse: p0::parse_name,
                input: name.as_str(),
                test: parsed_equals!(Name::new(name.as_bytes()).unwrap())
            }
        }
    }

    #[test]
    fn parse_name_with_numbers() {
        for n in 0..10 {
            let name = format!("rust{n}python", n = n);
            test! {
                parse: p0::parse_name,
                input: name.as_str(),
                test: parsed_equals!(Name::new(name.as_bytes()).unwrap())
            }
        }
    }

    #[test]
    fn parse_name_with_all_valid_char_types() {
        let names = &[
            "_a9",
            "b_0",
            "c0_",
            "rust_python2",
            "_rust___python___292921901",
            "r_jdjf929291lakd_9929292",
        ];

        for name in names {
            test! {
                parse: p0::parse_name,
                input: name,
                test: parsed_equals!(Name::new(name.as_bytes()).unwrap())
            }
        }
    }

    #[test]
    fn parse_name_containing_invalid_characters() {
        let invalid = "!@#$%^&*()-+=[]{}|\\\"':;?><,./";
        for c in invalid.chars() {
            let name = format!("rust{c}python", c = c);
            test! {
                parse: p0::parse_expression,
                input: name.as_str(),
                test: |parsed| {
                    match parsed {
                        Ok(Expression::Name(_)) => {
                            expected_ne!(parsed);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    #[test]
    fn parse_name_starting_with_number() {
        for n in 0..10 {
            let name = format!("{n}rust_python", n = n);
            test! {
                parse: p0::parse_name,
                input: name.as_str(),
                test: parsed_is_err!()
            }
        }
    }

    #[test]
    fn parse_underscore_first_letter() {
        test! {
            parse: p0::parse_name,
            input: "_rust_python",
            test: parsed_equals!(Name::new(b"_rust_python").unwrap())
        }
    }

    #[test]
    fn parse_underscore_name() {
        test! {
            parse: p0::parse_name,
            input: "_",
            test: parsed_equals!(Name::new(b"_").unwrap())
        }
    }

    #[test]
    fn parse_negative_number() {
        test! {
            parse: p0::parse_decimal_i32,
            input: "-71",
            test: parsed_equals!(DecimalI32(-71))
        }
    }

    #[test]
    fn parse_positive_number() {
        test! {
            parse: p0::parse_decimal_i32,
            input: "906",
            test: parsed_equals!(DecimalI32(906))
        }
    }

    #[test]
    fn parse_number_zero() {
        test! {
            parse: p0::parse_decimal_i32,
            input: "0",
            test: parsed_equals!(DecimalI32(0))
        }

        test! {
            parse: p0::parse_decimal_i32,
            input: "-0",
            test: parsed_equals!(DecimalI32(0))
        }
    }

    #[test]
    fn parse_01() {
        // TODO: This might not be the right
        // behavior to expect
        test! {
            parse: p0::parse_decimal_i32,
            input: "01",
            test: parsed_equals!(DecimalI32(0))
        }
    }

    #[test]
    fn parse_i32_min_value() {
        test! {
            parse: p0::parse_decimal_i32,
            input: format!("{}", i32::min_value()).as_str(),
            test: parsed_equals!(DecimalI32(i32::min_value()))
        }
    }

    #[test]
    fn parse_i32_max_value() {
        test! {
            parse: p0::parse_decimal_i32,
            input: format!("{}", i32::max_value()).as_str(),
            test: parsed_equals!(DecimalI32(i32::max_value()))
        }
    }

    #[test]
    fn parse_i32_max_value_plus1() {
        test! {
            parse: p0::parse_decimal_i32,
            input: format!("{}", i32::max_value() as i64 + 1).as_str(),
            test: parsed_is_err!()
        }
    }

    #[test]
    fn parse_i32_min_value_minus1() {
        test! {
            parse: p0::parse_decimal_i32,
            input: format!("{}", i32::min_value() as i64 - 1).as_str(),
            test: parsed_is_err!()
        }
    }

    #[test]
    fn parse_statement() {
        let statement = "1 + 2";
        let lexer = lexer::Lexer::new(statement);
        assert_eq!(
            p0::parse_statement(statement, lexer),
            Ok(Statement::Expression(Expression::Add(
                Expression::DecimalI32(DecimalI32(1)).into(),
                Expression::DecimalI32(DecimalI32(2)).into(),
            )))
        );
    }

    #[test]
    fn parse_statements() {
        let statements = "\n\nprint 1 + 2\n\n3 + 4\n\n\n";
        let lexer = lexer::Lexer::new(statements);
        assert_eq!(
            p0::parse_statements(statements, lexer),
            Ok(vec![
                Statement::Newline,
                Statement::Newline,
                Statement::Print(Expression::Add(
                    Expression::DecimalI32(DecimalI32(1)).into(),
                    Expression::DecimalI32(DecimalI32(2)).into(),
                )),
                Statement::Newline,
                Statement::Newline,
                Statement::Expression(Expression::Add(
                    Expression::DecimalI32(DecimalI32(3)).into(),
                    Expression::DecimalI32(DecimalI32(4)).into(),
                )),
                Statement::Newline,
                Statement::Newline,
                Statement::Newline,
            ])
        );

        test! {
            parse: p0::parse_statements,
            input: "\n\n1 + 2 \n\n \nx +1\nprint 1 + 2 + 3 + -4",
            test: parsed_equals!(
                vec![
                    Statement::Newline,
                    Statement::Newline,
                    Statement::Expression(
                        Expression::Add(
                            Expression::DecimalI32(DecimalI32(1)).into(),
                            Expression::DecimalI32(DecimalI32(2)).into()
                        ),
                    ),
                    Statement::Newline,
                    Statement::Newline,
                    Statement::Newline,
                    Statement::Expression(
                        Expression::Add(
                            Expression::Name(Name::new(b"x").unwrap()).into(),
                            Expression::DecimalI32(DecimalI32(1)).into()
                        )
                    ),
                    Statement::Newline,
                    Statement::Print(
                        Expression::Add(
                            Expression::Add(
                                Expression::Add(
                                    Expression::DecimalI32(DecimalI32(1)).into(),
                                    Expression::DecimalI32(DecimalI32(2)).into(),
                                ).into(),
                                Expression::DecimalI32(DecimalI32(3)).into()
                            ).into(),
                            Expression::DecimalI32(DecimalI32(-4)).into()
                        )
                    )
                ]
            )
        }
    }
}
