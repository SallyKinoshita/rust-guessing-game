use rand::Rng;
use std::io;

fn main() {
    println!("数字当てゲームを始めるよ");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("数字を入力してね");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗しました");

        // let guess: u32 = guess.trim().parse().expect("数字を入力してください");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("小さすぎるよ"),
            std::cmp::Ordering::Greater => println!("大きすぎるよ"),
            std::cmp::Ordering::Equal => {
                println!("当たり！");
                break;
            }
        }
        // println!("秘密の数字: {}", secret_number);
        // println!("あなたの予想: {}", guess);
    }
}
