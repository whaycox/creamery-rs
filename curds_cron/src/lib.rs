#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! A library for creating and consuming Cron expressions. It supports some extended syntax.
//! 
//! # Overview
//! Each expression consists of five fields, space separated, representing different parts of the day in the following order:
//! * Minute
//! * Hour
//! * Day of Month
//! * Month
//! * Day of Week
//! 
//! Each field consists of some number of values, comma separated, indicating when the expression is a match.
//! In addition to the regular: 
//! * Single value 
//! * Inclusive range of values
//! * Wildcard
//! 
//! Expressions can also include:
//! * Step ranges
//! * Weekday nearest to Day of Month
//! * Last Day of Month (with or without an offset)
//! * Nth Day of Week
//! * Last Day of Week
//! 
//! Additionally, Month and Day of Week values can be represented numerically or with a three-letter abbreviation (JAN -> 01 or wed -> 3, respectively).
//! 
//! # Examples
//! An expression that is always a match.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let anytime = "* * * * *".parse::<CronExpression>()?;
//! assert_eq!(true, anytime.is_match(&Utc::now()));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! An expression that matches on the last day of February, April, or December.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let end_of_months = "* * L 2,APR,dec *".parse::<CronExpression>()?;
//! assert_eq!(true, end_of_months.is_match(&"2021-04-30T00:00:00Z".parse::<DateTime<Utc>>()?));
//! assert_eq!(false, end_of_months.is_match(&"2021-12-30T00:00:00Z".parse::<DateTime<Utc>>()?));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! An expression that matches at noon on Saturdays.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let saturday_noon = "0 12 * * SAT".parse::<CronExpression>()?;
//! assert_eq!(true, saturday_noon.is_match(&"2021-04-10T12:00:00Z".parse::<DateTime<Utc>>()?));
//! assert_eq!(false, saturday_noon.is_match(&"2021-04-11T12:00:00Z".parse::<DateTime<Utc>>()?));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! An expression that matches at the top of each hour between 8AM and 5PM on the last Friday of the month.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let last_friday = "0 8-17 * * friL".parse::<CronExpression>()?;
//! assert_eq!(false, last_friday.is_match(&"2021-04-23T12:00:00Z".parse::<DateTime<Utc>>()?));
//! assert_eq!(true, last_friday.is_match(&"2021-04-30T12:00:00Z".parse::<DateTime<Utc>>()?));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! An expression that matches on the third Monday of January.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let third_monday = "* * * Jan Mon#3".parse::<CronExpression>()?;
//! assert_eq!(false, third_monday.is_match(&"2021-01-11T00:00:00Z".parse::<DateTime<Utc>>()?));
//! assert_eq!(true, third_monday.is_match(&"2021-01-18T00:00:00Z".parse::<DateTime<Utc>>()?));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! An expression that matches on Wednesdays between the 10th and 20th.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let middle_weds = "* * 10-20 * wed".parse::<CronExpression>()?;
//! assert_eq!(false, middle_weds.is_match(&"2021-01-12T00:00:00Z".parse::<DateTime<Utc>>()?));
//! assert_eq!(true, middle_weds.is_match(&"2021-01-13T00:00:00Z".parse::<DateTime<Utc>>()?));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! An expression that matches every fifth minute five days before the last day of the month.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let fifths = "*/5 * L-5 * *".parse::<CronExpression>()?;
//! assert_eq!(false, fifths.is_match(&"2021-04-25T00:03:00Z".parse::<DateTime<Utc>>()?));
//! assert_eq!(true, fifths.is_match(&"2021-04-25T00:05:00Z".parse::<DateTime<Utc>>()?));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod datepart;
mod expression;
mod field;
mod parsing;
mod value;

use field::CronField;
use parsing::parser::{CronFieldParser, CronValueParsingHandler, CurdsCronParser};
use value::CronValue;
use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Weekday};
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Debug, Display, Error, Formatter};
use std::str::FromStr;

#[cfg(test)]
use mockall::*;
#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use chrono::Utc;
#[cfg(test)]
use parsing::parser::{MockCronFieldParser, MockCronValueParsingHandler};

pub use expression::CronExpression;
pub use datepart::CronDatePart;
pub use parsing::error::CronParsingError;

