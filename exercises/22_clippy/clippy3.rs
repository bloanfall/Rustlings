// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 使用 None 而不是 unwrap()
        println!("{:?}", my_option);
    }

    let my_arr = &[
        -1, -2, -3, // 需要在每个元素后添加逗号
        -4, -5, -6,
    ];
    // 修复格式化字符串
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = vec![]; // 直接创建一个空向量
    // 修复格式化字符串
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 使用 std::mem::swap 来交换变量
    std::mem::swap(&mut value_a, &mut value_b);
    // 修复格式化字符串
    println!("value a: {}; value b: {}", value_a, value_b);
}
