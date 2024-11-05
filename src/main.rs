use std::{collections::{hash_map::Entry, HashMap}, io::stdin};
fn main() {
    let mut memory = Memory::new();
    // let mut memories = vec![0.0; 10];
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
            memory.add_and_print(tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with("-") {
            memory.add_and_print(tokens[0], -prev_result);
            continue;
        }
        let left = memory.eval_token(tokens[0]);
        let right = memory.eval_token(tokens[2]);
        let result = eval_expression(left, tokens[1], right);
        print_output(result);
        prev_result = result;
    }
}

struct  Memory {
    slots: HashMap<String, f64>,
}
// Memory構造体にメソッドを定義
impl Memory {
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }
    fn add_and_print(&mut self, token: &str, prev_result: f64){
        let slot_name = token[3..token.len() - 1].to_string();
        match self.slots.entry(slot_name) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += prev_result;
                print_output(*entry.get());
            }
            Entry::Vacant(entry) => {
                entry.insert(prev_result);
                print_output(prev_result);
            }
        }
    }
    fn eval_token(&self, token: &str) -> f64 {
        if token.starts_with("mem") {
            let solt_name = &token[3..];
            self.slots.get(solt_name).copied().unwrap_or(0.0)
        } else {
            token.parse().unwrap()
        }
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
fn print_output(value: f64){
    println!(" => {}", value);
}
// fn eval_token(token: &str, memory: &Memory) -> f64 {
//     // ifとelseの返り値は同じでなければいけないというrustの制約から、
//     // 変数memoryの型から推論が効くようになるため let right: f64 と書かなくてもよい
//     if token.starts_with("mem") {
//         let solt_name = &token[3..];
//         for slot in &memory.slots{
//             if slot.0 == solt_name {
//                 return slot.1;
//             }
//         }
//         0.0
//     } else {
//         token.parse().unwrap()
//     }
// }
// fn add_and_print_memory(
//     memory: &mut Memory,
//     token: &str,
//     prev_result: f64
// ){
//     let slot_name = &token[3..token.len() -1];
//     for slot in memory.slots.iter_mut(){
//         if slot.0 == slot_name {
//             slot.1 += prev_result;
//             print_output(slot.1);
//             return;
//         }
//     }
//     memory.slots.push((slot_name.to_string(), prev_result));
//     print_output(prev_result);
// }
// fn subtract_and_print_memory(
//     memory: &mut Memory,
//     token: &str,
//     prev_result: f64
// ){
//     let slot_name = &token[3..token.len() -1];
//     for slot in memory.slots.iter_mut(){
//         if slot.0 == slot_name {
//             slot.1 += prev_result;
//             print_output(slot.1);
//             return;
//         }
//     }
//     memory.slots.push((slot_name.to_string(), prev_result));
//     print_output(prev_result);
// }
// デバックようなのか出力なのかがわかるため有用
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


