###############################################################################
# NAME:             fecccu.py
#
# AUTHOR:           Ethan D. Twardy <ethan.twardy@gmail.com>
#
# DESCRIPTION:      Module for parsing FECCCU bank statements
#
# CREATED:          09/20/2022
#
# LAST EDITED:      09/20/2022
###

import re
import subprocess

def header_row(input_file):
    """Locate the table header in the input stream"""
    for line in input_file:
        if re.match(r'^[\s]*Date[\s]*Transaction Type', line):
            return line

def print_table(input_file):
    print(header_row(input_file), end='')
    for line in input_file:
        if re.match(r'[\s0-9/]*\*\* Ending Balance \*\*', line):
            print(line, end='')
            return
        else:
            print(line, end='')

def extract_transactions(text_file: str):
    with open(text_file, 'r') as input_file:
        for line in input_file:
            if re.match(r'^[\s]*Transaction Detail', line):
                print_table(input_file)
                print()

def convert_to_text(input_file: str) -> str:
    """Convert the PDF file with path <input_file> to a text file"""
    subprocess.run("pdftotext -layout {}".format(input_file), check=True,
                   shell=True)
    return input_file.split('.')[0] + '.txt'

def parse(input_file: str):
    text_file = convert_to_text(input_file)

    # Get a number of tables for each account
    extract_transactions(text_file)

    # Filter on each account to remove erroneous transactions

###############################################################################
