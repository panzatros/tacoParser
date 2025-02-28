pub struct Scanner<'a> {
    input: &'a str,
    position: usize,
    // if all the brackets on the text are correct this sould be 0 at the end
    brackets_counter: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> self {
        Scanner {
            input,
            position: 0,
            brackets_counter: 0,
        }
    }

    pub fn print_the_string_one_by_one(&self) {
        for items in self.input.chars() {
            self.char_matching_impl(items);
        }
    }
    
    //pending fix
    fn char_matching_impl(&self, c: char) {
        match c {
	    self.check_alpha(c) => println!("letra!!"),
	    self.check_number(c) => println!("number!!"),
            _ => println!("nada"),
        }
    }

    fn check_alpha(&self, c: char) -> bool {
        if c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' {
            return true;
        };
        false;
    }

    fn check_number(&self, c: char) -> bool {
        if c >= '0' && c <= '9' {
            return true;
        };
        false;
    }

    fn check_relvant_symbols(&mut self, c: char) -> bool {
        match c {
            '{' => {
                self.brackets_counter += 1;
                true
            },
            '}' => {
                self.brackets_counter -= 1;
                true
            },
            _ => false,
        }
    }

    pub fn test() {
        println!("Is Working!!");
    }
}
