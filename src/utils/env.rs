use std::{env, str::FromStr};

use log::warn;
use num::PrimInt;

pub fn load_env_int<T>(env_key: &str, default_value: T) -> T
where
    T: FromStr + PrimInt + std::fmt::Display,
{
    env::var(env_key)
        .ok()
        .and_then(|val| {
            val.parse::<T>()
                .map_err(|_| {
                    warn!(
                        "Wrong format of an integer env key: {}. Using default value: {}.",
                        env_key, default_value
                    )
                })
                .ok()
        })
        .unwrap_or(default_value)
}
