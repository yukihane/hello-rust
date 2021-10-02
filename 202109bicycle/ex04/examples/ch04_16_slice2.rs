fn main() {
    let mut a4 = [6, 4, 2, 8, 0, 9, 4, 3, 7, 5, 1, 7];

    // 一部の要素を昇順にソートする
    a4[2..6].sort();
    assert_eq!(&a4[2..6], &[0, 2, 8, 9]);

    // スライスを2つの可変スライスへ分割する
    #[allow(unused_variables)]
    let (s4a, s4b) = (&mut a4).split_at_mut(5);

    // &mutを省略しても結果は同じ。型強制によって自動的にスライスが作られる
    // a4[2..6].sort();
    // let (s4a, s4b) = a4.split_at_mut(5);

    // 前半を逆順にする
    s4a.reverse();
    assert_eq!(s4a, &[8, 2, 0, 4, 6]);

    // 後半を昇順にソートする
    s4b.sort_unstable();
    assert_eq!(s4b, &[1, 3, 4, 5, 7, 7, 9]);

    // sort()とsort_unstable()の違い
    // sort()は安定ソートなので同順なデータのソート前の順序がソート後も保存される
    // soft_unstable()は安定ソートではないが、一般的にsort()より高速
}
