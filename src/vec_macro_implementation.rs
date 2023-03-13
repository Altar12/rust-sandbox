// vec! macro implementation
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($($ele:expr),+ $(,)?) => {{
        let mut res = Vec::new();
        $(
            res.push($ele);
        )+
        res
    }};
    ($m:expr; $n:expr) => {{
        let ele = $m;
        let mut res = Vec::new();
        for _ in 0..$n {
            res.push(ele.clone());
        }
        res
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn macro_works() {
        let vec1: Vec<i32> = vec![];
        let vec2: Vec<i32> = my_vec![];
        assert_eq!(vec1, vec2, "first matcher incorrectly implemented");
        assert_eq!(
            vec![1, 2],
            my_vec![1, 2,],
            "second matcher incorrectly implemented"
        );
        assert_eq!(
            vec![String::from("hello"); 5],
            my_vec![String::from("hello");5],
            "third matcher incorrectly implemented"
        );
    }
}
