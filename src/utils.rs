use std::str::FromStr;

pub fn csv<T>(input: &str) -> Vec<T>
where T: FromStr,
     <T as std::str::FromStr>::Err: std::fmt::Debug
{
    input.split(',').map(|e| T::from_str(e).expect("This should not happen!")).collect()
}

pub fn wsv<T>(input: &str) -> Vec<T>
where T: FromStr,
     <T as std::str::FromStr>::Err: std::fmt::Debug
{
    input.split_whitespace().map(|e| T::from_str(e).expect("This should not happen!")).collect()
}

