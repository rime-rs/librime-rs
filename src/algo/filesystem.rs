use std::time::{SystemTime, UNIX_EPOCH};

#[inline]
pub fn to_time_t<T>(tp: T) -> i64
where
    T: Into<SystemTime>,
{
    // Convert the given time point (tp) into SystemTime
    let system_time: SystemTime = tp.into();

    // Get the duration since UNIX_EPOCH
    let duration = system_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // Return the number of seconds since UNIX_EPOCH as a 64-bit integer
    duration.as_secs() as i64
}
