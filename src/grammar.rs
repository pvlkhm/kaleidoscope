// auto-generated: "lalrpop 0.20.0"
// sha3: db451173d02598fb0d6ef0f882b10f79e50b2c9fa0ea9c74c6d7b9e497e039bf
use crate::lexer::*;
use crate::parser::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]
mod __parse__ASTExpr {

    use crate::lexer::*;
    use crate::parser::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'a>
     {
        Variant0(Token<'a>),
        Variant1(ASTExpr),
        Variant2(alloc::vec::Vec<ASTExpr>),
        Variant3(ASTIdentifier),
        Variant4(alloc::vec::Vec<ASTIdentifier>),
        Variant5(core::option::Option<ASTExpr>),
        Variant6(ASTFile),
        Variant7(ASTFn),
        Variant8(alloc::vec::Vec<ASTFn>),
        Variant9(core::option::Option<ASTIdentifier>),
        Variant10(Vec<ASTExpr>),
        Variant11(Vec<ASTIdentifier>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 6, 7, 0,
        // State 1
        0, -27, 0, 0, 6, 7, 0,
        // State 2
        0, -29, 0, 0, 6, 7, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0,
        // State 4
        2, -13, -13, 0, 0, 0, 0,
        // State 5
        -23, -23, -23, 0, 0, 0, 0,
        // State 6
        0, -11, -11, 0, 0, 0, 0,
        // State 7
        0, -26, 11, 0, 0, 0, 0,
        // State 8
        0, 12, 0, 0, 0, 0, 0,
        // State 9
        0, -28, 13, 0, 0, 0, 0,
        // State 10
        0, -4, 0, 0, -4, -4, 0,
        // State 11
        0, -12, -12, 0, 0, 0, 0,
        // State 12
        0, -5, 0, 0, -5, -5, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 7 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        -34,
        // State 4
        -13,
        // State 5
        -23,
        // State 6
        -11,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        -12,
        // State 12
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 2,
            6 => match state {
                1 => 7,
                2 => 9,
                _ => 3,
            },
            12 => 4,
            14 => 8,
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""(""###,
        r###"")""###,
        r###"",""###,
        r###""fn""###,
        r###""id""###,
        r###""num""###,
        r###""op""###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'a,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'a ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'a>
    where 
    {
        __phantom: core::marker::PhantomData<(&'a ())>,
    }
    impl<'a> __state_machine::ParserDefinition for __StateMachine<'a>
    where 
    {
        type Location = usize;
        type Error = usize;
        type Token = Token<'a>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'a>;
        type Success = ASTExpr;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 7 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&())>)
        }
    }
    fn __token_to_integer<
        'a,
    >(
        __token: &Token<'a>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token::OpenPar if true => Some(0),
            Token::ClosePar if true => Some(1),
            Token::Comma if true => Some(2),
            Token::Fn if true => Some(3),
            Token::Identifier(_) if true => Some(4),
            Token::Num(_) if true => Some(5),
            Token::Operation(_) if true => Some(6),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'a,
    >(
        __token_index: usize,
        __token: Token<'a>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> __Symbol<'a>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 => __Symbol::Variant0(__token),
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'a,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'a>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 6,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 8,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 9,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 13,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 14,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 14,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 15,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            33 => __state_machine::SimulatedReduce::Accept,
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ASTExprParser {
        _priv: (),
    }

    impl ASTExprParser {
        pub fn new() -> ASTExprParser {
            ASTExprParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'a,
            __TOKEN: __ToTriple<'a, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<ASTExpr, __lalrpop_util::ParseError<usize, Token<'a>, usize>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'a,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'a,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> Option<Result<ASTExpr,__lalrpop_util::ParseError<usize, Token<'a>, usize>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                // __ASTExpr = ASTExpr => ActionFn(3);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action3::<>(__sym0);
                return Some(Ok(__nt));
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant1<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTExpr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTFile, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTFn, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTIdentifier, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, Token<'a>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, Vec<ASTExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, Vec<ASTIdentifier>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, alloc::vec::Vec<ASTExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, alloc::vec::Vec<ASTFn>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, alloc::vec::Vec<ASTIdentifier>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, core::option::Option<ASTExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, core::option::Option<ASTIdentifier>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",") = ASTExpr, "," => ActionFn(25);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action25::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")* =  => ActionFn(23);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action23::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")* = (<ASTExpr> ",")+ => ActionFn(24);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")+ = ASTExpr, "," => ActionFn(30);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action30::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")+ = (<ASTExpr> ",")+, ASTExpr, "," => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",") = ASTIdentifier, "," => ActionFn(20);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action20::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")* =  => ActionFn(18);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action18::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")* = (<ASTIdentifier> ",")+ => ActionFn(19);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")+ = ASTIdentifier, "," => ActionFn(34);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action34::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")+ = (<ASTIdentifier> ",")+, ASTIdentifier, "," => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr = "num" => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr = ASTIdentifier, "(", Comma<ASTExpr>, ")" => ActionFn(8);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action8::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 6)
    }
    pub(crate) fn __reduce12<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr = ASTIdentifier => ActionFn(9);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce13<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr? = ASTExpr => ActionFn(21);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce14<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr? =  => ActionFn(22);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action22::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce15<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFile =  => ActionFn(42);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action42::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce16<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFile = ASTFn+ => ActionFn(43);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce17<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn = "fn", ASTIdentifier, "(", Comma<ASTIdentifier>, ")", ASTExpr => ActionFn(5);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant1(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action5::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (6, 9)
    }
    pub(crate) fn __reduce18<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn* =  => ActionFn(12);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action12::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce19<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn* = ASTFn+ => ActionFn(13);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action13::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce20<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn+ = ASTFn => ActionFn(14);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce21<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn+ = ASTFn+, ASTFn => ActionFn(15);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action15::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce22<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTIdentifier = "id" => ActionFn(6);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce23<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTIdentifier? = ASTIdentifier => ActionFn(16);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce24<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTIdentifier? =  => ActionFn(17);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action17::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 13)
    }
    pub(crate) fn __reduce25<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> = ASTExpr => ActionFn(38);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce26<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> =  => ActionFn(39);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action39::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce27<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> = (<ASTExpr> ",")+, ASTExpr => ActionFn(40);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action40::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce28<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> = (<ASTExpr> ",")+ => ActionFn(41);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action41::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce29<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> = ASTIdentifier => ActionFn(44);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action44::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce30<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> =  => ActionFn(45);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action45::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 15)
    }
    pub(crate) fn __reduce31<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> = (<ASTIdentifier> ",")+, ASTIdentifier => ActionFn(46);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action46::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 15)
    }
    pub(crate) fn __reduce32<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> = (<ASTIdentifier> ",")+ => ActionFn(47);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce34<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // __ASTFile = ASTFile => ActionFn(0);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce35<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // __ASTFn = ASTFn => ActionFn(1);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce36<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // __ASTIdentifier = ASTIdentifier => ActionFn(2);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 19)
    }
}
pub use self::__parse__ASTExpr::ASTExprParser;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]
mod __parse__ASTFile {

