use std::str::FromStr;
use num::Complex;

// Parse a string as a pair of values separated by `separator`
pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(idx) => {
            match (T::from_str(&s[..idx]), T::from_str(&s[idx+1..])) {
                (Ok(l), Ok(r)) => Some((l,r)),
                _ => None

            }
        }
    }
}

// Parse a complex number from a string
pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("1x2",'x'), Some((1,2)));
    assert_eq!(parse_pair::<f64>("1.2:1.2",':'), Some((1.2,1.2)));
    assert_eq!(parse_pair::<f64>("1.2x1.2",':'), None);
    assert_eq!(parse_pair::<f64>("1.2xfoo",'x'), None);
}
