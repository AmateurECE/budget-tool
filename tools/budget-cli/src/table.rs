///////////////////////////////////////////////////////////////////////////////
// NAME:            table.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Simple table printing implementation for the CLI utility.
//
// CREATED:         11/10/2022
//
// LAST EDITED:     11/10/2022
////

const DEFAULT_PADDING_LENGTH: usize = 4;

// Calculate the length of the column (the length of the longest string in the
// column, plus some padding) for the dataset.
fn column_lengths<'a, T, I>(data: I, padding: Option<usize>) -> Vec<usize>
where
    T: AsRef<str> + 'a,
    I: Iterator<Item = &'a &'a [T]>,
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
        .unwrap_or(text.as_ref().len())))
        .map(|_| " ")
        .collect::<String>()
}

// Join a row of data with padding, concatenate into a string.
fn join_with_padding<T>(data: &[T], column_lengths: &[usize]) -> String
where
    T: AsRef<str>,
{
    data.iter()
        .enumerate()
        .map(|(i, text)| {
            text.as_ref().to_owned()
                + &get_column_padding(text, column_lengths.get(i).copied())
        })
        .collect::<String>()
}

// Print a table of data, with a row of headers
pub fn print_with_padding<T>(
    data: &[&[T]],
    headers: &[T],
    padding: Option<usize>,
) where
    T: AsRef<str>,
{
    // Calculate the lengths of all the columns
    let column_lengths =
        column_lengths(data.iter().chain(&vec![headers]), padding);

    // Start with the headers
    let sum: usize = column_lengths.iter().sum();
    let mut output = join_with_padding(headers, column_lengths.as_slice())
        + "\n"
        + &(0..sum).map(|_| "-").collect::<String>()
        + "\n";

    // Copy all the table data
    for row in data {
        output =
            output + &join_with_padding(row, column_lengths.as_slice()) + "\n";
    }
    print!("{}", output);
}

pub fn print<T>(data: &[&[T]], headers: &[T])
where
    T: AsRef<str>,
{
    print_with_padding(data, headers, None);
}

///////////////////////////////////////////////////////////////////////////////