    use crate::lexer::*;
    use crate::parser::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'a>
     {
        Variant0(Token<'a>),
        Variant1(ASTExpr),
        Variant2(alloc::vec::Vec<ASTExpr>),
        Variant3(ASTIdentifier),
        Variant4(alloc::vec::Vec<ASTIdentifier>),
        Variant5(core::option::Option<ASTExpr>),
        Variant6(ASTFile),
        Variant7(ASTFn),
        Variant8(alloc::vec::Vec<ASTFn>),
        Variant9(core::option::Option<ASTIdentifier>),
        Variant10(Vec<ASTExpr>),
        Variant11(Vec<ASTIdentifier>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 3, 0, 0, 0,
        // State 1
        0, 0, 0, 3, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 13, 0, 0,
        // State 3
        0, -31, 0, 0, 13, 0, 0,
        // State 4
        0, -33, 0, 0, 13, 0, 0,
        // State 5
        0, 0, 0, 0, 13, 21, 0,
        // State 6
        0, -27, 0, 0, 13, 21, 0,
        // State 7
        0, -29, 0, 0, 13, 21, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, -21, 0, 0, 0,
        // State 10
        0, 0, 0, -22, 0, 0, 0,
        // State 11
        4, 0, 0, 0, 0, 0, 0,
        // State 12
        -23, -23, -23, -23, 0, 0, 0,
        // State 13
        0, -30, 17, 0, 0, 0, 0,
        // State 14
        0, 6, 0, 0, 0, 0, 0,
        // State 15
        0, -32, 18, 0, 0, 0, 0,
        // State 16
        0, -9, 0, 0, -9, 0, 0,
        // State 17
        0, -10, 0, 0, -10, 0, 0,
        // State 18
        0, 0, 0, -18, 0, 0, 0,
        // State 19
        7, -13, -13, -13, 0, 0, 0,
        // State 20
        0, -11, -11, -11, 0, 0, 0,
        // State 21
        0, -26, 25, 0, 0, 0, 0,
        // State 22
        0, 26, 0, 0, 0, 0, 0,
        // State 23
        0, -28, 27, 0, 0, 0, 0,
        // State 24
        0, -4, 0, 0, -4, -4, 0,
        // State 25
        0, -12, -12, -12, 0, 0, 0,
        // State 26
        0, -5, 0, 0, -5, -5, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 7 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -16,
        // State 1
        -17,
        // State 2
        0,
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
        -35,
        // State 9
        -21,
        // State 10
        -22,
        // State 11
        0,
        // State 12
        -23,
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
        -18,
        // State 19
        -13,
        // State 20
        -11,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        -12,
        // State 26
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 7,
            5 => 4,
            6 => match state {
                6 => 21,
                7 => 23,
                _ => 18,
            },
            8 => 8,
            9 => match state {
                1 => 10,
                _ => 9,
            },
            11 => 1,
            12 => match state {
                3 => 13,
                4 => 15,
                5..=7 => 19,
                _ => 11,
            },
            14 => 22,
            15 => 14,
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""(""###,
        r###"")""###,
        r###"",""###,
        r###""fn""###,
        r###""id""###,
        r###""num""###,
        r###""op""###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'a,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'a ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'a>
    where 
    {
        __phantom: core::marker::PhantomData<(&'a ())>,
    }
    impl<'a> __state_machine::ParserDefinition for __StateMachine<'a>
    where 
    {
        type Location = usize;
        type Error = usize;
        type Token = Token<'a>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'a>;
        type Success = ASTFile;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 7 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&())>)
        }
    }
    fn __token_to_integer<
        'a,
    >(
        __token: &Token<'a>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token::OpenPar if true => Some(0),
            Token::ClosePar if true => Some(1),
            Token::Comma if true => Some(2),
            Token::Fn if true => Some(3),
            Token::Identifier(_) if true => Some(4),
            Token::Num(_) if true => Some(5),
            Token::Operation(_) if true => Some(6),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'a,
    >(
        __token_index: usize,
        __token: Token<'a>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> __Symbol<'a>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 => __Symbol::Variant0(__token),
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'a,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'a>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 6,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 8,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 9,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 13,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 14,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 14,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 15,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            34 => __state_machine::SimulatedReduce::Accept,
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ASTFileParser {
        _priv: (),
    }

    impl ASTFileParser {
        pub fn new() -> ASTFileParser {
            ASTFileParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'a,
            __TOKEN: __ToTriple<'a, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<ASTFile, __lalrpop_util::ParseError<usize, Token<'a>, usize>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'a,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'a,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> Option<Result<ASTFile,__lalrpop_util::ParseError<usize, Token<'a>, usize>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                // __ASTFile = ASTFile => ActionFn(0);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant1<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTExpr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTFile, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTFn, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTIdentifier, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, Token<'a>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, Vec<ASTExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, Vec<ASTIdentifier>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, alloc::vec::Vec<ASTExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, alloc::vec::Vec<ASTFn>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, alloc::vec::Vec<ASTIdentifier>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, core::option::Option<ASTExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, core::option::Option<ASTIdentifier>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",") = ASTExpr, "," => ActionFn(25);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action25::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")* =  => ActionFn(23);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action23::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")* = (<ASTExpr> ",")+ => ActionFn(24);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")+ = ASTExpr, "," => ActionFn(30);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action30::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")+ = (<ASTExpr> ",")+, ASTExpr, "," => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",") = ASTIdentifier, "," => ActionFn(20);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action20::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")* =  => ActionFn(18);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action18::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")* = (<ASTIdentifier> ",")+ => ActionFn(19);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")+ = ASTIdentifier, "," => ActionFn(34);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action34::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")+ = (<ASTIdentifier> ",")+, ASTIdentifier, "," => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr = "num" => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr = ASTIdentifier, "(", Comma<ASTExpr>, ")" => ActionFn(8);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action8::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 6)
    }
    pub(crate) fn __reduce12<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr = ASTIdentifier => ActionFn(9);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce13<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr? = ASTExpr => ActionFn(21);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce14<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr? =  => ActionFn(22);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action22::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce15<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFile =  => ActionFn(42);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action42::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce16<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFile = ASTFn+ => ActionFn(43);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce17<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn = "fn", ASTIdentifier, "(", Comma<ASTIdentifier>, ")", ASTExpr => ActionFn(5);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant1(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action5::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (6, 9)
    }
    pub(crate) fn __reduce18<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn* =  => ActionFn(12);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action12::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce19<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn* = ASTFn+ => ActionFn(13);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action13::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce20<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn+ = ASTFn => ActionFn(14);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce21<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn+ = ASTFn+, ASTFn => ActionFn(15);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action15::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce22<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTIdentifier = "id" => ActionFn(6);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce23<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTIdentifier? = ASTIdentifier => ActionFn(16);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce24<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTIdentifier? =  => ActionFn(17);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action17::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 13)
    }
    pub(crate) fn __reduce25<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> = ASTExpr => ActionFn(38);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce26<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> =  => ActionFn(39);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action39::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce27<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> = (<ASTExpr> ",")+, ASTExpr => ActionFn(40);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action40::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce28<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> = (<ASTExpr> ",")+ => ActionFn(41);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action41::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce29<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> = ASTIdentifier => ActionFn(44);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action44::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce30<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> =  => ActionFn(45);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action45::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 15)
    }
    pub(crate) fn __reduce31<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> = (<ASTIdentifier> ",")+, ASTIdentifier => ActionFn(46);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action46::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 15)
    }
    pub(crate) fn __reduce32<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> = (<ASTIdentifier> ",")+ => ActionFn(47);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce33<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // __ASTExpr = ASTExpr => ActionFn(3);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce35<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // __ASTFn = ASTFn => ActionFn(1);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce36<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // __ASTIdentifier = ASTIdentifier => ActionFn(2);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 19)
    }
}
pub use self::__parse__ASTFile::ASTFileParser;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]
mod __parse__ASTFn {

