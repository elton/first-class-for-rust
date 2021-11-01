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

pub fn fib_loop(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    loop {
        // loop
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);

        if i >= n {
            break;
        }
    }
}

pub fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);
    }
}

pub fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    for _i in 2..n {
        // 2..n means 2<= x < n
        // 2..=n means 2<= x <= n
        let c = a + b;
        a = b;
        b = c;

        println!("next val is {}", b);
    }
}
