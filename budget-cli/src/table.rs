///////////////////////////////////////////////////////////////////////////////
// NAME:            table.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Simple table printing implementation.
//
// CREATED:         11/10/2022
//
// LAST EDITED:     11/15/2022
//
// Copyright 2022, Ethan D. Twardy
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
////

use table_iter::prelude::*;

const DEFAULT_PADDING_LENGTH: usize = 4;

// Calculate the length of the column (the length of the longest string in the
// column, plus some padding) for the dataset.
fn column_lengths<'a, T, I>(data: I, padding: Option<usize>) -> Vec<usize>
where
    T: AsRef<str> + 'a,
    I: Iterator<Item = &'a [T]>,
{
    let mut lengths: Vec<usize> = Vec::new();
    let padding = padding.unwrap_or(DEFAULT_PADDING_LENGTH);
    for row in data {
        for (i, column) in row.iter().enumerate() {
            let length = column.as_ref().len()
                + (padding - (column.as_ref().len() % padding));
            if let Some(element) = lengths.get_mut(i) {
                if length > *element {
                    *element = length;
                }
            } else {
                lengths.insert(i, length);
            }
        }
    }

    lengths
}

// Calculate the length of the padding string for a column, given the element
// that's being printed, its column index, and the lengths of all the columns
fn get_column_padding<T>(text: T, column_length: Option<usize>) -> String
where
    T: AsRef<str>,
{
    (0..(column_length
        .map(|length| length - text.as_ref().len())
        .unwrap_or_else(|| text.as_ref().len())))
        .map(|_| " ")
        .collect::<String>()
}

// Join a row of data with padding, concatenate into a string.
fn join_with_padding<T, S>(data: S, column_lengths: &[usize]) -> String
where
    T: AsRef<str>,
    S: AsRef<[T]>,
{
    data.as_ref()
        .iter()
        .enumerate()
        .map(|(i, text)| {
            text.as_ref().to_owned()
                + &get_column_padding(text, column_lengths.get(i).copied())
        })
        .collect::<String>()
}

// Print a table of data, with a row of headers
pub fn print_with_padding<T>(data: &[T], padding: Option<usize>)
where
    T: Fields + FieldNames,
{
    let headers = T::field_names();
    let fields = data
        .iter()
        .map(|row| row.fields())
        .collect::<Vec<FieldView>>();

    // Calculate the lengths of all the columns (including headers)
    let column_lengths = column_lengths(
        fields
            .iter()
            .map(|view| view.as_ref())
            .chain(vec![headers.as_ref()]),
        padding,
    );

    // Start by rendering the headers
    let sum: usize = column_lengths.iter().sum();
    let mut output = join_with_padding(headers, column_lengths.as_slice())
        + "\n"
        + &(0..sum).map(|_| "-").collect::<String>()
        + "\n";

    // Copy all the table data
    for row in fields {
        output =
            output + &join_with_padding(row, column_lengths.as_slice()) + "\n";
    }
    print!("{}", output);
}

pub fn print<T>(data: &[T])
where
    T: Fields + FieldNames,
{
    print_with_padding(data, None);
}

///////////////////////////////////////////////////////////////////////////////