    use crate::lexer::*;
    use crate::parser::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'a>
     {
        Variant0(Token<'a>),
        Variant1(ASTExpr),
        Variant2(alloc::vec::Vec<ASTExpr>),
        Variant3(ASTIdentifier),
        Variant4(alloc::vec::Vec<ASTIdentifier>),
        Variant5(core::option::Option<ASTExpr>),
        Variant6(ASTFile),
        Variant7(ASTFn),
        Variant8(alloc::vec::Vec<ASTFn>),
        Variant9(core::option::Option<ASTIdentifier>),
        Variant10(Vec<ASTExpr>),
        Variant11(Vec<ASTIdentifier>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 2, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 10, 0, 0,
        // State 2
        0, -31, 0, 0, 10, 0, 0,
        // State 3
        0, -33, 0, 0, 10, 0, 0,
        // State 4
        0, 0, 0, 0, 10, 18, 0,
        // State 5
        0, -27, 0, 0, 10, 18, 0,
        // State 6
        0, -29, 0, 0, 10, 18, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0,
        // State 8
        3, 0, 0, 0, 0, 0, 0,
        // State 9
        -23, -23, -23, 0, 0, 0, 0,
        // State 10
        0, -30, 14, 0, 0, 0, 0,
        // State 11
        0, 5, 0, 0, 0, 0, 0,
        // State 12
        0, -32, 15, 0, 0, 0, 0,
        // State 13
        0, -9, 0, 0, -9, 0, 0,
        // State 14
        0, -10, 0, 0, -10, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0,
        // State 16
        6, -13, -13, 0, 0, 0, 0,
        // State 17
        0, -11, -11, 0, 0, 0, 0,
        // State 18
        0, -26, 22, 0, 0, 0, 0,
        // State 19
        0, 23, 0, 0, 0, 0, 0,
        // State 20
        0, -28, 24, 0, 0, 0, 0,
        // State 21
        0, -4, 0, 0, -4, -4, 0,
        // State 22
        0, -12, -12, 0, 0, 0, 0,
        // State 23
        0, -5, 0, 0, -5, -5, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 7 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        -36,
        // State 8
        0,
        // State 9
        -23,
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
        -18,
        // State 16
        -13,
        // State 17
        -11,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        -12,
        // State 23
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 6,
            5 => 3,
            6 => match state {
                5 => 18,
                6 => 20,
                _ => 15,
            },
            9 => 7,
            12 => match state {
                2 => 10,
                3 => 12,
                4..=6 => 16,
                _ => 8,
            },
            14 => 19,
            15 => 11,
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""(""###,
        r###"")""###,
        r###"",""###,
        r###""fn""###,
        r###""id""###,
        r###""num""###,
        r###""op""###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'a,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'a ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'a>
    where 
    {
        __phantom: core::marker::PhantomData<(&'a ())>,
    }
    impl<'a> __state_machine::ParserDefinition for __StateMachine<'a>
    where 
    {
        type Location = usize;
        type Error = usize;
        type Token = Token<'a>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'a>;
        type Success = ASTFn;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 7 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&())>)
        }
    }
    fn __token_to_integer<
        'a,
    >(
        __token: &Token<'a>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token::OpenPar if true => Some(0),
            Token::ClosePar if true => Some(1),
            Token::Comma if true => Some(2),
            Token::Fn if true => Some(3),
            Token::Identifier(_) if true => Some(4),
            Token::Num(_) if true => Some(5),
            Token::Operation(_) if true => Some(6),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'a,
    >(
        __token_index: usize,
        __token: Token<'a>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> __Symbol<'a>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 => __Symbol::Variant0(__token),
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'a,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'a>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 6,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 8,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 9,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 13,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 14,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 14,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 15,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            35 => __state_machine::SimulatedReduce::Accept,
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ASTFnParser {
        _priv: (),
    }

    impl ASTFnParser {
        pub fn new() -> ASTFnParser {
            ASTFnParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'a,
            __TOKEN: __ToTriple<'a, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<ASTFn, __lalrpop_util::ParseError<usize, Token<'a>, usize>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'a,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'a,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> Option<Result<ASTFn,__lalrpop_util::ParseError<usize, Token<'a>, usize>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                // __ASTFn = ASTFn => ActionFn(1);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action1::<>(__sym0);
                return Some(Ok(__nt));
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant1<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTExpr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTFile, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTFn, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTIdentifier, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, Token<'a>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, Vec<ASTExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, Vec<ASTIdentifier>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, alloc::vec::Vec<ASTExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, alloc::vec::Vec<ASTFn>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, alloc::vec::Vec<ASTIdentifier>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, core::option::Option<ASTExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, core::option::Option<ASTIdentifier>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",") = ASTExpr, "," => ActionFn(25);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action25::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")* =  => ActionFn(23);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action23::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")* = (<ASTExpr> ",")+ => ActionFn(24);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")+ = ASTExpr, "," => ActionFn(30);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action30::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")+ = (<ASTExpr> ",")+, ASTExpr, "," => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",") = ASTIdentifier, "," => ActionFn(20);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action20::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")* =  => ActionFn(18);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action18::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")* = (<ASTIdentifier> ",")+ => ActionFn(19);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")+ = ASTIdentifier, "," => ActionFn(34);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action34::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")+ = (<ASTIdentifier> ",")+, ASTIdentifier, "," => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr = "num" => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr = ASTIdentifier, "(", Comma<ASTExpr>, ")" => ActionFn(8);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action8::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 6)
    }
    pub(crate) fn __reduce12<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr = ASTIdentifier => ActionFn(9);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce13<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr? = ASTExpr => ActionFn(21);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce14<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr? =  => ActionFn(22);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action22::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce15<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFile =  => ActionFn(42);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action42::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce16<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFile = ASTFn+ => ActionFn(43);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce17<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn = "fn", ASTIdentifier, "(", Comma<ASTIdentifier>, ")", ASTExpr => ActionFn(5);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant1(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action5::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (6, 9)
    }
    pub(crate) fn __reduce18<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn* =  => ActionFn(12);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action12::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce19<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn* = ASTFn+ => ActionFn(13);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action13::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce20<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn+ = ASTFn => ActionFn(14);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce21<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn+ = ASTFn+, ASTFn => ActionFn(15);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action15::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce22<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTIdentifier = "id" => ActionFn(6);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce23<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTIdentifier? = ASTIdentifier => ActionFn(16);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce24<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTIdentifier? =  => ActionFn(17);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action17::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 13)
    }
    pub(crate) fn __reduce25<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> = ASTExpr => ActionFn(38);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce26<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> =  => ActionFn(39);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action39::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce27<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> = (<ASTExpr> ",")+, ASTExpr => ActionFn(40);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action40::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce28<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> = (<ASTExpr> ",")+ => ActionFn(41);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action41::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce29<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> = ASTIdentifier => ActionFn(44);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action44::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce30<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> =  => ActionFn(45);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action45::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 15)
    }
    pub(crate) fn __reduce31<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> = (<ASTIdentifier> ",")+, ASTIdentifier => ActionFn(46);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action46::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 15)
    }
    pub(crate) fn __reduce32<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> = (<ASTIdentifier> ",")+ => ActionFn(47);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce33<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // __ASTExpr = ASTExpr => ActionFn(3);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce34<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // __ASTFile = ASTFile => ActionFn(0);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce36<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // __ASTIdentifier = ASTIdentifier => ActionFn(2);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 19)
    }
}
pub use self::__parse__ASTFn::ASTFnParser;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]
mod __parse__ASTIdentifier {

