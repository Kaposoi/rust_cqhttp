/// 通用上报
/// 包括消息上报， 请求上报与通知上报

use serde::Deserialize;

/// 处理过程中所需的数据结构

/// PostType 枚举。
/// 表示上报的类型， 消息，消息发送，请求，通知，或元事件。
/// 取值可能是 `message`, `message_sent`, `request`, `notice`, `meta_event`
#[derive(Debug, Deserialize, Clone)]
enum PostType {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "message_sent")]
    MessageSent,
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "meta_event")]
    MetaEvent,
}

/// 所有上报包含的通用数据
/// `time`: 事件发生时的Unix时间戳
/// `self_id`: 收到事件的机器人的 QQ 号
/// `post_type`: [`PostType`]
#[derive(Debug, Deserialize, Clone)]
struct Report {
    time: i64,
    self_id: i64,
    post_type: PostType,
}

// /// `post_type`为`message`或是`message_sent`的上报
// /// `message_type`: 消息类型， 可以是`private`(私聊)， 或是`group`(群消息)
// /// `message_id`: 消息id
// /// `user_id`: 发送者qq号
// /// `message`: 消息链
// /// `raw_message`: `CQ码格式的消息`
// /// `font`: 字体， 只能是0
// /// `sender`: 发送者消息
// struct Message {
//     message_type: String,
//     sub_type: String,
//     message_id: i32,
//     user_id: i64,
//     message: MessageChain,
//     raw_message: String,
//     font: i32,
//     sender:
// }