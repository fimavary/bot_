use std::{fmt::Display, str::FromStr};

use anyhow::Context;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct UserCity(Option<City>);

impl Display for UserCity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(city) => f.write_fmt(format_args!("{city}"))?,
            None => f.write_str("Университет не указан")?,
        }

        Ok(())
    }
}

impl UserCity {
    pub const fn get_city(self) -> Option<City> {
        self.0
    }

    pub const fn unspecified() -> Self {
        Self(None)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct City(i32);

impl City {
    pub fn city(&self) -> &'static str {
        city_by_id(self.0).expect("university not found")
    }

    pub fn subject(&self) -> &'static str {
        self.city()
    }

    pub fn county(&self) -> &'static str {
        self.city()
    }
}

impl TryFrom<i32> for City {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        city_by_id(value).context("university not found")?;
        Ok(Self(value))
    }
}

impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.city())
    }
}

impl FromStr for UserCity {
    type Err = ();

    fn from_str(query: &str) -> Result<Self, Self::Err> {
        let normalized = query.trim().to_lowercase();
        let id = match normalized.as_str() {
            "мфти" => 1,
            "вшэ" => 2,
            "финансовый университет" => 3,
            _ => return Err(()),
        };

        Ok(Self(Some(City(id))))
    }
}

impl TryFrom<Option<i32>> for UserCity {
    type Error = anyhow::Error;

    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        let city = match value {
            Some(id) => Self(Some(id.try_into()?)),
            None => Self(None),
        };
        Ok(city)
    }
}

impl From<UserCity> for Option<i32> {
    fn from(value: UserCity) -> Self {
        value.0.map(|v| v.0)
    }
}

pub fn county_by_id(id: i32) -> Option<&'static str> {
    city_by_id(id)
}

pub fn subject_by_id(id: i32) -> Option<&'static str> {
    city_by_id(id)
}

pub fn city_by_id(id: i32) -> Option<&'static str> {
    match id {
        1 => Some("МФТИ"),
        2 => Some("ВШЭ"),
        3 => Some("Финансовый университет"),
        _ => None,
    }
}

pub fn county_exists(name: &str) -> bool {
    city_exists(name)
}

pub fn subject_exists(name: &str) -> bool {
    city_exists(name)
}

pub fn city_exists(name: &str) -> bool {
    matches!(
        name.trim().to_lowercase().as_str(),
        "мфти" | "вшэ" | "финансовый университет"
    )
}
