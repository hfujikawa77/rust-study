fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "      ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // データ型
    let guess: u32 = "42".parse().expect("Not a number!"); // 数値ではありません！

    // 浮動小数点数型
    let x2 = 2.0; // f64
    let y2: f32 = 3.0; // f32

    // 数値演算
    let sum = 5 + 10; // 足し算
    let difference = 95.9 - 4.3; // 引き算
    let product = 4 * 30; // 掛け算
    let quotient = 56.7 / 32.2; // 割り算
    let remainder = 43 % 5; // 余り

    println!(
        "sum={}\ndifference={}\nproduct={}\nquotient={}\nremainder={}",
        sum, difference, product, quotient, remainder
    );

    // 論理値型
    let t = true;
    let f: bool = false; // 明示的型注釈付きで
    println!("t={},f={}", t, f);

    // 文字型
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c={},z={},heart_eyed_cat={}", c, z, heart_eyed_cat);

    // 複合型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // println!("tup={}",tup); // <--- そのまま出力できない
    let (x3, y3, z3) = tup;
    println!("y3={}", y3);
    println!("tup.0={},tup.1={},tup.2={}", tup.0, tup.1, tup.2);

    // 配列型
    let a = [1, 2, 3, 4, 5];
    //println!("a={}", a);    // <--- そのまま出力できない
    println!("a[0]={},a[1]={},a[2]={}", a[0], a[1], a[2]);
    let index = 10;
    // let element = a[index];   // 配列要素への無効なアクセス
    // println!("element={}", element); // 配列要素への無効なアクセス
}
