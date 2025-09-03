use std::fmt;
use log::warn;

use crate::data::timeofday::TimeOfDay;

/// 时间段块
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// 时间段的开始时间
     start_time: TimeOfDay,
    /// 时间段的结束时间
     end_time: TimeOfDay,
    /// 时间段的名称
     name: String,
    /// 时间段的可选描述
     description: Option<String>,
    /// 标识该时间段是否为固定时间段
     is_fixed: bool,
}


impl Block {
    /// 创建 BlockBuilder
    pub fn builder() -> BlockBuilder {
        BlockBuilder::new()
    }
    /// 获取开始时间
    pub fn start_time(&self) -> TimeOfDay {
        self.start_time
    }    
    /// 获取结束时间 
    pub fn end_time(&self) -> TimeOfDay {
        self.end_time
    }
    /// 获取名称
    pub fn name(&self) -> &str {
        &self.name
    }
    /// 获取描述
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    /// 获取是否固定标识
    pub fn is_fixed(&self) -> bool {
        self.is_fixed
    }
    /// 修改时间段
    pub fn set_time(&mut self, start_time: TimeOfDay, end_time: TimeOfDay)->Result<(), BlockError>{
        if start_time >= end_time{
            return Err(BlockError::InvalidTime { start: start_time, end: end_time });
        }
        if self.is_fixed{
            return Err(BlockError::FixedBlockTimeChange);
        }
        self.start_time = start_time;
        self.end_time = end_time;
        Ok(())
    }
    /// 修改名称
    pub fn set_name(&mut self, name: String) -> Result<(),BlockError>{
        if name.is_empty(){
            return Err(BlockError::EmptyName);
        }
        self.name = name;
        Ok(())

    }
    /// 修改描述
    pub fn set_description(&mut self, description: Option<String>){
        self.description = description;

    }
    /// 修改是否固定
    pub fn set_is_fixed(&mut self, is_fixed: bool){
        self.is_fixed = is_fixed;
    }


}


/// Block 构建器
pub struct BlockBuilder {
    /// 可选的开始时间
    start_time: Option<TimeOfDay>,
    /// 可选的结束时间
    end_time: Option<TimeOfDay>,
    /// 可选的名称
    name: Option<String>,
    /// 可选的描述（嵌套 Option 表示字段本身可选）
    description: Option<Option<String>>,
    /// 可选的是否固定标识
    is_fixed: Option<bool>,
}

impl BlockBuilder { 
    /// 创建新的构建器
    pub fn new() -> BlockBuilder {
        BlockBuilder { 
            start_time: None,
            end_time : None,
            name: None,
            description: None,
            is_fixed: None,
        }
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: TimeOfDay) -> BlockBuilder {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: TimeOfDay) -> BlockBuilder {
        self.end_time = Some(end_time);
        self
    }

    /// 设置名称
    pub fn name(mut self, name: String) -> BlockBuilder {
        self.name = Some(name);
        self
    }

    /// 设置描述
    pub fn description(mut self, description: Option<String>) -> BlockBuilder {
        self.description = Some(description);
        self
    }

    /// 设置是否固定
    pub fn is_fixed(mut self, is_fixed: bool) -> BlockBuilder {
        self.is_fixed = Some(is_fixed);
        self
    }

    /// 构建 Block 实例
    pub fn build(self) -> Result<Block, BlockError> { 
        let start_time = self.start_time.ok_or(BlockError::MissingRequiredField("start_time"))?;
        let end_time = self.end_time.ok_or(BlockError::MissingRequiredField("end_time"))?;
        let name = self.name.ok_or(BlockError::MissingRequiredField("name"))?;
        let description = self.description.unwrap_or(None);
        let is_fixed = self.is_fixed.unwrap_or({
            warn!("未指定是否固定，默认为非固定");
            false
        });
        // 验证时间范围
        if start_time >= end_time {
            return Err(BlockError::InvalidTime { start: start_time, end: end_time });
        }

        Ok(Block {
            start_time,
            end_time,
            name,
            description,
            is_fixed,
        })
    }
}

/// 表示构建 `Block` 时可能发生的错误。
#[derive(Debug)]
pub enum BlockError {
    /// 时间范围无效，结束时间必须晚于开始时间
    InvalidTime { start: TimeOfDay, end: TimeOfDay },
    /// 缺少必需字段
    MissingRequiredField(&'static str),
    /// 固定时间段不能修改时间范围
    FixedBlockTimeChange,
    EmptyName,
}

impl fmt::Display for BlockError {
    /// 格式化错误信息
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockError::InvalidTime { start, end } => {
                write!(f, "时间范围：结束时间 {} 必须晚于开始时间 {}", end, start)
            }
            BlockError::MissingRequiredField(field) => {
                write!(f, "缺少必需字段：{}", field)
            }
            BlockError::FixedBlockTimeChange => {
                write!(f, "固定时间段不能修改时间范围")
            }
            BlockError::EmptyName   => {
                write!(f, "名称不能为空!")
            }
        }
    }
}

