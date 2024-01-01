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

fn print_12_days_of_christmas() {
    println!();
    let gifts = ["a partridge in a pear tree!", "Two turtle doves", "Three french hens", "Four calling birds",
    "Five golden rings!", "Six geese-a-layin'", "Seven swans-a-swimmin'", "Eight maids-a-milkin'", "Nine ladies dancin'",
    "Ten lords-a-leapin'", "Eleven pipers pipin'", "Twelve drummers drummin'"];
    let order_strs = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
    "tenth", "eleventh", "twelfth"];
    for day_num in 0..=11 {
        println!("On the {} day of Christmas,", order_strs[day_num]);
        println!("My true love sent to me:");
        let mut gift_num:i8 = day_num as i8;
        while gift_num >= 0 {
            let mut str_pr = format!("{},", gifts[gift_num as usize]);
            if gift_num == 4 || gift_num == 0 {
                str_pr = str_pr.trim_matches(',').parse().unwrap();
                if gift_num == 0 && day_num != 0 {
                    str_pr = format!("And {}", str_pr);
                }
            }
            println!("{}", str_pr);
            gift_num -= 1;
        }
        println!();
    }
}

fn main() {
    for t in TS {
        let c = conv_temp(&t);
        let mut str = format!("Converted {}°{} to {}°{}.", t.temp, t.tc.to_string(), c.temp, c.tc.to_string());
        if t.tc == TC::K { str = str.replacen("°", "", 1); }
        println!("{str}");
    }
    for x in 1..=100 { println!("Found Fibonacci number {x}: {}", gen_nth_fibonacci(x)) }
    print_12_days_of_christmas();
}
