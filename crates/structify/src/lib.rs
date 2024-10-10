use std::sync::Arc;

pub use structify_derive::structify;

pub struct Dep<T> {
    value: Arc<T>,
}

impl<T> Dep<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Arc::new(value),
        }
    }

    pub fn get(&self) -> Arc<T> {
        self.value.clone()
    }

    pub fn inner(&self) -> &T {
        &self.value
    }
}

impl<T> From<Arc<T>> for Dep<T> {
    fn from(value: Arc<T>) -> Self {
        Self { value }
    }
}

impl<T> From<T> for Dep<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{structify, Dep};

    #[structify]
    fn unit() {}

    #[test]
    fn test_unit() {
        Unit::new().execute();
    }

    #[structify]
    fn unit_with_return() -> i32 {
        42
    }

    #[test]
    fn test_unit_with_return() {
        assert_eq!(UnitWithReturn::new().execute(), 42);
    }

    #[structify]
    fn args_with_return(a: i32, b: i32) -> i32 {
        a + b
    }

    #[test]
    fn test_args_with_return() {
        assert_eq!(ArgsWithReturn::new(1, 2).execute(), 3);
    }

    #[structify]
    fn arg_and_dep_with_return(i: i32, dep: Dep<i32>) -> i32 {
        *dep.inner() + i
    }

    #[test]
    fn test_arg_and_dep_with_return() {
        let s = ArgAndDepWithReturn::new(1).execute(41);
        assert_eq!(s, 42);
    }

    #[structify]
    fn mixed_args_and_deps_with_return(dep: Dep<i32>, a: i32, dep2: Dep<i32>, b: i32) -> i32 {
        *dep.inner() + *dep2.inner() + a + b
    }

    #[test]
    fn test_mixed_args_and_deps_with_return() {
        let s = MixedArgsAndDepsWithReturn::new(1, 2).execute(3, 4);
        assert_eq!(s, 10);
    }

    #[structify(NameIsB)]
    fn name_is_a() {}

    #[test]
    fn test_name_is_not_a() {
        NameIsB::new().execute();
    }

    #[structify]
    async fn async_unit_with_return() -> i32 {
        42
    }

    #[tokio::test]
    async fn test_async_unit_with_return() {
        assert_eq!(AsyncUnitWithReturn::new().execute().await, 42);
    }

    #[structify]
    async fn async_args_with_return(a: i32, b: i32) -> i32 {
        a + b
    }

    #[tokio::test]
    async fn test_async_args_with_return() {
        assert_eq!(AsyncArgsWithReturn::new(1, 2).execute().await, 3);
    }

    #[structify]
    async fn async_arg_and_dep_with_return(i: i32, dep: Dep<i32>) -> i32 {
        *dep.inner() + i
    }

    #[tokio::test]
    async fn test_async_arg_and_dep_with_return() {
        let s = AsyncArgAndDepWithReturn::new(1).execute(41).await;
        assert_eq!(s, 42);
    }
}
