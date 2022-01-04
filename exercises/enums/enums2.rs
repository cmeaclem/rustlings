// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!



#[derive(Debug)]
enum Message {
    Quit,//No data
    Echo(String),//String data
    Move{x: i32, y: i32},//Anon struct
    ChangeColor(i32, i32, i32)//Tuple
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
