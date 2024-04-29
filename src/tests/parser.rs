#[cfg(test)]
mod parser_test {
    use murph::{
        opcodes::{
            OpCode, ADD, CALLDATALOAD, DUP1, EQ, JUMP, JUMPDEST, JUMPI, MSTORE,
            PUSH1, PUSH2, PUSH4, RETURN, SHR, SLOAD, SSTORE
        },
        parser::{self, JumpPack, JumpTable, JumpType},
        utils::{Byte, SourceByte},
    };
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_parse_add() {
        let code = hex::decode(String::from("61010201")).unwrap();
        let parsed = parser::parse(&code).unwrap().sb;

        assert_eq!(
            parsed,
            vec![
                SourceByte {
                    byte: vec![
                        Byte::Op(OpCode::new(PUSH2)),
                        Byte::Hex(String::from("01")),
                        Byte::Hex(String::from("02"))
                    ],
                    pc: 0
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(ADD))],
                    pc: 3
                },
            ]
        );
    }

    #[test]
    fn test_invalid_push() {
        let code = hex::decode(String::from("6100")).unwrap();
        let parsed = parser::parse(&code).unwrap().sb;

        assert_eq!(
            parsed,
            vec![SourceByte {
                byte: vec![
                    Byte::Op(OpCode::new(PUSH2)),
                    Byte::Hex(String::from("00")),
                    Byte::Hex(String::from("<UNFINISHED_PUSH>"))
                ],
                pc: 0
            }]
        )
    }

    #[test]
    fn test_jump_location() {
        let code = hex::decode(String::from("6003565B")).unwrap();
        let out = parser::parse(&code).unwrap();
        let (parsed, jump_table) = (out.sb, out.jt);

        assert_eq!(
            parsed,
            vec![
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(PUSH1)), Byte::Hex(String::from("03")),],
                    pc: 0
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(JUMP))],
                    pc: 2
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(JUMPDEST))],
                    pc: 3
                }
            ]
        );

        assert_eq!(
            jump_table,
            JumpTable {
                jump: HashMap::from([(
                    2,
                    JumpPack {
                        pc: 3,
                        jump_type: JumpType::JUMP
                    }
                )]),
                jumpdest: HashSet::from([3])
            }
        );
    }

    #[test]
    fn test_jumpi_location() {
        let code = hex::decode(String::from("632222222214601C575B")).unwrap();
        let out = parser::parse(&code).unwrap();
        let (parsed, jump_table) = (out.sb, out.jt);

        assert_eq!(
            parsed,
            vec![
                SourceByte {
                    byte: vec![
                        Byte::Op(OpCode::new(PUSH4)),
                        Byte::Hex(String::from("22")),
                        Byte::Hex(String::from("22")),
                        Byte::Hex(String::from("22")),
                        Byte::Hex(String::from("22")),
                    ],
                    pc: 0
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(EQ))],
                    pc: 5
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(PUSH1)), Byte::Hex(String::from("1c"))],
                    pc: 6
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(JUMPI))],
                    pc: 8
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(JUMPDEST))],
                    pc: 9
                },
            ]
        );

        assert_eq!(
            jump_table,
            JumpTable {
                // loc => to
                jump: HashMap::from([(
                    8,
                    JumpPack {
                        pc: 28,
                        jump_type: JumpType::JUMPI
                    }
                )]),
                // loc
                jumpdest: HashSet::from([(9)]),
            }
        )
    }

    #[test]
    fn test_simple_store() {
        let code = hex::decode(String::from("60003560e01c8063552410771461001c5780632096525514610023575b6004356000555b60005460005260206000f3")).unwrap();
        let out = parser::parse(&code).unwrap();
        let (parsed, jump_table) = (out.sb, out.jt);

        assert_eq!(
            parsed,
            vec![
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(PUSH1)), Byte::Hex(String::from("00")),],
                    pc: 0
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(CALLDATALOAD))],
                    pc: 2
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(PUSH1)), Byte::Hex(String::from("e0")),],
                    pc: 3
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(SHR))],
                    pc: 5
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(DUP1))],
                    pc: 6
                },
                SourceByte {
                    byte: vec![
                        Byte::Op(OpCode::new(PUSH4)),
                        Byte::Hex(String::from("55")),
                        Byte::Hex(String::from("24")),
                        Byte::Hex(String::from("10")),
                        Byte::Hex(String::from("77")),
                    ],
                    pc: 7
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(EQ))],
                    pc: 12
                },
                SourceByte {
                    byte: vec![
                        Byte::Op(OpCode::new(PUSH2)),
                        Byte::Hex(String::from("00")),
                        Byte::Hex(String::from("1c")),
                    ],
                    pc: 13
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(JUMPI))],
                    pc: 16
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(DUP1))],
                    pc: 17
                },
                SourceByte {
                    byte: vec![
                        Byte::Op(OpCode::new(PUSH4)),
                        Byte::Hex(String::from("20")),
                        Byte::Hex(String::from("96")),
                        Byte::Hex(String::from("52")),
                        Byte::Hex(String::from("55")),
                    ],
                    pc: 18
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(EQ))],
                    pc: 23
                },
                SourceByte {
                    byte: vec![
                        Byte::Op(OpCode::new(PUSH2)),
                        Byte::Hex(String::from("00")),
                        Byte::Hex(String::from("23")),
                    ],
                    pc: 24
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(JUMPI))],
                    pc: 27
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(JUMPDEST))],
                    pc: 28
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(PUSH1)), Byte::Hex(String::from("04")),],
                    pc: 29
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(CALLDATALOAD))],
                    pc: 31
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(PUSH1)), Byte::Hex(String::from("00")),],
                    pc: 32
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(SSTORE))],
                    pc: 34
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(JUMPDEST))],
                    pc: 35
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(PUSH1)), Byte::Hex(String::from("00")),],
                    pc: 36
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(SLOAD))],
                    pc: 38
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(PUSH1)), Byte::Hex(String::from("00")),],
                    pc: 39
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(MSTORE))],
                    pc: 41
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(PUSH1)), Byte::Hex(String::from("20")),],
                    pc: 42
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(PUSH1)), Byte::Hex(String::from("00")),],
                    pc: 44
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode::new(RETURN))],
                    pc: 46
                },
            ]
        );

        assert_eq!(
            jump_table,
            JumpTable {
                jump: HashMap::from([
                    (
                        16,
                        JumpPack {
                            pc: 28,
                            jump_type: JumpType::JUMPI
                        }
                    ),
                    (
                        27,
                        JumpPack {
                            pc: 35,
                            jump_type: JumpType::JUMPI
                        }
                    )
                ]),
                jumpdest: HashSet::from([28, 35])
            }
        )
    }
}
