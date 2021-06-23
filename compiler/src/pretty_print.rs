pub trait PrettyPrint {
    fn print(&self, formatter: &mut PrettyFormatter);
}

fn gen_padding(indentation: usize) -> String {
    if indentation == 0 {
        return "".to_owned();
    }

    let mut result = String::new();

    if indentation > 2 {
        for _ in 0..(indentation - 2) {
            result.push(' ');
        }
    }

    result.push('-');
    result.push('>');

    result
}

pub struct PrettyFormatter {
    step_size: usize,
    indentation: usize,
    padding: String,
}

impl PrettyFormatter {
    pub fn new(step_size: usize, indentation: usize) -> Self {
        let padding = gen_padding(indentation);

        Self {
            step_size,
            indentation,
            padding,
        }
    }

    pub fn print_str(&mut self, content: &str) {
        println!("{}{}", self.padding, content);
    }
    pub fn print_sub(&mut self) -> Self {
        Self::new(self.step_size, self.indentation + self.step_size)
    }
}

pub fn pretty_print<E>(element: &E)
where
    E: PrettyPrint,
{
    let mut init_formatter = PrettyFormatter::new(2, 0);

    element.print(&mut init_formatter);
}
