use serde::{Deserialize, Serialize};
/// 实现了一些CQ码

use crate::message::cq_code::{CQCode, CQCodeType};
use crate::message::RawMessage;
use macros::CQCode;

/// `id`: 表情id
#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct FaceInner {
    id: String,
}

/// QQ表情
/// [`FaceInner`]
#[derive(Debug, Deserialize, Serialize, Clone, CQCode)]
pub struct Face {
    #[serde(rename = "type")]
    typ: CQCodeType,
    data: FaceInner,
}

impl Face {
    fn from_string(value: String) -> Self {
        Face {
            typ: CQCodeType::Face,
            data: FaceInner {
                id: value
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct VoiceInner {
    /// 语音文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
    /// 发送时可选，默认为0， 1表示变声
    #[serde(skip_serializing_if = "Option::is_none")]
    magic: Option<i32>,
    /// 语音url
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    /// 只在通过网络 URL 发送时有效，表示是否使用已缓存的文件，默认 1
    #[serde(skip_serializing_if = "Option::is_none")]
    cache: Option<String>,
    /// 只在通过网络 URL 发送时有效，表示是否通过代理下载文件 (需通过环境变量或配置文件配置代理) , 默认 1
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy: Option<i32>,
    /// 只在通过网络 URL 发送时有效，单位秒，表示下载网络文件的超时时间，默认不超时
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<i32>,
}

/// 语音信息
/// [`VoiceInner`]
#[derive(Debug, Deserialize, Serialize, Clone, CQCode)]
pub struct Voice {
    #[serde(rename = "type")]
    typ: CQCodeType,
    data: VoiceInner,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct VideoInner {
    /// 视频地址，支持 http 和 file 发送
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
    /// 视频封面，支持 http, file 和 base64 发送，格式必须为 jpg
    #[serde(skip_serializing_if = "Option::is_none")]
    cover: Option<String>,
    /// 通过网络下载视频时的线程数，默认单线程. (在资源不支持并发时会自动处理)
    #[serde(skip_serializing_if = "Option::is_none")]
    c: Option<i32>,
}

/// 视频消息
/// [`VideoInner`]
#[derive(Debug, Deserialize, Serialize, Clone, CQCode)]
pub struct Video {
    #[serde(rename = "type")]
    typ: CQCodeType,
    data: VideoInner,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct AtInner {
    /// @的 QQ 号，all 表示全体成员
    #[serde(skip_serializing_if = "Option::is_none")]
    qq: Option<String>,
    /// 当在群中找不到此 QQ 号的名称时才会生效
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

/// At信息
/// [`AtInner`]
#[derive(Debug, Deserialize, Serialize, Clone, CQCode)]
pub struct At {
    #[serde(rename = "type")]
    typ: CQCodeType,
    data: AtInner,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct ShareInner {
    /// URL
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 发送时可选，内容描述
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 发送时可选，图片 URL
    image: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, CQCode)]
pub struct Share {
    #[serde(rename = "type")]
    typ: CQCodeType,
    data: ShareInner,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct MusicInner {
    /// 可能是qq， 163， xm 分别表示使用 QQ 音乐、网易云音乐、虾米音乐， 也可能是custom
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    typ: Option<String>,
    /// 当不是custom时使用这个， 代表歌曲id
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    /// 点击后跳转目标 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    /// 音乐 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    audio: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    /// 发送时可选，内容描述
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    /// 发送时可选，图片 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, CQCode)]
pub struct Music {
    #[serde(rename = "type")]
    typ: CQCodeType,
    data: MusicInner,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct ImageInner {
    /// 图片文件名
    /// - 绝对路径，例如 file:///C:\\Users\Alice\Pictures\1.png，格式使用 file URI
    /// - 网络 URL，例如 https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png
    /// - Base64 编码，例如 base64://iVBORw0KGgoAAAANSUhEUgAAABQAAAAVCAIAAADJt1n/AAAAKElEQVQ4EWPk5+RmIBcwkasRpG9UM4mhNxpgowFGMARGEwnBIEJVAAAdBgBNAZf+QAAAAABJRU5ErkJggg==
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
    /// 图片类型，flash 表示闪照，show 表示秀图，没有代表普通图片
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    typ: Option<String>,
    /// 图片子类型，只出现在群聊.
    /// - 0: 正常图片
    /// - 1: 表情包
    /// - 2: 热图
    /// - 3: 斗图
    /// - 7: 贴图
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subType")]
    sub_type: Option<String>,
    /// 图片url
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    /// 只在通过网络 URL 发送时有效，表示是否使用已缓存的文件，默认 1
    #[serde(skip_serializing_if = "Option::is_none")]
    cache: Option<i32>,
    /// 发送秀图时的特效 id, 默认为 40000
    /// - 40000: 普通
    /// - 40001: 幻影
    /// - 40002: 抖动
    /// - 40003: 生日
    /// - 40004: 爱你
    /// - 40005: 征友
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    /// 通过网络下载图片时的线程数，默认单线程. (在资源不支持并发时会自动处理)
    #[serde(skip_serializing_if = "Option::is_none")]
    c: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, CQCode)]
pub struct Image {
    #[serde(rename = "type")]
    typ: CQCodeType,
    data: ImageInner,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct ReplyInner {
    /// 回复时所引用的消息 id, 必须为本群消息.
    /// 如果 id 和 text 同时存在，将采用自定义 reply 并替换原有信息 如果 id 获取失败，将回退到自定义 reply
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    /// 自定义回复的信息
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    /// 自定义回复时的自定义 QQ, 如果使用自定义信息必须指定.
    #[serde(skip_serializing_if = "Option::is_none")]
    qq: Option<i64>,
    /// 自定义回复时的时间，格式为 Unix 时间
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<i64>,
    /// 起始消息序号，可通过 get_msg 获得
    #[serde(skip_serializing_if = "Option::is_none")]
    seq: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, CQCode)]
pub struct Reply {
    #[serde(rename = "type")]
    typ: CQCodeType,
    data: ReplyInner
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct PokeInner {
    /// 需要戳的成员
    #[serde(skip_serializing_if = "Option::is_none")]
    qq: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, CQCode)]
pub struct Poke {
    #[serde(rename = "type")]
    typ: CQCodeType,
    data: PokeInner,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct ForwardInner {
    /// 合并转发 ID, 需要通过 /get_forward_msg API 获取转发的具体内容
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

/// 合并转发消息， 仅用于接收
/// [`ForwardInner`]
#[derive(Debug, Deserialize, Serialize, Clone, CQCode)]
pub struct Forward {
    #[serde(rename = "type")]
    typ: CQCodeType,
    data: FaceInner,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct XmlInner {
    /// xml 内容，xml 中的 value 部分，记得实体化处理
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
    /// 可能为空，或空字符串
    #[serde(skip_serializing_if = "Option::is_none")]
    resid: Option<i32>,
}

/// xml卡片消息
/// [`XmlInner`]
/// 一些示例:
/// - QQ音乐
/// ```xml
/// <?xml version='1.0' encoding='UTF-8' standalone='yes' ?><msg serviceID="2" templateID="1" action="web" brief="&#91;分享&#93; 十年" sourceMsgId="0" url="https://i.y.qq.com/v8/playsong.html?_wv=1&amp;songid=4830342&amp;souce=qqshare&amp;source=qqshare&amp;ADTAG=qqshare" flag="0" adverSign="0" multiMsgFlag="0" ><item layout="2"><audio cover="http://imgcache.qq.com/music/photo/album_500/26/500_albumpic_89526_0.jpg" src="http://ws.stream.qqmusic.qq.com/C400003mAan70zUy5O.m4a?guid=1535153710&amp;vkey=D5315B8C0603653592AD4879A8A3742177F59D582A7A86546E24DD7F282C3ACF81526C76E293E57EA1E42CF19881C561275D919233333ADE&amp;uin=&amp;fromtag=3" /><title>十年</title><summary>陈奕迅</summary></item><source name="QQ音乐" icon="https://i.gtimg.cn/open/app_icon/01/07/98/56/1101079856_100_m.png" url="http://web.p.qq.com/qqmpmobile/aio/app.html?id=1101079856" action="app"  a_actionData="com.tencent.qqmusic" i_actionData="tencent1101079856://" appid="1101079856" /></msg>
/// ```
/// - 网易云音乐
/// ```xml
/// <?xml version='1.0' encoding='UTF-8' standalone='yes' ?><msg serviceID="2" templateID="1" action="web" brief="&#91;分享&#93; 十年" sourceMsgId="0" url="http://music.163.com/m/song/409650368" flag="0" adverSign="0" multiMsgFlag="0" ><item layout="2"><audio cover="http://p2.music.126.net/g-Qgb9ibk9Wp_0HWra0xQQ==/16636710440565853.jpg?param=90y90" src="https://music.163.com/song/media/outer/url?id=409650368.mp3" /><title>十年</title><summary>黄梦之</summary></item><source name="网易云音乐" icon="https://pic.rmb.bdstatic.com/911423bee2bef937975b29b265d737b3.png" url="http://web.p.qq.com/qqmpmobile/aio/app.html?id=1101079856" action="app" a_actionData="com.netease.cloudmusic" i_actionData="tencent100495085://" appid="100495085" /></msg>
/// ```
/// - 卡片消息
/// ```xml
/// <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
// <msg serviceID="1">
// <item><title>生死8秒！女司机高速急刹, 他一个操作救下一车性命</title></item>
// <source name="官方认证消息" icon="https://qzs.qq.com/ac/qzone_v5/client/auth_icon.png" action="" appid="-1" />
// </msg>
/// ```
/// - 卡片消息2
/// ```xml
/// <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
// <msg serviceID="1">
// <item layout="4">
// <title>test title</title>
// <picture cover="http://url.cn/5CEwIUy"/>
// </item>
// </msg>
/// ```
#[derive(Debug, Deserialize, Serialize, Clone, CQCode)]
pub struct Xml {
    #[serde(rename = "type")]
    typ: CQCodeType,
    data: XmlInner,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct TtsInner {
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

/// 文本转语音消息
/// [`TtsInner`]
/// ```
#[derive(Debug, Deserialize, Serialize, Clone, CQCode)]
pub struct Tts {
    #[serde(rename = "type")]
    typ: CQCodeType,
    data: TtsInner,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct JsonInner {
    /// json 内容，json 的所有字符串记得实体化处理
    /// - "," => `&#44`
    /// - "&" => `&amp`
    /// - "[" => `&#91`
    /// - "]" => `&#93`
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
    /// 默认不填为 0, 走小程序通道，填了走富文本通道发送
    #[serde(skip_serializing_if = "Option::is_none")]
    resid: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, CQCode)]
pub struct Json {
    #[serde(rename = "type")]
    typ: CQCodeType,
    data: JsonInner,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use crate::message::cq_code::{CQCode, CQCodeType};
    use crate::message::cq_codes::{Face, Voice, VoiceInner};
    use crate::message::RawMessage;

    #[test]
    fn test_face() {
        let json = r#"{
    "type": "face",
    "data": {
        "id": "1"
    }
}"#;
        let value: Face = CQCode::from_raw_message(json);
        let out: RawMessage = CQCode::to_raw_message(&value);

        assert_eq!(out, r#"{"type":"face","data":{"id":"1"}}"#);
        assert_eq!(value.cq_code(), CQCodeType::Face);
    }

    #[test]
    fn test_voice() {
        let st = r#"{
            "type": "face",
            "data": {
                "file": "111",
                "magic": 0
            }
        }"#;
        // "url": "111",
        // "cache": "111",
        // "proxy": 1,
        // "timeout": 5
        let voice = Voice {
            typ: CQCodeType::Face,
            data: VoiceInner {
                file: None,
                magic: Option::Some(2),
                url: None,
                cache: None,
                proxy: None,
                timeout: None,
            }
        };

        let out: Voice = CQCode::from_raw_message(st);

        println!("{:#?}", out);
    }
}