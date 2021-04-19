A library for creating and consuming Cron expressions. It supports some extended syntax.

# Overview
A Cron expression is made up of five fields representing different parts of the current time. Each field defines some number of values that represent when the expression is matched. A Cron expression is matched when *all* of its fields are matched; a field is matched when *any* of its values are matched.

A simple Cron expression that matches *all* times looks like `* * * * *`. It specifies a single value for each field, each of which is a wildcard.
The fields appear in the following order:
1 Minute
1 Hour
1 Day of Month
1 Month
1 Day of Week

# Reference Expressions
|Expression|Description|
--- | ---
|* * * * *|Matches all times|

# Possibilities
There are three standard values that can be supplied for all fields:
* Wildcard (`*`)
  * This value matches all times
* Range (`5-10`)
  * This is an *inclusive* range of values for when the expression matches
* Single (`10`)
  * This represents a single value for when the expression matches
* Step Range (`*/2`)
  * This represents a set of values for when the expression matches
  * It starts at the minimum for the specified field, incrementing by the step until the whole range is covered

The Day of Month field has two special values that can be supplied:
* Weekday Nearest (`15W`)
  * This value matches the weekday nearest the supplied day of month
  * When the day falls on a Saturday, it will match the preceding Friday
  * When the day falls on a Sunday, it will match the following Monday
* Last Day of Month (`L` or `L-5`)
  * This value matches on the last day of month
  * It can optionally be offset by some number of days to match "X days before the last day of the month"

The Day of Week field has two special values that can be supplied:
* Nth Day of Week (`3#2`)
  * This value matches on a specific occurrence of the supplied day of week.
  * In the example value it matches on `2`nd occurrence of the day of week `3` (Wednesday)
* Last Day of Week (`2L`)
  * This value matches on the *last* occurrence of the supplied day of week.

Additionally, the _Month_ and _Day of Week_ fields can either have a numeric value or a three-letter alias supplied.
## Months
|Alias|Value|
--- | ---
|JAN|1|
--- | ---
|FEB|2|
--- | ---
|MAR|3|
--- | ---
|APR|4|
--- | ---
|MAY|5|
--- | ---
|JUN|6|
--- | ---
|JUL|7|
--- | ---
|AUG|8|
--- | ---
|SEP|9|
--- | ---
|OCT|10|
--- | ---
|NOV|11|
--- | ---
|DEC|12|
## Days of Week
|Alias|Value|
--- | ---
|SUN|0|
--- | ---
|MON|1|
--- | ---
|TUE|2|
--- | ---
|WED|3|
--- | ---
|THU|4|
--- | ---
|FRI|5|
--- | ---
|SAT|6|