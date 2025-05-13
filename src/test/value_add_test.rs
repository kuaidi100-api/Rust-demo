use crate::value_add::*;
use mockall::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        address_resolution::address_resolution();
        
        auto_number::auto_number();
        
        back_order::back_order();
        
        det_ocr::det_ocr();
        
        estimate_price::estimate_price();
        
        estimate_time::estimate_time();
        
        intercept_order::intercept_order();
        
        reachable::reachable();
        
        sms_send::sms_send();
        
    }
}