use std::{
    convert::TryFrom,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use chrono::{Local, Utc};
use time::{format_description, macros::offset, Date, Month, OffsetDateTime, PrimitiveDateTime};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthData {
    #[serde(with = "time::serde::timestamp")]
    issue_time: OffsetDateTime,
}

#[test]
fn test_time() -> Result<()> {
    let now = SystemTime::now();
    println!("now: {:?}", &now);

    // 获取从 UNIX_EPOCH 开始的时间戳
    let time_sec = now.duration_since(UNIX_EPOCH)?.as_secs();
    println!("time_sec: {:?}", &time_sec);

    // 这与系统时间无关, 只是记录一个当前时刻, 与 Duration 一起使用
    let now = std::time::Instant::now();
    println!("now: {:?}", &now);

    // 使用 chrono 库, 当前UTC时间
    let now = Utc::now();
    println!("now: {:?}", &now);

    // 使用 chrono 库, 当前系统时间
    let now = Local::now();
    println!("now: {:?}", &now);

    // 使用 time 库, 当前系统时间
    let now = OffsetDateTime::now_local()?;
    println!("now: {:?}", &now);

    // 使用 time 库, 当前UTC时间
    let now = OffsetDateTime::now_utc();
    println!("now: {:?}", &now);

    // 使用 time 库, 获取 date 和 time 和 时间戳
    let now = OffsetDateTime::now_local()?;
    println!("now.date: {:?}", now.date());
    println!("now.time: {:?}", now.time());
    println!("now.unix_timestamp: {:?}", now.unix_timestamp());

    // 时间格式化
    // 2022.1.1 - 2022.1.2
    let date_time1 = Date::from_calendar_date(2022, Month::try_from(1)?, 1)?.midnight();
    println!(
        "date_time.date: {:?}, date_time.time: {:?}",
        date_time1.date(),
        date_time1.time()
    );
    let date_time2 = Date::from_calendar_date(2022, Month::try_from(1)?, 2)?.midnight();
    if date_time2 > date_time1 {
        println!("date_time2 is greater than date_time1");
    } else {
        println!("date_time1 is greater than date_time2");
    }

    // 格式化 和 解析日期时间
    // 2022.12.31

    // 解析, 输入的信息需要足够的时间参数, 缺少某些必要字段就会报错
    let date_time = PrimitiveDateTime::parse(
        "2022.12.31 00:00",
        &format_description::parse("[year].[month].[day] [hour]:[minute]")?,
    )?;
    println!("date_time: {:?}", &date_time);
    println!("date_time: {:?}", date_time.assume_offset(offset!(-1)));
    // 格式化
    let date_time_str = date_time.format(&format_description::parse("[year]-[month]-[day]")?)?;
    println!("date_time_str: {:?}", &date_time_str);

    Ok(())
}
