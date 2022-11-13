///////////////////////////////////////////////////////////////////////////////
// NAME:            fields.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Traits and structs related to the iteration over the fields
//                  of a struct.
//
// CREATED:         10/12/2022
//
// LAST EDITED:     11/12/2022
////

///////////////////////////////////////////////////////////////////////////////
// FieldSpec
////

#[derive(Clone, Default, PartialEq, Eq)]
pub struct FieldSpec(Vec<String>);
impl FieldSpec {
    pub fn iter(&self) -> impl Iterator<Item = &'_ String> + '_ {
        self.0.iter()
    }
}

impl FromIterator<String> for FieldSpec {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self(iter.into_iter().collect::<Vec<String>>())
    }
}

impl From<Vec<String>> for FieldSpec {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl AsRef<[String]> for FieldSpec {
    fn as_ref(&self) -> &[String] {
        self.0.as_slice()
    }
}

///////////////////////////////////////////////////////////////////////////////
// FieldView
////

#[derive(Clone, Default, PartialEq, Eq)]
pub struct FieldView(Vec<String>);
impl FieldView {
    pub fn iter(&self) -> impl Iterator<Item = &'_ String> + '_ {
        self.0.iter()
    }
}

impl FromIterator<String> for FieldView {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self(iter.into_iter().collect::<Vec<String>>())
    }
}

impl From<Vec<String>> for FieldView {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl AsRef<[String]> for FieldView {
    fn as_ref(&self) -> &[String] {
        self.0.as_slice()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Traits
////

pub trait FieldNames {
    fn field_names() -> FieldSpec;
}

pub trait Fields {
    fn fields(&self) -> FieldView;
}

///////////////////////////////////////////////////////////////////////////////
