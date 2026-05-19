// Compute message-vector degree sets for K=4 using RFC 5053 Trip[] and LTEnc[].
//
// This example prints every intermediate step so you can follow the math.
// Comments are in English by request; printed explanations are in Chinese.

use fountain_raptor_10::generator::{degree_generator::deg, random_generator::rand};

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut d = 3u32;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn next_prime(mut n: u32) -> u32 {
    while !is_prime(n) {
        n += 1;
    }
    n
}

fn binomial(n: u32, k: u32) -> u64 {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let k = k.min(n - k);
    let mut result: u64 = 1;
    for i in 0..k {
        result = result * (n - i) as u64 / (i + 1) as u64;
    }
    result
}

fn derive_l_parameters(k: u32) -> (u32, u32, u32, u32, u32) {
    // Implements RFC 5053 Section 5.4.2.3
    // Returns (X, S, H, L, L')
    let mut x = 1u32;
    while x * (x - 1) < 2 * k {
        x += 1;
    }

    let s_threshold = ((k as f64 * 0.01).ceil() as u32) + x;
    let s = next_prime(s_threshold);

    let target = k + s;
    let mut h = 1u32;
    loop {
        let h_half = ((h as f64) / 2.0).ceil() as u32;
        if binomial(h, h_half) >= target as u64 {
            break;
        }
        h += 1;
    }

    let l = k + s + h;
    let l_prime = next_prime(l);
    (x, s, h, l, l_prime)
}

fn trip(x: u32, l_prime: u32, j_k: u32) -> (u32, u32, u32, u32, u32, u32, u32) {
    // Implements RFC 5053 Section 5.4.4.4
    // Returns (A, B, Y, v, d, a, b)
    const Q: u32 = 65521;
    let a_cap = (53591 + j_k * 997) % Q;
    let b_cap = (10267 * (j_k + 1)) % Q;
    let y = (b_cap + x * a_cap) % Q;

    let v = rand(y, 0, 1 << 20);
    let d = deg(v);
    let a = 1 + rand(y, 1, l_prime - 1);
    let b = rand(y, 2, l_prime);
    (a_cap, b_cap, y, v, d, a, b)
}

fn ltenc_indices(l: u32, l_prime: u32, d: u32, a: u32, b: u32) -> (Vec<u32>, Vec<String>) {
    // Implements RFC 5053 Section 5.4.4.3 but returns index set instead of XOR result.
    let mut trace: Vec<String> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let mut b_cur = b;
    trace.push(format!("初始化: b = {}", b_cur));
    while b_cur >= l {
        let next = (b_cur + a) % l_prime;
        trace.push(format!("  b >= L ({} >= {}) -> b = (b + a) % L' = ({} + {}) % {} = {}", b_cur, l, b_cur, a, l_prime, next));
        b_cur = next;
    }
    trace.push(format!("  现在 b < L ({} < {})，选中第 1 个索引 {}", b_cur, l, b_cur));
    indices.push(b_cur);

    let iters = std::cmp::min(d.saturating_sub(1), l.saturating_sub(1));
    for j in 1..=iters {
        let stepped = (b_cur + a) % l_prime;
        trace.push(format!("j = {}: 先走一步 b = ({} + {}) % {} = {}", j, b_cur, a, l_prime, stepped));
        b_cur = stepped;
        while b_cur >= l {
            let next = (b_cur + a) % l_prime;
            trace.push(format!("  b >= L ({} >= {}) -> b = ({} + {}) % {} = {}", b_cur, l, b_cur, a, l_prime, next));
            b_cur = next;
        }
        trace.push(format!("  现在 b < L ({} < {})，选中索引 {}", b_cur, l, b_cur));
        indices.push(b_cur);
    }

    (indices, trace)
}

fn main() {
    // We compute for message vectors i=0..K-1 where (d[i],a[i],b[i]) = Trip[K,i].
    let k: u32 = 4;
    let j_k: u32 = 18; // From RFC 5053 Section 5.7 list: first entry corresponds to K=4.

    println!("=== RFC5053: K=4 每个 message vector 的 degree set（逐步输出）===\n");

    // Step A: derive L parameters from K
    let (x_val, s, h, l, l_prime) = derive_l_parameters(k);
    let h_prime = ((h as f64) / 2.0).ceil() as u32;

    println!("[Step A] 由 K 推导参数（RFC 5.4.2.3）");
    println!("K = {}", k);
    println!("X: 最小正整数使得 X*(X-1) >= 2*K");
    println!("  2*K = {}", 2 * k);
    println!("  X = {}", x_val);
    println!("S: 最小素数使得 S >= ceil(0.01*K) + X");
    println!("  ceil(0.01*K) = {}", ((k as f64) * 0.01).ceil() as u32);
    println!("  S = {}", s);
    println!("H: 最小整数使得 choose(H, ceil(H/2)) >= K + S");
    println!("  K + S = {}", k + s);
    println!("  H = {}", h);
    println!("H' = ceil(H/2) = {}", h_prime);
    println!("L = K + S + H = {}", l);
    println!("L' = 最小素数且 L' >= L = {}", l_prime);
    println!();

    // Step B/C: Trip + LTEnc for each i
    println!("[Step B] 系统索引 J(K)（RFC 5.7）");
    println!("J(4) = {}\n", j_k);

    for i in 0..k {
        println!("------------------------------");
        println!("[message vector i = {}]", i);

        println!("\n[Step C] 计算 Trip[K,i]（RFC 5.4.4.4）");
        let (a_cap, b_cap, y, v, d, a, b) = trip(i, l_prime, j_k);
        println!("Q = 65521");
        println!("A = (53591 + J(K)*997) % Q = (53591 + {}*997) % {} = {}", j_k, 65521, a_cap);
        println!("B = 10267*(J(K)+1) % Q = 10267*({}+1) % {} = {}", j_k, 65521, b_cap);
        println!("Y = (B + X*A) % Q = ({} + {}*{}) % {} = {}", b_cap, i, a_cap, 65521, y);
        println!("v = Rand[Y, 0, 2^20] = Rand[{},0,1048576] = {}", y, v);
        println!("d = Deg[v] = Deg[{}] = {}", v, d);
        println!("a = 1 + Rand[Y, 1, L'-1] = 1 + Rand[{},1,{}] = {}", y, l_prime - 1, a);
        println!("b = Rand[Y, 2, L'] = Rand[{},2,{}] = {}", y, l_prime, b);

        println!("\n[Step D] 用 LTEnc 生成 degree set（RFC 5.4.4.3）");
        let (indices, trace) = ltenc_indices(l, l_prime, d, a, b);
        for line in trace {
            println!("{}", line);
        }
        println!("=> degree (选中索引个数) = {}", indices.len());
        println!("=> degree set = {:?}", indices);
        println!();
    }

    println!("=== 完成 ===");
}


