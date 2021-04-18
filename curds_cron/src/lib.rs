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
//! * [Single value](#single-values) 
//! * [Inclusive range of values](#range-values)
//! * [Wildcard](#wildcard-values)
//! 
//! Expressions can also include:
//! * [Step ranges](#step-range-values)
//! * [Weekday nearest to Day of Month](#weekday-nearest-to-day-of-month)
//! * [Last Day of Month (with or without an offset)](#last-day-of-month)
//! * [Nth Day of Week](#nth-day-of-week)
//! * [Last Day of Week](#last-day-of-week)
//! 
//! Additionally, Month and Day of Week values can be represented numerically or with a three-letter abbreviation (JAN -> 01 or wed -> 3, respectively).
//! # Examples
//! ## Wildcard Values
//! An expression that is always a match.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let anytime = "* * * * *".parse::<CronExpression>()?;
//! assert_eq!(true, anytime.is_match(&Utc::now()));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! ## Single Values
//! An expression that matches at the start of a new year.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let new_year = "0 0 1 1 *".parse::<CronExpression>()?;
//! assert_eq!(true, new_year.is_match(&"2021-01-01T00:00:00Z".parse::<DateTime<Utc>>()?));
//! assert_eq!(false, new_year.is_match(&"2021-01-01T00:01:00Z".parse::<DateTime<Utc>>()?));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! ## Range Values
//! An expression that matches in the middle of everything.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let middles = "15-45 6-18 10-20 3-9 2-4".parse::<CronExpression>()?;
//! assert_eq!(true, middles.is_match(&"2021-04-13T07:24:00Z".parse::<DateTime<Utc>>()?));
//! assert_eq!(false, middles.is_match(&"2021-04-13T18:46:00Z".parse::<DateTime<Utc>>()?));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! ## Step Range Values
//! An expression that matches on even minutes in even hours on odd days in odd months.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let steps = "*/2 */2 */2 */2 *".parse::<CronExpression>()?;
//! assert_eq!(true, steps.is_match(&"2021-01-05T06:44:00Z".parse::<DateTime<Utc>>()?));
//! assert_eq!(false, steps.is_match(&"2021-01-05T06:43:00Z".parse::<DateTime<Utc>>()?));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! ## Weekday Nearest to Day of Month
//! An expression that matches the weekday nearest to the 10th in August.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let tenth = "* * 10W Aug *".parse::<CronExpression>()?;
//! assert_eq!(false, tenth.is_match(&"2019-08-10T00:00:00Z".parse::<DateTime<Utc>>()?));
//! assert_eq!(true, tenth.is_match(&"2019-08-09T00:00:00Z".parse::<DateTime<Utc>>()?));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! ## Last Day of Month
//! An expression that matches on the two last days of every month.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let last_two = "* * L,L-1 * *".parse::<CronExpression>()?;
//! assert_eq!(false, last_two.is_match(&"2021-04-28T00:00:00Z".parse::<DateTime<Utc>>()?));
//! assert_eq!(true, last_two.is_match(&"2021-04-29T00:00:00Z".parse::<DateTime<Utc>>()?));
//! assert_eq!(true, last_two.is_match(&"2021-04-30T00:00:00Z".parse::<DateTime<Utc>>()?));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! ## Nth Day of Week
//! An expression that matches on the 2nd Monday of June.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let monday = "* * * jun 1#2".parse::<CronExpression>()?;
//! assert_eq!(false, monday.is_match(&"2021-06-07T12:00:00Z".parse::<DateTime<Utc>>()?));
//! assert_eq!(true, monday.is_match(&"2021-06-14T12:00:00Z".parse::<DateTime<Utc>>()?));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! ## Last Day of Week
//! An expression that matches on the last Friday of every month.
//! ```
//! use chrono::{DateTime, Utc};
//! use curds_cron::CronExpression;
//! let friday = "* * * * FriL".parse::<CronExpression>()?;
//! assert_eq!(false, friday.is_match(&"2021-01-22T00:00:00Z".parse::<DateTime<Utc>>()?));
//! assert_eq!(true, friday.is_match(&"2021-01-29T00:00:00Z".parse::<DateTime<Utc>>()?));
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

