use indexmap::IndexMap;
use jaq_interpret::{Filter, ParseCtx};
use serde::{de::Error, Deserialize, Deserializer};
use serde_json::Value;

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default, rename_all = "kebab-case")]
pub struct CompilerManifest {
    pub compiler: Compiler,
    pub diagnostics: Diagnostics,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default, rename_all = "kebab-case")]
pub struct Compiler {}

fn deserialize_filter<'de, D: Deserializer<'de>>(de: D) -> Result<Option<Filter>, D::Error> {
    let st = <&str>::deserialize(de)?;

    let (output, errors) = jaq_parse::parse(st, jaq_parse::main());

    if let Some(err) = errors.first() {
        return Err(D::Error::custom(err));
    }

    let output = output
        .ok_or_else(|| D::Error::invalid_value(serde::de::Unexpected::Str(st), &"A jq filter"))?;

    let mut defs = ParseCtx::new(Vec::new());
    let f = defs.compile(output);

    Ok(Some(f))
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default, rename_all = "kebab-case")]
pub struct Diagnostics {
    #[serde(deserialize_with = "deserialize_filter")]
    pub error_type_path: Option<Filter>,
    pub codes: IndexMap<String, Value>,
}
