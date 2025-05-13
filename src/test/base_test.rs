use crate::base::*;
use mockall::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_query() {
        // 执行被测试的函数
        query::query();
        
        poll::poll();

        poll_map::poll_map();
        
        map_track::map_track();
    }
}