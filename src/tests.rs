mod tests {
    #![allow(dead_code, unused_variables, unused_imports, non_snake_case)]
    use crate::{Contract, SEC_PER_HOUR, SEC_PER_DAY, NANO_POW};

    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{assert_self, testing_env, MockedBlockchain, VMContext, env};
    use std::convert::TryInto;
    use std::time::{Duration, Instant, SystemTime};

    fn get_context(signer: String, owner: String, is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(signer.to_owned().try_into().unwrap())
            .current_account_id(owner.to_owned().try_into().unwrap())
            .is_view(is_view)
            .build()
    }

    fn setup(signer: String, owner: String, is_view: bool) -> Contract {
        let context = get_context(signer, owner, false);
        testing_env!(context.clone());
        Contract::default()
    }

    fn get_sys_hour() -> u32 {
        if let Ok(n) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            (((n.as_nanos() as u64 / NANO_POW) % SEC_PER_DAY) / SEC_PER_HOUR) as u32
        } else {
            println!("some error while fetchin the time in nano secs");
            0b0
        }
    }

    #[test]
    #[should_panic]
    fn change_owner() {
        let mut contract = setup("signer".to_string(), "owner".to_string(), false);
        assert!(
            contract.change_owner("owner".to_string()),
            "only owner can call this function"
        );
    }

    #[test]
    #[should_panic]
    fn change_owner_1() {
        let mut contract = setup("owner".to_string(), "owner".to_string(), false);
        assert!(
            contract.change_owner("owner".to_string()),
            "new owner can not be the same owner"
        );
    }

    #[test]
    fn change_owner_2() {
        let mut contract = setup("owner".to_string(), "owner".to_string(), false);
        println!(
            "Signer {} , Owner {}",
            env::signer_account_id(),
            env::current_account_id()
        );
        assert!(contract.change_owner("user".to_string()));
    }

    #[test]
    fn get_count() {
        let contract = setup("signer".to_string(), "owner".to_string(), false);
        assert_eq!(
            contract.get_count(),
            0,
            "At the begining, the default lamp should have count (id) 0"
        );
    }

    #[test]
    #[ignore = "This test depends on current time of the day. e.g. if you will try to switch off a lamp at dark time , its expected someone will yell :)"]
    fn can_set_lamp_state() {
        let lamp_state = String::from("Off");
        let contract = setup("signer".to_string(), "owner".to_string(), false);
        assert!(
            contract.can_set_lamp_state(lamp_state.clone()),
            "Why system wants to switch {} a lamp at this time ?. Current hour : {}",
            lamp_state,
            get_sys_hour(),
        );
    }

    #[test]
    #[should_panic]
    fn new_lamp() {
        let mut contract = setup("signer".to_string(), "owner".to_string(), false);
        assert!(contract.new_lamp("Off".to_string(), "Neon".to_string()) > 0);
    }

    #[test]
    fn new_lamp_1() {
        let mut contract = setup("owner".to_string(), "owner".to_string(), false);
        assert!(contract.new_lamp("Off".to_string(), "Neon".to_string()) > 0);
    }

    #[test]
    fn disable_lamp() {
        let lamp_id: u16 = 1;
        let mut contract = setup("owner".to_string(), "owner".to_string(), false);
        contract.new_lamp("Off".to_string(), "Neon".to_string());
        contract.disable_lamp(lamp_id);
        let lamp = match contract.get_lamp(lamp_id) {
            Some(la) => la,
            None => panic!("No lamp found"),
        };
        // is_active = false means disable
        assert!(lamp.is_active == false, "Should not fail");
    }
}
