###############################################################################
# NAME:             bankstatement.py
#
# AUTHOR:           Ethan D. Twardy <ethan.twardy@gmail.com>
#
# DESCRIPTION:      Entrypoint for the tool.
#
# CREATED:          09/20/2022
#
# LAST EDITED:      09/20/2022
###

import argparse
import fecccu

def main():
    parser = argparse.ArgumentParser(description='Convert bank statements')
    parser.add_argument('-f', '--format', default='FECCCU', choices=['FECCCU'],
                        dest='statement_format', help='Input format')
    parser.add_argument('statement', help='Input file (bank statement)')
    args = parser.parse_args()

    if 'FECCCU' == args.statement_format:
        fecccu.parse(args.statement)

if __name__ == '__main__':
    main()

###############################################################################
