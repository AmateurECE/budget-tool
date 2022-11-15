###############################################################################
# NAME:             transactions.py
#
# AUTHOR:           Ethan D. Twardy <ethan.twardy@gmail.com>
#
# DESCRIPTION:      Convert a csv table of transactions to a JSON input.
#
# CREATED:          09/22/2022
#
# LAST EDITED:      09/22/2022
###

import csv
import json
from typing import BinaryIO

def transform(input_filename: str, output_file: BinaryIO):
    """Read the input transactions from input_file (csv) and write as JSON"""
    fieldnames = ['date', 'description', 'amount']
    transactions = []
    with open(input_filename, 'r', encoding='utf-8') as input_file:
        reader = csv.DictReader(input_file, fieldnames=fieldnames)
        reader.__next__()
        for row in reader:
            transactions.append({
                'date': row['date'],
                'description': row['description'],
                'amount': float(row['amount']),
            })

    json.dump(transactions, output_file)

###############################################################################