    use crate::lexer::*;
    use crate::parser::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'a>
     {
        Variant0(Token<'a>),
        Variant1(ASTExpr),
        Variant2(alloc::vec::Vec<ASTExpr>),
        Variant3(ASTIdentifier),
        Variant4(alloc::vec::Vec<ASTIdentifier>),
        Variant5(core::option::Option<ASTExpr>),
        Variant6(ASTFile),
        Variant7(ASTFn),
        Variant8(alloc::vec::Vec<ASTFn>),
        Variant9(core::option::Option<ASTIdentifier>),
        Variant10(Vec<ASTExpr>),
        Variant11(Vec<ASTIdentifier>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 3, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 7 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -37,
        // State 2
        -23,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            12 => 1,
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""(""###,
        r###"")""###,
        r###"",""###,
        r###""fn""###,
        r###""id""###,
        r###""num""###,
        r###""op""###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'a,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'a ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'a>
    where 
    {
        __phantom: core::marker::PhantomData<(&'a ())>,
    }
    impl<'a> __state_machine::ParserDefinition for __StateMachine<'a>
    where 
    {
        type Location = usize;
        type Error = usize;
        type Token = Token<'a>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'a>;
        type Success = ASTIdentifier;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 7 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&())>)
        }
    }
    fn __token_to_integer<
        'a,
    >(
        __token: &Token<'a>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token::OpenPar if true => Some(0),
            Token::ClosePar if true => Some(1),
            Token::Comma if true => Some(2),
            Token::Fn if true => Some(3),
            Token::Identifier(_) if true => Some(4),
            Token::Num(_) if true => Some(5),
            Token::Operation(_) if true => Some(6),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'a,
    >(
        __token_index: usize,
        __token: Token<'a>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> __Symbol<'a>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 => __Symbol::Variant0(__token),
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'a,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'a>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 6,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 8,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 9,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 13,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 14,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 14,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 15,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            36 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ASTIdentifierParser {
        _priv: (),
    }

    impl ASTIdentifierParser {
        pub fn new() -> ASTIdentifierParser {
            ASTIdentifierParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'a,
            __TOKEN: __ToTriple<'a, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<ASTIdentifier, __lalrpop_util::ParseError<usize, Token<'a>, usize>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'a,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'a,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> Option<Result<ASTIdentifier,__lalrpop_util::ParseError<usize, Token<'a>, usize>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                // __ASTIdentifier = ASTIdentifier => ActionFn(2);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action2::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant1<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTExpr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTFile, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTFn, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, ASTIdentifier, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, Token<'a>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, Vec<ASTExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, Vec<ASTIdentifier>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, alloc::vec::Vec<ASTExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, alloc::vec::Vec<ASTFn>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, alloc::vec::Vec<ASTIdentifier>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, core::option::Option<ASTExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'a,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>
    ) -> (usize, core::option::Option<ASTIdentifier>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",") = ASTExpr, "," => ActionFn(25);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action25::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")* =  => ActionFn(23);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action23::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")* = (<ASTExpr> ",")+ => ActionFn(24);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")+ = ASTExpr, "," => ActionFn(30);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action30::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTExpr> ",")+ = (<ASTExpr> ",")+, ASTExpr, "," => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",") = ASTIdentifier, "," => ActionFn(20);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action20::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")* =  => ActionFn(18);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action18::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")* = (<ASTIdentifier> ",")+ => ActionFn(19);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")+ = ASTIdentifier, "," => ActionFn(34);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action34::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // (<ASTIdentifier> ",")+ = (<ASTIdentifier> ",")+, ASTIdentifier, "," => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr = "num" => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr = ASTIdentifier, "(", Comma<ASTExpr>, ")" => ActionFn(8);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action8::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 6)
    }
    pub(crate) fn __reduce12<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr = ASTIdentifier => ActionFn(9);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce13<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr? = ASTExpr => ActionFn(21);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce14<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTExpr? =  => ActionFn(22);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action22::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce15<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFile =  => ActionFn(42);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action42::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce16<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFile = ASTFn+ => ActionFn(43);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce17<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn = "fn", ASTIdentifier, "(", Comma<ASTIdentifier>, ")", ASTExpr => ActionFn(5);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant1(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action5::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (6, 9)
    }
    pub(crate) fn __reduce18<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn* =  => ActionFn(12);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action12::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce19<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn* = ASTFn+ => ActionFn(13);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action13::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce20<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn+ = ASTFn => ActionFn(14);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce21<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTFn+ = ASTFn+, ASTFn => ActionFn(15);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action15::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce22<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTIdentifier = "id" => ActionFn(6);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce23<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTIdentifier? = ASTIdentifier => ActionFn(16);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce24<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // ASTIdentifier? =  => ActionFn(17);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action17::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 13)
    }
    pub(crate) fn __reduce25<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> = ASTExpr => ActionFn(38);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce26<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> =  => ActionFn(39);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action39::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce27<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> = (<ASTExpr> ",")+, ASTExpr => ActionFn(40);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action40::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce28<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTExpr> = (<ASTExpr> ",")+ => ActionFn(41);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action41::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce29<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> = ASTIdentifier => ActionFn(44);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action44::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce30<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> =  => ActionFn(45);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action45::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 15)
    }
    pub(crate) fn __reduce31<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> = (<ASTIdentifier> ",")+, ASTIdentifier => ActionFn(46);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action46::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 15)
    }
    pub(crate) fn __reduce32<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // Comma<ASTIdentifier> = (<ASTIdentifier> ",")+ => ActionFn(47);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce33<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // __ASTExpr = ASTExpr => ActionFn(3);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce34<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // __ASTFile = ASTFile => ActionFn(0);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce35<
        'a,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'a>,usize)>,
        _: core::marker::PhantomData<(&'a ())>,
    ) -> (usize, usize)
    {
        // __ASTFn = ASTFn => ActionFn(1);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
}
pub use self::__parse__ASTIdentifier::ASTIdentifierParser;

