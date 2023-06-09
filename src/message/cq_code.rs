use serde::{Deserialize, Serialize};
use crate::message::RawMessage;

/// 包含对于CQ码的处理
/// CQ码是CQ中特殊消息类型的文本格式
/// 基本语法如下:
///
/// `[CQ:类型,参数=值,参数=值]`
///

/// CQ码的类型
/// 可以是
/// - `Text` 普通文本也认为是CQ码， 便于处理
/// - `Face` QQ表情
/// - `Record` 语音
/// - `Video` 短视频
/// - `At` at某人
/// - `Share` 链接分享
/// - `Music` 音乐分享(使用id)
/// - `Image` 图片
/// - `Reply` 回复
/// - `Poke` 戳一戳
/// - `Forward` 合并转发
/// - `Node` 合并转发消息节点
/// - `Xml` Xml 消息
/// - `Json` Json 消息
/// - `Tts` Tts 文本转语音
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum CQCodeType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "face")]
    Face,
    #[serde(rename = "record")]
    Record,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "at")]
    At,
    #[serde(rename = "share")]
    Share,
    #[serde(rename = "music")]
    Music,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "reply")]
    Reply,
    #[serde(rename = "poke")]
    Poke,
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "xml")]
    Xml,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "tts")]
    Tts,
}

/// 需为特殊消息类型实现`CQCode`，
/// 特殊消息类型参见: [`CQCodeType`]
/// 包含三个方法:
/// [`CQCode::cq_code`]: 返回消息类型的CQ码， 使用[`macros::CQCode`]可以为结构体提供默认实现
/// [`CQCode::from`]: 从`RawMessage`生成对应的CQ码, 提供了默认实现
/// [`CQCode::into`]: 从对象生成对应的`RawMessage`, 提供了默认实现
/// 实现了`CQCode`的结构体就可以认为是一种消息
pub trait CQCode {
    fn cq_code(&self) -> CQCodeType;

    fn from_raw_message(s: &str) -> Self
    where Self: Sized + serde::de::DeserializeOwned {
        serde_json::from_str(s).unwrap()
    }

    fn to_raw_message(&self) -> RawMessage
    where Self: Sized + serde::Serialize {
        serde_json::to_string(&self).unwrap()
    }
}


