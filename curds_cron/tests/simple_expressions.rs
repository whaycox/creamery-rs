use curds_cron::CronExpression;
use chrono::prelude::{DateTime, Utc};

macro_rules! test_expression {
    ($expression:expr => ($($test_date:expr => $expected:expr),+)) => {
        let test_object = $expression.parse::<CronExpression>().unwrap();
        
        $(
            assert_eq!($expected, test_object.is_match(&$test_date.parse::<DateTime<Utc>>().unwrap()),
                "Expected {:?} to fire {:?} for {:?}", $expression, $expected, $test_date);
        )+        
    };
    ($expression:expr => now => $expected:expr) => {
        let test_date = Utc::now();

        let test_object = $expression.parse::<CronExpression>().unwrap();

        assert_eq!($expected, test_object.is_match(&test_date), "Expected {:?} to fire {:?} for {:?}", $expression, $expected, &test_date);
    }
}

#[test]
fn parses_all_wildcard_expression() {
    test_expression!("* * * * *" => now => true);
}

#[test]
fn parses_specific_time() {
    test_expression!("1 2 3 4 5" => 
    (
        "2021-04-03T02:01:00Z" => false,
        "2020-04-03T02:01:00Z" => true,
        "2019-04-03T02:01:00Z" => false
    ));
}

#[test]
fn parses_steps() {
    test_expression!("*/3 */2 */5 */2 */2" => 
    (
        "2021-03-11T02:13:00Z" => false,
        "2021-03-11T02:12:00Z" => true,
        "2021-03-10T02:12:00Z" => false
    ));
}

#[test]
fn parses_ranges() {    
    test_expression!("2-10 18-20 5-8 MAR-JUN SUN-WED" => 
    (
        "2021-02-08T18:07:00Z" => false,
        "2021-03-08T18:07:00Z" => true,
        "2021-04-08T18:07:00Z" => false
    ));
}

#[test]
fn parses_last_day_of_month() {
    test_expression!("* * L FEB *" => 
    (
        "2020-02-29T00:00:00Z" => true,
        "2021-02-28T00:00:00Z" => true,
        "2020-02-28T00:00:00Z" => false
    ));
}

#[test]
fn parses_last_day_of_week() {
    test_expression!("* * * * SUNL" => 
    (
        "2021-04-18T00:00:00Z" => false,
        "2021-04-25T00:00:00Z" => true
    ));
}

#[test]
fn parses_nth_day_of_week() {
    test_expression!("* * * * MON#2" => 
    (
        "2021-04-05T00:00:00Z" => false,
        "2021-04-12T00:00:00Z" => true,
        "2021-04-19T00:00:00Z" => false
    ));
}

#[test]
fn parses_nearest_weekday() {
    test_expression!("* * 18W * *" => 
    (
        "2021-04-18T00:00:00Z" => false,
        "2021-04-19T00:00:00Z" => true
    ));
}