use crate::border::*;
use mockall::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        
        border_create::create();
        
        border_price::price();
        
        border_cancel::cancel();
        
        border_detail::detail();
        
    }
}