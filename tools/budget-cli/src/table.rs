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

use std::cmp;

const PADDING_LENGTH: usize = 4;

pub fn print_table(data: Vec<Vec<String>>, headers: Vec<String>) {
    // Calculate the lengths of all the columns
    let column_length = data
        .iter()
        .map(|column| {
            let max = cmp::max(
                column
                    .iter()
                    .map(|element| element.len())
                    .max()
                    .unwrap_or(0),
                headers.first().map(|h| h.len()).unwrap_or(0),
            );
            if 0 == max % PADDING_LENGTH {
                max
            } else {
                max + (PADDING_LENGTH - (max % PADDING_LENGTH))
            }
        })
        .collect::<Vec<usize>>();
    let sum: usize = column_length.iter().sum();

    // Start with the headers
    let mut output = headers
        .iter()
        .enumerate()
        .map(|(i, header)| {
            header.to_owned()
                + &(0..(column_length[i] - header.len()))
                    .map(|_| " ")
                    .collect::<String>()
        })
        .collect::<String>()
        + "\n";
    output += &(0..sum).map(|_| "-").collect::<String>();
    output += "\n";

    // Copy all the table data
    for i in 0..data[0].len() {
        for j in 0..data.len() {
            output += &data[j][i];
            output += &(0..(column_length[j] - data[j][i].len()))
                .map(|_| " ")
                .collect::<String>();
        }
    }
    println!("{}", output);
}

///////////////////////////////////////////////////////////////////////////////
