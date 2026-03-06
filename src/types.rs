use std::{fmt::Display, str::FromStr};

use anyhow::bail;
use bitflags::bitflags;
use chrono::Datelike;
use entities::{
    sea_orm_active_enums::{self, Gender},
    users,
};
use itertools::Itertools;
use sea_orm::ActiveValue;

use crate::cities::UserCity;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocationFilter {
    City,
    Subject,
    County,
    Country,
}

impl From<sea_orm_active_enums::LocationFilter> for LocationFilter {
    fn from(value: sea_orm_active_enums::LocationFilter) -> Self {
        match value {
            sea_orm_active_enums::LocationFilter::SameCity => Self::City,
            sea_orm_active_enums::LocationFilter::SameSubject => Self::Subject,
            sea_orm_active_enums::LocationFilter::SameCounty => Self::County,
            sea_orm_active_enums::LocationFilter::SameCountry => Self::Country,
        }
    }
}

impl From<LocationFilter> for sea_orm_active_enums::LocationFilter {
    fn from(value: LocationFilter) -> Self {
        match value {
            LocationFilter::City => Self::SameCity,
            LocationFilter::Subject => Self::SameSubject,
            LocationFilter::County => Self::SameCounty,
            LocationFilter::Country => Self::SameCountry,
        }
    }
}

impl FromStr for LocationFilter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s == "Все университеты" {
            Self::Country
        } else if s.starts_with("Только ") {
            Self::City
        } else {
            bail!("can't parse text into LocationFilter")
        })
    }
}

/// Gender of user
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UserGender {
    Female,
    Male,
}

impl From<Gender> for UserGender {
    fn from(value: Gender) -> Self {
        match value {
            Gender::Female => Self::Female,
            Gender::Male => Self::Male,
        }
    }
}

impl From<UserGender> for Gender {
    fn from(value: UserGender) -> Self {
        match value {
            UserGender::Female => Self::Female,
            UserGender::Male => Self::Male,
        }
    }
}

impl FromStr for UserGender {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Я парень" => Ok(Self::Male),
            "Я девушка" => Ok(Self::Female),
            _ => bail!("can't parse"),
        }
    }
}

impl Display for UserGender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let emoji = match self {
            Self::Female => "♀️",
            Self::Male => "♂️",
        };

        f.write_str(emoji)
    }
}

/// Filter of partner's gender
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenderFilter {
    Female,
    Male,
    All,
}

impl From<Option<Gender>> for GenderFilter {
    fn from(value: Option<Gender>) -> Self {
        match value {
            Some(Gender::Female) => Self::Female,
            Some(Gender::Male) => Self::Male,
            None => Self::All,
        }
    }
}

impl From<GenderFilter> for Option<Gender> {
    fn from(value: GenderFilter) -> Self {
        match value {
            GenderFilter::Female => Some(Gender::Female),
            GenderFilter::Male => Some(Gender::Male),
            GenderFilter::All => None,
        }
    }
}

impl FromStr for GenderFilter {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Девушку" => Ok(Self::Female),
            "Парня" => Ok(Self::Male),
            "Не важно" => Ok(Self::All),
            _ => bail!("can't parse"),
        }
    }
}

impl Display for GenderFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let emoji = match self {
            Self::Female => "Девушку",
            Self::Male => "Парня",
            Self::All => "Не важно",
        };

        f.write_str(emoji)
    }
}

pub struct GraduationYear(i16);

impl From<i16> for GraduationYear {
    fn from(value: i16) -> Self {
        Self(value)
    }
}

impl From<GraduationYear> for i16 {
    fn from(value: GraduationYear) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grade(i8);

impl TryFrom<i8> for Grade {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if (1..=6).contains(&value) {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

impl Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} курс", self.0))
    }
}

impl From<Grade> for GraduationYear {
    fn from(grade: Grade) -> Self {
        let date = chrono::Local::now();

        let year = if date.month() < 9 {
            date.year() as i16 + (6 - i16::from(grade.0))
        } else {
            date.year() as i16 + (6 - i16::from(grade.0)) + 1
        };

        Self(year)
    }
}

impl From<GraduationYear> for Grade {
    fn from(graduation_year: GraduationYear) -> Self {
        let date = chrono::Local::now();

        let grade = if date.month() < 9 {
            6 - (graduation_year.0 - date.year() as i16)
        } else {
            6 - (graduation_year.0 - date.year() as i16) + 1
        };

        Self(grade as i8)
    }
}

// pub struct UserSettings {
//     id: i64,
// }

