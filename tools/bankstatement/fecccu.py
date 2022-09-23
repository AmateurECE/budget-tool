###############################################################################
# NAME:             fecccu.py
#
# AUTHOR:           Ethan D. Twardy <ethan.twardy@gmail.com>
#
# DESCRIPTION:      Module for parsing FECCCU bank statements
#
# CREATED:          09/20/2022
#
# LAST EDITED:      09/22/2022
###

import re
import subprocess

# Columns
DATE = 0
DESCRIPTION = 1
DIFFERENCE = -2

def header_row(input_file) -> str:
    """Locate the table header in the input stream"""
    for line in input_file:
        if re.match(r'^[\s]*Date[\s]*Transaction Type', line):
            if re.match(r'.*\*\* Check Recon \*\*$', line):
                return line + input_file.readline()
            return line
    return None

def page_break(input_file):
    """Consume a page break from the input stream"""
    header_row(input_file) # Find the next header row

def beginning_balance():
    """Consume a period-beginning balance record from the input stream"""

def ending_balance():
    """Consume a period-end balance record from the input stream"""

def transaction_record(line: str, records):
    """Consume a transaction record from the input stream"""
    parts = re.split(r'\s{2,}', line.strip())
    records.append({
        'date': parts[DATE],
        'description': parts[DESCRIPTION],
        'difference': parts[DIFFERENCE],
    })

def transaction_record_continuation(line: str, records):
    """line is a continuation of the last transaction record"""
    records[-1]['description'] += ' ' + line.strip()

def parse_table(input_file) -> list[dict]:
    """Parse a single table of transactions from the input stream"""
    header_row(input_file)
    records = []
    for line in input_file:
        if re.match(r'[\s0-9/]*\*\* Ending Balance \*\*', line):
            ending_balance()
            return records
        if re.match(r'[\s0-9/]*\* Beginning Balance \*', line):
            beginning_balance()
        elif re.match(r'.*Page [0-9] of [0-9]$', line):
            page_break(input_file)
        elif not re.match(r'^[0-9/]+', line.strip()):
            transaction_record_continuation(line, records)
        else:
            transaction_record(line, records)
    return records

def extract_transactions(text_file: str) -> list[dict]:
    """Extract all transactions from this statement"""
    records = []
    with open(text_file, 'r', encoding='utf-8') as input_file:
        for line in input_file:
            if re.match(r'^[\s]*Transaction Detail', line):
                records.extend(parse_table(input_file))
    return records

def convert_to_text(input_file: str) -> str:
    """Convert the PDF file with path <input_file> to a text file"""
    subprocess.run(f'pdftotext -layout {input_file}', check=True,
                   shell=True)
    return input_file.split('.')[0] + '.txt'

def filter_transactions(records: list[dict]):
    """Filter records to remove duplicates."""

def parse(input_file: str):
    text_file = convert_to_text(input_file)

    # Get a number of tables for each account
    records = extract_transactions(text_file)

    # Filter on each account to remove duplicate transactions
    filter_transactions(records)
    return records

###############################################################################
