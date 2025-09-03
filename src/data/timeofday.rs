use std::fmt;
use log::error;
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeOfDay(u16);


impl TimeOfDay{
    /// 创建时间
    pub fn new(hour: u16, minute: u16) -> Option<TimeOfDay> {
        if hour > 23 || minute > 59 {
            error!("Invalid time");
            return None;
        }
        Some(TimeOfDay(hour * 60 + minute))
    }
    /// 获取小时
    pub fn hour(&self) -> u16 {
        self.0 / 60
    }
    /// 获取分钟
    pub fn minute(&self) -> u16 {
        self.0 % 60
    }
}

impl From<TimeOfDay> for u16 {
    ///TimeOfDay转为u16
    fn from(t: TimeOfDay) -> u16 {
        t.0
    }
}
impl TryFrom<u16> for TimeOfDay {
    type Error = ();
    ///u16转为TimeOfDay
    fn try_from(u: u16) -> Result<TimeOfDay, ()> {
        if u>=24*60 {
            return Err(())
        }
        Ok(TimeOfDay(u))
    }
}

impl fmt::Display for TimeOfDay {
    /// 时间显示格式
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour(), self.minute())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    /// 正常情况测试
    #[test]
    fn new_valid_and_accessors_and_display() {
        // 最小边界：00:00
        let t = TimeOfDay::new(0, 0).expect("00:00 应该合法");
        assert_eq!(t.hour(), 0);
        assert_eq!(t.minute(), 0);
        assert_eq!(format!("{}", t), "00:00");

        // 最大边界：23:59
        let t2 = TimeOfDay::new(23, 59).expect("23:59 应该合法");
        assert_eq!(t2.hour(), 23);
        assert_eq!(t2.minute(), 59);
        assert_eq!(format!("{}", t2), "23:59");
    }

    /// 非法输入测试
    #[test]
    fn new_invalid() {
        assert!(TimeOfDay::new(24, 0).is_none(), "24:00 不合法");
        assert!(TimeOfDay::new(0, 60).is_none(), "00:60 不合法");
        assert!(TimeOfDay::new(100, 10).is_none(), "100:10 不合法");
    }

    /// 转换测试
    #[test]
    fn from_and_tryfrom() {
        // 01:30 -> 90 分钟
        let t = TimeOfDay::new(1, 30).unwrap();
        let minutes: u16 = u16::from(t);
        assert_eq!(minutes, 90, "01:30 应该是 90 分钟");

        // 90 -> 01:30
        let t2 = TimeOfDay::try_from(90).expect("90 分钟应该合法");
        assert_eq!(t2.hour(), 1);
        assert_eq!(t2.minute(), 30);

        // 非法边界
        assert!(TimeOfDay::try_from(24 * 60).is_err(), "1440 不合法");
        assert!(TimeOfDay::try_from(24 * 60 + 1).is_err(), "1441 不合法");

        // 合法边界：1439 -> 23:59
        let last = TimeOfDay::try_from(24 * 60 - 1).expect("1439 合法");
        assert_eq!(last.hour(), 23);
        assert_eq!(last.minute(), 59);
    }

    /// 排序测试
    #[test]
    fn ordering_and_equality() {
        let mut v = vec![
            TimeOfDay::new(12, 0).unwrap(),
            TimeOfDay::new(1, 30).unwrap(),
            TimeOfDay::new(0, 45).unwrap(),
            TimeOfDay::new(23, 59).unwrap(),
        ];
        // 排序从早到晚
        v.sort(); // 使用 Ord
        let expected = vec![
            TimeOfDay::new(0, 45).unwrap(),
            TimeOfDay::new(1, 30).unwrap(),
            TimeOfDay::new(12, 0).unwrap(),
            TimeOfDay::new(23, 59).unwrap(),
        ];
        assert_eq!(v, expected, "排序应按照从早到晚");
        // 顺序检查
        assert!(expected[0] < expected[1]);
        assert!(expected[2] < expected[3]);
        // 相等性检查
        assert_eq!(TimeOfDay::new(12, 0), TimeOfDay::new(12, 0));
    }
}