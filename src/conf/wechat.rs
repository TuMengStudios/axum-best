use derivative::Derivative;
use serde::Deserialize;

/// 微信小程序配置，由组装层注入微信接口仓储实现。
#[derive(Derivative, Deserialize, Clone)]
#[derivative(Debug)]
pub(crate) struct WeChatConf {
    pub appid: String,
    #[derivative(Debug = "ignore")]
    pub secret: String,
}
