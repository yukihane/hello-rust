// （`use`を使用し、）`fmt`モジュールをインポートします。

use std::fmt;


fn main() {
    print();
    print_enshu();
    print_display();
}

// 1.2 フォーマットしてプリント
fn print() {
    // 一般的に `{} `はどんな引数であろうと自動的に置き換えられます。
    // 例えば以下は文字列に変換されます
    println!("{} days", 31);

    // サフィックスで型を指定しなければ31はi32として扱われます。
    // サフィックスの指定により、31の型を自由に変換することができます。

    // 引数の位置から埋め込まれる場所を指定することができます。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 名前での指定も可能です。
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // `:` のあとにフォーマット型を指定することによる特殊なフォーマットも可能です.
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 指定した幅の中に、右寄せで文字列を挿入することができます。
    // 以下の例では"     1". というように、５つの半角空白のあとに"1"が入ります.
    println!("{number:>width$}", number = 1, width = 6);

    // 空白の代わりに0を使うこともできます. このアウトプットは "000001" になります.
    println!("{number:>0width$}", number = 1, width = 6);

    // 引数の数が正しいかのチェックも行ってくれます。
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // `i32`保持する `Structure` という名の構造体を定義します.
    #[allow(dead_code)]
    struct Structure(i32);

    // このようにカスタム型を用いる場合、少々扱いが複雑になります。
    // 以下は動作しません。
    //    println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
}

fn print_enshu() {
    let pi = 22.0 / 7.0;
    println!("Pi is roughly {:.3}", pi);
}

// `fmt::Display`を実装するための構造体を定義します。
// これは`Structure`という名前に紐付けられた、`i32`を含むタプルです。
struct Structure(i32);

// `{}` というマーカーを使用するためには、
// この型専用の`fmt::Display`というトレイトが実装されていなくてはなりません。
impl fmt::Display for Structure {
    // このトレイトは`fmt`が想定通りのシグネチャであることを要求します。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 必ず、第一の要素が出力されるようにしています。
        // `f`は`fmt::Result`を返します。これはオペレーションが成功したか否か
        // を表します。
        // `write!`は`println!`に非常によく似た文法を使用していることに注目。
        write!(f, "{}", self.0)
    }
}

// 1.2.2 ディスプレイ
fn print_display() {
    println!("hello");
}
