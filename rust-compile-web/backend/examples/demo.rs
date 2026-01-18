//! 演示程序 - 包含各种Rust特性用于编译探索

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }
}

#[inline(never)]
fn compute_factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * compute_factorial(n - 1)
    }
}

fn process_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .collect()
}

macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

fn main() {
    let p1 = Point::new(0, 0);
    let p2 = Point::new(3, 4);

    println!("点之间的距离: {:.2}", p1.distance(&p2));

    let n = 5;
    println!("{}的阶乘: {}", n, compute_factorial(n));

    let numbers = vec![-1, 2, -3, 4, 5];
    let processed = process_numbers(&numbers);
    println!("处理后的数字: {:?}", processed);

    debug_print!("这是调试信息");

    // 模式匹配
    match processed.len() {
        0 => println!("没有数字"),
        1 => println!("只有一个数字"),
        n => println!("有{}个数字", n),
    }
}