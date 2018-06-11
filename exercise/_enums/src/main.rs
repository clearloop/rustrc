//enum Message {
//    Quit,
//    ChangeColor(i32, i32, i32),
//    Move { x: i32, y: i32},
//    Write(String),
//}

//enum Message {
//    Move { x: i32, y: i32 },
//}
//
//enum BoardGameTurn {
//    Move { squares: i32 },
//    Pass
//}
//
//fn main() {
//    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };
//    println!("{:?}", y);
//}

enum Message {
    Write(String),
}

fn main() {
    let v = vec!["Hello".to_string(), "World".to_string()];
    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
}
