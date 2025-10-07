use derivative::Derivative;
use serde::Deserialize;
use serde::Serialize;
use smart_default::SmartDefault;
use validator::Validate;

/// Request structure for foo search operations
#[derive(Derivative, Deserialize, SmartDefault, Validate)]
#[derivative(Debug)]
pub struct FooRequest {
    /// Search keyword for foo items
    #[default("please input search key words")]
    #[validate(length(min = 2, max = 200))]
    pub key_word: String,
}

/// Response structure for foo search operations
#[derive(Serialize, Default, Debug)]
pub struct FooResponse {
    /// List of foo items matching the search criteria
    pub list: Vec<FooItem>,
}

/// Individual foo item structure
#[derive(Default, Debug, Serialize)]
pub struct FooItem {
    /// Title of the foo item
    pub title: String,
    /// Content of the foo item
    pub content: String,
}
