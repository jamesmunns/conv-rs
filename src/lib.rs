#![cfg_attr(feature = "unstable", feature(test))]

pub fn convolution(input_a: &[f64], input_b: &[f64]) -> Vec<f64> {
    let mut vec: Vec<f64> = vec![0f64; (input_a.len() + input_b.len() - 1) as usize];

    for (a_i, &a) in input_a.iter().enumerate() {
        for (b_i, &b) in input_b.iter().enumerate() {
            vec[a_i+b_i] += a*b;
        }
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(convolution(&[2.0, -2.0, 1.0],
                               &[1.0, 3.0, 0.5, -1.0]),
                   [2.0, 4.0, -4.0, 0.0, 2.5, -1.0]);
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use self::test::Bencher;
    use std::f64::consts;

    #[bench]
    fn bench_test(b: &mut Bencher) {
        let mut c: Vec<f64> = Vec::with_capacity(5000);
        let mut d: Vec<f64> = Vec::with_capacity(5000);

        for i in 0..5000 {
            let j = i as f64;
            c.push((consts::PI as f64 * (j/8f64)).sin());
            d.push((consts::PI as f64 * (j/8f64)).cos());
        }

        b.iter(|| {
            convolution(&c, &d)
        });
    }
}