// impl From<users::Model> for UserSettings {
//     fn from(value: users::Model) -> Self {
//         Self { id: value.id }
//     }
// }

/// Public profile of user
pub struct PublicProfile {
    name: String,
    gender: UserGender,
    grade: Grade,
    subjects: UserSubjects,
    dating_purpose: DatingPurpose,
    city: UserCity,
    about: String,
}

impl TryFrom<&users::Model> for PublicProfile {
    type Error = anyhow::Error;

    fn try_from(value: &users::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.clone(),
            gender: value.gender.clone().into(),
            grade: GraduationYear::from(value.graduation_year).into(),
            subjects: value.subjects.try_into()?,
            dating_purpose: value.dating_purpose.try_into()?,
            city: value.city.try_into()?,
            about: value.about.clone(),
        })
    }
}

impl Display for PublicProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {}, {}.\n🔎 Интересует: {}.\n📚 {}\n.🧭 {}.\n\n{}",
            self.gender,
            self.name,
            self.grade,
            self.dating_purpose,
            self.subjects,
            self.city,
            self.about
        ))
    }
}

bitflags! {
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
    pub struct Subjects: i32 {
        const Art = 1 << 0;
        const Astronomy = 1 << 1;
        const Biology = 1 << 2;
        const Chemistry = 1 << 3;
        const Chinese = 1 << 4;
        const Ecology = 1 << 5;
        const Economics = 1 << 6;
        const English = 1 << 7;
        const French = 1 << 8;
        const Geography = 1 << 9;
        const German = 1 << 10;
        const History = 1 << 11;
        const Informatics = 1 << 12;
        const Italian = 1 << 13;
        const Law = 1 << 14;
        const Literature = 1 << 15;
        const Math = 1 << 16;
        const Physics = 1 << 17;
        const Russian = 1 << 18;
        const Safety = 1 << 19;
        const Social = 1 << 20;
        const Spanish = 1 << 21;
        const Sport = 1 << 22;
        const Technology = 1 << 23;
    }
}

impl Subjects {
    /// Name of exactly one subject
    pub const fn name(&self) -> std::result::Result<&'static str, ()> {
        Ok(match *self {
            Self::Art => "Тех-стартапы 🚀",
            Self::Astronomy => "Наука и исследования 🔬",
            Self::Biology => "Медицина и биотех 🧬",
            Self::Chemistry => "Программирование 💻",
            Self::Chinese => "ИИ и Data Science 🤖",
            Self::Ecology => "Менеджмент 📈",
            Self::Economics => "Экономика и финансы 💰",
            Self::English => "Маркетинг 📣",
            Self::French => "Дизайн 🎨",
            Self::Geography => "Кибербезопасность 🔐",
            Self::German => "Геймдев 🎮",
            Self::History => "Робототехника 🦾",
            Self::Informatics => "Математика и аналитика 📐",
            Self::Italian => "Физика и инженерия ⚙️",
            Self::Law => "Право ⚖️",
            Self::Literature => "Психология 🧠",
            Self::Math => "Иностранные языки 🌍",
            Self::Physics => "Путешествия ✈️",
            Self::Russian => "Спорт 🏅",
            Self::Safety => "Музыка 🎵",
            Self::Social => "Волонтёрство 🤝",
            Self::Spanish => "Кино и сериалы 🎬",
            Self::Sport => "Книги 📚",
            Self::Technology => "Настолки и квизы 🎲",
            _ => return Err(()),
        })
    }
}

macro_rules! impl_display_bitflags {
    ($type:ident) => {
        impl Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for (i, name) in Self::all()
                    .into_iter()
                    .filter(|s| self.contains(*s))
                    .map(|s| s.name().unwrap())
                    .sorted_unstable_by_key(|n| n.to_lowercase())
                    .enumerate()
                {
                    if i != 0 {
                        f.write_str(", ")?;
                    }
                    f.write_str(name)?;
                }

                Ok(())
            }
        }
    };
}

impl_display_bitflags! {Subjects}

bitflags! {
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
    pub struct DatingPurpose: i16 {
        const Friendship = 1 << 0;
        const Studies = 1 << 1;
        const Relationship = 1 << 2;
    }
}

impl DatingPurpose {
    /// Name of exactly one purpose
    pub const fn name(&self) -> std::result::Result<&'static str, ()> {
        Ok(match *self {
            Self::Friendship => "Дружба 🧑‍🤝‍🧑",
            Self::Studies => "Учёба 📚",
            Self::Relationship => "Отношения 💕",
            _ => return Err(()),
        })
    }
}

impl_display_bitflags! {DatingPurpose}

