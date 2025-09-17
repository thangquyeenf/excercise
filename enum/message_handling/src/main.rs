enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Exiting..."),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(s) => println!("Message: {}", s),
        Message::ChangeColor(r, g, b) => println!("Color changed to ({}, {}, {})", r, g, b),
    }
}

fn main() {
    let quit = Message::Quit;
    process_message(quit);
    let move_msg = Message::Move { x: 2, y: 5 };
    process_message(move_msg);
    process_message(Message::Write("Thangquyeenf".to_string()));
    process_message(Message::ChangeColor(134, 255, 43));
}
