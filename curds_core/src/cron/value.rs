#[derive(Debug, PartialEq)]
pub enum CronValue {
    Any,
    Single(u32),
    Step(u32),
    Range { min: u32, max: u32 },
    LastDayOfMonth { offset: u32 },
    LastDayOfWeek { day_of_week: u32 },
    NthDayOfWeek { day_of_week: u32, n: u32 },
    NearestWeekday { day_of_month: u32 },
}