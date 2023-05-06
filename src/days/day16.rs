use std::str::Chars;

use crate::problem::problemdef::Problem;

#[derive(Debug, PartialEq, Clone, Copy)]
enum LengthType {
    TotalBits(i32),
    NumberSubPackets(i32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct LiteralData {
    value: i128,
    length: i32,
    version: i32,
    id: i32,
}

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Operator(Box<OperatorData>),
    Literal(Box<LiteralData>),
}

#[derive(Debug, PartialEq, Clone)]
struct OperatorData {
    sub_operators: Vec<Operator>,
    operation: i32,
    version: i32,
    length: i32,
}

pub struct DaySixteen {}

impl DaySixteen {
    fn to_binary(c: char) -> &'static str {
        match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        }
    }
    fn convert_to_binary_from_hex(hex: &str) -> String {
        hex.chars().map(Self::to_binary).collect()
    }
    fn is_literal_value(i: &Chars) -> bool {
        i.as_str().to_string()[3..6] == *"100"
    }
    fn get_version(i: &Chars) -> i32 {
        let i = i.clone();
        i32::from_str_radix(&i.as_str().to_string()[0..3], 2).unwrap()
    }
    fn get_operation_id(i: &Chars) -> i32 {
        let i = i.clone();
        i32::from_str_radix(&i.as_str().to_string()[3..6], 2).unwrap()
    }
    fn get_length_type_id(i: &Chars) -> LengthType {
        let i = i.as_str().to_string();
        match i.chars().nth(6).unwrap() {
            '0' => LengthType::TotalBits(i32::from_str_radix(&i[7..(7 + 15)], 2).unwrap()),
            '1' => LengthType::NumberSubPackets(i32::from_str_radix(&i[7..(7 + 11)], 2).unwrap()),
            _ => unreachable!(),
        }
    }
    fn read_literal(i: &mut Chars) -> LiteralData {
        let v = Self::get_version(i);
        i.advance_by(6).unwrap();
        let mut read_bits = 6i32;
        let mut wholenumber = "".to_string();
        loop {
            let chunk = i.next_chunk::<5>().unwrap();
            read_bits += 5;
            wholenumber = format!("{}{}", wholenumber, String::from_iter(chunk[1..5].iter()));
            if chunk[0] == '0' {break;}
        }

        LiteralData {
            value: i128::from_str_radix(&wholenumber, 2).unwrap(),
            length: read_bits,
            version: v,
            id: 4,
        }
    }
    fn read_operator(i: &mut Chars) -> OperatorData {
        let length_type = Self::get_length_type_id(i);
        let mut total_length = 0;
        let v = Self::get_version(i);
        let op = Self::get_operation_id(i);
        total_length += 6;
        i.advance_by(6).unwrap();
        let mut sub_operators = vec![];
        match length_type {
            LengthType::NumberSubPackets(n) => {
                i.advance_by(12).unwrap();
                total_length += 12;
                for _ in 0..n {
                    if Self::is_literal_value(i) {
                        let next_literal = Self::read_literal(i);
                        sub_operators.push(Operator::Literal(Box::new(next_literal)));
                        total_length += next_literal.length;
                    } else {
                        let next_op = Self::read_operator(i);
                        total_length += next_op.length;
                        sub_operators.push(Operator::Operator(Box::new(next_op)));
                    }
                }
            }
            LengthType::TotalBits(n) => {
                i.advance_by(16).unwrap();
                total_length += 16;
                let mut read_bits = 0;
                while read_bits < n {
                    if Self::is_literal_value(i) {
                        let next_literal = Self::read_literal(i);
                        read_bits += next_literal.length;
                        sub_operators.push(Operator::Literal(Box::new(next_literal)));
                        total_length += next_literal.length;
                    } else {
                        let next_op = Self::read_operator(i);
                        read_bits += next_op.length;
                        total_length += next_op.length;
                        sub_operators.push(Operator::Operator(Box::new(next_op)));
                    }
                }
            }
        }
        OperatorData {
            sub_operators,
            operation: op,
            version: v,
            length: total_length,
        }
    }

    fn add_version_numbers(od: &OperatorData) -> i32 {
        let mut r = od.version;

        for op in &od.sub_operators {
            match op {
                Operator::Literal(l) => r += l.length,
                Operator::Operator(o) => r += Self::add_version_numbers(o),
            }
        }
        r
    }

    fn evaluate_operator(od: &Operator) -> i128 {
        match od {
            Operator::Literal(l) => l.value,
            Operator::Operator(o) => match o.operation {
                0 => o.sub_operators.iter().map(Self::evaluate_operator).sum(),
                1 => o
                    .sub_operators
                    .iter()
                    .map(Self::evaluate_operator)
                    .product(),
                2 => o
                    .sub_operators
                    .iter()
                    .map(Self::evaluate_operator)
                    .min()
                    .unwrap(),
                3 => o
                    .sub_operators
                    .iter()
                    .map(Self::evaluate_operator)
                    .max()
                    .unwrap(),
                5 => {
                    if Self::evaluate_operator(&o.sub_operators[0])
                        > Self::evaluate_operator(&o.sub_operators[1])
                    {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if Self::evaluate_operator(&o.sub_operators[0])
                        < Self::evaluate_operator(&o.sub_operators[1])
                    {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if Self::evaluate_operator(&o.sub_operators[0])
                        == Self::evaluate_operator(&o.sub_operators[1])
                    {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

impl Problem for DaySixteen {
    fn part_one(&self, input: &str) -> String {
        let input = input.split('\n').find(|l| !l.is_empty()).unwrap();
        let bin_string = Self::convert_to_binary_from_hex(input);
        let mut bin_iter = bin_string.chars();

        let data = Self::read_operator(&mut bin_iter);

        format!("{}", Self::add_version_numbers(&data))
    }

    fn part_two(&self, input: &str) -> String {
        let input = input.split('\n').find(|l| !l.is_empty()).unwrap();
        let bin_string = Self::convert_to_binary_from_hex(input);
        let mut bin_iter = bin_string.chars();

        let data = Self::read_operator(&mut bin_iter);

        format!(
            "{}",
            Self::evaluate_operator(&Operator::Operator(Box::new(data)))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hex_parse() {
        let result = "00001010".to_string();
        assert_eq!(result, DaySixteen::convert_to_binary_from_hex("0A"));
    }

    #[test]
    fn is_literal_value() {
        assert!(DaySixteen::is_literal_value(
            &"110100101111111000101000".chars()
        ));
        assert!(!DaySixteen::is_literal_value(
            &"110101101111111000101000".chars()
        ));
    }
    #[test]
    fn literal_version() {
        let expected = 6i32;
        assert_eq!(
            DaySixteen::get_version(&"110100101111111000101000".chars()),
            expected
        );
    }
    #[test]
    fn length_type() {
        let expected = LengthType::TotalBits(27);
        assert_eq!(
            DaySixteen::get_length_type_id(
                &"00111000000000000110111101000101001010010001001000000000".chars()
            ),
            expected
        );
        let expected = LengthType::NumberSubPackets(3);
        assert_eq!(
            DaySixteen::get_length_type_id(
                &"11101110000000001101010000001100100000100011000001100000".chars()
            ),
            expected
        );
    }

    #[test]
    fn read_literal() {
        let result = DaySixteen::read_literal(&mut "110100101111111000101000".chars());
        let expected = LiteralData {
            length: "110100101111111000101".len() as i32,
            value: 2021,
            version: 6,
            id: 4,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn read_literal_2() {
        let result = DaySixteen::read_literal(&mut "11010001010".chars());
        let expected = LiteralData {
            length: "11010001010".len() as i32,
            value: 10,
            version: 6,
            id: 4,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn read_literal_with_more_trailing_stuff() {
        let input = format!("{}{}", "110100101111111000101000", "10010101");
        let mut it = input.chars();
        let result = DaySixteen::read_literal(&mut it);
        let expected = LiteralData {
            length: "110100101111111000101".len() as i32,
            value: 2021,
            version: 6,
            id: 4,
        };
        assert_eq!(result, expected);
        assert_eq!(it.count(), 11);
    }
    #[test]
    fn read_operator_len_sub_packets() {
        let expected = OperatorData {
            sub_operators: vec![
                Operator::Literal(Box::new(LiteralData {
                    value: 10,
                    length: 11,
                    version: 6,
                    id: 4,
                })),
                Operator::Literal(Box::new(LiteralData {
                    value: 20,
                    length: 16,
                    version: 2,
                    id: 4,
                })),
            ],
            length: "0011100000000000011011110100010100101001000100100".len() as i32,
            operation: 6,
            version: 1,
        };
        let result = DaySixteen::read_operator(
            &mut "00111000000000000110111101000101001010010001001000000000".chars(),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn read_operator_number_sub_packets() {
        let expected = OperatorData {
            sub_operators: vec![
                Operator::Literal(Box::new(LiteralData {
                    value: 1,
                    length: 11,
                    version: 2,
                    id: 4,
                })),
                Operator::Literal(Box::new(LiteralData {
                    value: 2,
                    length: 11,
                    version: 4,
                    id: 4,
                })),
                Operator::Literal(Box::new(LiteralData {
                    value: 3,
                    length: 11,
                    version: 1,
                    id: 4,
                })),
            ],
            operation: 3,
            version: 7,
            length: "111011100000000011010100000011001000001000110000011".len() as i32,
        };
        let result = DaySixteen::read_operator(
            &mut "11101110000000001101010000001100100000100011000001100000".chars(),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn nested_structure_version_sum() {
        let result = DaySixteen::add_version_numbers(&DaySixteen::read_operator(
            &mut (DaySixteen::convert_to_binary_from_hex("A0016C880162017C3686B18A3D4780")).chars(),
        ));
        let expected = 64;
        assert_eq!(result, expected);
    }
}
