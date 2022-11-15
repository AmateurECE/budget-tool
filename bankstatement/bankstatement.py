###############################################################################
# NAME:             bankstatement.py
#
# AUTHOR:           Ethan D. Twardy <ethan.twardy@gmail.com>
#
# DESCRIPTION:      Entrypoint for the tool.
#
# CREATED:          09/20/2022
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

import argparse
import sys
import fecccu
import transactions

def main():
    parser = argparse.ArgumentParser(description='Convert bank statements')
    parser.add_argument('-f', '--format', default='fecccu',
                        choices=['fecccu', 'csv'],
                        dest='statement_format', help='Input format')
    parser.add_argument('statement', help='Input file (bank statement)')
    parser.add_argument('-o', '--output', help='Output file')
    args = parser.parse_args()

    # Parse records from input statement
    if 'fecccu' == args.statement_format:
        if args.output:
            with open(args.output, 'w', encoding='utf-8') as output_file:
                fecccu.write(args.statement, output_file)
        else:
            fecccu.write(args.statement, sys.stdout)
    elif 'csv' == args.statement_format:
        if args.output:
            with open(args.output, 'w', encoding='utf-8') as output_file:
                transactions.transform(args.statement, output_file)
        else:
            transactions.transform(args.statement, sys.stdout)

if __name__ == '__main__':
    main()

###############################################################################
