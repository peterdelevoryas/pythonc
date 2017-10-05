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
        Ntstatement_2a(::std::vec::Vec<Statement>),
        Ntstatement_2b(::std::vec::Vec<Statement>),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0,
        // State 1
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        // State 2
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -1,
        -9,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                return Some(Ok(__nt));
            }
            2 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____expression(__nt), __end));
                1
            }
            3 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____module(__nt), __end));
                2
            }
            4 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                3
            }
            5 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____program(__nt), __end));
                4
            }
            6 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statement(__nt), __end));
                5
            }
            7 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statements(__nt), __end));
                6
            }
            8 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____term(__nt), __end));
                7
            }
            9 => {
                // decimal_i32 = "decimal_i32" => ActionFn(24);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                8
            }
            10 => {
                // expression = expression, "+", term => ActionFn(16);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            11 => {
                // expression = term => ActionFn(17);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            12 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                10
            }
            13 => {
                // name = "name" => ActionFn(23);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                11
            }
            14 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                12
            }
            15 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            16 => {
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
                13
            }
            17 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            18 => {
                // statement = "\\n" => ActionFn(14);
                let __sym0 = __pop_Term_22_5c_5cn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            19 => {
                // statement = ";" => ActionFn(15);
                let __sym0 = __pop_Term_22_3b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            20 => {
                // statement* =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            21 => {
                // statement* = statement+ => ActionFn(26);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            22 => {
                // statement+ = statement => ActionFn(27);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            23 => {
                // statement+ = statement+, statement => ActionFn(28);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            24 => {
                // statements =  => ActionFn(29);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action29::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            25 => {
                // statements = statement+ => ActionFn(30);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            26 => {
                // term = name => ActionFn(18);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            27 => {
                // term = decimal_i32 => ActionFn(19);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            28 => {
                // term = "-", term => ActionFn(20);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            29 => {
                // term = "(", expression, ")" => ActionFn(21);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            30 => {
                // term = "input", "(", ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
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
    fn __pop_Ntstatement_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2b(__v), __r) => (__l, __v, __r),
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
        Ntstatement_2a(::std::vec::Vec<Statement>),
        Ntstatement_2b(::std::vec::Vec<Statement>),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 8, 9, 10, 0,
        // State 1
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 2
        0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 4
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 5
        6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 8, 9, 10, 0,
        // State 6
        6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 8, 9, 10, 0,
        // State 7
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 8
        14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 10
        6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 8, 9, 10, 0,
        // State 11
        0, 16, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 13
        0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 15
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 16
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -27,
        -2,
        -26,
        -11,
        0,
        0,
        -9,
        0,
        -13,
        0,
        0,
        -28,
        0,
        -10,
        -29,
        -30,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 0, 0, 0, 0, 0, 5,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 2, 12, 0, 4, 0, 0, 0, 0, 0, 5,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 4, 0, 0, 0, 0, 0, 13,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 4, 0, 0, 0, 0, 0, 15,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____decimal__i32(__nt), __end));
                0
            }
            2 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                return Some(Ok(__nt));
            }
            3 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____module(__nt), __end));
                2
            }
            4 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                3
            }
            5 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____program(__nt), __end));
                4
            }
            6 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statement(__nt), __end));
                5
            }
            7 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statements(__nt), __end));
                6
            }
            8 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____term(__nt), __end));
                7
            }
            9 => {
                // decimal_i32 = "decimal_i32" => ActionFn(24);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                8
            }
            10 => {
                // expression = expression, "+", term => ActionFn(16);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            11 => {
                // expression = term => ActionFn(17);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            12 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                10
            }
            13 => {
                // name = "name" => ActionFn(23);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                11
            }
            14 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                12
            }
            15 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            16 => {
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
                13
            }
            17 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            18 => {
                // statement = "\\n" => ActionFn(14);
                let __sym0 = __pop_Term_22_5c_5cn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            19 => {
                // statement = ";" => ActionFn(15);
                let __sym0 = __pop_Term_22_3b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            20 => {
                // statement* =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            21 => {
                // statement* = statement+ => ActionFn(26);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            22 => {
                // statement+ = statement => ActionFn(27);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            23 => {
                // statement+ = statement+, statement => ActionFn(28);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            24 => {
                // statements =  => ActionFn(29);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action29::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            25 => {
                // statements = statement+ => ActionFn(30);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            26 => {
                // term = name => ActionFn(18);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            27 => {
                // term = decimal_i32 => ActionFn(19);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            28 => {
                // term = "-", term => ActionFn(20);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            29 => {
                // term = "(", expression, ")" => ActionFn(21);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            30 => {
                // term = "input", "(", ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
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
    fn __pop_Ntstatement_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2b(__v), __r) => (__l, __v, __r),
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
        Ntstatement_2a(::std::vec::Vec<Statement>),
        Ntstatement_2b(::std::vec::Vec<Statement>),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        10, 0, 0, 0, 11, 12, 0, 0, 0, 13, 14, 15, 16, 17,
        // State 1
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 2
        -17, 0, 18, 0, -17, -17, 0, 0, 0, -17, -17, -17, -17, -17,
        // State 3
        -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3,
        // State 4
        -26, 0, -26, 0, -26, -26, 0, 19, 0, -26, -26, -26, -26, -26,
        // State 5
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 6
        10, 0, 0, 0, 11, 12, 0, 0, 0, 13, 14, 15, 16, 17,
        // State 7
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 8
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 9
        10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 14, 15, 16, 0,
        // State 10
        10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 14, 15, 16, 0,
        // State 11
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 12
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 13
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 14
        24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 16
        10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 14, 15, 16, 0,
        // State 17
        10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 14, 15, 16, 0,
        // State 18
        10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 14, 15, 16, 0,
        // State 19
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 20
        0, 28, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 22
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 23
        0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        -15, 0, 18, 0, -15, -15, 0, 0, 0, -15, -15, -15, -15, -15,
        // State 25
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 26
        -16, 0, 18, 0, -16, -16, 0, 0, 0, -16, -16, -16, -16, -16,
        // State 27
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 28
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -24,
        -27,
        -17,
        -3,
        -26,
        -22,
        -25,
        -12,
        -11,
        0,
        0,
        -19,
        -18,
        -9,
        0,
        -13,
        0,
        0,
        0,
        -23,
        0,
        -26,
        -28,
        0,
        -15,
        -10,
        -16,
        -29,
        -30,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 5, 0, 6, 0, 7, 8, 9,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 5, 0, 20, 0, 0, 0, 9,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 2, 21, 0, 22, 0, 0, 0, 0, 0, 9,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 22, 0, 0, 0, 0, 0, 23,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 2, 25, 0, 22, 0, 0, 0, 0, 0, 9,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 22, 0, 0, 0, 0, 0, 26,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 2, 27, 0, 22, 0, 0, 0, 0, 0, 9,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____decimal__i32(__nt), __end));
                0
            }
            2 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____expression(__nt), __end));
                1
            }
            3 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                return Some(Ok(__nt));
            }
            4 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                3
            }
            5 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____program(__nt), __end));
                4
            }
            6 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statement(__nt), __end));
                5
            }
            7 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statements(__nt), __end));
                6
            }
            8 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____term(__nt), __end));
                7
            }
            9 => {
                // decimal_i32 = "decimal_i32" => ActionFn(24);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                8
            }
            10 => {
                // expression = expression, "+", term => ActionFn(16);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            11 => {
                // expression = term => ActionFn(17);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            12 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                10
            }
            13 => {
                // name = "name" => ActionFn(23);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                11
            }
            14 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                12
            }
            15 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            16 => {
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
                13
            }
            17 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            18 => {
                // statement = "\\n" => ActionFn(14);
                let __sym0 = __pop_Term_22_5c_5cn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            19 => {
                // statement = ";" => ActionFn(15);
                let __sym0 = __pop_Term_22_3b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            20 => {
                // statement* =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            21 => {
                // statement* = statement+ => ActionFn(26);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            22 => {
                // statement+ = statement => ActionFn(27);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            23 => {
                // statement+ = statement+, statement => ActionFn(28);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            24 => {
                // statements =  => ActionFn(29);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action29::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            25 => {
                // statements = statement+ => ActionFn(30);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            26 => {
                // term = name => ActionFn(18);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            27 => {
                // term = decimal_i32 => ActionFn(19);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            28 => {
                // term = "-", term => ActionFn(20);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            29 => {
                // term = "(", expression, ")" => ActionFn(21);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            30 => {
                // term = "input", "(", ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
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
    fn __pop_Ntstatement_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2b(__v), __r) => (__l, __v, __r),
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
        Ntstatement_2a(::std::vec::Vec<Statement>),
        Ntstatement_2b(::std::vec::Vec<Statement>),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 1
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 2
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -4,
        -13,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____decimal__i32(__nt), __end));
                0
            }
            2 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____expression(__nt), __end));
                1
            }
            3 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____module(__nt), __end));
                2
            }
            4 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                return Some(Ok(__nt));
            }
            5 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____program(__nt), __end));
                4
            }
            6 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statement(__nt), __end));
                5
            }
            7 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statements(__nt), __end));
                6
            }
            8 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____term(__nt), __end));
                7
            }
            9 => {
                // decimal_i32 = "decimal_i32" => ActionFn(24);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                8
            }
            10 => {
                // expression = expression, "+", term => ActionFn(16);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            11 => {
                // expression = term => ActionFn(17);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            12 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                10
            }
            13 => {
                // name = "name" => ActionFn(23);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                11
            }
            14 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                12
            }
            15 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            16 => {
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
                13
            }
            17 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            18 => {
                // statement = "\\n" => ActionFn(14);
                let __sym0 = __pop_Term_22_5c_5cn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            19 => {
                // statement = ";" => ActionFn(15);
                let __sym0 = __pop_Term_22_3b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            20 => {
                // statement* =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            21 => {
                // statement* = statement+ => ActionFn(26);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            22 => {
                // statement+ = statement => ActionFn(27);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            23 => {
                // statement+ = statement+, statement => ActionFn(28);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            24 => {
                // statements =  => ActionFn(29);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action29::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            25 => {
                // statements = statement+ => ActionFn(30);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            26 => {
                // term = name => ActionFn(18);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            27 => {
                // term = decimal_i32 => ActionFn(19);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            28 => {
                // term = "-", term => ActionFn(20);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            29 => {
                // term = "(", expression, ")" => ActionFn(21);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            30 => {
                // term = "input", "(", ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
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
    fn __pop_Ntstatement_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2b(__v), __r) => (__l, __v, __r),
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
        Ntstatement_2a(::std::vec::Vec<Statement>),
        Ntstatement_2b(::std::vec::Vec<Statement>),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        11, 0, 0, 0, 12, 13, 0, 0, 0, 14, 15, 16, 17, 18,
        // State 1
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 2
        -17, 0, 19, 0, -17, -17, 0, 0, 0, -17, -17, -17, -17, -17,
        // State 3
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 4
        -26, 0, -26, 0, -26, -26, 0, 20, 0, -26, -26, -26, -26, -26,
        // State 5
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 6
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 7
        11, 0, 0, 0, 12, 13, 0, 0, 0, 14, 15, 16, 17, 18,
        // State 8
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 9
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 10
        11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 11
        11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 12
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 13
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 14
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 15
        25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 17
        11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 18
        11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 19
        11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 20
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 21
        0, 29, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 23
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 24
        0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        -15, 0, 19, 0, -15, -15, 0, 0, 0, -15, -15, -15, -15, -15,
        // State 26
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 27
        -16, 0, 19, 0, -16, -16, 0, 0, 0, -16, -16, -16, -16, -16,
        // State 28
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 29
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -24,
        -27,
        -17,
        -14,
        -26,
        -5,
        -22,
        -25,
        -12,
        -11,
        0,
        0,
        -19,
        -18,
        -9,
        0,
        -13,
        0,
        0,
        0,
        -23,
        0,
        -26,
        -28,
        0,
        -15,
        -10,
        -16,
        -29,
        -30,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 5, 6, 7, 0, 8, 9, 10,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 5, 0, 21, 0, 0, 0, 10,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 2, 22, 0, 23, 0, 0, 0, 0, 0, 10,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 23, 0, 0, 0, 0, 0, 24,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 2, 26, 0, 23, 0, 0, 0, 0, 0, 10,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 23, 0, 0, 0, 0, 0, 27,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 2, 28, 0, 23, 0, 0, 0, 0, 0, 10,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____decimal__i32(__nt), __end));
                0
            }
            2 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____expression(__nt), __end));
                1
            }
            3 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____module(__nt), __end));
                2
            }
            4 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                3
            }
            5 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            6 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statement(__nt), __end));
                5
            }
            7 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statements(__nt), __end));
                6
            }
            8 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____term(__nt), __end));
                7
            }
            9 => {
                // decimal_i32 = "decimal_i32" => ActionFn(24);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                8
            }
            10 => {
                // expression = expression, "+", term => ActionFn(16);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            11 => {
                // expression = term => ActionFn(17);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            12 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                10
            }
            13 => {
                // name = "name" => ActionFn(23);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                11
            }
            14 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                12
            }
            15 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            16 => {
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
                13
            }
            17 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            18 => {
                // statement = "\\n" => ActionFn(14);
                let __sym0 = __pop_Term_22_5c_5cn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            19 => {
                // statement = ";" => ActionFn(15);
                let __sym0 = __pop_Term_22_3b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            20 => {
                // statement* =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            21 => {
                // statement* = statement+ => ActionFn(26);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            22 => {
                // statement+ = statement => ActionFn(27);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            23 => {
                // statement+ = statement+, statement => ActionFn(28);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            24 => {
                // statements =  => ActionFn(29);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action29::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            25 => {
                // statements = statement+ => ActionFn(30);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            26 => {
                // term = name => ActionFn(18);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            27 => {
                // term = decimal_i32 => ActionFn(19);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            28 => {
                // term = "-", term => ActionFn(20);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            29 => {
                // term = "(", expression, ")" => ActionFn(21);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            30 => {
                // term = "input", "(", ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
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
    fn __pop_Ntstatement_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2b(__v), __r) => (__l, __v, __r),
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
        Ntstatement_2a(::std::vec::Vec<Statement>),
        Ntstatement_2b(::std::vec::Vec<Statement>),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        7, 0, 0, 0, 8, 9, 0, 0, 0, 10, 11, 12, 13, 14,
        // State 1
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 2
        0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, -26, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0,
        // State 4
        -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6,
        // State 5
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 6
        7, 0, 0, 0, 8, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 7
        7, 0, 0, 0, 8, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 8
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 9
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 10
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 11
        20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 13
        7, 0, 0, 0, 8, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 14
        7, 0, 0, 0, 8, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 15
        7, 0, 0, 0, 8, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 16
        0, 24, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 18
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 19
        0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 22
        0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 24
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -27,
        -17,
        -26,
        -6,
        -11,
        0,
        0,
        -19,
        -18,
        -9,
        0,
        -13,
        0,
        0,
        0,
        0,
        -26,
        -28,
        0,
        -15,
        -10,
        -16,
        -29,
        -30,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 0, 5, 0, 0, 0, 6,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 2, 17, 0, 18, 0, 0, 0, 0, 0, 6,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 18, 0, 0, 0, 0, 0, 19,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 2, 21, 0, 18, 0, 0, 0, 0, 0, 6,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 18, 0, 0, 0, 0, 0, 22,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 2, 23, 0, 18, 0, 0, 0, 0, 0, 6,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____decimal__i32(__nt), __end));
                0
            }
            2 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____expression(__nt), __end));
                1
            }
            3 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____module(__nt), __end));
                2
            }
            4 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                3
            }
            5 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____program(__nt), __end));
                4
            }
            6 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                return Some(Ok(__nt));
            }
            7 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statements(__nt), __end));
                6
            }
            8 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____term(__nt), __end));
                7
            }
            9 => {
                // decimal_i32 = "decimal_i32" => ActionFn(24);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                8
            }
            10 => {
                // expression = expression, "+", term => ActionFn(16);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            11 => {
                // expression = term => ActionFn(17);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            12 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                10
            }
            13 => {
                // name = "name" => ActionFn(23);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                11
            }
            14 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                12
            }
            15 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            16 => {
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
                13
            }
            17 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            18 => {
                // statement = "\\n" => ActionFn(14);
                let __sym0 = __pop_Term_22_5c_5cn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            19 => {
                // statement = ";" => ActionFn(15);
                let __sym0 = __pop_Term_22_3b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            20 => {
                // statement* =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            21 => {
                // statement* = statement+ => ActionFn(26);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            22 => {
                // statement+ = statement => ActionFn(27);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            23 => {
                // statement+ = statement+, statement => ActionFn(28);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            24 => {
                // statements =  => ActionFn(29);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action29::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            25 => {
                // statements = statement+ => ActionFn(30);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            26 => {
                // term = name => ActionFn(18);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            27 => {
                // term = decimal_i32 => ActionFn(19);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            28 => {
                // term = "-", term => ActionFn(20);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            29 => {
                // term = "(", expression, ")" => ActionFn(21);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            30 => {
                // term = "input", "(", ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
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
    fn __pop_Ntstatement_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2b(__v), __r) => (__l, __v, __r),
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
        Ntstatement_2a(::std::vec::Vec<Statement>),
        Ntstatement_2b(::std::vec::Vec<Statement>),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        9, 0, 0, 0, 10, 11, 0, 0, 0, 12, 13, 14, 15, 16,
        // State 1
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 2
        -17, 0, 17, 0, -17, -17, 0, 0, 0, -17, -17, -17, -17, -17,
        // State 3
        -26, 0, -26, 0, -26, -26, 0, 18, 0, -26, -26, -26, -26, -26,
        // State 4
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 5
        9, 0, 0, 0, 10, 11, 0, 0, 0, 12, 13, 14, 15, 16,
        // State 6
        -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7,
        // State 7
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 8
        9, 0, 0, 0, 10, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 9
        9, 0, 0, 0, 10, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 10
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 11
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 12
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 13
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 15
        9, 0, 0, 0, 10, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 16
        9, 0, 0, 0, 10, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 17
        9, 0, 0, 0, 10, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 18
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 19
        0, 27, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 21
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 22
        0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        -15, 0, 17, 0, -15, -15, 0, 0, 0, -15, -15, -15, -15, -15,
        // State 24
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 25
        -16, 0, 17, 0, -16, -16, 0, 0, 0, -16, -16, -16, -16, -16,
        // State 26
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 27
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -24,
        -27,
        -17,
        -26,
        -22,
        -25,
        -7,
        -11,
        0,
        0,
        -19,
        -18,
        -9,
        0,
        -13,
        0,
        0,
        0,
        -23,
        0,
        -26,
        -28,
        0,
        -15,
        -10,
        -16,
        -29,
        -30,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 0, 5, 0, 6, 7, 8,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 0, 19, 0, 0, 0, 8,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 2, 20, 0, 21, 0, 0, 0, 0, 0, 8,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 21, 0, 0, 0, 0, 0, 22,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 2, 24, 0, 21, 0, 0, 0, 0, 0, 8,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 21, 0, 0, 0, 0, 0, 25,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 2, 26, 0, 21, 0, 0, 0, 0, 0, 8,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____decimal__i32(__nt), __end));
                0
            }
            2 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____expression(__nt), __end));
                1
            }
            3 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____module(__nt), __end));
                2
            }
            4 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                3
            }
            5 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____program(__nt), __end));
                4
            }
            6 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statement(__nt), __end));
                5
            }
            7 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                return Some(Ok(__nt));
            }
            8 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____term(__nt), __end));
                7
            }
            9 => {
                // decimal_i32 = "decimal_i32" => ActionFn(24);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                8
            }
            10 => {
                // expression = expression, "+", term => ActionFn(16);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            11 => {
                // expression = term => ActionFn(17);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            12 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                10
            }
            13 => {
                // name = "name" => ActionFn(23);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                11
            }
            14 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                12
            }
            15 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            16 => {
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
                13
            }
            17 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            18 => {
                // statement = "\\n" => ActionFn(14);
                let __sym0 = __pop_Term_22_5c_5cn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            19 => {
                // statement = ";" => ActionFn(15);
                let __sym0 = __pop_Term_22_3b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            20 => {
                // statement* =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            21 => {
                // statement* = statement+ => ActionFn(26);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            22 => {
                // statement+ = statement => ActionFn(27);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            23 => {
                // statement+ = statement+, statement => ActionFn(28);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            24 => {
                // statements =  => ActionFn(29);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action29::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            25 => {
                // statements = statement+ => ActionFn(30);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            26 => {
                // term = name => ActionFn(18);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            27 => {
                // term = decimal_i32 => ActionFn(19);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            28 => {
                // term = "-", term => ActionFn(20);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            29 => {
                // term = "(", expression, ")" => ActionFn(21);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            30 => {
                // term = "input", "(", ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
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
    fn __pop_Ntstatement_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2b(__v), __r) => (__l, __v, __r),
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
        Ntstatement_2a(::std::vec::Vec<Statement>),
        Ntstatement_2b(::std::vec::Vec<Statement>),
        Ntstatements(Vec<Statement>),
        Ntterm(Expression),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        5, 0, 0, 0, 6, 0, 0, 0, 0, 0, 7, 8, 9, 0,
        // State 1
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 2
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 3
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 4
        5, 0, 0, 0, 6, 0, 0, 0, 0, 0, 7, 8, 9, 0,
        // State 5
        5, 0, 0, 0, 6, 0, 0, 0, 0, 0, 7, 8, 9, 0,
        // State 6
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 7
        13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 9
        0, 14, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 11
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 12
        0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 14
        5, 0, 0, 0, 6, 0, 0, 0, 0, 0, 7, 8, 9, 0,
        // State 15
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 16
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -27,
        -26,
        -8,
        0,
        0,
        -9,
        0,
        -13,
        0,
        -11,
        -28,
        0,
        -29,
        0,
        -30,
        -10,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 0, 0, 0, 4,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 2, 10, 0, 3, 0, 0, 0, 0, 0, 11,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 0, 0, 0, 12,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 0, 0, 0, 17,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                // __decimal_i32 = decimal_i32 => ActionFn(7);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____decimal__i32(__nt), __end));
                0
            }
            2 => {
                // __expression = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____expression(__nt), __end));
                1
            }
            3 => {
                // __module = module => ActionFn(1);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____module(__nt), __end));
                2
            }
            4 => {
                // __name = name => ActionFn(6);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                3
            }
            5 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____program(__nt), __end));
                4
            }
            6 => {
                // __statement = statement => ActionFn(3);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statement(__nt), __end));
                5
            }
            7 => {
                // __statements = statements => ActionFn(2);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____statements(__nt), __end));
                6
            }
            8 => {
                // __term = term => ActionFn(5);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                return Some(Ok(__nt));
            }
            9 => {
                // decimal_i32 = "decimal_i32" => ActionFn(24);
                let __sym0 = __pop_Term_22decimal__i32_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntdecimal__i32(__nt), __end));
                8
            }
            10 => {
                // expression = expression, "+", term => ActionFn(16);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            11 => {
                // expression = term => ActionFn(17);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                9
            }
            12 => {
                // module = statements => ActionFn(9);
                let __sym0 = __pop_Ntstatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntmodule(__nt), __end));
                10
            }
            13 => {
                // name = "name" => ActionFn(23);
                let __sym0 = __pop_Term_22name_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                11
            }
            14 => {
                // program = module => ActionFn(8);
                let __sym0 = __pop_Ntmodule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                12
            }
            15 => {
                // statement = "print", expression => ActionFn(11);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            16 => {
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
                13
            }
            17 => {
                // statement = expression => ActionFn(13);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            18 => {
                // statement = "\\n" => ActionFn(14);
                let __sym0 = __pop_Term_22_5c_5cn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            19 => {
                // statement = ";" => ActionFn(15);
                let __sym0 = __pop_Term_22_3b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement(__nt), __end));
                13
            }
            20 => {
                // statement* =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            21 => {
                // statement* = statement+ => ActionFn(26);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2a(__nt), __end));
                14
            }
            22 => {
                // statement+ = statement => ActionFn(27);
                let __sym0 = __pop_Ntstatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            23 => {
                // statement+ = statement+, statement => ActionFn(28);
                let __sym1 = __pop_Ntstatement(__symbols);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntstatement_2b(__nt), __end));
                15
            }
            24 => {
                // statements =  => ActionFn(29);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action29::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            25 => {
                // statements = statement+ => ActionFn(30);
                let __sym0 = __pop_Ntstatement_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstatements(__nt), __end));
                16
            }
            26 => {
                // term = name => ActionFn(18);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            27 => {
                // term = decimal_i32 => ActionFn(19);
                let __sym0 = __pop_Ntdecimal__i32(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            28 => {
                // term = "-", term => ActionFn(20);
                let __sym1 = __pop_Ntterm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            29 => {
                // term = "(", expression, ")" => ActionFn(21);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            30 => {
                // term = "input", "(", ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                17
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
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
    fn __pop_Ntstatement_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstatement_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstatement_2b(__v), __r) => (__l, __v, __r),
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
    (_, __0, _): (usize, Token, usize),
) -> Statement
{
    Statement::Newline
}

fn __action15<
>(
    (_, __0, _): (usize, Token, usize),
) -> Statement
{
    Statement::Newline
}

fn __action16<
>(
    (_, left, _): (usize, Expression, usize),
    (_, _, _): (usize, Token, usize),
    (_, right, _): (usize, Expression, usize),
) -> Expression
{
    Expression::Add(left.into(), right.into())
}

fn __action17<
>(
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

fn __action18<
>(
    (_, __0, _): (usize, Name, usize),
) -> Expression
{
    Expression::Name(__0)
}

fn __action19<
>(
    (_, __0, _): (usize, DecimalI32, usize),
) -> Expression
{
    Expression::DecimalI32(__0)
}

fn __action20<
>(
    (_, _, _): (usize, Token, usize),
    (_, t, _): (usize, Expression, usize),
) -> Expression
{
    Expression::UnaryNeg(t.into())
}

fn __action21<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, Expression, usize),
    (_, _, _): (usize, Token, usize),
) -> Expression
{
    (__0)
}

fn __action22<
>(
    (_, __0, _): (usize, Token, usize),
    (_, __1, _): (usize, Token, usize),
    (_, __2, _): (usize, Token, usize),
) -> Expression
{
    Expression::Input(Input)
}

fn __action23<
>(
    (_, s, _): (usize, String, usize),
) -> Name
{
    Name(s)
}

fn __action24<
>(
    (_, i, _): (usize, i32, usize),
) -> DecimalI32
{
    DecimalI32(i)
}

fn __action25<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Statement>
{
    vec![]
}

fn __action26<
>(
    (_, v, _): (usize, ::std::vec::Vec<Statement>, usize),
) -> ::std::vec::Vec<Statement>
{
    v
}

fn __action27<
>(
    (_, __0, _): (usize, Statement, usize),
) -> ::std::vec::Vec<Statement>
{
    vec![__0]
}

fn __action28<
>(
    (_, v, _): (usize, ::std::vec::Vec<Statement>, usize),
    (_, e, _): (usize, Statement, usize),
) -> ::std::vec::Vec<Statement>
{
    { let mut v = v; v.push(e); v }
}

fn __action29<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Statement>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action25(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        __temp0,
    )
}

fn __action30<
>(
    __0: (usize, ::std::vec::Vec<Statement>, usize),
) -> Vec<Statement>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
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
