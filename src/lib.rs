//use chrono::{NaiveDateTime,DateTime, Duration, Utc};
//use chrono::{Timelike, Utc};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, log, near_bindgen};
use std::collections::HashMap;

const SEC_PER_HOUR: u64 = 60 * 60;
const SEC_PER_DAY: u64 = 24 * SEC_PER_HOUR;
const NANO_POW: u64 = u64::pow(10, 9);

const HALO_RATE_PER_SEC: f64 = 0.10 / NANO_POW as f64;
const NEON_RATE_PER_SEC: f64 = 0.05 / NANO_POW as f64;

pub fn get_epoc_hour() -> u32 {
    let today_time_in_seconds = (env::block_timestamp() / NANO_POW) % SEC_PER_DAY;
    let today_time_in_hours = today_time_in_seconds / SEC_PER_HOUR;
    log!("current_time_in_hours (GMT): {} ", today_time_in_hours);
    today_time_in_hours as u32
}

#[derive(Serialize, BorshDeserialize, BorshSerialize, Debug)]
pub enum LampType {
    Halogen,
    Neon,
}

impl Default for LampType {
    fn default() -> Self {
        LampType::Neon
    }
}

#[derive(Serialize, BorshDeserialize, BorshSerialize, PartialEq, Debug)]
pub enum LampState {
    On,
    Off,
}

impl Default for LampState {
    fn default() -> Self {
        LampState::Off
    }
}


#[derive(Serialize, BorshDeserialize, BorshSerialize, Debug)]
pub struct Lamp {
    lamp_id: u16,
    lamp_type: LampType,
    lamp_state: LampState,
    updated_by: String,
    updated_on: u64,
    is_active: bool,
}

#[derive(Serialize, BorshDeserialize, BorshSerialize)]
#[near_bindgen]
pub struct Contract {
    owner: String,
    last_id: u16,
    ontime: u32,
    offtime: u32,
    lamp: HashMap<u16, Lamp>,
}

#[near_bindgen]
impl Default for Contract {
    fn default() -> Self {
        Self {
            last_id: 0,
            owner: env::current_account_id(),
            // a lamp can be switched on only or after 18:00 GMT
            ontime: 18,
            // a lamp can be switced off only or after 6:00 GMT
            offtime: 6,
            lamp: HashMap::new(),
        }
    }
}

#[derive(Serialize, BorshDeserialize, BorshSerialize)]
pub struct Reward {
    reward_to: String,
    amount: f64,
    current_time: u64,
}

#[near_bindgen]
impl Contract {

    #[private]
    pub fn gen_lamp(&self, state: String, ltype: String) -> (u16, Lamp) {
        let new_id: u16 = self.last_id + 1;
        let state: LampState = if state == "On" {
            LampState::On
        } else {
            LampState::default()
        };

        let l_type = if ltype == "Halogen" {
            LampType::Halogen
        } else {
            LampType::default()
        };

        let new_lamp = Lamp {
            lamp_id: new_id,
            lamp_type: l_type,
            lamp_state: state,
            updated_by: self.owner.to_string(),
            updated_on: env::block_timestamp(),
            is_active: true,
        };

        (new_id, new_lamp)
    }

    // only contract owner
    #[payable]
    pub fn new_lamp(&mut self, state: String, ltype: String) -> u16 {
        // validate state and ltype for their respective enums
        self.only_owner_allowed();

        let (id, new_lamp_struct) = self.gen_lamp(state, ltype);
        self.lamp.insert(id, new_lamp_struct);
        self.last_id = id;
        id
    }
    
    pub fn get_count(&self) -> u16 {
        self.last_id
    }
    
        // a lamp can not be deleted from blockchain , it can be de-activated only
    #[payable]
    pub fn disable_lamp(&mut self, lamp_id: u16) {
        self.only_owner_allowed();

        if let Some(lamp) = self.lamp.get_mut(&lamp_id) {
            if lamp.is_active == false {
                env::log(b"Lamp is already inactive");
            } else {
                lamp.is_active = false;
            }
        } else {
            panic!("No lamp exists for id : {}", lamp_id);
        }
    }

    // once the maintainance of lamp is over we can enable the lamp
    #[payable]
    pub fn enable_lamp(&mut self, lamp_id: u16) {
        self.only_owner_allowed();

        if let Some(lamp) = self.lamp.get_mut(&lamp_id) {
            if lamp.is_active == true {
                env::log(b"Lamp is already active");
            } else {
                lamp.is_active = true;
            }
        } else {
            panic!("No lamp exists for id : {}", lamp_id);
        }
    }
       
