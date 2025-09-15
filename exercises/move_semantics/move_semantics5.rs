// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut x = 100;
    // 有的兄弟有的，创建完可变引用之后一定要立马用掉
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    vaild_mut_ref();
    assert_eq!(x, 1200);
}

// 来看看这个
fn invalid_mut_ref1() {
    let mut x = 100;
    let y = &mut x;
    // let z = &mut x; // 你怎么敢同时创建两个可变引用的啊😡
}

fn invalid_mut_ref2() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    // 你怎么敢同时创建不可变引用的啊😡（打印x相当于创建不可变引用，这个能报错属实是震惊到我了，这么严格）
    // println!("Add 100 to y, then x: {}", x);
}

fn vaild_mut_ref() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    {
        // 此时不可变引用被隐藏，因此出了这个作用域就失效哦啊，编译能过
        println!("Add 100 to y, then x: {}", x);
    }
}