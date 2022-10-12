///////////////////////////////////////////////////////////////////////////////
// NAME:            fields.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Traits and structs related to the use of fields for
//                  populating/rendering an HTML table
//
// CREATED:         10/12/2022
//
// LAST EDITED:     10/12/2022
////

///////////////////////////////////////////////////////////////////////////////
// FieldSpec
////

#[derive(Clone, Default, PartialEq)]
pub struct FieldSpec(Vec<String>);
impl FieldSpec {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a String> + 'a {
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

///////////////////////////////////////////////////////////////////////////////
// FieldView
////

#[derive(Clone, Default, PartialEq)]
pub struct FieldView(Vec<String>);
impl FieldView {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a String> + 'a {
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
