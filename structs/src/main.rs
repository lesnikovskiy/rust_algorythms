#[derive(Debug)]
pub enum Color {
    Red(String),
    Green(String),
    Blue(String),
}

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
    children: u32,
    fave_color: Color,
}

impl Person {
    pub fn print(&self) -> String {
        format!("name = {}, age = {} has {} children", self.name, self.age, self.children)
    }
}

fn main() {
    let matt = Person {
        name: "Matt".to_string(),
        age: 35,
        children: 4,
        fave_color: Color::Green("Green".to_string()),
    };
    println!("{}", matt.print());
    println!("{:#?}", matt);

    let c = Color::Red("Red".to_string());
    match c {
        Color::Red(c) => println!("It's {}", c),
        Color::Green(c) => println!("It's {}", c),
        Color::Blue(c) => println!("It's {}", c),
    }
}
