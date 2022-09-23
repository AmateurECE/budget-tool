###############################################################################
# NAME:             bankstatement.py
#
# AUTHOR:           Ethan D. Twardy <ethan.twardy@gmail.com>
#
# DESCRIPTION:      Entrypoint for the tool.
#
# CREATED:          09/20/2022
#
# LAST EDITED:      09/22/2022
###

import argparse
import sys
import fecccu

def main():
    parser = argparse.ArgumentParser(description='Convert bank statements')
    parser.add_argument('-f', '--format', default='FECCCU', choices=['FECCCU'],
                        dest='statement_format', help='Input format')
    parser.add_argument('statement', help='Input file (bank statement)')
    parser.add_argument('-o', '--output', help='Output file')
    args = parser.parse_args()

    # Parse records from input statement
    if 'FECCCU' == args.statement_format:
        if args.output:
            with open(args.output, 'w', encoding='utf-8') as output_file:
                fecccu.write(args.statement, output_file)
        else:
            fecccu.write(args.statement, sys.stdout)

if __name__ == '__main__':
    main()

###############################################################################
