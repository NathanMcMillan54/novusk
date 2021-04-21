import sys


def main(arg):
    board_file = open("arch/arm/src/akernel/board.rs", "r+")
    board_file.seek(0)
    board_file.writelines(f'pub const BOARD: &str = "{arg}";')
    board_file.close()


if __name__ == '__main__':
    board = sys.argv[1]
    main(board)