    pub fn get_all_lamps(&self) -> &HashMap<u16, Lamp> {
        &self.lamp
    }

    pub fn get_lamp(&self, lamp_id: u16) -> Option<&Lamp> {
        self.lamp.get(&lamp_id)
    }

    pub fn can_set_lamp_state(&self, new_state: String) -> bool {
        let now_hour = get_epoc_hour();
        if new_state == "On" {
            if now_hour < self.offtime || now_hour >= self.ontime {
                return true;
            }
            log!(
                "not a time to switch on a lamp"
            );
            return false;
        } else if new_state == "Off" {
            if now_hour < self.ontime && now_hour >= self.offtime {
                return true;
            }
            log!(
                "not a time to switch off a lamp"
            );
            return false;
        } else {
            log!("Valid states for a lamp is 'On' or 'Off'");
            return false;
        }
    }
   
    #[payable]
    pub fn set_lamp_state(&mut self, lamp_id: u16, new_state: String) -> bool {
        self.owner_not_allowed();

        if !(new_state == "On" || new_state == "Off") {
            log!("Valid states for a lamp are 'On' and 'Off'");
            return false;
        }

        let state: LampState = if new_state == "On" {
            LampState::On
        } else {
            LampState::Off
        };

        if self.can_set_lamp_state(new_state.to_owned()) {
            if let Some(lamp_struct) = self.lamp.get_mut(&lamp_id) {
                if lamp_struct.is_active {
                    if lamp_struct.lamp_state != state {
                        lamp_struct.lamp_state = state;
                        lamp_struct.updated_by = env::signer_account_id();
                        lamp_struct.updated_on = env::block_timestamp();
                        log!("Great work!!. One day you will be rewarded for this kind deed.");
                    } else {
                        log!("Lamp state is already in {} state", new_state);
                        return false;
                    }
                } else {
                    log!("Lamp id is not active, can not be updated");
                    return false;
                }
            } else {
                log!("No lamp exists for id : {}", lamp_id);
                return false;
            };
        } else {
            return false;
        }

        return true;
    }

    pub fn get_owner(&self) -> &str {
        &self.owner
    }

    #[payable]
    pub fn change_owner(&mut self, new_owner: String) -> bool {
        self.only_owner_allowed();
        if new_owner != self.owner {
            self.owner = new_owner;
            return true;
        }
        panic!("existing owner is same as asked new owner");
    }

    pub fn only_owner_allowed(&self) {
        if env::signer_account_id() == self.owner {
            return;
        }
        env::panic(b"Only contract owner can invoke this method");
    }

    pub fn owner_not_allowed(&self) {
        if env::signer_account_id() != self.owner {
            return;
        }
        env::panic(b"Contract owner can not invoke this method");
    }  
    
    // reward_for can have value Self and All
    pub fn calculate_reward(&self, reward_for: String) -> Option<HashMap<u16, Reward>> {
        if reward_for == "All" {
            self.only_owner_allowed();
        }

        if !(reward_for == "Self" || reward_for == "All") {
            env::panic(b"Configured values for 'reward_for' are 'Self' and 'All'");
        }

        let mut reward_hashmap: HashMap<u16, Reward> = HashMap::new();

        for i in 1..self.get_count() + 1 {
            match self.get_lamp(i) {
                None => {
                    log!("No lamp exists for lamp_id : {}", i);
                }
                Some(lamp) => {
                    let rate = match &lamp.lamp_type {
                        LampType::Halogen => HALO_RATE_PER_SEC,
                        LampType::Neon => NEON_RATE_PER_SEC,
                    };

                    let current_time = env::block_timestamp();
                    if reward_for == "All" {
                        if lamp.updated_by != self.owner.to_string() {
                            let reward = Reward {
                                reward_to: lamp.updated_by.to_owned(),
                                current_time,
                                amount: ((current_time - lamp.updated_on) / NANO_POW) as f64 * rate,
                            };
                            reward_hashmap.insert(lamp.lamp_id, reward);
                        }
                    } else {
                        if lamp.updated_by == env::signer_account_id().to_string() {
                            let reward = Reward {
                                reward_to: lamp.updated_by.to_owned(),
                                current_time,
                                amount: ((current_time - lamp.updated_on) / NANO_POW) as f64 * rate,
                            };
                            reward_hashmap.insert(lamp.lamp_id, reward);
                        }
                    }
                }
            };
        }

        if reward_hashmap.is_empty() {
            None
        } else {
            Some(reward_hashmap)
        }
    }
    
}

// ******************************************************************* //
// *********************        TEST     ***************************** //
// ******************************************************************* //

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;