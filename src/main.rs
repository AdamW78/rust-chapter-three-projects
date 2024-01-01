use std::fmt;

struct Temp {
    temp: f32,
    tc: TC,
}

#[derive(Debug, PartialEq)]
enum TC {
    K,
    F,
    C,
}
// CODE BELOW FROM https://stackoverflow.com/questions/32710187/how-do-i-get-an-enum-as-a-string
impl fmt::Display for TC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
const TS: [Temp; 9] = [Temp{temp: 300.0, tc: TC::K}, Temp{temp: 0.0, tc: TC::C}, Temp{temp: 50.0, tc: TC::C},
                    Temp{temp: 100.0, tc: TC::C}, Temp{temp: -40.0, tc: TC::C}, Temp{temp: 32.0, tc: TC::F},
                    Temp{temp: 212.0, tc: TC::F}, Temp{temp: 72.0, tc: TC::F}, Temp{temp: -40.0, tc: TC::F}];

fn conv_temp(t: &Temp) -> Temp {
    let temp: f32;
    let tc: TC;
    match t.tc {
        TC::F => {
            temp = (5.0/9.0 * (t.temp - 32.0) * 1000.0).round() / 1000.0;
            tc = TC::C;
        }
        TC::C => {
            temp = ((1.8 * t.temp + 32.0) * 1000.0).round() / 1000.0;
            tc = TC::F;
        }
        TC::K => {
            temp = ((1.8 * (t.temp - 273.15) + 32.0) * 1000.0).round() / 1000.0;
            tc = TC::F;
        }
    }
    Temp{ temp, tc }
}

fn gen_nth_fibonacci(n: u8) -> i128 {
    let mut n = n;
    if n == 1 || n == 2 { return 1 }
    let mut n_minus_oneth_fib: i128 = 1; n -= 1;
    let mut nth_fib: i128 = 1; n -= 1;
    while n > 0 {
        let tmp = n_minus_oneth_fib;
        n_minus_oneth_fib = nth_fib;
        nth_fib = nth_fib + tmp;
        n -= 1;
    }
    nth_fib

}

fn main() {
    for t in TS {
        let c = conv_temp(&t);
        let mut str = format!("Converted {}°{} to {}°{}.", t.temp, t.tc.to_string(), c.temp, c.tc.to_string());
        if t.tc == TC::K { str = str.replacen("°", "", 1); }
        println!("{str}");
    }
    for x in 1..=100 { println!("Found Fibonacci number {x}: {}", gen_nth_fibonacci(x)) }
}