#[allow(clippy::too_many_arguments)]
fn __action0<
    'a,
>(
    (_, __0, _): (usize, ASTFile, usize),
) -> ASTFile
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action1<
    'a,
>(
    (_, __0, _): (usize, ASTFn, usize),
) -> ASTFn
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action2<
    'a,
>(
    (_, __0, _): (usize, ASTIdentifier, usize),
) -> ASTIdentifier
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action3<
    'a,
>(
    (_, __0, _): (usize, ASTExpr, usize),
) -> ASTExpr
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action4<
    'a,
>(
    (_, fns, _): (usize, alloc::vec::Vec<ASTFn>, usize),
) -> ASTFile
{
    ASTFile { fns: fns }
}

#[allow(clippy::too_many_arguments)]
fn __action5<
    'a,
>(
    (_, _, _): (usize, Token<'a>, usize),
    (_, name, _): (usize, ASTIdentifier, usize),
    (_, _, _): (usize, Token<'a>, usize),
    (_, args, _): (usize, Vec<ASTIdentifier>, usize),
    (_, _, _): (usize, Token<'a>, usize),
    (_, expr, _): (usize, ASTExpr, usize),
) -> ASTFn
{
    ASTFn {
        decl: ASTFnDecl { name: name, args: args },
        body: expr
    }
}

#[allow(clippy::too_many_arguments)]
fn __action6<
    'a,
>(
    (_, __0, _): (usize, Token<'a>, usize),
) -> ASTIdentifier
{
    ASTIdentifier { name: __0.as_string() }
}

#[allow(clippy::too_many_arguments)]
fn __action7<
    'a,
>(
    (_, __0, _): (usize, Token<'a>, usize),
) -> ASTExpr
{
    ASTExpr::Literal(__0.as_f64())
}

#[allow(clippy::too_many_arguments)]
fn __action8<
    'a,
>(
    (_, fn_name, _): (usize, ASTIdentifier, usize),
    (_, _, _): (usize, Token<'a>, usize),
    (_, fn_args, _): (usize, Vec<ASTExpr>, usize),
    (_, _, _): (usize, Token<'a>, usize),
) -> ASTExpr
{
    ASTExpr::FnCall(ASTFnCall {
        name: fn_name,
        args: fn_args
    })
}

#[allow(clippy::too_many_arguments)]
fn __action9<
    'a,
>(
    (_, __0, _): (usize, ASTIdentifier, usize),
) -> ASTExpr
{
    ASTExpr::Var(__0)
}

#[allow(clippy::too_many_arguments)]
fn __action10<
    'a,
>(
    (_, mut v, _): (usize, alloc::vec::Vec<ASTExpr>, usize),
    (_, e, _): (usize, core::option::Option<ASTExpr>, usize),
) -> Vec<ASTExpr>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn __action11<
    'a,
>(
    (_, mut v, _): (usize, alloc::vec::Vec<ASTIdentifier>, usize),
    (_, e, _): (usize, core::option::Option<ASTIdentifier>, usize),
) -> Vec<ASTIdentifier>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn __action12<
    'a,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ASTFn>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments)]
fn __action13<
    'a,
>(
    (_, v, _): (usize, alloc::vec::Vec<ASTFn>, usize),
) -> alloc::vec::Vec<ASTFn>
{
    v
}

#[allow(clippy::too_many_arguments)]
fn __action14<
    'a,
>(
    (_, __0, _): (usize, ASTFn, usize),
) -> alloc::vec::Vec<ASTFn>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments)]
fn __action15<
    'a,
>(
    (_, v, _): (usize, alloc::vec::Vec<ASTFn>, usize),
    (_, e, _): (usize, ASTFn, usize),
) -> alloc::vec::Vec<ASTFn>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments)]
fn __action16<
    'a,
>(
    (_, __0, _): (usize, ASTIdentifier, usize),
) -> core::option::Option<ASTIdentifier>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments)]
fn __action17<
    'a,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ASTIdentifier>
{
    None
}

#[allow(clippy::too_many_arguments)]
fn __action18<
    'a,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ASTIdentifier>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments)]
fn __action19<
    'a,
>(
    (_, v, _): (usize, alloc::vec::Vec<ASTIdentifier>, usize),
) -> alloc::vec::Vec<ASTIdentifier>
{
    v
}

#[allow(clippy::too_many_arguments)]
fn __action20<
    'a,
>(
    (_, __0, _): (usize, ASTIdentifier, usize),
    (_, _, _): (usize, Token<'a>, usize),
) -> ASTIdentifier
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action21<
    'a,
>(
    (_, __0, _): (usize, ASTExpr, usize),
) -> core::option::Option<ASTExpr>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments)]
fn __action22<
    'a,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ASTExpr>
{
    None
}

