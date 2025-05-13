use crate::monitor::*;
use mockall::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        monitor_orderExport::order_export();
        
        monitor_sendOut::send_out();
    }
}