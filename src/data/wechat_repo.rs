use async_trait::async_trait;
use derivative::Derivative;
use serde::Deserialize;

use crate::core::rest::AppError;
use crate::errors;
use crate::repos::wechat::WechatRepo;
use crate::repos::wechat::WechatSession;

const CODE_TO_SESSION_URL: &str = "https://api.weixin.qq.com/sns/jscode2session";

/// 微信小程序接口配置，由微信接口实现持有。
#[derive(Derivative, Deserialize, Clone)]
#[derivative(Debug)]
pub(crate) struct WeChatConf {
    pub appid: String,
    #[derivative(Debug = "ignore")]
    pub secret: String,
}

#[derive(Debug, Deserialize)]
struct CodeToSessionResponse {
    openid: Option<String>,
    session_key: Option<String>,
    #[serde(default)]
    errcode: Option<i64>,
    #[serde(default)]
    errmsg: Option<String>,
    #[serde(default)]
    unionid: Option<String>,
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
    async fn exchange_login_code(&self, code: &str) -> Result<WechatSession, AppError> {
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

        if let Some(errcode) = response.errcode.filter(|code| *code != 0) {
            let errmsg = response
                .errmsg
                .as_deref()
                .unwrap_or("unknown WeChat API error");
            return Err(errors::ErrWechatLogin.with_cause(
                std::io::Error::other(format!("errcode={errcode}, errmsg={errmsg}")),
                "WeChat API returned an application error",
            ));
        }

        let open_id = response.openid.ok_or_else(|| {
            errors::ErrWechatLogin.with_cause(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "WeChat API response did not contain openid",
                ),
                "WeChat API response missing openid",
            )
        })?;
        let session_key = response.session_key.ok_or_else(|| {
            errors::ErrWechatLogin.with_cause(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "WeChat API response did not contain session_key",
                ),
                "WeChat API response missing session_key",
            )
        })?;

        Ok(WechatSession {
            open_id,
            session_key,
            union_id: response.unionid,
        })
    }
}
