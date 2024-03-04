#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pokemon {
    pub number: PokemonNumber,
    pub name: PokemonName,
    pub types: PokemonTypes,
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

#[cfg(test)]
impl PokemonNumber {
    pub fn pikachu() -> Self {
        Self(42)
    }
}

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

#[cfg(test)]
impl PokemonName {
    pub fn pikachu() -> Self {
        Self("pikachu".to_owned())
    }

    pub fn charmander() -> Self {
        Self("charmander".to_owned())
    }

    pub fn empty() -> Self {
        Self("".to_owned())
    }
}

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

impl From<PokemonName> for String {
    fn from(v: PokemonName) -> Self {
        v.0
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

impl From<PokemonType> for String {
    fn from(value: PokemonType) -> Self {
        match value {
            PokemonType::Electric => "Electric".to_owned(),
            PokemonType::Fire => "Fire".to_owned(),
        }
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

#[cfg(test)]
impl PokemonTypes {
    pub fn pikachu() -> Self {
        Self(vec![PokemonType::Electric])
    }

    pub fn charmander() -> Self {
        Self(vec![PokemonType::Fire])
    }
}

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

impl From<PokemonTypes> for Vec<String> {
    fn from(v: PokemonTypes) -> Self {
        v.0.into_iter().map(|t| t.into()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pokemon_number_should_be_valid() {
        assert!(PokemonNumber::try_from(42).is_ok());
        assert!(PokemonNumber::try_from(0).is_err());
        assert!(PokemonNumber::try_from(1000).is_err());
    }
}
