use crate::base::query::{self};
use mockall::*;

#[cfg(test)]

mod tests {
    use super::*;
    
    // 测试query函数
    #[test]
    fn test_query() {
        // 执行被测试的函数
        query::query();
    }
}