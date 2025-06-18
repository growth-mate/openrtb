// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Content {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isrc: Option<String>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub producer: Option<Producer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prodq: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<i64>,

    #[serde(rename = "contentrating", skip_serializing_if = "Option::is_none")]
    pub content_rating: Option<String>,

    #[serde(rename = "userrating", skip_serializing_if = "Option::is_none")]
    pub user_rating: Option<String>,

    #[serde(rename = "qagmediarating", skip_serializing_if = "Option::is_none")]
    pub qag_media_rating: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    #[serde(rename = "livestream", skip_serializing_if = "Option::is_none")]
    pub live_stream: Option<i64>,

    #[serde(rename = "sourcerelationship", skip_serializing_if = "Option::is_none")]
    pub source_relationship: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddable: Option<i64>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub data: Option<Vec<Data>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialization_skip_fields() {
        let serialized = r#"{
            "id": "1234"
        }"#;

        let res = serde_json::from_str(serialized);

        let _: Content = match res {
            Ok(x) => x,
            Err(e) => panic!("{:?}", e),
        };
    }
}
