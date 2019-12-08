#[macro_export]
macro_rules! hashed_fill {
    ($type:ty ,$vec:expr) => {
        {
            let mut hashset : HashSet<$type> = HashSet::new();
            for x in $vec {
                hashset.insert(x);
            }
            hashset
        }

    };
}