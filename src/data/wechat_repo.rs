use async_trait::async_trait;
use serde::Deserialize;

use crate::conf::wechat::WeChatConf;
use crate::core::rest::AppError;
use crate::errors;
use crate::repos::wechat::WechatRepo;

const CODE_TO_SESSION_URL: &str = "https://api.weixin.qq.com/sns/jscode2session";

#[derive(Debug, Deserialize)]
struct CodeToSessionResponse {
    openid: String,
}

/// 微信小程序接口的 HTTP 实现。
pub struct WechatApiRepo {
    client: reqwest::Client,
    appid: String,
    secret: String,
}

impl WechatApiRepo {
    pub(crate) fn new(config: &WeChatConf) -> WechatApiRepo {
        WechatApiRepo {
            client: reqwest::Client::new(),
            appid: config.appid.clone(),
            secret: config.secret.clone(),
        }
    }
}

#[async_trait]
impl WechatRepo for WechatApiRepo {
    async fn exchange_code_for_open_id(&self, code: &str) -> Result<String, AppError> {
        let response = self
            .client
            .get(CODE_TO_SESSION_URL)
            .query(&[
                ("appid", self.appid.as_str()),
                ("secret", self.secret.as_str()),
                ("js_code", code),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await
            .map_err(|err| errors::ErrWechatLogin.with_cause(err, "call WeChat API"))?
            .error_for_status()
            .map_err(|err| {
                errors::ErrWechatLogin.with_cause(err, "WeChat API returned an error status")
            })?
            .json::<CodeToSessionResponse>()
            .await
            .map_err(|err| {
                errors::ErrUnmarshalJSON.with_cause(err, "decode WeChat API response")
            })?;

        Ok(response.openid)
    }
}
