// auto-generated: "lalrpop 0.14.0"
use spec;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Specification {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use spec;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22depth_22(&'input str),
        Term_22fingerprint_22(&'input str),
        Term_22lengthtag_22(&'input str),
        Term_22name_22(&'input str),
        Term_22size_22(&'input str),
        Term_22t1_22(&'input str),
        Term_22t2_22(&'input str),
        Term_22t4_22(&'input str),
        Term_22t8_22(&'input str),
        Term_22typelength_22(&'input str),
        Term_22version_22(&'input str),
        Termr_23_22_5c_22_2e_2a_5c_22_22_23(&'input str),
        Termr_23_22_5b_2d_5d_3f_5c_5cd_7b1_2c39_7d_22_23(&'input str),
        Termr_23_22_5b0_2d9a_2df_5d_7b40_7d_22_23(&'input str),
        NtDepth(spec::Integer),
        NtFingerprint(spec::Hash),
        NtInteger(spec::Integer),
        NtLengthTag(spec::Tag),
        NtName(String),
        NtSize(spec::Size),
        NtSpecification(spec::Specification),
        NtStringLiteral(String),
        NtTag(spec::Tag),
        NtTypeLength(spec::Integer),
        NtVersion(String),
        Nt____Specification(spec::Specification),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0,
        // State 7
        14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0,
        // State 10
        0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21,
        // State 15
        0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0,
        // State 20
        0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0,
        // State 26
        0, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0,
        // State 27
        -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0,
        // State 31
        0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 39, 40, 41, 42, 0, 0, 0, 0, 0,
        // State 34
        0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        -15,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -7,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        -4,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""depth""###,
            r###""fingerprint""###,
            r###""lengthtag""###,
            r###""name""###,
            r###""size""###,
            r###""t1""###,
            r###""t2""###,
            r###""t4""###,
            r###""t8""###,
            r###""typelength""###,
            r###""version""###,
            r###"r#"\".*\""#"###,
            r###"r#"[-]?\\d{1,39}"#"###,
            r###"r#"[0-9a-f]{40}"#"###,
        ];
        __ACTION[(__state * 16)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    #[allow(dead_code)]
    pub fn parse_Specification<
        'input,
    >(
        input: &'input str,
    ) -> Result<spec::Specification, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let __last_location = &mut Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            *__last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token(3, _) if true => 0,
                Token(4, _) if true => 1,
                Token(5, _) if true => 2,
                Token(6, _) if true => 3,
                Token(7, _) if true => 4,
                Token(8, _) if true => 5,
                Token(9, _) if true => 6,
                Token(10, _) if true => 7,
                Token(11, _) if true => 8,
                Token(12, _) if true => 9,
                Token(13, _) if true => 10,
                Token(14, _) if true => 11,
                Token(15, _) if true => 12,
                Token(0, _) if true => 13,
                Token(1, _) if true => 14,
                Token(2, _) if true => 15,
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
                let __action = __ACTION[__state * 16 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            Token(3, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token(4, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            Token(5, __tok0) => __Symbol::Term_22depth_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token(6, __tok0) => __Symbol::Term_22fingerprint_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token(7, __tok0) => __Symbol::Term_22lengthtag_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token(8, __tok0) => __Symbol::Term_22name_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            Token(9, __tok0) => __Symbol::Term_22size_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            Token(10, __tok0) => __Symbol::Term_22t1_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            Token(11, __tok0) => __Symbol::Term_22t2_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            Token(12, __tok0) => __Symbol::Term_22t4_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token(13, __tok0) => __Symbol::Term_22t8_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            Token(14, __tok0) => __Symbol::Term_22typelength_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token(15, __tok0) => __Symbol::Term_22version_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            Token(0, __tok0) => __Symbol::Termr_23_22_5c_22_2e_2a_5c_22_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            Token(1, __tok0) => __Symbol::Termr_23_22_5b_2d_5d_3f_5c_5cd_7b1_2c39_7d_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            Token(2, __tok0) => __Symbol::Termr_23_22_5b0_2d9a_2df_5d_7b40_7d_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        if r.is_err() {
                            return r;
                        }
                        return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                    }
                } else {
                    let mut __err_lookahead = Some(__lookahead);
                    let mut __err_integer: Option<usize> = Some(__integer);
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
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
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let mut __err_lookahead = None;
                let mut __err_integer: Option<usize> = None;
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: __err_lookahead,
                    expected: __expected_tokens(__state),
                };
                return Err(__error)
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<spec::Specification,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Depth = "(", "depth", Integer, ")" => ActionFn(6);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtInteger(__symbols);
                let __sym1 = __pop_Term_22depth_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtDepth(__nt), __end));
                0
            }
            2 => {
                // Fingerprint = "(", "fingerprint", r#"[0-9a-f]{40}"#, ")" => ActionFn(4);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_Termr_23_22_5b0_2d9a_2df_5d_7b40_7d_22_23(__symbols);
                let __sym1 = __pop_Term_22fingerprint_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtFingerprint(__nt), __end));
                1
            }
            3 => {
                // Integer = r#"[-]?\\d{1,39}"# => ActionFn(9);
                let __sym0 = __pop_Termr_23_22_5b_2d_5d_3f_5c_5cd_7b1_2c39_7d_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInteger(__nt), __end));
                2
            }
            4 => {
                // LengthTag = "(", "lengthtag", Tag, ")" => ActionFn(8);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtTag(__symbols);
                let __sym1 = __pop_Term_22lengthtag_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtLengthTag(__nt), __end));
                3
            }
            5 => {
                // Name = "(", "name", StringLiteral, ")" => ActionFn(2);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtStringLiteral(__symbols);
                let __sym1 = __pop_Term_22name_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtName(__nt), __end));
                4
            }
            6 => {
                // Size = "(", "size", Integer, Integer, ")" => ActionFn(5);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtInteger(__symbols);
                let __sym2 = __pop_NtInteger(__symbols);
                let __sym1 = __pop_Term_22size_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtSize(__nt), __end));
                5
            }
            7 => {
                // Specification = Name, Version, Fingerprint, Size, Depth, TypeLength, LengthTag => ActionFn(1);
                let __sym6 = __pop_NtLengthTag(__symbols);
                let __sym5 = __pop_NtTypeLength(__symbols);
                let __sym4 = __pop_NtDepth(__symbols);
                let __sym3 = __pop_NtSize(__symbols);
                let __sym2 = __pop_NtFingerprint(__symbols);
                let __sym1 = __pop_NtVersion(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtSpecification(__nt), __end));
                6
            }
            8 => {
                // StringLiteral = r#"\".*\""# => ActionFn(14);
                let __sym0 = __pop_Termr_23_22_5c_22_2e_2a_5c_22_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStringLiteral(__nt), __end));
                7
            }
            9 => {
                // Tag = "t1" => ActionFn(10);
                let __sym0 = __pop_Term_22t1_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTag(__nt), __end));
                8
            }
            10 => {
                // Tag = "t2" => ActionFn(11);
                let __sym0 = __pop_Term_22t2_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTag(__nt), __end));
                8
            }
            11 => {
                // Tag = "t4" => ActionFn(12);
                let __sym0 = __pop_Term_22t4_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTag(__nt), __end));
                8
            }
            12 => {
                // Tag = "t8" => ActionFn(13);
                let __sym0 = __pop_Term_22t8_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTag(__nt), __end));
                8
            }
            13 => {
                // TypeLength = "(", "typelength", Integer, ")" => ActionFn(7);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtInteger(__symbols);
                let __sym1 = __pop_Term_22typelength_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTypeLength(__nt), __end));
                9
            }
            14 => {
                // Version = "(", "version", StringLiteral, ")" => ActionFn(3);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtStringLiteral(__symbols);
                let __sym1 = __pop_Term_22version_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtVersion(__nt), __end));
                10
            }
            15 => {
                // __Specification = Specification => ActionFn(0);
                let __sym0 = __pop_NtSpecification(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 12 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22depth_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22depth_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22fingerprint_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22fingerprint_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22lengthtag_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22lengthtag_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22name_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22name_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22size_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22size_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22t1_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22t1_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22t2_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22t2_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22t4_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22t4_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22t8_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22t8_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22typelength_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22typelength_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22version_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22version_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_22_2e_2a_5c_22_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_22_2e_2a_5c_22_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_2d_5d_3f_5c_5cd_7b1_2c39_7d_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_2d_5d_3f_5c_5cd_7b1_2c39_7d_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9a_2df_5d_7b40_7d_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9a_2df_5d_7b40_7d_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDepth<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, spec::Integer, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDepth(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFingerprint<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, spec::Hash, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFingerprint(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtInteger<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, spec::Integer, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInteger(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLengthTag<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, spec::Tag, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLengthTag(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSize<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, spec::Size, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSize(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSpecification<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, spec::Specification, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSpecification(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtStringLiteral<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStringLiteral(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTag<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, spec::Tag, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTag(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTypeLength<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, spec::Integer, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTypeLength(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVersion<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVersion(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Specification<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, spec::Specification, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Specification(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Specification::parse_Specification;
mod __intern_token {
    #![allow(unused_imports)]
    use spec;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    use std::fmt as __fmt;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Token<'input>(pub usize, pub &'input str);
    impl<'a> __fmt::Display for Token<'a> {
        fn fmt(&self, formatter: &mut __fmt::Formatter) -> Result<(), __fmt::Error> {
            __fmt::Display::fmt(self.1, formatter)
        }
    }

    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:\")(?u:.)*(?u:\")",
                "^(?u:[-])?(?u:[0-9٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑑐-𑑙𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𑱐-𑱙𑵐-𑵙𖩠-𖩩𖭐-𖭙𝟎-𝟿𞥐-𞥙]){1, 39}",
                "^(?u:[0-9a-f]){40}",
                "^(?u:\\()",
                "^(?u:\\))",
                "^(?u:depth)",
                "^(?u:fingerprint)",
                "^(?u:lengthtag)",
                "^(?u:name)",
                "^(?u:size)",
                "^(?u:t1)",
                "^(?u:t2)",
                "^(?u:t4)",
                "^(?u:t8)",
                "^(?u:typelength)",
                "^(?u:version)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:\")(?u:.)*(?u:\")").unwrap(),
                __regex::Regex::new("^(?u:[-])?(?u:[0-9٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑑐-𑑙𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𑱐-𑱙𑵐-𑵙𖩠-𖩩𖭐-𖭙𝟎-𝟿𞥐-𞥙]){1, 39}").unwrap(),
                __regex::Regex::new("^(?u:[0-9a-f]){40}").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:depth)").unwrap(),
                __regex::Regex::new("^(?u:fingerprint)").unwrap(),
                __regex::Regex::new("^(?u:lengthtag)").unwrap(),
                __regex::Regex::new("^(?u:name)").unwrap(),
                __regex::Regex::new("^(?u:size)").unwrap(),
                __regex::Regex::new("^(?u:t1)").unwrap(),
                __regex::Regex::new("^(?u:t2)").unwrap(),
                __regex::Regex::new("^(?u:t4)").unwrap(),
                __regex::Regex::new("^(?u:t8)").unwrap(),
                __regex::Regex::new("^(?u:typelength)").unwrap(),
                __regex::Regex::new("^(?u:version)").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize,Token<'input>,&'static str>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 16 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, Token(__index, __result), __end_offset)))
                }
            }
        }
    }
}
pub use self::__intern_token::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, spec::Specification, usize),
) -> spec::Specification
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, name, _): (usize, String, usize),
    (_, version, _): (usize, String, usize),
    (_, fingerprint, _): (usize, spec::Hash, usize),
    (_, size, _): (usize, spec::Size, usize),
    (_, depth, _): (usize, spec::Integer, usize),
    (_, type_length, _): (usize, spec::Integer, usize),
    (_, length_tag, _): (usize, spec::Tag, usize),
) -> spec::Specification
{
    spec::Specification{ name:name, version:version, fingerprint:fingerprint, size:size, depth:depth, type_length:type_length, length_tag:length_tag }
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    s
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    s
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, &'input str, usize),
    (_, __3, _): (usize, &'input str, usize),
) -> spec::Hash
{
    __0, __1, __2, __3.into()
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, min, _): (usize, spec::Integer, usize),
    (_, max, _): (usize, spec::Integer, usize),
    (_, _, _): (usize, &'input str, usize),
) -> spec::Size
{
    spec::Size{ min:min, max:max }
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, spec::Integer, usize),
    (_, __3, _): (usize, &'input str, usize),
) -> spec::Integer
{
    __0, __1, __2, __3
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, spec::Integer, usize),
    (_, __3, _): (usize, &'input str, usize),
) -> spec::Integer
{
    __0, __1, __2, __3
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, spec::Tag, usize),
    (_, __3, _): (usize, &'input str, usize),
) -> spec::Tag
{
    __0, __1, __2, __3
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
) -> spec::Integer
{
    spec::Integer::from_str(i).unwrap()
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> spec::Tag
{
    spec::Tag::T1
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> spec::Tag
{
    spec::Tag::T2
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> spec::Tag
{
    spec::Tag::T4
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> spec::Tag
{
    spec::Tag::T8
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize),&'static str> {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        value
    }
}