#[allow(clippy::too_many_arguments)]
fn __action23<
    'a,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ASTExpr>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments)]
fn __action24<
    'a,
>(
    (_, v, _): (usize, alloc::vec::Vec<ASTExpr>, usize),
) -> alloc::vec::Vec<ASTExpr>
{
    v
}

#[allow(clippy::too_many_arguments)]
fn __action25<
    'a,
>(
    (_, __0, _): (usize, ASTExpr, usize),
    (_, _, _): (usize, Token<'a>, usize),
) -> ASTExpr
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action26<
    'a,
>(
    (_, __0, _): (usize, ASTExpr, usize),
) -> alloc::vec::Vec<ASTExpr>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments)]
fn __action27<
    'a,
>(
    (_, v, _): (usize, alloc::vec::Vec<ASTExpr>, usize),
    (_, e, _): (usize, ASTExpr, usize),
) -> alloc::vec::Vec<ASTExpr>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments)]
fn __action28<
    'a,
>(
    (_, __0, _): (usize, ASTIdentifier, usize),
) -> alloc::vec::Vec<ASTIdentifier>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments)]
fn __action29<
    'a,
>(
    (_, v, _): (usize, alloc::vec::Vec<ASTIdentifier>, usize),
    (_, e, _): (usize, ASTIdentifier, usize),
) -> alloc::vec::Vec<ASTIdentifier>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments)]
fn __action30<
    'a,
