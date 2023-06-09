use crate::message::cq_code::CQCode;
use crate::message::RawMessage;

/// 消息连

/// 生成消息链， 消息链生成完毕后无法更改。
pub trait MessageChainBuilder {
    /// 朝构造器中添加一个`message`
    fn add(message: impl CQCode) -> Self;
    /// 将构造器转换为对应的`RawMessage`
    fn raw_message(&self) -> RawMessage;
    /// 从消息链中过滤对象
    fn filter_instance<T>(&self) -> T
    where T: CQCode;
}