fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let sum = v.into_iter().reduce(|x, y| x + y).unwrap();
    println!("{}", sum); // 15

    // 要素1個
    let v = vec![1];
    let sum = v.into_iter().reduce(|x, y| x + y).unwrap();
    println!("{}", sum); // 1

    // 要素無し
    let v = vec![0; 0];
    let sum = v.into_iter().reduce(|x, y| x + y);
    println!("{:?}", sum); // None
}
