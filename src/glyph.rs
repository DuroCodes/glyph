use crate::operations::Op;

pub struct Glyph {
    pub stack: Vec<Vec<i32>>,
}

impl Glyph {
    pub fn new() -> Self {
        Glyph { stack: Vec::new() }
    }

    pub fn evaluate(&mut self, op: Op) -> Result<(), String> {
        match op {
            Op::Number(n) => {
                self.stack.push(vec![n]);
                Ok(())
            }
            Op::Range => {
                if let Some(vec) = self.stack.pop() {
                    if let Some(n) = vec.first() {
                        self.stack.push((1..=*n).collect());
                        Ok(())
                    } else {
                        Err("Must have a number on the stack to create a range".into())
                    }
                } else {
                    Err("Must have a number on the stack to create a range".into())
                }
            }
            Op::Add => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(match (a.len(), b.len()) {
                        (1, _) => b.iter().map(|x| x + a[0]).collect(),
                        (_, 1) => a.iter().map(|x| x + b[0]).collect(),
                        (_, _) => a.iter().zip(b.iter()).map(|(x, y)| x + y).collect(),
                    });
                    Ok(())
                } else {
                    Err("Must have two numbers on the stack to add".into())
                }
            }
            Op::Subtract => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(match (a.len(), b.len()) {
                        (1, _) => b.iter().map(|x| a[0] - x).collect(),
                        (_, 1) => a.iter().map(|x| x - b[0]).collect(),
                        (_, _) => a.iter().zip(b.iter()).map(|(x, y)| x - y).collect(),
                    });
                    Ok(())
                } else {
                    Err("Must have two numbers on the stack to subtract".into())
                }
            }
            Op::Multiply => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack
                        .push(a.iter().zip(b.iter()).map(|(x, y)| x * y).collect());
                    Ok(())
                } else {
                    Err("Must have two numbers on the stack to multiply".into())
                }
            }
            Op::Divide => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(match (a.len(), b.len()) {
                        (1, _) => b
                            .iter()
                            .map(|x| if *x != 0 { a[0] / x } else { 0 })
                            .collect(),
                        (_, 1) => a
                            .iter()
                            .map(|x| if b[0] != 0 { x / b[0] } else { 0 })
                            .collect(),
                        (_, _) => a
                            .iter()
                            .zip(b.iter())
                            .map(|(x, y)| if *y != 0 { x / y } else { 0 })
                            .collect(),
                    });
                    Ok(())
                } else {
                    Err("Must have two numbers on the stack to divide".into())
                }
            }
            Op::Modulo => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(match (a.len(), b.len()) {
                        (1, _) => b
                            .iter()
                            .map(|x| if *x != 0 { a[0] % x } else { 0 })
                            .collect(),
                        (_, 1) => a
                            .iter()
                            .map(|x| if b[0] != 0 { x % b[0] } else { 0 })
                            .collect(),
                        (_, _) => a
                            .iter()
                            .zip(b.iter())
                            .map(|(x, y)| if *y != 0 { x % y } else { 0 })
                            .collect(),
                    });
                    Ok(())
                } else {
                    Err("Must have two numbers on the stack to modulo".into())
                }
            }
            Op::Sum => {
                if let Some(vec) = self.stack.pop() {
                    let sum = vec.iter().sum();
                    self.stack.push(vec![sum]);
                    Ok(())
                } else {
                    Err("Must have an array on the stack to sum".into())
                }
            }
            Op::Product => {
                if let Some(vec) = self.stack.pop() {
                    let product = vec.iter().product();
                    self.stack.push(vec![product]);
                    Ok(())
                } else {
                    Err("Must have an array on the stack to product".into())
                }
            }
            Op::Reverse => {
                if let Some(mut vec) = self.stack.pop() {
                    vec.reverse();
                    self.stack.push(vec);
                    Ok(())
                } else {
                    Err("Must have an array on the stack to reverse".into())
                }
            }
            Op::Sort => {
                if let Some(mut vec) = self.stack.pop() {
                    vec.sort();
                    self.stack.push(vec);
                    Ok(())
                } else {
                    Err("Must have an array on the stack to sort".into())
                }
            }
            Op::Length => {
                if let Some(vec) = self.stack.pop() {
                    self.stack.push(vec![vec.len() as i32]);
                    Ok(())
                } else {
                    Err("Must have an array on the stack to get its length".into())
                }
            }
            Op::Greater => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(match (a.len(), b.len()) {
                        (1, _) => b.iter().map(|x| i32::from(a[0] > *x)).collect(),
                        (_, 1) => a.iter().map(|x| i32::from(*x > b[0])).collect(),
                        (_, _) => a
                            .iter()
                            .zip(b.iter())
                            .map(|(x, y)| i32::from(x > y))
                            .collect(),
                    });
                    Ok(())
                } else {
                    Err("Must have two elements on the stack to compare".into())
                }
            }
            Op::GreaterEqual => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(match (a.len(), b.len()) {
                        (1, _) => b.iter().map(|x| i32::from(a[0] >= *x)).collect(),
                        (_, 1) => a.iter().map(|x| i32::from(*x >= b[0])).collect(),
                        (_, _) => a
                            .iter()
                            .zip(b.iter())
                            .map(|(x, y)| i32::from(x >= y))
                            .collect(),
                    });
                    Ok(())
                } else {
                    Err("Must have two elements on the stack to compare".into())
                }
            }
            Op::Less => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(match (a.len(), b.len()) {
                        (1, _) => b.iter().map(|x| i32::from(a[0] < *x)).collect(),
                        (_, 1) => a.iter().map(|x| i32::from(*x < b[0])).collect(),
                        (_, _) => a
                            .iter()
                            .zip(b.iter())
                            .map(|(x, y)| i32::from(x < y))
                            .collect(),
                    });
                    Ok(())
                } else {
                    Err("Must have two elements on the stack to compare".into())
                }
            }
            Op::LessEqual => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(match (a.len(), b.len()) {
                        (1, _) => b.iter().map(|x| i32::from(a[0] <= *x)).collect(),
                        (_, 1) => a.iter().map(|x| i32::from(*x <= b[0])).collect(),
                        (_, _) => a
                            .iter()
                            .zip(b.iter())
                            .map(|(x, y)| i32::from(x <= y))
                            .collect(),
                    });
                    Ok(())
                } else {
                    Err("Must have two elements on the stack to compare".into())
                }
            }
            Op::Equal => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(match (a.len(), b.len()) {
                        (1, _) => b.iter().map(|x| i32::from(a[0] == *x)).collect(),
                        (_, 1) => a.iter().map(|x| i32::from(*x == b[0])).collect(),
                        (_, _) => a
                            .iter()
                            .zip(b.iter())
                            .map(|(x, y)| i32::from(x == y))
                            .collect(),
                    });
                    Ok(())
                } else {
                    Err("Must have two elements on the stack to compare".into())
                }
            }
            Op::Clear => {
                self.stack.clear();
                Ok(())
            }
            Op::Filter => {
                if let (Some(condition), Some(array)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(
                        array
                            .into_iter()
                            .zip(condition)
                            .filter(|(_, cond)| *cond != 0)
                            .map(|(val, _)| val)
                            .collect(),
                    );
                    Ok(())
                } else {
                    Err("Must have an array and a condition array on the stack to filter".into())
                }
            }
            Op::Not => {
                if let Some(vec) = self.stack.pop() {
                    self.stack.push(
                        vec.into_iter()
                            .map(|x| if x == 0 { 1 } else { 0 })
                            .collect(),
                    );
                    Ok(())
                } else {
                    Err("Must have an array on the stack to invert".into())
                }
            }
            Op::Duplicate => {
                if let Some(vec) = self.stack.last().cloned() {
                    self.stack.push(vec);
                    Ok(())
                } else {
                    Err("Stack is empty".into())
                }
            }
            Op::Concatenate => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    let mut result = a;
                    result.extend(b);
                    self.stack.push(result);
                    Ok(())
                } else {
                    Err("Must have two arrays on the stack to concatenate".into())
                }
            }
            Op::Split => {
                if let (Some(idx), Some(array)) = (self.stack.pop(), self.stack.pop()) {
                    if let Some(i) = idx.first() {
                        let i = *i as usize;
                        if i <= array.len() {
                            let (left, right) = array.split_at(i);
                            self.stack.push(left.to_vec());
                            self.stack.push(right.to_vec());
                            Ok(())
                        } else {
                            Err("Split index out of bounds".into())
                        }
                    } else {
                        Err("Split index array is empty".into())
                    }
                } else {
                    Err("Must have an array and index on the stack to split".into())
                }
            }
            Op::Array(nums) => {
                self.stack.push(nums);
                Ok(())
            }
            Op::PopLeft => {
                if let Some(mut vec) = self.stack.pop() {
                    if !vec.is_empty() {
                        let first = vec.remove(0);
                        self.stack.push(vec);
                        self.stack.push(vec![first]);
                        Ok(())
                    } else {
                        Err("Cannot pop from empty array".into())
                    }
                } else {
                    Err("Must have an array on the stack".into())
                }
            }
            Op::PopRight => {
                if let Some(mut vec) = self.stack.pop() {
                    if let Some(last) = vec.pop() {
                        self.stack.push(vec);
                        self.stack.push(vec![last]);
                        Ok(())
                    } else {
                        Err("Cannot pop from empty array".into())
                    }
                } else {
                    Err("Must have an array on the stack".into())
                }
            }
            Op::AppendLeft => {
                if let (Some(element), Some(mut array)) = (self.stack.pop(), self.stack.pop()) {
                    if let Some(&value) = element.first() {
                        array.insert(0, value);
                        self.stack.push(array);
                        Ok(())
                    } else {
                        Err("Element array is empty".into())
                    }
                } else {
                    Err("Must have an array and element on the stack".into())
                }
            }
            Op::AppendRight => {
                if let (Some(element), Some(mut array)) = (self.stack.pop(), self.stack.pop()) {
                    if let Some(&value) = element.first() {
                        array.push(value);
                        self.stack.push(array);
                        Ok(())
                    } else {
                        Err("Element array is empty".into())
                    }
                } else {
                    Err("Must have an array and element on the stack".into())
                }
            }
        }
    }

    pub fn evaluate_sequence(&mut self, ops: Vec<Op>) {
        for op in ops {
            if let Err(e) = self.evaluate(op) {
                println!("Error: {e}");
                break;
            }
        }
    }

    pub fn parse(input: &str) -> Vec<Op> {
        let mut ops = Vec::new();
        let mut chars = input.chars().peekable();
        let mut current_array: Option<Vec<i32>> = None;
        let mut current_num = String::new();

        while let Some(c) = chars.next() {
            match c {
                '#' => {
                    // Skip the rest of the line
                    while let Some(c) = chars.next() {
                        if c == '\n' {
                            break;
                        }
                    }
                }
                '[' => {
                    current_array = Some(Vec::new());
                    current_num.clear();
                }
                ']' => {
                    if !current_num.is_empty() {
                        if let Ok(n) = current_num.parse() {
                            if let Some(array) = &mut current_array {
                                array.push(n);
                            }
                        }
                        current_num.clear();
                    }
                    if let Some(array) = current_array.take() {
                        if array.len() == 1 {
                            ops.push(Op::Number(array[0]));
                        } else {
                            ops.push(Op::Array(array));
                        }
                    }
                }
                c if c.is_whitespace() => {
                    if !current_num.is_empty() {
                        if let Ok(n) = current_num.parse() {
                            if let Some(array) = &mut current_array {
                                array.push(n);
                            } else {
                                ops.push(Op::Number(n));
                            }
                        }
                        current_num.clear();
                    }
                }
                d if d.is_ascii_digit() || d == '-' => {
                    current_num.push(d);
                }
                op => {
                    if !current_num.is_empty() {
                        if let Ok(n) = current_num.parse() {
                            if let Some(array) = &mut current_array {
                                array.push(n);
                            } else {
                                ops.push(Op::Number(n));
                            }
                        }
                        current_num.clear();
                    }
                    match op {
                        '↑' => ops.push(Op::Range),
                        '+' => ops.push(Op::Add),
                        '-' => {
                            if chars.peek().map_or(false, |c| c.is_ascii_digit()) {
                                current_num.push('-');
                            } else {
                                ops.push(Op::Subtract);
                            }
                        }
                        '×' => ops.push(Op::Multiply),
                        '/' => ops.push(Op::Divide),
                        '%' => ops.push(Op::Modulo),
                        '∑' => ops.push(Op::Sum),
                        '∏' => ops.push(Op::Product),
                        '↔' => ops.push(Op::Reverse),
                        '⇕' => ops.push(Op::Sort),
                        '⋕' => ops.push(Op::Length),
                        '>' => ops.push(Op::Greater),
                        '≥' => ops.push(Op::GreaterEqual),
                        '<' => ops.push(Op::Less),
                        '≤' => ops.push(Op::LessEqual),
                        '=' => ops.push(Op::Equal),
                        '∅' => ops.push(Op::Clear),
                        '⊃' => ops.push(Op::Filter),
                        '¬' => ops.push(Op::Not),
                        '⊕' => ops.push(Op::Duplicate),
                        '⋈' => ops.push(Op::Concatenate),
                        '⋉' => ops.push(Op::Split),
                        '⊣' => ops.push(Op::PopLeft),
                        '⊢' => ops.push(Op::PopRight),
                        '⊲' => ops.push(Op::AppendLeft),
                        '⊳' => ops.push(Op::AppendRight),
                        _ => {}
                    }
                }
            }
        }
        if !current_num.is_empty() {
            if let Ok(n) = current_num.parse() {
                if let Some(array) = &mut current_array {
                    array.push(n);
                } else {
                    ops.push(Op::Number(n));
                }
            }
        }
        ops
    }
}
