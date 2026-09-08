use derivative::Derivative;
use serde::Deserialize;

/// 微信小程序配置
#[derive(Derivative, Deserialize, Clone)]
#[derivative(Debug)]
pub struct WeChatConf {
    pub appid: String,
    #[derivative(Debug = "ignore")]
    pub secret: String,
}
