use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 04)
}

fn md5(mut bytes: Vec<u8>) -> u128 {
    #[rustfmt::skip]
    const S: [u32; 64] = [
        7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
        5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
        4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
        6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21
    ];

    #[rustfmt::skip]
    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
        0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
        0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
        0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
        0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
        0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
        0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
        0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];

    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;

    let og_length = bytes.len() * 8;
    bytes.push(0x80);
    while bytes.len() % 64 != 56 {
        bytes.push(0);
    }
    bytes.extend_from_slice(&og_length.to_le_bytes());

    for chunk in bytes.chunks_exact(64) {
        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        for i in 0u32..64 {
            let (f, g) = match i {
                _ if (0..16).contains(&i) => ((b & c) | (!b & d), i),
                _ if (16..32).contains(&i) => ((b & d) | (!d & c), (5 * i + 1) % 16),
                _ if (32..48).contains(&i) => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };
            let g = g as usize * 4;
            let indexed = u32::from_le_bytes([chunk[g], chunk[g + 1], chunk[g + 2], chunk[g + 3]]);
            let f = f
                .wrapping_add(a)
                .wrapping_add(K[i as usize])
                .wrapping_add(indexed);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(S[i as usize]));
        }

        a0 += a;
        b0 += b;
        c0 += c;
        d0 += d;
    }

    let [d0, d1, d2, d3] = d0.to_le_bytes();
    let [c0, c1, c2, c3] = c0.to_le_bytes();
    let [b0, b1, b2, b3] = b0.to_le_bytes();
    let [a0, a1, a2, a3] = a0.to_le_bytes();

    u128::from_le_bytes([
        d3, d2, d1, d0, c3, c2, c1, c0, b3, b2, b1, b0, a3, a2, a1, a0,
    ])
}

fn find(key: &str, prefix: &str) -> i64 {
    for i in 1.. {
        let test = format!("{key}{i}");
        let md5 = format!("{:032x}", md5(test.into_bytes()));

        if md5.starts_with(prefix) {
            return i;
        }
    }

    -1
}

fn part1(key: &str) -> i64 {
    let key = key.trim();
    find(key, "00000")
}

fn part2(key: &str) -> i64 {
    let key = key.trim();
    find(key, "000000")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(part1("abcdef"), 609043);
    assert_eq!(part1("pqrstuv"), 1048970);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 254575);
    assert_eq!(part2(input), 1038736);
}
