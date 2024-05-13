fn main() {

    let s1 = "hello";
    let s2 = "world";

    // この書き方はコンパイルエラーになる cannot add `&str` to `&str`
    //let s3 = s1 + " " + s2;

    let s3 = s1.to_string() + " " + s2;
    assert_eq!(s3, "hello world");


    // ユニット型。空を表す型
    let ret = hello();
    assert_eq!(ret, ());
    // print!("ret = {}", ret);


    // bool type
    let b1 = true;
    let b2 = !b1;

    assert_eq!(b2, false);
    println!("b2 = {}", b2);

    let n1 = 8;
    let n2 = 12;
    let b3 = n1 >= 10;
    let b4 = n2 >= 10;
    let b5 = b3 && b4;  // 論理積
    println!("b5 = {}", b5);
    let b6 = b3 || b4;  // 論理和
    println!("b6 = {}", b6);

    // 整数リテラル
    let n1 = 10;       // サフィックス付けないとi32になった
    let f2 = 0.1;      // サフィックス付けないとf64になった

    let n2 = 10_000; // サフィックスの_000はなに？→みやすさのためだけの区切り文字。つまり10000と同じ
    let n3 = 0_u8;
    let n4 = -100_isize;

    let n5 = n2 + n4;

    println!("n5 = {}", n5);

    let h1 = 0xff;  // 16進数
    let o1 = 0o744; // 8進数
    let b1 = 0b1010_0110; // 2進数
    println!("h1 = {}, o1 = {}, b1 = {}", h1, o1, b1);

    let n6 = b'A';  // ASCIIコード
    println!("n6 = {}", n6);
    assert_eq!(n6, 65u8);

    // 三項演算子はない
    // i++ や i-- もない

    // MAX, MINは定数定義されている
    println!("u8 MAX = {}", std::u8::MAX);
    println!("u8 MIN = {}", std::u8::MIN);
    println!("i8 MAX = {}", std::i8::MAX);
    println!("i8 MIN = {}", std::i8::MIN);

    // デバッグモード時に以下のコードは実行時にオーバーフローを起こし、下記のエラーが発生する
    // attempt to compute `u8::MAX + 1_u8`, which would overflow
    // これはコンパイル時にチェックされない
    // 一方でリリースモード時はエラーにはならない。結果が誤った値になる
    // 実際やってみると事前にbuild時にチェックされている
    // lintのdeny(arithmetic_overflow)でチェックされているっぽい
    // let n7 = 200u8 * 100u8;
    // println!("n7 = {}", n7);

    // 浮動小数点数
    let f1 = 1.0;  // デフォルトf64
    let f2 = 1_234.56f32;  // 明示的にf32を指定
    let f3 = 578.6E+1;  // 指数表記

    println!("f3 = {}", f3);
    println!("{}, 切り捨て: {}, 切り上げ: {}, 四捨五入: {}", f2, f2.floor(), f2.ceil(), f2.round());

    // 文字型
    // Unicodeの１文字分が入る。４バイト使う
    let c1 = 'A';
    let c2 = 'a';
    assert!(c1 < c2);
    assert!(c1.is_uppercase());

    let c3 = '0';
    assert!(c3.is_digit(10));

    let c4 = 'が';
    assert!(c4.is_alphabetic()); // これ成功するので注意

    let mut n = 0;
    println!("main: n = {}", n);

    fnc1(n);
    println!("main: n = {}", n);

    fnc2(&mut n);
    println!("main: n = {}", n);

}

fn hello() {
    println!("Hello, world!");
}

fn fnc1(mut n: u32) {
    n = 1;
    println!("f1: n = {}", n);
}

fn fnc2(n_ptr: &mut u32) {
    println!("f2: n_ptr = {:p}", n_ptr);

    *n_ptr = 2;
    println!("f2 *n_ptr = {}", *n_ptr);

}
