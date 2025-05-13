use crate::corder::*;
use mockall::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {

        corder_create::create();
        
        corder_price::price();
        
        corder_cancel::cancel();
        
        corder_detail::detail();
    }
}