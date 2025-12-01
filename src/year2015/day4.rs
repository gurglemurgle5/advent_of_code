use std::num::Wrapping;

use aoc_utils::Day;

pub struct Day4(String);

impl Day for Day4 {
    fn init(input: String) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(input))
    }

    fn part1(&self) -> String {
        let input = self.0.trim();
        let mut num = 0;
        loop {
            let hash = md5(format!("{input}{num}").as_bytes());
            if &hash[0..5] == "00000" {
                return num.to_string();
            }
            num += 1;
        }
    }

    fn part2(&self) -> String {
        let input = self.0.trim();
        let mut num = 0;
        loop {
            let hash = md5(format!("{input}{num}").as_bytes());
            if &hash[0..6] == "000000" {
                return num.to_string();
            }
            num += 1;
        }
    }
}

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

fn md5(input: &[u8]) -> String {
    let len = input.len() as u64 * 8;
    let mut buf = input.to_owned();
    buf.push(0b1000_0000);
    while buf.len() % 64 != 56 {
        buf.push(0);
    }
    buf.extend(len.to_le_bytes());
    assert!(buf.len().is_multiple_of(64));

    let mut a0 = Wrapping(0x67452301);
    let mut b0 = Wrapping(0xefcdab89);
    let mut c0 = Wrapping(0x98badcfe);
    let mut d0 = Wrapping(0x10325476);

    for chunk in buf.as_chunks::<64>().0 {
        let m: Vec<u32> = chunk
            .as_chunks()
            .0
            .iter()
            .map(|word| u32::from_le_bytes(*word))
            .collect();
        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        for i in 0..=63 {
            let mut f: Wrapping<u32>;
            let g;
            match i {
                0..=15 => {
                    f = (b & c) | (!b & d);
                    g = i
                }
                16..=31 => {
                    f = (d & b) | (!d & c);
                    g = (5 * i + 1) % 16;
                }
                32..=47 => {
                    f = b ^ c ^ d;
                    g = (3 * i + 5) % 16;
                }
                48..=63 => {
                    f = c ^ (b | !d);
                    g = (7 * i) % 16
                }
                _ => unreachable!("i should only have itered between the matched values"),
            }
            f += a + Wrapping(K[i]) + Wrapping(m[g]);
            a = d;
            d = c;
            c = b;
            b += (f.0).rotate_left(S[i]);
        }
        a0 += a;
        b0 += b;
        c0 += c;
        d0 += d;
    }

    let digest: [u8; 16] = [
        a0.0.to_le_bytes(),
        b0.0.to_le_bytes(),
        c0.0.to_le_bytes(),
        d0.0.to_le_bytes(),
    ]
    .as_flattened()
    .try_into()
    .expect("what");

    digest
        .into_iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn md5_test() {
        assert_eq!(
            md5("The quick brown fox jumps over the lazy dog".as_bytes()),
            "9e107d9d372bb6826bd81d3542a419d6"
        );
        assert_eq!(
            md5("The quick brown fox jumps over the lazy dog.".as_bytes()),
            "e4d909c290d0fb1ca068ffaddf22cbd0"
        );
        assert_eq!(md5("".as_bytes()), "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(Day4::init("abcdef".into()).part1(), "609043");
        assert_eq!(Day4::init("pqrstuv".into()).part1(), "1048970");
    }
}
