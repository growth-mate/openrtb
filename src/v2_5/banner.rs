// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::serde_utils;

use super::banner_ad_type::BannerAdType;
use super::format::Format;

// 3.2.6 Object: Banner
//
// This object represents the most general type of impression. Although the
// term “banner” may have very specific meaning in other contexts, here it can
// be many things including a simple static image, an expandable ad unit, or
// even in-banner video (refer to the Video object in Section 3.2.7 for the more
// generalized and full featured video ad units). An array of Banner objects
// can also appear within the Video to describe optional companion ads defined
// in the VAST specification. The presence of a Banner as a subordinate of the
// Imp object indicates that this impression is offered as a banner type
// impression. At the publisher’s discretion, that same impression may also be
// offered as video, audio, and/or native by also including as Imp subordinates
// objects of those types. However, any given bid for the impression must
// conform to one of the offered types.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Banner {
    // Array of format objects (Section 3.2.10) representing the
    // banner sizes permitted. If none are specified, then use of the
    // h and w attributes is highly recommended.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub format: Vec<Format>,

    // Exact width in device independent pixels (DIPS);
    // recommended if no format objects are specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<u32>,

    // Exact height in device independent pixels (DIPS);
    // recommended if no format objects are specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<u32>,

    // Blocked banner ad types. Refer to List 5.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub btype: Vec<BannerAdType>,

    /// Blocked creative attributes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub battr: Vec<u64>,

    /// Indicates if the banner is in the top frame as opposed to an
    /// iframe, where 0 = no, 1 = yes.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serde_utils::mbool_to_u8",
        deserialize_with = "serde_utils::u8_to_mbool"
    )]
    pub topframe: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialization_skip_fields() {
        let banner = Banner {
            format: vec![],
            w: None,
            h: None,
            btype: vec![],
            battr: vec![],
            topframe: None,
            ext: None,
        };

        let expected = r#"{}"#;
        let serialized = serde_json::to_string(&banner).unwrap();

        assert_eq!(expected, serialized)
    }

    #[test]
    fn serialization_with_topframe() {
        let banner = Banner {
            format: vec![],
            w: Some(300),
            h: Some(250),
            btype: vec![],
            battr: vec![],
            topframe: Some(true),
            ext: None,
        };

        let serialized = serde_json::to_string(&banner).unwrap();
        let deserialized: Banner = serde_json::from_str(&serialized).unwrap();

        assert_eq!(banner, deserialized);
        assert_eq!(banner.topframe, Some(true));
        
        // Verify it serializes to integer
        assert!(serialized.contains("\"topframe\":1"));
    }

    #[test]
    fn deserialize_topframe_yes() {
        let json = r#"{"w":300,"h":250,"topframe":1}"#;
        let banner: Banner = serde_json::from_str(json).unwrap();
        
        assert_eq!(banner.w, Some(300));
        assert_eq!(banner.h, Some(250));
        assert_eq!(banner.topframe, Some(true));
    }

    #[test]
    fn deserialize_topframe_no() {
        let json = r#"{"w":300,"h":250,"topframe":0}"#;
        let banner: Banner = serde_json::from_str(json).unwrap();
        
        assert_eq!(banner.w, Some(300));
        assert_eq!(banner.h, Some(250));
        assert_eq!(banner.topframe, Some(false));
    }

    #[test]
    fn deserialize_without_topframe() {
        let json = r#"{"w":300,"h":250}"#;
        let banner: Banner = serde_json::from_str(json).unwrap();
        
        assert_eq!(banner.w, Some(300));
        assert_eq!(banner.h, Some(250));
        assert_eq!(banner.topframe, None);
    }

    #[test]
    fn test_rubiconproject_compatibility() {
        // Test with the actual structure from rubiconproject test files
        let json = r#"{
            "w": 300,
            "h": 250,
            "pos": 1,
            "battr": [9, 1, 14014, 3, 13, 10, 8, 14],
            "topframe": 1
        }"#;
        
        let banner: Banner = serde_json::from_str(json).unwrap();
        
        assert_eq!(banner.w, Some(300));
        assert_eq!(banner.h, Some(250));
        assert_eq!(banner.topframe, Some(true));
        assert_eq!(banner.battr, vec![9, 1, 14014, 3, 13, 10, 8, 14]);
    }
}