impl std::error::Error for BlockError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_builder_success() {
        let start = TimeOfDay::new(9, 0).unwrap();
        let end = TimeOfDay::new(10, 30).unwrap();
        
        let block = Block::builder()
            .start_time(start)
            .end_time(end)
            .name("会议".to_string())
            .description(Some("项目讨论".to_string()))
            .is_fixed(true)
            .build()
            .expect("应该成功创建 Block");

        // 测试字段值（需要添加 getter 方法才能测试）
        assert_eq!(block.start_time, start);
        assert_eq!(block.end_time, end);
        assert_eq!(block.name, "会议");
        assert_eq!(block.description, Some("项目讨论".to_string()));
        assert_eq!(block.is_fixed, true);
    }

    #[test]
    fn test_block_builder_minimal() {
        let start = TimeOfDay::new(14, 0).unwrap();
        let end = TimeOfDay::new(15, 0).unwrap();
        
        let block = Block::builder()
            .start_time(start)
            .end_time(end)
            .name("午休".to_string())
            .build()
            .expect("应该成功创建最小 Block");

        assert_eq!(block.description, None);
        assert_eq!(block.is_fixed, false); // 默认值
    }

    #[test]
    fn test_missing_required_fields() {
        // 缺少开始时间
        let result = Block::builder()
            .end_time(TimeOfDay::new(10, 0).unwrap())
            .name("测试".to_string())
            .build();
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BlockError::MissingRequiredField("start_time")));

        // 缺少结束时间
        let result = Block::builder()
            .start_time(TimeOfDay::new(9, 0).unwrap())
            .name("测试".to_string())
            .build();
            
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BlockError::MissingRequiredField("end_time")));

        // 缺少名称
        let result = Block::builder()
            .start_time(TimeOfDay::new(9, 0).unwrap())
            .end_time(TimeOfDay::new(10, 0).unwrap())
            .build();
            
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BlockError::MissingRequiredField("name")));
    }

    #[test]
    fn test_invalid_time_range() {
        let start = TimeOfDay::new(10, 30).unwrap();
        let end = TimeOfDay::new(9, 0).unwrap(); // 结束时间早于开始时间
        
        let result = Block::builder()
            .start_time(start)
            .end_time(end)
            .name("无效时间块".to_string())
            .build();

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), 
            BlockError::InvalidTime { start: s, end: e } if s == start && e == end
        ));
    }

    #[test]
    fn test_equal_start_end_time() {
        let time = TimeOfDay::new(12, 0).unwrap();
        
        let result = Block::builder()
            .start_time(time)
            .end_time(time) // 开始和结束时间相同
            .name("零时长块".to_string())
            .build();

        assert!(result.is_err()); // 应该失败，因为 start_time >= end_time
    }

    #[test]
    fn test_error_display() {
        let start = TimeOfDay::new(10, 0).unwrap();
        let end = TimeOfDay::new(9, 0).unwrap();
        
        let error = BlockError::InvalidTime { start, end };
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("09:00"));
        assert!(error_msg.contains("10:00"));
        
        let error2 = BlockError::MissingRequiredField("name");
        let error_msg2 = format!("{}", error2);
        assert!(error_msg2.contains("name"));
    }

    #[test]
    fn test_block_getters() {
        let start = TimeOfDay::new(9, 30).unwrap();
        let end = TimeOfDay::new(11, 0).unwrap();
        let block = Block::builder()
            .start_time(start)
            .end_time(end)
            .name("工作".to_string())
            .description(Some("重要任务".to_string()))
            .is_fixed(true)
            .build()
            .unwrap();

        assert_eq!(block.start_time(), start);
        assert_eq!(block.end_time(), end);
        assert_eq!(block.name(), "工作");
        assert_eq!(block.description(), Some("重要任务"));
        assert_eq!(block.is_fixed(), true);
    }

    #[test]
    fn test_block_getters_with_none_description() {
        let block = Block::builder()
            .start_time(TimeOfDay::new(14, 0).unwrap())
            .end_time(TimeOfDay::new(15, 30).unwrap())
            .name("休息".to_string())
            .build()
            .unwrap();

        assert_eq!(block.description(), None);
        assert_eq!(block.is_fixed(), false);
    }

    #[test]
    fn test_set_time_success() {
        let mut block = Block::builder()
            .start_time(TimeOfDay::new(9, 0).unwrap())
            .end_time(TimeOfDay::new(10, 0).unwrap())
            .name("会议".to_string())
            .is_fixed(false)
            .build()
            .unwrap();

        let new_start = TimeOfDay::new(10, 30).unwrap();
        let new_end = TimeOfDay::new(11, 30).unwrap();
        
        let result = block.set_time(new_start, new_end);
        assert!(result.is_ok());
        assert_eq!(block.start_time(), new_start);
        assert_eq!(block.end_time(), new_end);
    }

    #[test]
    fn test_set_time_invalid_range() {
        let mut block = Block::builder()
            .start_time(TimeOfDay::new(9, 0).unwrap())
            .end_time(TimeOfDay::new(10, 0).unwrap())
            .name("会议".to_string())
            .is_fixed(false)
            .build()
            .unwrap();

        let invalid_start = TimeOfDay::new(11, 0).unwrap();
        let invalid_end = TimeOfDay::new(10, 0).unwrap();
        
        let result = block.set_time(invalid_start, invalid_end);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BlockError::InvalidTime { .. }));
        
        // 原始时间应该保持不变
        assert_eq!(block.start_time(), TimeOfDay::new(9, 0).unwrap());
        assert_eq!(block.end_time(), TimeOfDay::new(10, 0).unwrap());
    }

    #[test]
    fn test_set_time_equal_times() {
        let mut block = Block::builder()
            .start_time(TimeOfDay::new(9, 0).unwrap())
            .end_time(TimeOfDay::new(10, 0).unwrap())
            .name("会议".to_string())
            .is_fixed(false)
            .build()
            .unwrap();

        let same_time = TimeOfDay::new(12, 0).unwrap();
        
        let result = block.set_time(same_time, same_time);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BlockError::InvalidTime { .. }));
    }

    #[test]
    fn test_set_time_on_fixed_block() {
        let mut block = Block::builder()
            .start_time(TimeOfDay::new(9, 0).unwrap())
            .end_time(TimeOfDay::new(10, 0).unwrap())
            .name("固定会议".to_string())
            .is_fixed(true)
            .build()
            .unwrap();

        let new_start = TimeOfDay::new(11, 0).unwrap();
        let new_end = TimeOfDay::new(12, 0).unwrap();
        
        let result = block.set_time(new_start, new_end);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BlockError::FixedBlockTimeChange));
        
        // 固定块的时间应该保持不变
        assert_eq!(block.start_time(), TimeOfDay::new(9, 0).unwrap());
        assert_eq!(block.end_time(), TimeOfDay::new(10, 0).unwrap());
    }

    #[test]
    fn test_set_name() {
        let mut block = Block::builder()
            .start_time(TimeOfDay::new(9, 0).unwrap())
            .end_time(TimeOfDay::new(10, 0).unwrap())
            .name("原名称".to_string())
            .build()
            .unwrap();

        let result = block.set_name("新名称".to_string());
        assert!(result.is_ok());
        assert_eq!(block.name(), "新名称");
        
        // 测试空字符串
        let result = block.set_name("".to_string());
        assert!(matches!(result, Err(BlockError::EmptyName)));
        // 名称应该保持不变
        assert_eq!(block.name(), "新名称");
        
    }

    #[test]
    fn test_set_description() {
        let mut block = Block::builder()
            .start_time(TimeOfDay::new(9, 0).unwrap())
            .end_time(TimeOfDay::new(10, 0).unwrap())
            .name("会议".to_string())
            .description(Some("原描述".to_string()))
            .build()
            .unwrap();

        // 修改描述
        block.set_description(Some("新描述".to_string()));
        assert_eq!(block.description(), Some("新描述"));
        
        // 清空描述
        block.set_description(None);
        assert_eq!(block.description(), None);
        
        // 重新设置描述
        block.set_description(Some("再次设置".to_string()));
        assert_eq!(block.description(), Some("再次设置"));
    }

    #[test]
    fn test_set_is_fixed() {
        let mut block = Block::builder()
            .start_time(TimeOfDay::new(9, 0).unwrap())
            .end_time(TimeOfDay::new(10, 0).unwrap())
            .name("会议".to_string())
            .is_fixed(false)
            .build()
            .unwrap();

        assert_eq!(block.is_fixed(), false);
        
        block.set_is_fixed(true);
        assert_eq!(block.is_fixed(), true);
        
        block.set_is_fixed(false);
        assert_eq!(block.is_fixed(), false);
    }

    #[test]
    fn test_block_modification_workflow() {
        let mut block = Block::builder()
            .start_time(TimeOfDay::new(9, 0).unwrap())
            .end_time(TimeOfDay::new(10, 0).unwrap())
            .name("会议".to_string())
            .is_fixed(false)
            .build()
            .unwrap();

        // 修改时间
        block.set_time(TimeOfDay::new(10, 0).unwrap(), TimeOfDay::new(11, 0).unwrap()).unwrap();
        
        // 修改名称和描述
        block.set_name("重要会议".to_string()).unwrap();
        block.set_description(Some("项目讨论".to_string()));
        
        // 设为固定
        block.set_is_fixed(true);
        
        // 验证最终状态
        assert_eq!(block.start_time(), TimeOfDay::new(10, 0).unwrap());
        assert_eq!(block.end_time(), TimeOfDay::new(11, 0).unwrap());
        assert_eq!(block.name(), "重要会议");
        assert_eq!(block.description(), Some("项目讨论"));
        assert_eq!(block.is_fixed(), true);
        
        // 现在应该无法修改时间
        let result = block.set_time(TimeOfDay::new(12, 0).unwrap(), TimeOfDay::new(13, 0).unwrap());
        assert!(result.is_err());
    }
}