pub fn q1() -> u64 {
    (1..1000).filter(|x| (x % 3 == 0) || (x % 5 == 0)).sum()
}

pub fn fibonacci(i: u64) -> u64 {
    if i == 0 || i == 1 {
        1
    } else {
        fibonacci(i - 1) + fibonacci(i - 2)
    }
}

pub fn q2() -> u64 {
    let mut v: Vec<u64> = vec![];
    for i in 1.. {
        let f = fibonacci(i);
        v.push(f);
        if f > 4_000_000 {
            break;
        }
    }
    v.iter().filter(|&n| n % 2 == 0).sum()
}

pub fn is_palindromic(n: u64) -> bool {
    let s = n.to_string();
    let m = s.len() / 2;
    let xs = s.bytes().take(m);
    let ys = s.bytes().rev().take(m);
    xs.zip(ys).all(|(x, y)| x == y)
}

pub fn q4() -> u64 {
    let mut m = 0;
    for i in 100..999 {
        for j in (100..999).rev() {
            if i < j {
                continue;
            }
            let p = i * j;
            if is_palindromic(p) {
                if m < p {
                    m = p;
                }
                break;
            }
        }
    }
    m
}
