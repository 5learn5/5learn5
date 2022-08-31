pub mod utils {
    use near_sdk::{env, log};

    pub const SEC_PER_HOUR: u64 = 60 * 60;
    pub const SEC_PER_DAY: u64 = 24 * SEC_PER_HOUR;
    pub const NANO_POW: u64 = u64::pow(10, 9);
    //pub const YOCTO_NEAR: u128 = u128::pow(10, 24);

    pub const HALO_RATE_PER_SEC: f64 = 0.10 / NANO_POW as f64;
    pub const NEON_RATE_PER_SEC: f64 = 0.05 / NANO_POW as f64;

    pub fn get_epoc_hour() -> u32 {
        let today_time_in_seconds = (env::block_timestamp() / NANO_POW) % SEC_PER_DAY;
        let today_time_in_hours = today_time_in_seconds / SEC_PER_HOUR;
        log!("current_time_in_hours (GMT): {} ", today_time_in_hours);
        today_time_in_hours as u32
    }
}
