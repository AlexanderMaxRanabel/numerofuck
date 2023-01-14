struct Interpreter {
    tape: [u8; 30000],
    pointer: usize,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            tape: [0; 30000],
            pointer: 0,
        }
    }

    fn execute(&mut self, code: &str) {
        let mut i = 0;
        for c in code.chars() {
            match c {
                '0' => self.pointer += 1,
                '1' => self.pointer -= 1,
                '2' => self.tape[self.pointer] += 1,
                '3' => self.tape[self.pointer] -= 1,
                '4' => print!("{}", self.tape[self.pointer] as char),
                '5' => self.tape[self.pointer] = get_input(),
                '6' => {
                    if self.tape[self.pointer] == 0 {
                        let mut count = 1;
                        while count > 0 {
                            i += 1;
                            c = code.chars().nth(i).unwrap();
                            if c == '6' {
                                count += 1;
                            } else if c == '7' {
                                count -= 1;
                            }
                        }
                    }
                }
                '7' => {
                    if self.tape[self.pointer] != 0 {
                        let mut count = 1;
                        while count > 0 {
                            i -= 1;
                            c = code.chars().nth(i).unwrap();
                            if c == '6' {
                                count -= 1;
                            } else if c == '7' {
                                count += 1;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn get_input() -> u8 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let mut interpreter = Interpreter::new();
    let code = "52321704";
    interpreter.execute(code);
}
