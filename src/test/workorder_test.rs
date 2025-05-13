use crate::workorder::*;
use mockall::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {

        work_order_create::work_order_create();
        
        work_order_query::work_order_query();
        
        work_order_reply::work_order_reply();
        
        work_order_upload::work_order_upload("C:\\Users\\Public\\Downloads\\test.txt");
    }
}