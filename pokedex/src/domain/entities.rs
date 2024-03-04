#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pokemon {
    pub number: PokemonNumber,
    name: PokemonName,
    types: PokemonTypes,
}

impl Pokemon {
    pub fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
        Self {
            number,
            name,
            types,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PokemonNumber(u16);

impl TryFrom<u16> for PokemonNumber {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        if v > 0 && v < 1000 {
            Ok(Self(v))
        } else {
            Err(())
        }
    }
}

impl From<PokemonNumber> for u16 {
    fn from(v: PokemonNumber) -> Self {
        v.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PokemonName(String);

impl TryFrom<String> for PokemonName {
    type Error = ();

    fn try_from(v: String) -> Result<Self, Self::Error> {
        if v.is_empty() {
            Err(())
        } else {
            Ok(Self(v))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PokemonType {
    Electric,
    Fire,
}

impl TryFrom<String> for PokemonType {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for PokemonType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Electric" => Ok(Self::Electric),
            "Fire" => Ok(Self::Fire),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PokemonTypes(Vec<PokemonType>);

impl TryFrom<Vec<String>> for PokemonTypes {
    type Error = ();

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(())
        } else {
            Ok(Self(
                value
                    .iter()
                    .filter_map(|t| t.as_str().try_into().ok())
                    .collect(),
            ))
        }
    }
}
