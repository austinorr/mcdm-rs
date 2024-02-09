// use std::collections::HashMap;

use promrs::*;

// fn times2(value: i32) -> i32 {
//     2 * value
// }

// fn fun_test(value: i32, f: fn(i32) -> i32) -> i32 {
//     println!("{}", f(value));
//     value
// }

fn test_uniflow() {
    use pref_functions::usual;
    use types::*;
    use unicriterion_flow::unicriterion_flow;

    let array: Vec<Fl> = vec![0.8, 0.2, 0.5];
    let mut plus: Vec<Fl> = vec![0.0, 0.0, 0.0];
    let mut minus: Vec<Fl> = vec![0.0, 0.0, 0.0];
    let func: FPref = usual;
    let q: Fl = 0.0;
    let p: Fl = 0.0;

    println!("uniflow plus before: {:#?}", plus);

    unicriterion_flow(&array, &mut plus, &mut minus, func, &q, &p);

    println!("uniflow plus after: {:#?}", plus);
}

fn test_mc_flow() {
    use matrix::transpose;
    use multicriterion_flow::multicriterion_flow;

    let arr = vec![vec![0.8, 0.2, 0.5], vec![0.8, 0.2, 0.5]];
    let weights = vec![1., 1.];
    let criteria_type = vec![-1, 1];
    let func_names = vec!["usual", "usual"];
    let q = vec![0., 0.];
    let p = vec![0., 0.];

    let (plus, minus) = multicriterion_flow(&arr, &weights, &criteria_type, &func_names, &q, &p);

    println!("multi: {:#?} {:#?}", transpose(plus), transpose(minus))
}

fn main() {
    let func_name: &str = "usual";

    let map = pref_functions::get_pref_functions();
    let func = map
        .get(func_name)
        .unwrap_or_else(|| panic!("function not found: {func_name}"));

    assert_eq!(func(&0.0, &0.0, &0.0), 0.0);

    test_uniflow();
    test_mc_flow();
}

// #[test]
// fn test_usual() {
//     let funcs: std::collections::HashMap<String, fn(f32, f32, f32) -> f32> =
//         pref_functions::get_pref_functions();
// }

// #[test]
// fn test_func_lookup() {
//     let func_name: &str = "usual";

//     let map = pref_functions::get_pref_functions();
//     let func = map
//         .get(func_name)
//         .expect(&format!("{} not found", func_name));

//     assert_eq!(func(&0.0, &0.0, &0.0), 0.0);
// }
