// Copyright 2021 Elton Zheng
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// rust 是函数式语言，函数是一等公民
pub fn function() {
  println!("apply square: {}", apply(2, square));
  println!("apply cube: {}", apply(2, cube));
}

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
  f(value)
}

// 函数
// 函数的参数类型和返回值的类型，必须都显示的定义
// 如果没有返回值，可以省略，返回 unit
// 函数内部提前返回，需要使用 return 关键字，否则就返回最后一个表达式的值
// 表达式是没有;号的，加了；号就变成了语句
fn square(value: i32) -> i32 {
  value * value
}

fn cube(value: i32) -> i32 {
  value * value * value
}

pub fn fn_or_statement() {
  let is_pi = pi();
  let is_unit1 = not_pi();
  let is_unit2 = {
    pi();
  };
  println!(
    "is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}",
    is_pi, is_unit1, is_unit2
  );
}

fn pi() -> f64 {
  3.1415926
}

fn not_pi() {
  3.1415926;
}
