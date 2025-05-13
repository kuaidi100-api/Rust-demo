use crate::international::*;
use mockall::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        
        api_call::api_call();

        pick_up::pick_up();

        cancel_pick_up::cancel_pick_up();
        
        international_address_resolution::international_address_resolution();
        
    }
}