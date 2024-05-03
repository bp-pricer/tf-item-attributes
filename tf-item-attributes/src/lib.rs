use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TFItemAttribute {
    UniqueCraftIndex = 229,
    SpellSetItemTintRBG = 1004,
    SpellSetHalloweenfootsteptype = 1005,
    SpellHalloweenVoiceModulation = 1006,
    SpellPumpkinExplosions = 1007,
    SpellGreenFlames = 1008,
    SpellDeathGhosts = 1009,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AttributeParseError {
    InvalidAttribute(String),
}

impl TryFrom<u64> for TFItemAttribute {
    type Error = AttributeParseError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            229 => Ok(TFItemAttribute::UniqueCraftIndex),
            1004 => Ok(TFItemAttribute::SpellSetItemTintRBG),
            1005 => Ok(TFItemAttribute::SpellSetHalloweenfootsteptype),
            1006 => Ok(TFItemAttribute::SpellHalloweenVoiceModulation),
            1007 => Ok(TFItemAttribute::SpellPumpkinExplosions),
            1008 => Ok(TFItemAttribute::SpellGreenFlames),
            1009 => Ok(TFItemAttribute::SpellDeathGhosts),
            _ => Err(AttributeParseError::InvalidAttribute(format!(
                "DefIndex ({}) is not mapped to a known TFItemAttribute, this is likely a useless attribute",
                value
            ))),
        }
    }
}

impl From<TFItemAttribute> for u64 {
    fn from(attribute: TFItemAttribute) -> u64 {
        match attribute {
            TFItemAttribute::UniqueCraftIndex => 229,
            TFItemAttribute::SpellSetItemTintRBG => 1004,
            TFItemAttribute::SpellSetHalloweenfootsteptype => 1005,
            TFItemAttribute::SpellHalloweenVoiceModulation => 1006,
            TFItemAttribute::SpellPumpkinExplosions => 1007,
            TFItemAttribute::SpellGreenFlames => 1008,
            TFItemAttribute::SpellDeathGhosts => 1009,
        }
    }
}
