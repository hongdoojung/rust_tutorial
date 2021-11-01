enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("method called");
        // 메소드 내용은 여기 정의할 수 있습니다.
    }
}

fn main(){
    let m = Message::Write(String::from("hello"));
    m.call();

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; 옵션타입이기 때문에 컴파일 안됨
    let sum = x + y.unwrap();
    println!("{}", sum)
}
