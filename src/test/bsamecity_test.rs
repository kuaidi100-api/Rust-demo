use crate::bsamecity::*;
use mockall::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {

        bsamecity_order::order();
        
        bsamecity_price::price();

        bsamecity_precancel::pre_cancel();
        
        bsamecity_cancel::cancel();
        
        bsamecity_addfee::add_fee();

    }
}