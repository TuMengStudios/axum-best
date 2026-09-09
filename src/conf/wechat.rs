use derivative::Derivative;
use serde::Deserialize;

/// 微信小程序配置
///
/// 暂无消费组件（微信客户端尚未实现），寄存在 conf/ 聚合层；
/// 客户端模块落地后应将本结构体迁移到它旁边。
#[derive(Derivative, Deserialize, Clone)]
#[derivative(Debug)]
#[allow(dead_code)] // fields are consumed once the wechat client exists
pub(crate) struct WeChatConf {
    pub appid: String,
    #[derivative(Debug = "ignore")]
    pub secret: String,
}
