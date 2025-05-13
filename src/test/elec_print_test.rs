use crate::elec_print::*;
use mockall::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        
        authThird::auth_third();
        
        elec_order::order();
        
        elec_cancel::cancel();
        
        elec_custom::custom();
        
        elec_printOld::print_old();
        
        logistic_send::logistic_send();
        
        order_task::logistic_send();
        
        print_task::print_task();
        
        refundOrder_task::refund_order_task();
        
        shop_authorize::shop_authorize();
        
        third_info::get_third_info();
    }
}