use std::io::stdin;
fn main() {
    // let mut memory = 0.0;
    let mut memories = vec![0.0; 10];
    let mut prev_result = 0.0;
    for line in stdin().lines(){
        let line = line.unwrap();
        // 1行読み取って空行だったら終了
        if line.is_empty(){
            break;
        }
        // 空白で分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();
        
        // if tokens[0] == "mem+" {
        //     add_and_print_memory(&mut memory, prev_result);
        //     continue;
        // } else if tokens[0] == "mem-" {
        //     subtract_and_print_memory(&mut memory, prev_result);
        //     continue;
        // }
        let is_memory = tokens[0].starts_with("mem");
        if is_memory && tokens[0].ends_with("+"){
            add_and_print_memory(&mut memories, tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with("-") {
            subtract_and_print_memory(&mut memories, tokens[0], prev_result);
            continue;
        }

        // mem導入により変更　
        // let left: f64 = tokens[0].parse().unwrap();
        // let right: f64 = tokens[2].parse().unwrap();
        let left = eval_token(tokens[0], &memories[..]);
        let right = eval_token(tokens[2], &memories[..]);

        let result = eval_expression(left, tokens[1], right);
        print_output(result);
        prev_result = result;
    }
}

fn eval_token(token: &str, memories: &[f64]) -> f64 {
    // ifとelseの返り値は同じでなければいけないというrustの制約から、
    // 変数memoryの型から推論が効くようになるため let right: f64 と書かなくてもよい
    if token.starts_with("mem") {
        let solt_index: usize = token[3..].parse().unwrap();
        memories[solt_index]
    } else {
        token.parse().unwrap()
    }
}
fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match  operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => {
            // 到達不能マクロ
            unreachable!()
        }
    }
}
fn add_and_print_memory(
    memories: &mut [f64],
    token: &str,
    prev_result: f64
){
    let slot_index: usize = token[3..token.len() -1].parse().unwrap();
    memories[slot_index] += prev_result;
    print_output(memories[slot_index]);
}
fn subtract_and_print_memory(
    memories: &mut [f64],
    token: &str,
    prev_result: f64
){
    let slot_index: usize = token[3..token.len() -1].parse().unwrap();
    memories[slot_index] -= prev_result;
    print_output(memories[slot_index]);
}
// デバックようなのか出力なのかがわかるため有用
fn print_output(value: f64){
    println!(" => {}", value);
}
// 以下のような処理は関数に分けると読みにくくなってしまう。
// fn add_value(left: f64, right: f64) -> f64 {
//     left + right
// }
// fn subtract_value(left: f64, right: f64) -> f64 {
//     left - right
// }
// fn multiply_value(left: f64, right: f64) -> f64 {
//     left * right
// }
// fn divide_value(left: f64, right: f64) -> f64 {
//     left / right
// }