impl TryFrom<i16> for DatingPurpose {
    type Error = anyhow::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let Some(purpose) = Self::from_bits(value) else {
            bail!("can't construct purpose from bits")
        };

        Ok(purpose)
    }
}

impl From<DatingPurpose> for i16 {
    fn from(value: DatingPurpose) -> Self {
        value.0.bits()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserSubjects(Subjects);

impl Display for UserSubjects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.bits() == 0 {
            f.write_str("Интересы не указаны.")?;
        } else {
            f.write_fmt(format_args!("Интересы: {}", self.0))?;
        }

        Ok(())
    }
}

impl TryFrom<i32> for UserSubjects {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let Some(subjects) = Subjects::from_bits(value) else {
            bail!("can't construct subjects from bits")
        };

        Ok(Self(subjects))
    }
}

impl From<UserSubjects> for i32 {
    fn from(value: UserSubjects) -> Self {
        value.0.bits()
    }
}

impl From<UserSubjects> for Subjects {
    fn from(value: UserSubjects) -> Self {
        value.0
    }
}

impl From<Subjects> for UserSubjects {
    fn from(value: Subjects) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubjectsFilter(Subjects);

impl Display for SubjectsFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.bits() == 0 {
            f.write_str("Интересы партнёра не важны.")?;
        } else {
            f.write_fmt(format_args!(
                "Интересы, хотя-бы один из которых должен быть у человека, которого вы ищете: {}",
                self.0
            ))?;
        }

        Ok(())
    }
}

impl From<SubjectsFilter> for Subjects {
    fn from(value: SubjectsFilter) -> Self {
        value.0
    }
}

impl From<Subjects> for SubjectsFilter {
    fn from(value: Subjects) -> Self {
        Self(value)
    }
}

impl TryFrom<i32> for SubjectsFilter {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let Some(subjects) = Subjects::from_bits(value) else {
            bail!("can't construct subjects from bits")
        };

        Ok(Self(subjects))
    }
}

impl From<SubjectsFilter> for i32 {
    fn from(value: SubjectsFilter) -> Self {
        value.0.bits()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct UserSettings {
    pub id: i64,
    pub name: Option<String>,
    pub gender: Option<UserGender>,
    pub gender_filter: Option<GenderFilter>,
    pub about: Option<String>,
    pub active: Option<bool>,
    pub grade: Option<Grade>,
    pub grade_up_filter: Option<i16>,
    pub grade_down_filter: Option<i16>,
    pub subjects: Option<UserSubjects>,
    pub subjects_filter: Option<SubjectsFilter>,
    pub dating_purpose: Option<DatingPurpose>,
    pub city: Option<UserCity>,
    pub location_filter: Option<LocationFilter>,
}

impl TryFrom<users::Model> for UserSettings {
    type Error = anyhow::Error;

    fn try_from(value: users::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            name: Some(value.name),
            gender: Some(value.gender.into()),
            gender_filter: Some(value.gender_filter.into()),
            about: Some(value.about),
            active: Some(value.active),
            grade: Some(GraduationYear::from(value.graduation_year).into()),
            grade_up_filter: Some(value.grade_up_filter),
            grade_down_filter: Some(value.grade_down_filter),
            subjects: Some(value.subjects.try_into()?),
            subjects_filter: Some(value.subjects_filter.try_into()?),
            dating_purpose: Some(value.dating_purpose.try_into()?),
            city: Some(value.city.try_into()?),
            location_filter: Some(value.location_filter.into()),
        })
    }
}

impl UserSettings {
    pub fn with_id(id: i64) -> Self {
        Self { id, ..Default::default() }
    }

    pub fn into_active_model(self) -> users::ActiveModel {
        macro_rules! convert {
            ($name:expr) => {
                $name.map_or_else(
                    || ActiveValue::NotSet,
                    |v| ActiveValue::Set(v.into()),
                )
            };
        }

        users::ActiveModel {
            id: ActiveValue::Set(self.id),
            name: convert!(self.name),
            gender: convert!(self.gender),
            gender_filter: convert!(self.gender_filter),
            about: convert!(self.about),
            active: convert!(self.active),
            last_activity: ActiveValue::NotSet,
            graduation_year: convert!(self.grade.map(GraduationYear::from)),
            grade_up_filter: convert!(self.grade_up_filter),
            grade_down_filter: convert!(self.grade_down_filter),
            subjects: convert!(self.subjects),
            subjects_filter: convert!(self.subjects_filter),
            dating_purpose: convert!(self.dating_purpose),
            city: convert!(self.city),
            location_filter: convert!(self.location_filter),
        }
    }
}
