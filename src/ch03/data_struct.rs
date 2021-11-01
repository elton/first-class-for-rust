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

// 数据结构
// struct 定义结构体
// enum 标签联合体(tagged union)
// tuple 定义元组

pub fn data_struct() {
    let alice = User {
        id: UserId(1),
        name: "Alice".into(),
        gender: Gender::Female,
    };

    let bob = User {
        id: UserId(2),
        name: "Bob".into(),
        gender: Gender::Male,
    };

    let topic = Topic {
        id: TopicId(1),
        name: "rust".into(),
        owner: UserId(1),
    };

    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "hello world!".into()));

    println!(
        "event1: {:?}, event2: {:?}, event3: {:?}",
        event1, event2, event3
    );
}

// 枚举结构体
#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

// 元组结构体， 它的字段都是匿名的，可以通过索引访问，适合简单的结构体。
// Debug - 表示打印调试信息， Clone - 表示让数据结构可以被复制， Copy - 让数据结构在参数传递的时候，自动按照字节拷贝
#[derive(Debug, Clone, Copy)]
struct UserId(u64);

// 元组结构体， 它的字段都是匿名的，可以通过索引访问，适合简单的结构体。
#[derive(Debug, Clone, Copy)]
struct TopicId(u64);

// 标准结构体
#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

// 标签结构体
#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}
