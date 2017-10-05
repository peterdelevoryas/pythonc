use token::{Token, Error};
use ::{
    Expression,
    Statement,
    Name,
    DecimalI32,
    Input,
    Module,
    Program,
};
extern crate lalrpop_util as __lalrpop_util;

mod __parse__decimal_i32 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use token::{Token, Error};
    use ::{
    Expression,
    Statement,
    Name,
    DecimalI32,
    Input,
    Module,
    Program,
};
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<> {
        Term_22_28_22(Token),
        Term_22_29_22(Token),
        Term_22_2b_22(Token),
        Term_22_2c_22(Token),
        Term_22_2d_22(Token),
        Term_22_3b_22(Token),
        Term_22_3c_22(Token),
        Term_22_3d_22(Token),
        Term_22_3e_22(Token),
        Term_22_5c_5cn_22(Token),
        Term_22decimal__i32_22(i32),
        Term_22input_22(Token),
        Term_22name_22(String),
        Term_22print_22(Token),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(Statement),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(::std::vec::Vec<Statement>),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(::std::vec::Vec<Statement>),
        Nt____decimal__i32(DecimalI32),
        Nt____expression(Expression),
        Nt____module(Module),
        Nt____name(Name),
        Nt____program(Program),
        Nt____statement(Statement),
        Nt____statements(Vec<Statement>),
        Nt____term(Expression),
        Ntdecimal__i32(DecimalI32),
        Ntexpression(Expression),
        Ntmodule(Module),
        Ntname(Name),
        Ntprogram(Program),
        Ntstatement(Statement),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0,
        // State 1
        -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6,
        // State 2
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -6,
        -14,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"";""###,
            r###""<""###,
            r###""=""###,
            r###"">""###,
            r###""\\n""###,
            r###""decimal_i32""###,
            r###""input""###,
            r###""name""###,
            r###""print""###,
        ];
        __ACTION[(__state * 14)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_decimal_i32<
        __TOKEN: __ToTriple<Error=Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<DecimalI32, __lalrpop_util::ParseError<usize, Token, Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::LeftParens if true => 0,
                Token::RightParens if true => 1,
                Token::Plus if true => 2,
                Token::Comma if true => 3,
                Token::Minus if true => 4,
                Token::Semicolon if true => 5,
                Token::Lt if true => 6,
                Token::Equals if true => 7,
                Token::Gt if true => 8,
                Token::Newline if true => 9,
                Token::DecimalI32(_) if true => 10,
                Token::Input if true => 11,
                Token::Name(_) if true => 12,
                Token::Print if true => 13,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 14 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::LeftParens => __Symbol::Term_22_28_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Token::RightParens => __Symbol::Term_22_29_22((__tok)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Plus => __Symbol::Term_22_2b_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Token::Comma => __Symbol::Term_22_2c_22((__tok)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Token::Minus => __Symbol::Term_22_2d_22((__tok)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Token::Semicolon => __Symbol::Term_22_3b_22((__tok)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::Lt => __Symbol::Term_22_3c_22((__tok)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Token::Equals => __Symbol::Term_22_3d_22((__tok)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Token::Gt => __Symbol::Term_22_3e_22((__tok)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Token::Newline => __Symbol::Term_22_5c_5cn_22((__tok)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token::DecimalI32(__tok0) => __Symbol::Term_22decimal__i32_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Token::Input => __Symbol::Term_22input_22((__tok)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token::Name(__tok0) => __Symbol::Term_22name_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            __tok @ Token::Print => __Symbol::Term_22print_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
    >(
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<DecimalI32,__lalrpop_util::ParseError<usize, Token, Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<statement> "\\n") = statement, "\\n" => ActionFn(25);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__nt), __end));
                0
            }
            2 => {
                // (<statement> "\\n")* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<statement> "\\n")* = (<statement> "\\n")+ => ActionFn(24);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<statement> "\\n")+ = statement, "\\n" => ActionFn(28);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<statement> "\\n")+ = (<statement> "\\n")+, statement, "\\n" => ActionFn(29);
                let __sym2 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                return Some(Ok(__nt));
            }
            7 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____expression(__nt), __end));
                4
            }
            8 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____module(__nt), __end));
                5
            }
            9 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                6
            }
            10 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____program(__nt), __end));
                7
            }
            11 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statement(__nt), __end));
                8
            }
            12 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statements(__nt), __end));
                9
            }
            13 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____term(__nt), __end));
                10
            }
            14 => {
                // decimal_i32 = "decimal_i32" => ActionFn(22);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                11
            }
            15 => {
                // expression = expression, "+", term => ActionFn(14);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            16 => {
                // expression = term => ActionFn(15);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            17 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                13
            }
            18 => {
                // name = "name" => ActionFn(21);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                14
            }
            19 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                15
            }
            20 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            21 => {
                // statement = name, "=", expression => ActionFn(12);
                let __sym2 = __pop_Ntexpression(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            22 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            23 => {
                // statements =  => ActionFn(30);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action30::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            24 => {
                // statements = (<statement> "\\n")+ => ActionFn(31);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            25 => {
                // term = name => ActionFn(16);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            26 => {
                // term = decimal_i32 => ActionFn(17);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            27 => {
                // term = "-", term => ActionFn(18);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            28 => {
                // term = "(", expression, ")" => ActionFn(19);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            29 => {
                // term = "input", "(", ")" => ActionFn(20);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_5cn_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_5cn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22decimal__i32_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22decimal__i32_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22input_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22input_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22name_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22name_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22print_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22print_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____decimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____decimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____expression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____expression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____module<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____module(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____program<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____term<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntdecimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntdecimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntexpression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntexpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntmodule<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntmodule(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntprogram<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntprogram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntterm<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntterm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__decimal_i32::parse_decimal_i32;

mod __parse__expression {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use token::{Token, Error};
    use ::{
    Expression,
    Statement,
    Name,
    DecimalI32,
    Input,
    Module,
    Program,
};
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<> {
        Term_22_28_22(Token),
        Term_22_29_22(Token),
        Term_22_2b_22(Token),
        Term_22_2c_22(Token),
        Term_22_2d_22(Token),
        Term_22_3b_22(Token),
        Term_22_3c_22(Token),
        Term_22_3d_22(Token),
        Term_22_3e_22(Token),
        Term_22_5c_5cn_22(Token),
        Term_22decimal__i32_22(i32),
        Term_22input_22(Token),
        Term_22name_22(String),
        Term_22print_22(Token),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(Statement),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(::std::vec::Vec<Statement>),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(::std::vec::Vec<Statement>),
        Nt____decimal__i32(DecimalI32),
        Nt____expression(Expression),
        Nt____module(Module),
        Nt____name(Name),
        Nt____program(Program),
        Nt____statement(Statement),
        Nt____statements(Vec<Statement>),
        Nt____term(Expression),
        Ntdecimal__i32(DecimalI32),
        Ntexpression(Expression),
        Ntmodule(Module),
        Ntname(Name),
        Ntprogram(Program),
        Ntstatement(Statement),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 8, 9, 10, 0,
        // State 1
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 2
        0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 4
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 5
        6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 8, 9, 10, 0,
        // State 6
        6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 8, 9, 10, 0,
        // State 7
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 8
        14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 10
        6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 8, 9, 10, 0,
        // State 11
        0, 16, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 13
        0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 15
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 16
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -26,
        -7,
        -25,
        -16,
        0,
        0,
        -14,
        0,
        -18,
        0,
        0,
        -27,
        0,
        -15,
        -28,
        -29,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 0, 0, 0, 5,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 12, 0, 4, 0, 0, 0, 5,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 4, 0, 0, 0, 13,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 4, 0, 0, 0, 15,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"";""###,
            r###""<""###,
            r###""=""###,
            r###"">""###,
            r###""\\n""###,
            r###""decimal_i32""###,
            r###""input""###,
            r###""name""###,
            r###""print""###,
        ];
        __ACTION[(__state * 14)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_expression<
        __TOKEN: __ToTriple<Error=Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<Expression, __lalrpop_util::ParseError<usize, Token, Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::LeftParens if true => 0,
                Token::RightParens if true => 1,
                Token::Plus if true => 2,
                Token::Comma if true => 3,
                Token::Minus if true => 4,
                Token::Semicolon if true => 5,
                Token::Lt if true => 6,
                Token::Equals if true => 7,
                Token::Gt if true => 8,
                Token::Newline if true => 9,
                Token::DecimalI32(_) if true => 10,
                Token::Input if true => 11,
                Token::Name(_) if true => 12,
                Token::Print if true => 13,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 14 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::LeftParens => __Symbol::Term_22_28_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Token::RightParens => __Symbol::Term_22_29_22((__tok)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Plus => __Symbol::Term_22_2b_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Token::Comma => __Symbol::Term_22_2c_22((__tok)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Token::Minus => __Symbol::Term_22_2d_22((__tok)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Token::Semicolon => __Symbol::Term_22_3b_22((__tok)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::Lt => __Symbol::Term_22_3c_22((__tok)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Token::Equals => __Symbol::Term_22_3d_22((__tok)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Token::Gt => __Symbol::Term_22_3e_22((__tok)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Token::Newline => __Symbol::Term_22_5c_5cn_22((__tok)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token::DecimalI32(__tok0) => __Symbol::Term_22decimal__i32_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Token::Input => __Symbol::Term_22input_22((__tok)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token::Name(__tok0) => __Symbol::Term_22name_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            __tok @ Token::Print => __Symbol::Term_22print_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
    >(
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Expression,__lalrpop_util::ParseError<usize, Token, Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<statement> "\\n") = statement, "\\n" => ActionFn(25);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__nt), __end));
                0
            }
            2 => {
                // (<statement> "\\n")* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<statement> "\\n")* = (<statement> "\\n")+ => ActionFn(24);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<statement> "\\n")+ = statement, "\\n" => ActionFn(28);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<statement> "\\n")+ = (<statement> "\\n")+, statement, "\\n" => ActionFn(29);
                let __sym2 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____decimal__i32(__nt), __end));
                3
            }
            7 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                return Some(Ok(__nt));
            }
            8 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____module(__nt), __end));
                5
            }
            9 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                6
            }
            10 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____program(__nt), __end));
                7
            }
            11 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statement(__nt), __end));
                8
            }
            12 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statements(__nt), __end));
                9
            }
            13 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____term(__nt), __end));
                10
            }
            14 => {
                // decimal_i32 = "decimal_i32" => ActionFn(22);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                11
            }
            15 => {
                // expression = expression, "+", term => ActionFn(14);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            16 => {
                // expression = term => ActionFn(15);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            17 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                13
            }
            18 => {
                // name = "name" => ActionFn(21);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                14
            }
            19 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                15
            }
            20 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            21 => {
                // statement = name, "=", expression => ActionFn(12);
                let __sym2 = __pop_Ntexpression(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            22 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            23 => {
                // statements =  => ActionFn(30);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action30::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            24 => {
                // statements = (<statement> "\\n")+ => ActionFn(31);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            25 => {
                // term = name => ActionFn(16);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            26 => {
                // term = decimal_i32 => ActionFn(17);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            27 => {
                // term = "-", term => ActionFn(18);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            28 => {
                // term = "(", expression, ")" => ActionFn(19);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            29 => {
                // term = "input", "(", ")" => ActionFn(20);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_5cn_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_5cn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22decimal__i32_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22decimal__i32_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22input_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22input_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22name_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22name_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22print_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22print_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____decimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____decimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____expression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____expression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____module<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____module(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____program<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____term<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntdecimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntdecimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntexpression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntexpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntmodule<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntmodule(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntprogram<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntprogram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntterm<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntterm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__expression::parse_expression;

mod __parse__module {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use token::{Token, Error};
    use ::{
    Expression,
    Statement,
    Name,
    DecimalI32,
    Input,
    Module,
    Program,
};
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<> {
        Term_22_28_22(Token),
        Term_22_29_22(Token),
        Term_22_2b_22(Token),
        Term_22_2c_22(Token),
        Term_22_2d_22(Token),
        Term_22_3b_22(Token),
        Term_22_3c_22(Token),
        Term_22_3d_22(Token),
        Term_22_3e_22(Token),
        Term_22_5c_5cn_22(Token),
        Term_22decimal__i32_22(i32),
        Term_22input_22(Token),
        Term_22name_22(String),
        Term_22print_22(Token),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(Statement),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(::std::vec::Vec<Statement>),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(::std::vec::Vec<Statement>),
        Nt____decimal__i32(DecimalI32),
        Nt____expression(Expression),
        Nt____module(Module),
        Nt____name(Name),
        Nt____program(Program),
        Nt____statement(Statement),
        Nt____statements(Vec<Statement>),
        Nt____term(Expression),
        Ntdecimal__i32(DecimalI32),
        Ntexpression(Expression),
        Ntmodule(Module),
        Ntname(Name),
        Ntprogram(Program),
        Ntstatement(Statement),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 12, 13, 14, 15,
        // State 1
        10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 12, 13, 14, 15,
        // State 2
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 3
        0, 0, 17, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0,
        // State 4
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 5
        0, 0, -25, 0, 0, 0, 0, 18, 0, -25, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0,
        // State 7
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 8
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 9
        10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 10
        10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 11
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 12
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 14
        10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0,
        // State 16
        10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 17
        10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 18
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 19
        0, 28, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 21
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 22
        0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 17, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0,
        // State 24
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 25
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 26
        0, 0, 17, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0,
        // State 27
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 28
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -23,
        -24,
        -26,
        0,
        -8,
        0,
        0,
        -17,
        -16,
        0,
        0,
        -14,
        0,
        -18,
        0,
        0,
        0,
        0,
        -4,
        0,
        -25,
        -27,
        0,
        0,
        -5,
        -15,
        0,
        -28,
        -29,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 6, 0, 7, 8, 9,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 0, 6, 0, 16, 0, 9,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 20, 0, 21, 0, 0, 0, 9,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 21, 0, 0, 0, 22,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 24, 0, 21, 0, 0, 0, 9,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 21, 0, 0, 0, 26,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 27, 0, 21, 0, 0, 0, 9,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"";""###,
            r###""<""###,
            r###""=""###,
            r###"">""###,
            r###""\\n""###,
            r###""decimal_i32""###,
            r###""input""###,
            r###""name""###,
            r###""print""###,
        ];
        __ACTION[(__state * 14)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_module<
        __TOKEN: __ToTriple<Error=Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<Module, __lalrpop_util::ParseError<usize, Token, Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::LeftParens if true => 0,
                Token::RightParens if true => 1,
                Token::Plus if true => 2,
                Token::Comma if true => 3,
                Token::Minus if true => 4,
                Token::Semicolon if true => 5,
                Token::Lt if true => 6,
                Token::Equals if true => 7,
                Token::Gt if true => 8,
                Token::Newline if true => 9,
                Token::DecimalI32(_) if true => 10,
                Token::Input if true => 11,
                Token::Name(_) if true => 12,
                Token::Print if true => 13,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 14 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::LeftParens => __Symbol::Term_22_28_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Token::RightParens => __Symbol::Term_22_29_22((__tok)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Plus => __Symbol::Term_22_2b_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Token::Comma => __Symbol::Term_22_2c_22((__tok)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Token::Minus => __Symbol::Term_22_2d_22((__tok)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Token::Semicolon => __Symbol::Term_22_3b_22((__tok)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::Lt => __Symbol::Term_22_3c_22((__tok)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Token::Equals => __Symbol::Term_22_3d_22((__tok)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Token::Gt => __Symbol::Term_22_3e_22((__tok)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Token::Newline => __Symbol::Term_22_5c_5cn_22((__tok)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token::DecimalI32(__tok0) => __Symbol::Term_22decimal__i32_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Token::Input => __Symbol::Term_22input_22((__tok)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token::Name(__tok0) => __Symbol::Term_22name_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            __tok @ Token::Print => __Symbol::Term_22print_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
    >(
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Module,__lalrpop_util::ParseError<usize, Token, Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<statement> "\\n") = statement, "\\n" => ActionFn(25);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__nt), __end));
                0
            }
            2 => {
                // (<statement> "\\n")* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<statement> "\\n")* = (<statement> "\\n")+ => ActionFn(24);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<statement> "\\n")+ = statement, "\\n" => ActionFn(28);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<statement> "\\n")+ = (<statement> "\\n")+, statement, "\\n" => ActionFn(29);
                let __sym2 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____decimal__i32(__nt), __end));
                3
            }
            7 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____expression(__nt), __end));
                4
            }
            8 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                return Some(Ok(__nt));
            }
            9 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                6
            }
            10 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____program(__nt), __end));
                7
            }
            11 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statement(__nt), __end));
                8
            }
            12 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statements(__nt), __end));
                9
            }
            13 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____term(__nt), __end));
                10
            }
            14 => {
                // decimal_i32 = "decimal_i32" => ActionFn(22);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                11
            }
            15 => {
                // expression = expression, "+", term => ActionFn(14);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            16 => {
                // expression = term => ActionFn(15);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            17 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                13
            }
            18 => {
                // name = "name" => ActionFn(21);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                14
            }
            19 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                15
            }
            20 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            21 => {
                // statement = name, "=", expression => ActionFn(12);
                let __sym2 = __pop_Ntexpression(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            22 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            23 => {
                // statements =  => ActionFn(30);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action30::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            24 => {
                // statements = (<statement> "\\n")+ => ActionFn(31);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            25 => {
                // term = name => ActionFn(16);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            26 => {
                // term = decimal_i32 => ActionFn(17);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            27 => {
                // term = "-", term => ActionFn(18);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            28 => {
                // term = "(", expression, ")" => ActionFn(19);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            29 => {
                // term = "input", "(", ")" => ActionFn(20);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_5cn_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_5cn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22decimal__i32_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22decimal__i32_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22input_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22input_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22name_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22name_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22print_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22print_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____decimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____decimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____expression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____expression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____module<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____module(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____program<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____term<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntdecimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntdecimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntexpression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntexpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntmodule<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntmodule(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntprogram<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntprogram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntterm<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntterm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__module::parse_module;

mod __parse__name {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use token::{Token, Error};
    use ::{
    Expression,
    Statement,
    Name,
    DecimalI32,
    Input,
    Module,
    Program,
};
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<> {
        Term_22_28_22(Token),
        Term_22_29_22(Token),
        Term_22_2b_22(Token),
        Term_22_2c_22(Token),
        Term_22_2d_22(Token),
        Term_22_3b_22(Token),
        Term_22_3c_22(Token),
        Term_22_3d_22(Token),
        Term_22_3e_22(Token),
        Term_22_5c_5cn_22(Token),
        Term_22decimal__i32_22(i32),
        Term_22input_22(Token),
        Term_22name_22(String),
        Term_22print_22(Token),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(Statement),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(::std::vec::Vec<Statement>),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(::std::vec::Vec<Statement>),
        Nt____decimal__i32(DecimalI32),
        Nt____expression(Expression),
        Nt____module(Module),
        Nt____name(Name),
        Nt____program(Program),
        Nt____statement(Statement),
        Nt____statements(Vec<Statement>),
        Nt____term(Expression),
        Ntdecimal__i32(DecimalI32),
        Ntexpression(Expression),
        Ntmodule(Module),
        Ntname(Name),
        Ntprogram(Program),
        Ntstatement(Statement),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 1
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 2
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -9,
        -18,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"";""###,
            r###""<""###,
            r###""=""###,
            r###"">""###,
            r###""\\n""###,
            r###""decimal_i32""###,
            r###""input""###,
            r###""name""###,
            r###""print""###,
        ];
        __ACTION[(__state * 14)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_name<
        __TOKEN: __ToTriple<Error=Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<Name, __lalrpop_util::ParseError<usize, Token, Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::LeftParens if true => 0,
                Token::RightParens if true => 1,
                Token::Plus if true => 2,
                Token::Comma if true => 3,
                Token::Minus if true => 4,
                Token::Semicolon if true => 5,
                Token::Lt if true => 6,
                Token::Equals if true => 7,
                Token::Gt if true => 8,
                Token::Newline if true => 9,
                Token::DecimalI32(_) if true => 10,
                Token::Input if true => 11,
                Token::Name(_) if true => 12,
                Token::Print if true => 13,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 14 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::LeftParens => __Symbol::Term_22_28_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Token::RightParens => __Symbol::Term_22_29_22((__tok)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Plus => __Symbol::Term_22_2b_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Token::Comma => __Symbol::Term_22_2c_22((__tok)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Token::Minus => __Symbol::Term_22_2d_22((__tok)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Token::Semicolon => __Symbol::Term_22_3b_22((__tok)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::Lt => __Symbol::Term_22_3c_22((__tok)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Token::Equals => __Symbol::Term_22_3d_22((__tok)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Token::Gt => __Symbol::Term_22_3e_22((__tok)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Token::Newline => __Symbol::Term_22_5c_5cn_22((__tok)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token::DecimalI32(__tok0) => __Symbol::Term_22decimal__i32_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Token::Input => __Symbol::Term_22input_22((__tok)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token::Name(__tok0) => __Symbol::Term_22name_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            __tok @ Token::Print => __Symbol::Term_22print_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
    >(
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Name,__lalrpop_util::ParseError<usize, Token, Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<statement> "\\n") = statement, "\\n" => ActionFn(25);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__nt), __end));
                0
            }
            2 => {
                // (<statement> "\\n")* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<statement> "\\n")* = (<statement> "\\n")+ => ActionFn(24);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<statement> "\\n")+ = statement, "\\n" => ActionFn(28);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<statement> "\\n")+ = (<statement> "\\n")+, statement, "\\n" => ActionFn(29);
                let __sym2 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____decimal__i32(__nt), __end));
                3
            }
            7 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____expression(__nt), __end));
                4
            }
            8 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____module(__nt), __end));
                5
            }
            9 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                return Some(Ok(__nt));
            }
            10 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____program(__nt), __end));
                7
            }
            11 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statement(__nt), __end));
                8
            }
            12 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statements(__nt), __end));
                9
            }
            13 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____term(__nt), __end));
                10
            }
            14 => {
                // decimal_i32 = "decimal_i32" => ActionFn(22);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                11
            }
            15 => {
                // expression = expression, "+", term => ActionFn(14);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            16 => {
                // expression = term => ActionFn(15);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            17 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                13
            }
            18 => {
                // name = "name" => ActionFn(21);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                14
            }
            19 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                15
            }
            20 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            21 => {
                // statement = name, "=", expression => ActionFn(12);
                let __sym2 = __pop_Ntexpression(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            22 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            23 => {
                // statements =  => ActionFn(30);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action30::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            24 => {
                // statements = (<statement> "\\n")+ => ActionFn(31);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            25 => {
                // term = name => ActionFn(16);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            26 => {
                // term = decimal_i32 => ActionFn(17);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            27 => {
                // term = "-", term => ActionFn(18);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            28 => {
                // term = "(", expression, ")" => ActionFn(19);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            29 => {
                // term = "input", "(", ")" => ActionFn(20);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_5cn_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_5cn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22decimal__i32_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22decimal__i32_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22input_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22input_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22name_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22name_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22print_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22print_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____decimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____decimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____expression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____expression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____module<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____module(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____program<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____term<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntdecimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntdecimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntexpression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntexpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntmodule<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntmodule(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntprogram<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntprogram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntterm<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntterm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__name::parse_name;

mod __parse__program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use token::{Token, Error};
    use ::{
    Expression,
    Statement,
    Name,
    DecimalI32,
    Input,
    Module,
    Program,
};
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<> {
        Term_22_28_22(Token),
        Term_22_29_22(Token),
        Term_22_2b_22(Token),
        Term_22_2c_22(Token),
        Term_22_2d_22(Token),
        Term_22_3b_22(Token),
        Term_22_3c_22(Token),
        Term_22_3d_22(Token),
        Term_22_3e_22(Token),
        Term_22_5c_5cn_22(Token),
        Term_22decimal__i32_22(i32),
        Term_22input_22(Token),
        Term_22name_22(String),
        Term_22print_22(Token),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(Statement),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(::std::vec::Vec<Statement>),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(::std::vec::Vec<Statement>),
        Nt____decimal__i32(DecimalI32),
        Nt____expression(Expression),
        Nt____module(Module),
        Nt____name(Name),
        Nt____program(Program),
        Nt____statement(Statement),
        Nt____statements(Vec<Statement>),
        Nt____term(Expression),
        Ntdecimal__i32(DecimalI32),
        Ntexpression(Expression),
        Ntmodule(Module),
        Ntname(Name),
        Ntprogram(Program),
        Ntstatement(Statement),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 13, 14, 15, 16,
        // State 1
        11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 13, 14, 15, 16,
        // State 2
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 3
        0, 0, 18, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0,
        // State 4
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 5
        0, 0, -25, 0, 0, 0, 0, 19, 0, -25, 0, 0, 0, 0,
        // State 6
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0,
        // State 8
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 9
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 10
        11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 11
        11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 12
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 13
        24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 15
        11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0,
        // State 17
        11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 18
        11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 19
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 20
        0, 29, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 22
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 23
        0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 18, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0,
        // State 25
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 26
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 27
        0, 0, 18, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0,
        // State 28
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 29
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -23,
        -24,
        -26,
        0,
        -19,
        0,
        -10,
        0,
        -17,
        -16,
        0,
        0,
        -14,
        0,
        -18,
        0,
        0,
        0,
        0,
        -4,
        0,
        -25,
        -27,
        0,
        0,
        -5,
        -15,
        0,
        -28,
        -29,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 6, 7, 8, 9, 10,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 0, 6, 0, 17, 0, 10,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 21, 0, 22, 0, 0, 0, 10,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 22, 0, 0, 0, 23,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 25, 0, 22, 0, 0, 0, 10,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 22, 0, 0, 0, 27,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 28, 0, 22, 0, 0, 0, 10,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"";""###,
            r###""<""###,
            r###""=""###,
            r###"">""###,
            r###""\\n""###,
            r###""decimal_i32""###,
            r###""input""###,
            r###""name""###,
            r###""print""###,
        ];
        __ACTION[(__state * 14)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_program<
        __TOKEN: __ToTriple<Error=Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<Program, __lalrpop_util::ParseError<usize, Token, Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::LeftParens if true => 0,
                Token::RightParens if true => 1,
                Token::Plus if true => 2,
                Token::Comma if true => 3,
                Token::Minus if true => 4,
                Token::Semicolon if true => 5,
                Token::Lt if true => 6,
                Token::Equals if true => 7,
                Token::Gt if true => 8,
                Token::Newline if true => 9,
                Token::DecimalI32(_) if true => 10,
                Token::Input if true => 11,
                Token::Name(_) if true => 12,
                Token::Print if true => 13,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 14 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::LeftParens => __Symbol::Term_22_28_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Token::RightParens => __Symbol::Term_22_29_22((__tok)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Plus => __Symbol::Term_22_2b_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Token::Comma => __Symbol::Term_22_2c_22((__tok)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Token::Minus => __Symbol::Term_22_2d_22((__tok)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Token::Semicolon => __Symbol::Term_22_3b_22((__tok)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::Lt => __Symbol::Term_22_3c_22((__tok)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Token::Equals => __Symbol::Term_22_3d_22((__tok)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Token::Gt => __Symbol::Term_22_3e_22((__tok)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Token::Newline => __Symbol::Term_22_5c_5cn_22((__tok)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token::DecimalI32(__tok0) => __Symbol::Term_22decimal__i32_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Token::Input => __Symbol::Term_22input_22((__tok)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token::Name(__tok0) => __Symbol::Term_22name_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            __tok @ Token::Print => __Symbol::Term_22print_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
    >(
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Program,__lalrpop_util::ParseError<usize, Token, Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<statement> "\\n") = statement, "\\n" => ActionFn(25);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__nt), __end));
                0
            }
            2 => {
                // (<statement> "\\n")* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<statement> "\\n")* = (<statement> "\\n")+ => ActionFn(24);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<statement> "\\n")+ = statement, "\\n" => ActionFn(28);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<statement> "\\n")+ = (<statement> "\\n")+, statement, "\\n" => ActionFn(29);
                let __sym2 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____decimal__i32(__nt), __end));
                3
            }
            7 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____expression(__nt), __end));
                4
            }
            8 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____module(__nt), __end));
                5
            }
            9 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                6
            }
            10 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            11 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statement(__nt), __end));
                8
            }
            12 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statements(__nt), __end));
                9
            }
            13 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____term(__nt), __end));
                10
            }
            14 => {
                // decimal_i32 = "decimal_i32" => ActionFn(22);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                11
            }
            15 => {
                // expression = expression, "+", term => ActionFn(14);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            16 => {
                // expression = term => ActionFn(15);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            17 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                13
            }
            18 => {
                // name = "name" => ActionFn(21);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                14
            }
            19 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                15
            }
            20 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            21 => {
                // statement = name, "=", expression => ActionFn(12);
                let __sym2 = __pop_Ntexpression(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            22 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            23 => {
                // statements =  => ActionFn(30);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action30::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            24 => {
                // statements = (<statement> "\\n")+ => ActionFn(31);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            25 => {
                // term = name => ActionFn(16);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            26 => {
                // term = decimal_i32 => ActionFn(17);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            27 => {
                // term = "-", term => ActionFn(18);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            28 => {
                // term = "(", expression, ")" => ActionFn(19);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            29 => {
                // term = "input", "(", ")" => ActionFn(20);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_5cn_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_5cn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22decimal__i32_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22decimal__i32_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22input_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22input_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22name_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22name_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22print_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22print_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____decimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____decimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____expression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____expression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____module<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____module(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____program<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____term<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntdecimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntdecimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntexpression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntexpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntmodule<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntmodule(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntprogram<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntprogram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntterm<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntterm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__program::parse_program;

mod __parse__statement {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use token::{Token, Error};
    use ::{
    Expression,
    Statement,
    Name,
    DecimalI32,
    Input,
    Module,
    Program,
};
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<> {
        Term_22_28_22(Token),
        Term_22_29_22(Token),
        Term_22_2b_22(Token),
        Term_22_2c_22(Token),
        Term_22_2d_22(Token),
        Term_22_3b_22(Token),
        Term_22_3c_22(Token),
        Term_22_3d_22(Token),
        Term_22_3e_22(Token),
        Term_22_5c_5cn_22(Token),
        Term_22decimal__i32_22(i32),
        Term_22input_22(Token),
        Term_22name_22(String),
        Term_22print_22(Token),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(Statement),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(::std::vec::Vec<Statement>),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(::std::vec::Vec<Statement>),
        Nt____decimal__i32(DecimalI32),
        Nt____expression(Expression),
        Nt____module(Module),
        Nt____name(Name),
        Nt____program(Program),
        Nt____statement(Statement),
        Nt____statements(Vec<Statement>),
        Nt____term(Expression),
        Ntdecimal__i32(DecimalI32),
        Ntexpression(Expression),
        Ntmodule(Module),
        Ntname(Name),
        Ntprogram(Program),
        Ntstatement(Statement),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        7, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 11, 12,
        // State 1
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 2
        0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, -25, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0,
        // State 4
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 5
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 6
        7, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 11, 0,
        // State 7
        7, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 11, 0,
        // State 8
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 9
        18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 11
        7, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 11, 0,
        // State 12
        7, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 11, 0,
        // State 13
        7, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 11, 0,
        // State 14
        0, 22, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 16
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 17
        0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 20
        0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 22
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -26,
        -22,
        -25,
        -11,
        -16,
        0,
        0,
        -14,
        0,
        -18,
        0,
        0,
        0,
        0,
        -25,
        -27,
        0,
        -20,
        -15,
        -21,
        -28,
        -29,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 0, 5, 0, 6,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 15, 0, 16, 0, 0, 0, 6,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 16, 0, 0, 0, 17,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 19, 0, 16, 0, 0, 0, 6,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 16, 0, 0, 0, 20,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 21, 0, 16, 0, 0, 0, 6,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"";""###,
            r###""<""###,
            r###""=""###,
            r###"">""###,
            r###""\\n""###,
            r###""decimal_i32""###,
            r###""input""###,
            r###""name""###,
            r###""print""###,
        ];
        __ACTION[(__state * 14)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_statement<
        __TOKEN: __ToTriple<Error=Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<Statement, __lalrpop_util::ParseError<usize, Token, Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::LeftParens if true => 0,
                Token::RightParens if true => 1,
                Token::Plus if true => 2,
                Token::Comma if true => 3,
                Token::Minus if true => 4,
                Token::Semicolon if true => 5,
                Token::Lt if true => 6,
                Token::Equals if true => 7,
                Token::Gt if true => 8,
                Token::Newline if true => 9,
                Token::DecimalI32(_) if true => 10,
                Token::Input if true => 11,
                Token::Name(_) if true => 12,
                Token::Print if true => 13,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 14 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::LeftParens => __Symbol::Term_22_28_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Token::RightParens => __Symbol::Term_22_29_22((__tok)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Plus => __Symbol::Term_22_2b_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Token::Comma => __Symbol::Term_22_2c_22((__tok)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Token::Minus => __Symbol::Term_22_2d_22((__tok)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Token::Semicolon => __Symbol::Term_22_3b_22((__tok)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::Lt => __Symbol::Term_22_3c_22((__tok)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Token::Equals => __Symbol::Term_22_3d_22((__tok)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Token::Gt => __Symbol::Term_22_3e_22((__tok)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Token::Newline => __Symbol::Term_22_5c_5cn_22((__tok)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token::DecimalI32(__tok0) => __Symbol::Term_22decimal__i32_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Token::Input => __Symbol::Term_22input_22((__tok)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token::Name(__tok0) => __Symbol::Term_22name_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            __tok @ Token::Print => __Symbol::Term_22print_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
    >(
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Statement,__lalrpop_util::ParseError<usize, Token, Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<statement> "\\n") = statement, "\\n" => ActionFn(25);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__nt), __end));
                0
            }
            2 => {
                // (<statement> "\\n")* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<statement> "\\n")* = (<statement> "\\n")+ => ActionFn(24);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<statement> "\\n")+ = statement, "\\n" => ActionFn(28);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<statement> "\\n")+ = (<statement> "\\n")+, statement, "\\n" => ActionFn(29);
                let __sym2 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____decimal__i32(__nt), __end));
                3
            }
            7 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____expression(__nt), __end));
                4
            }
            8 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____module(__nt), __end));
                5
            }
            9 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                6
            }
            10 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____program(__nt), __end));
                7
            }
            11 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                return Some(Ok(__nt));
            }
            12 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statements(__nt), __end));
                9
            }
            13 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____term(__nt), __end));
                10
            }
            14 => {
                // decimal_i32 = "decimal_i32" => ActionFn(22);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                11
            }
            15 => {
                // expression = expression, "+", term => ActionFn(14);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            16 => {
                // expression = term => ActionFn(15);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            17 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                13
            }
            18 => {
                // name = "name" => ActionFn(21);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                14
            }
            19 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                15
            }
            20 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            21 => {
                // statement = name, "=", expression => ActionFn(12);
                let __sym2 = __pop_Ntexpression(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            22 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            23 => {
                // statements =  => ActionFn(30);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action30::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            24 => {
                // statements = (<statement> "\\n")+ => ActionFn(31);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            25 => {
                // term = name => ActionFn(16);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            26 => {
                // term = decimal_i32 => ActionFn(17);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            27 => {
                // term = "-", term => ActionFn(18);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            28 => {
                // term = "(", expression, ")" => ActionFn(19);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            29 => {
                // term = "input", "(", ")" => ActionFn(20);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_5cn_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_5cn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22decimal__i32_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22decimal__i32_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22input_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22input_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22name_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22name_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22print_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22print_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____decimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____decimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____expression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____expression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____module<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____module(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____program<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____term<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntdecimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntdecimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntexpression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntexpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntmodule<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntmodule(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntprogram<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntprogram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntterm<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntterm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__statement::parse_statement;

mod __parse__statements {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use token::{Token, Error};
    use ::{
    Expression,
    Statement,
    Name,
    DecimalI32,
    Input,
    Module,
    Program,
};
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<> {
        Term_22_28_22(Token),
        Term_22_29_22(Token),
        Term_22_2b_22(Token),
        Term_22_2c_22(Token),
        Term_22_2d_22(Token),
        Term_22_3b_22(Token),
        Term_22_3c_22(Token),
        Term_22_3d_22(Token),
        Term_22_3e_22(Token),
        Term_22_5c_5cn_22(Token),
        Term_22decimal__i32_22(i32),
        Term_22input_22(Token),
        Term_22name_22(String),
        Term_22print_22(Token),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(Statement),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(::std::vec::Vec<Statement>),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(::std::vec::Vec<Statement>),
        Nt____decimal__i32(DecimalI32),
        Nt____expression(Expression),
        Nt____module(Module),
        Nt____name(Name),
        Nt____program(Program),
        Nt____statement(Statement),
        Nt____statements(Vec<Statement>),
        Nt____term(Expression),
        Ntdecimal__i32(DecimalI32),
        Ntexpression(Expression),
        Ntmodule(Module),
        Ntname(Name),
        Ntprogram(Program),
        Ntstatement(Statement),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        9, 0, 0, 0, 10, 0, 0, 0, 0, 0, 11, 12, 13, 14,
        // State 1
        9, 0, 0, 0, 10, 0, 0, 0, 0, 0, 11, 12, 13, 14,
        // State 2
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 3
        0, 0, 16, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0,
        // State 4
        0, 0, -25, 0, 0, 0, 0, 17, 0, -25, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0,
        // State 6
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 7
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 8
        9, 0, 0, 0, 10, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 9
        9, 0, 0, 0, 10, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 10
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 11
        22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 13
        9, 0, 0, 0, 10, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0,
        // State 15
        9, 0, 0, 0, 10, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 16
        9, 0, 0, 0, 10, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 17
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 18
        0, 27, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 20
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 21
        0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 16, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0,
        // State 23
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 24
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 25
        0, 0, 16, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0,
        // State 26
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 27
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -23,
        -24,
        -26,
        0,
        0,
        0,
        -12,
        -16,
        0,
        0,
        -14,
        0,
        -18,
        0,
        0,
        0,
        0,
        -4,
        0,
        -25,
        -27,
        0,
        0,
        -5,
        -15,
        0,
        -28,
        -29,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 0, 5, 0, 6, 7, 8,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 0, 5, 0, 15, 0, 8,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 19, 0, 20, 0, 0, 0, 8,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 20, 0, 0, 0, 21,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 23, 0, 20, 0, 0, 0, 8,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 20, 0, 0, 0, 25,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 26, 0, 20, 0, 0, 0, 8,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"";""###,
            r###""<""###,
            r###""=""###,
            r###"">""###,
            r###""\\n""###,
            r###""decimal_i32""###,
            r###""input""###,
            r###""name""###,
            r###""print""###,
        ];
        __ACTION[(__state * 14)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_statements<
        __TOKEN: __ToTriple<Error=Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<Vec<Statement>, __lalrpop_util::ParseError<usize, Token, Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::LeftParens if true => 0,
                Token::RightParens if true => 1,
                Token::Plus if true => 2,
                Token::Comma if true => 3,
                Token::Minus if true => 4,
                Token::Semicolon if true => 5,
                Token::Lt if true => 6,
                Token::Equals if true => 7,
                Token::Gt if true => 8,
                Token::Newline if true => 9,
                Token::DecimalI32(_) if true => 10,
                Token::Input if true => 11,
                Token::Name(_) if true => 12,
                Token::Print if true => 13,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 14 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::LeftParens => __Symbol::Term_22_28_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Token::RightParens => __Symbol::Term_22_29_22((__tok)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Plus => __Symbol::Term_22_2b_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Token::Comma => __Symbol::Term_22_2c_22((__tok)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Token::Minus => __Symbol::Term_22_2d_22((__tok)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Token::Semicolon => __Symbol::Term_22_3b_22((__tok)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::Lt => __Symbol::Term_22_3c_22((__tok)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Token::Equals => __Symbol::Term_22_3d_22((__tok)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Token::Gt => __Symbol::Term_22_3e_22((__tok)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Token::Newline => __Symbol::Term_22_5c_5cn_22((__tok)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token::DecimalI32(__tok0) => __Symbol::Term_22decimal__i32_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Token::Input => __Symbol::Term_22input_22((__tok)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token::Name(__tok0) => __Symbol::Term_22name_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            __tok @ Token::Print => __Symbol::Term_22print_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
    >(
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Statement>,__lalrpop_util::ParseError<usize, Token, Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<statement> "\\n") = statement, "\\n" => ActionFn(25);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__nt), __end));
                0
            }
            2 => {
                // (<statement> "\\n")* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<statement> "\\n")* = (<statement> "\\n")+ => ActionFn(24);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<statement> "\\n")+ = statement, "\\n" => ActionFn(28);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<statement> "\\n")+ = (<statement> "\\n")+, statement, "\\n" => ActionFn(29);
                let __sym2 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____decimal__i32(__nt), __end));
                3
            }
            7 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____expression(__nt), __end));
                4
            }
            8 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____module(__nt), __end));
                5
            }
            9 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                6
            }
            10 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____program(__nt), __end));
                7
            }
            11 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statement(__nt), __end));
                8
            }
            12 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                return Some(Ok(__nt));
            }
            13 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____term(__nt), __end));
                10
            }
            14 => {
                // decimal_i32 = "decimal_i32" => ActionFn(22);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                11
            }
            15 => {
                // expression = expression, "+", term => ActionFn(14);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            16 => {
                // expression = term => ActionFn(15);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            17 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                13
            }
            18 => {
                // name = "name" => ActionFn(21);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                14
            }
            19 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                15
            }
            20 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            21 => {
                // statement = name, "=", expression => ActionFn(12);
                let __sym2 = __pop_Ntexpression(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            22 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            23 => {
                // statements =  => ActionFn(30);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action30::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            24 => {
                // statements = (<statement> "\\n")+ => ActionFn(31);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            25 => {
                // term = name => ActionFn(16);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            26 => {
                // term = decimal_i32 => ActionFn(17);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            27 => {
                // term = "-", term => ActionFn(18);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            28 => {
                // term = "(", expression, ")" => ActionFn(19);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            29 => {
                // term = "input", "(", ")" => ActionFn(20);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_5cn_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_5cn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22decimal__i32_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22decimal__i32_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22input_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22input_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22name_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22name_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22print_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22print_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____decimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____decimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____expression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____expression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____module<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____module(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____program<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____term<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntdecimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntdecimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntexpression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntexpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntmodule<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntmodule(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntprogram<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntprogram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntterm<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntterm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__statements::parse_statements;

mod __parse__term {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use token::{Token, Error};
    use ::{
    Expression,
    Statement,
    Name,
    DecimalI32,
    Input,
    Module,
    Program,
};
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<> {
        Term_22_28_22(Token),
        Term_22_29_22(Token),
        Term_22_2b_22(Token),
        Term_22_2c_22(Token),
        Term_22_2d_22(Token),
        Term_22_3b_22(Token),
        Term_22_3c_22(Token),
        Term_22_3d_22(Token),
        Term_22_3e_22(Token),
        Term_22_5c_5cn_22(Token),
        Term_22decimal__i32_22(i32),
        Term_22input_22(Token),
        Term_22name_22(String),
        Term_22print_22(Token),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(Statement),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(::std::vec::Vec<Statement>),
        Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(::std::vec::Vec<Statement>),
        Nt____decimal__i32(DecimalI32),
        Nt____expression(Expression),
        Nt____module(Module),
        Nt____name(Name),
        Nt____program(Program),
        Nt____statement(Statement),
        Nt____statements(Vec<Statement>),
        Nt____term(Expression),
        Ntdecimal__i32(DecimalI32),
        Ntexpression(Expression),
        Ntmodule(Module),
        Ntname(Name),
        Ntprogram(Program),
        Ntstatement(Statement),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        5, 0, 0, 0, 6, 0, 0, 0, 0, 0, 7, 8, 9, 0,
        // State 1
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 2
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 3
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 4
        5, 0, 0, 0, 6, 0, 0, 0, 0, 0, 7, 8, 9, 0,
        // State 5
        5, 0, 0, 0, 6, 0, 0, 0, 0, 0, 7, 8, 9, 0,
        // State 6
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 7
        13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 9
        0, 14, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 11
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 12
        0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 14
        5, 0, 0, 0, 6, 0, 0, 0, 0, 0, 7, 8, 9, 0,
        // State 15
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 16
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -26,
        -25,
        -13,
        0,
        0,
        -14,
        0,
        -18,
        0,
        -16,
        -27,
        0,
        -28,
        0,
        -29,
        -15,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 0, 4,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 10, 0, 3, 0, 0, 0, 11,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 0, 12,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 0, 17,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"";""###,
            r###""<""###,
            r###""=""###,
            r###"">""###,
            r###""\\n""###,
            r###""decimal_i32""###,
            r###""input""###,
            r###""name""###,
            r###""print""###,
        ];
        __ACTION[(__state * 14)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_term<
        __TOKEN: __ToTriple<Error=Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<Expression, __lalrpop_util::ParseError<usize, Token, Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::LeftParens if true => 0,
                Token::RightParens if true => 1,
                Token::Plus if true => 2,
                Token::Comma if true => 3,
                Token::Minus if true => 4,
                Token::Semicolon if true => 5,
                Token::Lt if true => 6,
                Token::Equals if true => 7,
                Token::Gt if true => 8,
                Token::Newline if true => 9,
                Token::DecimalI32(_) if true => 10,
                Token::Input if true => 11,
                Token::Name(_) if true => 12,
                Token::Print if true => 13,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 14 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::LeftParens => __Symbol::Term_22_28_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Token::RightParens => __Symbol::Term_22_29_22((__tok)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Plus => __Symbol::Term_22_2b_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Token::Comma => __Symbol::Term_22_2c_22((__tok)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Token::Minus => __Symbol::Term_22_2d_22((__tok)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Token::Semicolon => __Symbol::Term_22_3b_22((__tok)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::Lt => __Symbol::Term_22_3c_22((__tok)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Token::Equals => __Symbol::Term_22_3d_22((__tok)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Token::Gt => __Symbol::Term_22_3e_22((__tok)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Token::Newline => __Symbol::Term_22_5c_5cn_22((__tok)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token::DecimalI32(__tok0) => __Symbol::Term_22decimal__i32_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Token::Input => __Symbol::Term_22input_22((__tok)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token::Name(__tok0) => __Symbol::Term_22name_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            __tok @ Token::Print => __Symbol::Term_22print_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
    >(
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Expression,__lalrpop_util::ParseError<usize, Token, Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<statement> "\\n") = statement, "\\n" => ActionFn(25);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__nt), __end));
                0
            }
            2 => {
                // (<statement> "\\n")* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<statement> "\\n")* = (<statement> "\\n")+ => ActionFn(24);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<statement> "\\n")+ = statement, "\\n" => ActionFn(28);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<statement> "\\n")+ = (<statement> "\\n")+, statement, "\\n" => ActionFn(29);
                let __sym2 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____decimal__i32(__nt), __end));
                3
            }
            7 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____expression(__nt), __end));
                4
            }
            8 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____module(__nt), __end));
                5
            }
            9 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                6
            }
            10 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____program(__nt), __end));
                7
            }
            11 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statement(__nt), __end));
                8
            }
            12 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statements(__nt), __end));
                9
            }
            13 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                return Some(Ok(__nt));
            }
            14 => {
                // decimal_i32 = "decimal_i32" => ActionFn(22);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                11
            }
            15 => {
                // expression = expression, "+", term => ActionFn(14);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            16 => {
                // expression = term => ActionFn(15);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                12
            }
            17 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                13
            }
            18 => {
                // name = "name" => ActionFn(21);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                14
            }
            19 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                15
            }
            20 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            21 => {
                // statement = name, "=", expression => ActionFn(12);
                let __sym2 = __pop_Ntexpression(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            22 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                16
            }
            23 => {
                // statements =  => ActionFn(30);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action30::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            24 => {
                // statements = (<statement> "\\n")+ => ActionFn(31);
                let __sym0 = __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                17
            }
            25 => {
                // term = name => ActionFn(16);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            26 => {
                // term = decimal_i32 => ActionFn(17);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            27 => {
                // term = "-", term => ActionFn(18);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            28 => {
                // term = "(", expression, ")" => ActionFn(19);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            29 => {
                // term = "input", "(", ")" => ActionFn(20);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                18
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_5cn_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_5cn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22decimal__i32_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22decimal__i32_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22input_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22input_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22name_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22name_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22print_22<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22print_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cstatement_3e_20_22_5c_5cn_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____decimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____decimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____expression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____expression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____module<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____module(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____program<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____statements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____statements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____term<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntdecimal__i32<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DecimalI32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntdecimal__i32(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntexpression<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntexpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntmodule<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Module, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntmodule(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntprogram<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntprogram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatements<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntterm<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntterm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__term::parse_term;

fn __action0<
>(
    (_, __0, _): (usize, Program, usize),
) -> Program
{
    (__0)
}

fn __action1<
>(
    (_, __0, _): (usize, Module, usize),
) -> Module
{
    (__0)
}

fn __action2<
>(
    (_, __0, _): (usize, Vec<Statement>, usize),
) -> Vec<Statement>
{
    (__0)
}

fn __action3<
>(
    (_, __0, _): (usize, Statement, usize),
) -> Statement
{
    (__0)
}

fn __action4<
>(
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

fn __action5<
>(
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

fn __action6<
>(
    (_, __0, _): (usize, Name, usize),
) -> Name
{
    (__0)
}

fn __action7<
>(
    (_, __0, _): (usize, DecimalI32, usize),
) -> DecimalI32
{
    (__0)
}

fn __action8<
>(
    (_, m, _): (usize, Module, usize),
) -> Program
{
    Program { module: m }
}

fn __action9<
>(
    (_, s, _): (usize, Vec<Statement>, usize),
) -> Module
{
    Module { statements: s }
}

fn __action10<
>(
    (_, v, _): (usize, ::std::vec::Vec<Statement>, usize),
) -> Vec<Statement>
{
    v
}

fn __action11<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, Expression, usize),
) -> Statement
{
    Statement::Print(__0)
}

fn __action12<
>(
    (_, __0, _): (usize, Name, usize),
    (_, _, _): (usize, Token, usize),
    (_, __1, _): (usize, Expression, usize),
) -> Statement
{
    Statement::Assign(__0, __1)
}

fn __action13<
>(
    (_, __0, _): (usize, Expression, usize),
) -> Statement
{
    Statement::Expression(__0)
}

fn __action14<
>(
    (_, left, _): (usize, Expression, usize),
    (_, _, _): (usize, Token, usize),
    (_, right, _): (usize, Expression, usize),
) -> Expression
{
    Expression::Add(left.into(), right.into())
}

fn __action15<
>(
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

fn __action16<
>(
    (_, __0, _): (usize, Name, usize),
) -> Expression
{
    Expression::Name(__0)
}

fn __action17<
>(
    (_, __0, _): (usize, DecimalI32, usize),
) -> Expression
{
    Expression::DecimalI32(__0)
}

fn __action18<
>(
    (_, _, _): (usize, Token, usize),
    (_, t, _): (usize, Expression, usize),
) -> Expression
{
    Expression::UnaryNeg(t.into())
}

fn __action19<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, Expression, usize),
    (_, _, _): (usize, Token, usize),
) -> Expression
{
    (__0)
}

fn __action20<
>(
    (_, __0, _): (usize, Token, usize),
    (_, __1, _): (usize, Token, usize),
    (_, __2, _): (usize, Token, usize),
) -> Expression
{
    Expression::Input(Input)
}

fn __action21<
>(
    (_, s, _): (usize, String, usize),
) -> Name
{
    Name(s)
}

fn __action22<
>(
    (_, i, _): (usize, i32, usize),
) -> DecimalI32
{
    DecimalI32(i)
}

fn __action23<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Statement>
{
    vec![]
}

fn __action24<
>(
    (_, v, _): (usize, ::std::vec::Vec<Statement>, usize),
) -> ::std::vec::Vec<Statement>
{
    v
}

fn __action25<
>(
    (_, __0, _): (usize, Statement, usize),
    (_, _, _): (usize, Token, usize),
) -> Statement
{
    (__0)
}

fn __action26<
>(
    (_, __0, _): (usize, Statement, usize),
) -> ::std::vec::Vec<Statement>
{
    vec![__0]
}

fn __action27<
>(
    (_, v, _): (usize, ::std::vec::Vec<Statement>, usize),
    (_, e, _): (usize, Statement, usize),
) -> ::std::vec::Vec<Statement>
{
    { let mut v = v; v.push(e); v }
}

fn __action28<
>(
    __0: (usize, Statement, usize),
    __1: (usize, Token, usize),
) -> ::std::vec::Vec<Statement>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action25(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        __temp0,
    )
}

fn __action29<
>(
    __0: (usize, ::std::vec::Vec<Statement>, usize),
    __1: (usize, Statement, usize),
    __2: (usize, Token, usize),
) -> ::std::vec::Vec<Statement>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action25(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        __0,
        __temp0,
    )
}

fn __action30<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Statement>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action23(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        __temp0,
    )
}

fn __action31<
>(
    __0: (usize, ::std::vec::Vec<Statement>, usize),
) -> Vec<Statement>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action24(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        __temp0,
    )
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Token,usize),Self::Error>;
}

impl<> __ToTriple<> for (usize, Token, usize) {
    type Error = Error;
    fn to_triple(value: Self) -> Result<(usize,Token,usize),Error> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, Token, usize),Error> {
    type Error = Error;
    fn to_triple(value: Self) -> Result<(usize,Token,usize),Error> {
        value
    }
}
