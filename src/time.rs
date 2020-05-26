pub fn parse_time(time_string: &str, rate_per_second: u64) -> u64 {
  const MINUTE_IN_SECONDS: u64 = 60;
  const HOUR_IN_SECONDS: u64 = MINUTE_IN_SECONDS * 24;
  const DAY_IN_SECONDS: u64 = HOUR_IN_SECONDS * 24;
  const MONTH_IN_SECONDS: u64 = DAY_IN_SECONDS * 30;
  const YEAR_IN_SECONDS: u64 = MONTH_IN_SECONDS * 12;

  return match time_string.chars().last() {
    Some('s') => {
      let len = time_string.len();
      &time_string[0..len - 1]
        .parse::<u64>()
        .expect("Time string must be a number optionally ending in a unit")
        * rate_per_second
    }
    Some('m') => {
      let len = time_string.len();
      &time_string[0..len - 1]
        .parse::<u64>()
        .expect("Time string must be a number optionally ending in a unit")
        * MINUTE_IN_SECONDS
        * rate_per_second
    }
    Some('h') => {
      let len = time_string.len();
      &time_string[0..len - 1]
        .parse::<u64>()
        .expect("Time string must be a number optionally ending in a unit")
        * HOUR_IN_SECONDS
        * rate_per_second
    }
    Some('D') => {
      let len = time_string.len();
      &time_string[0..len - 1]
        .parse::<u64>()
        .expect("Time string must be a number optionally ending in a unit")
        * DAY_IN_SECONDS
        * rate_per_second
    }
    Some('M') => {
      let len = time_string.len();
      &time_string[0..len - 1]
        .parse::<u64>()
        .expect("Time string must be a number optionally ending in a unit")
        * MONTH_IN_SECONDS
        * rate_per_second
    }
    Some('Y') => {
      let len = time_string.len();
      &time_string[0..len - 1]
        .parse::<u64>()
        .expect("Time string must be a number optionally ending in a unit")
        * YEAR_IN_SECONDS
        * rate_per_second
    }
    Some(_) => time_string
      .parse()
      .expect("Time string must be a number optionally ending in a unit"),
    None => panic!("Time string must not be empty"),
  };
}