>(
    __0: (usize, ASTExpr, usize),
    __1: (usize, Token<'a>, usize),
) -> alloc::vec::Vec<ASTExpr>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action25(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action31<
    'a,
>(
    __0: (usize, alloc::vec::Vec<ASTExpr>, usize),
    __1: (usize, ASTExpr, usize),
    __2: (usize, Token<'a>, usize),
) -> alloc::vec::Vec<ASTExpr>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
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

#[allow(clippy::too_many_arguments)]
fn __action32<
    'a,
>(
    __0: (usize, core::option::Option<ASTExpr>, usize),
) -> Vec<ASTExpr>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action23(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        __temp0,
        __0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action33<
    'a,
>(
    __0: (usize, alloc::vec::Vec<ASTExpr>, usize),
    __1: (usize, core::option::Option<ASTExpr>, usize),
) -> Vec<ASTExpr>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action24(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        __temp0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action34<
    'a,
>(
    __0: (usize, ASTIdentifier, usize),
    __1: (usize, Token<'a>, usize),
) -> alloc::vec::Vec<ASTIdentifier>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action20(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action35<
    'a,
>(
    __0: (usize, alloc::vec::Vec<ASTIdentifier>, usize),
    __1: (usize, ASTIdentifier, usize),
    __2: (usize, Token<'a>, usize),
) -> alloc::vec::Vec<ASTIdentifier>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action20(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action36<
    'a,
>(
    __0: (usize, core::option::Option<ASTIdentifier>, usize),
) -> Vec<ASTIdentifier>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action18(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        __temp0,
        __0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action37<
    'a,
>(
    __0: (usize, alloc::vec::Vec<ASTIdentifier>, usize),
    __1: (usize, core::option::Option<ASTIdentifier>, usize),
) -> Vec<ASTIdentifier>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action19(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        __temp0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action38<
    'a,
>(
    __0: (usize, ASTExpr, usize),
) -> Vec<ASTExpr>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action21(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action39<
    'a,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<ASTExpr>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action22(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action40<
    'a,
>(
    __0: (usize, alloc::vec::Vec<ASTExpr>, usize),
    __1: (usize, ASTExpr, usize),
) -> Vec<ASTExpr>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action21(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action41<
    'a,
>(
    __0: (usize, alloc::vec::Vec<ASTExpr>, usize),
) -> Vec<ASTExpr>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action22(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action42<
    'a,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ASTFile
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action12(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action43<
    'a,
>(
    __0: (usize, alloc::vec::Vec<ASTFn>, usize),
) -> ASTFile
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action13(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action44<
    'a,
>(
    __0: (usize, ASTIdentifier, usize),
) -> Vec<ASTIdentifier>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action16(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action45<
    'a,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<ASTIdentifier>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action17(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action46<
    'a,
>(
    __0: (usize, alloc::vec::Vec<ASTIdentifier>, usize),
    __1: (usize, ASTIdentifier, usize),
) -> Vec<ASTIdentifier>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action16(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action47<
    'a,
>(
    __0: (usize, alloc::vec::Vec<ASTIdentifier>, usize),
) -> Vec<ASTIdentifier>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action17(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        __0,
        __temp0,
    )
}
#[allow(clippy::type_complexity)]

pub trait __ToTriple<'a, >
{
    fn to_triple(value: Self) -> Result<(usize,Token<'a>,usize), __lalrpop_util::ParseError<usize, Token<'a>, usize>>;
}

impl<'a, > __ToTriple<'a, > for (usize, Token<'a>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'a>,usize), __lalrpop_util::ParseError<usize, Token<'a>, usize>> {
        Ok(value)
    }
}
impl<'a, > __ToTriple<'a, > for Result<(usize, Token<'a>, usize), usize>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'a>,usize), __lalrpop_util::ParseError<usize, Token<'a>, usize>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
