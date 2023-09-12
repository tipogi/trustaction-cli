pub enum ScriptType {
    // P2PKH, Starts with 1
    Legacy,
    // P2SH, Starts with 3
    NestedSegwit,
    // P2WPKH, Starts with bc1
    NativeSegwit
}

impl ScriptType {
    pub fn into_purpose<'a>(&self) -> &'a str {
        match self {
            Self::Legacy => "44h",
            Self::NestedSegwit => "49h",
            Self::NativeSegwit => "84h"
        }
    }

    pub fn into_descriptor<'a>(&self) -> String {
        let script_type = match self {
            Self::Legacy => "pkh(",
            Self::NestedSegwit => "sh(wpkh",
            Self::NativeSegwit => "wpkh("
        };
        String::from(script_type)
    }
}