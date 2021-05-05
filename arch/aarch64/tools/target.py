#!/bin/python3
''' Setup board specific target '''
import sys


def main(board_name):
    board_file = open(f"boards/{board_name}.txt").read()
    aarch64_file_target = open("targets/aarch64-novusk.json", "r+")
    aarch64_file_target.writelines(board_file)


if __name__ == '__main__':
    board = sys.argv[1]
    main(board)
