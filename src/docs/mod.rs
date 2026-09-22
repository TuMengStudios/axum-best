use serde::Deserialize;

pub mod openapi;

/// Swagger UI and OpenAPI document exposure settings.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SwaggerConf {
    /// Exposes Swagger UI and the OpenAPI document when enabled.
    #[serde(default)]
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::SwaggerConf;

    #[test]
    fn swagger_is_disabled_by_default() {
        assert!(!SwaggerConf::default().enabled);
    }

    #[test]
    fn swagger_settings_are_configurable() {
        let config: SwaggerConf =
            toml::from_str("enabled = true").expect("swagger configuration should parse");

        assert!(config.enabled);
    }
}
