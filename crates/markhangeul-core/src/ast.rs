use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::errors::ParseError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkHangeulDocument {
    pub r#type: String,
    pub source: String,
    pub nodes: Vec<MarkHangeulToken>,
    pub errors: Vec<ParseError>,
}

impl MarkHangeulDocument {
    pub fn new(
        source: impl Into<String>,
        nodes: Vec<MarkHangeulToken>,
        errors: Vec<ParseError>,
    ) -> Self {
        Self {
            r#type: "document".to_string(),
            source: source.into(),
            nodes,
            errors,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum MarkHangeulToken {
    Text(TextNode),
    Markhangeul(Box<MarkHangeulNode>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextNode {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkHangeulNode {
    pub id: String,
    pub text: String,
    pub raw_annotation: String,
    pub scope: Scope,
    pub attributes: MarkHangeulAttributes,
    pub start: usize,
    pub end: usize,
    pub annotation_start: usize,
    pub annotation_end: usize,
    pub errors: Vec<ParseError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Scope {
    Grapheme,
    Word,
    Range,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkHangeulAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<Pitch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stress: Option<Stress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Volume>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tone: Option<Tone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tone_system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tone_contour: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonation: Option<Phonation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound_shape: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_color: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phoneme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nasal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspiration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fortis: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lenis: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub palatalization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retroflexion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liaison: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduced: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assimilation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mora: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syllable_role: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extras: BTreeMap<String, AttributeValue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttributeValue {
    Bool(bool),
    Number(i64),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Pitch {
    Low,
    Mid,
    High,
    Rise,
    Fall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Duration {
    ExtraShort,
    Short,
    SlightShort,
    Normal,
    SlightLong,
    Long,
    ExtraLong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Stress {
    Weak,
    Normal,
    Strong,
    ExtraStrong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Volume {
    Soft,
    Normal,
    Loud,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Tone(pub String);

impl Tone {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn neutral() -> Self {
        Self::new("neutral")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Phonation {
    #[default]
    Modal,
    Breathy,
    Creaky,
    Glottalized,
}

impl Phonation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Modal => "modal",
            Self::Breathy => "breathy",
            Self::Creaky => "creaky",
            Self::Glottalized => "glottalized",
        }
    }
}
