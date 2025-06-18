// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{Category, Content, Publisher};
use crate::serde_utils;

/// 3.2.13 Object: Site
/// 
/// This object should be included if the ad supported content is a website as opposed to a non-browser
/// application. A bid request must not contain both a Site and an App object. At a minimum, it is useful
/// to provide a site ID or page URL, but this is not strictly required.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Site {
    /// Exchange-specific site ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Site name (may be aliased at the publisher's request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Domain of the site (e.g., "mysite.foo.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Array of IAB content categories of the site. Refer to List 5.1.
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_utils::string_or_vec_category")]
    pub cat: Option<Vec<Category>>,

    /// Array of IAB content categories that describe the current section of the site. Refer to List 5.1.
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_utils::string_or_vec_category")]
    pub sectioncat: Option<Vec<Category>>,

    /// Array of IAB content categories that describe the current page or view of the site. Refer to List 5.1.
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_utils::string_or_vec_category")]
    pub pagecat: Option<Vec<Category>>,

    /// URL of the page where the impression will be shown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Referrer URL that caused navigation to the current page.
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,

    /// Search string that caused navigation to the current page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Indicates if the site has been programmed to optimize layout when viewed on mobile devices, where 0 = no, 1 = yes.
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_utils::bool_or_int_to_i32")]
    pub mobile: Option<i32>,

    /// Indicates if the site has a privacy policy, where 0 = no, 1 = yes.
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_utils::bool_or_int_to_i32")]
    pub privacypolicy: Option<i32>,

    /// Details about the Publisher (Section 3.2.15) of the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,

    /// Details about the Content (Section 3.2.16) within the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,

    /// Comma separated list of keywords about the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialization_skip_fields() {
        let site = Site {
            id: Some("site123".to_string()),
            domain: Some("example.com".to_string()),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&site).unwrap();
        let deserialized: Site = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(site, deserialized);
    }

    #[test]
    fn deserialize_minimal() {
        let json = r#"{"id":"site123"}"#;
        let site: Site = serde_json::from_str(json).unwrap();
        
        assert_eq!(site.id, Some("site123".to_string()));
        assert_eq!(site.name, None);
        assert_eq!(site.domain, None);
        assert_eq!(site.cat, None);
    }

    #[test]
    fn deserialize_full() {
        let json = r#"{
            "id": "site123",
            "name": "Example Site",
            "domain": "example.com",
            "cat": ["IAB1"],
            "sectioncat": ["IAB1-1"],
            "pagecat": ["IAB1-2"],
            "page": "https://example.com/page",
            "ref": "https://google.com",
            "search": "test query",
            "mobile": 1,
            "privacypolicy": 1,
            "keywords": "keyword1,keyword2"
        }"#;
        
        let site: Site = serde_json::from_str(json).unwrap();
        
        assert_eq!(site.id, Some("site123".to_string()));
        assert_eq!(site.name, Some("Example Site".to_string()));
        assert_eq!(site.domain, Some("example.com".to_string()));
        assert_eq!(site.page, Some("https://example.com/page".to_string()));
        assert_eq!(site.referrer, Some("https://google.com".to_string()));
        assert_eq!(site.search, Some("test query".to_string()));
        assert_eq!(site.mobile, Some(1));
        assert_eq!(site.privacypolicy, Some(1));
        assert_eq!(site.keywords, Some("keyword1,keyword2".to_string()));
    }

    #[test]
    fn serialize_with_publisher_and_content() {
        let mut site = Site::default();
        site.id = Some("site123".to_string());
        site.publisher = Some(Publisher {
            id: "pub123".to_string(),
            name: Some("Example Publisher".to_string()),
            ..Default::default()
        });
        site.content = Some(Content {
            id: Some("content123".to_string()),
            title: Some("Example Content".to_string()),
            ..Default::default()
        });

        let serialized = serde_json::to_string(&site).unwrap();
        let deserialized: Site = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(site, deserialized);
        assert!(site.publisher.is_some());
        assert!(site.content.is_some());
    }
}
