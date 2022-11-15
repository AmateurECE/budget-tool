###############################################################################
# NAME:             transactions.py
#
# AUTHOR:           Ethan D. Twardy <ethan.twardy@gmail.com>
#
# DESCRIPTION:      Convert a csv table of transactions to a JSON input.
#
# CREATED:          09/22/2022
#
# LAST EDITED:      11/15/2022
#
# Copyright 2022, Ethan D. Twardy
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
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
