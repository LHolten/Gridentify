from gridentify import *
import numpy as np
import time

good_values = set([1, 2, 3, 6, 12, 24, 48, 96, 192, 384, 768, 1536, 3072, 6144, 12288, 24578, 49152])

weights = np.array([
    [128, 256, 512, 1024, 2048],
    [64, 32, 16, 8, 4],
    [2, 1, 0, 1, 2],
    [4, 8, 16, 32, 64],
    [2048, 1024, 512, 256, 128]
])

hsnail3_weights = np.array([
    [20, 17, 16, 15, 15],
    [0, 0, 0, 0, 12],
    [1, 0, 0, 0, 11],
    [2, 0, 0, 0, 10],
    [5, 5, 6, 7, 10]
])
hsnail3_weights = 2 ** hsnail3_weights

hsnail4_weights = np.array([
    [2, 1, 1, 1, 2],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [2, 1, 1, 1, 2]
])
hsnail4_weights = 2 ** hsnail4_weights

corner_only_weights = np.array([
    [4, 3, 2, 3, 4],
    [3, 2, 1, 2, 3],
    [2, 1, 0, 1, 2],
    [3, 2, 1, 2, 3],
    [4, 3, 2, 3, 4]
])
corner_only_weights = 2 ** corner_only_weights

supernova_weights = np.array([
    [6, 3, 4, 5, 6],
    [5, 2, 1, 2, 3],
    [4, 1, 0, 1, 4],
    [3, 2, 1, 2, 5],
    [6, 5, 4, 3, 6]
])
supernova_weights = 2 ** supernova_weights

weights = corner_only_weights

a_weights = weights.reshape((25,))
b_weights = np.rot90(weights, 1).reshape((25,))
c_weights = np.rot90(weights, 2).reshape((25,))
d_weights = np.rot90(weights, 3).reshape((25,))
e_weights = np.fliplr(weights).reshape((25,))
f_weights = np.fliplr(np.rot90(weights, 1)).reshape((25,))
g_weights = np.fliplr(np.rot90(weights, 2)).reshape((25,))
h_weights = np.fliplr(np.rot90(weights, 3)).reshape((25,))


def eval_num_moves(game: Gridentify):
    num_ok_moves = 0

    for move in game.valid_moves():
        result = game.board[move[0]] * len(move)
        if result not in good_values:
            continue
        else:
            num_ok_moves += 1

    return num_ok_moves


def board_eval(game: Gridentify):
    board = np.array(game.board)
    # Scrabble eval
    a = np.einsum('x, x', a_weights, board)
    b = np.einsum('x, x', b_weights, board)
    c = np.einsum('x, x', c_weights, board)
    d = np.einsum('x, x', d_weights, board)
    e = np.einsum('x, x', e_weights, board)
    f = np.einsum('x, x', f_weights, board)
    g = np.einsum('x, x', g_weights, board)
    h = np.einsum('x, x', h_weights, board)
    scr = max(a, b, c, d, e, f, g, h)

    # Neighbor eval
    nbo = sum([len(n) for n in game.get_neighbours_of()]) + 1

    return nbo * scr


def tree_search(game: Gridentify, depth):
    if depth == 0:
        return board_eval(game), None

    valid_moves = game.valid_moves()

    if len(valid_moves) == 0:
        return board_eval(game), None

    good_moves = [move for move in valid_moves if game.board[move[0]] * len(move) in good_values]
    if len(good_moves) == 0:
        good_moves = valid_moves

    new_depth = depth // len(good_moves)
    move_evals = np.zeros(len(good_moves))

    for i, move in enumerate(good_moves):
        temp_game = game.copy()
        temp_game.make_move(move)
        move_evals[i], best_move = tree_search(temp_game, new_depth)

    move_index = np.argmax(move_evals)
    best_eval = move_evals[move_index]

    return best_eval, good_moves[move_index]


def bot():
    # Start a timer.
    start_time = time.time()

    # Make new game.
    test_seed = 20766236554
    # print(f'seed: {test_seed}')
    game = Gridentify(seed=test_seed)
    game.show_board()

    # Initial moves.
    valid_moves = game.valid_moves()

    move_num = 0
    while len(valid_moves) > 0:
        move_num += 1
        print(f'\n--- Move #{move_num} ---')
        print(f'Number of valid moves: {len(valid_moves)}')

        move = []
        while move not in valid_moves:
            # THIS IS WHERE THE MOVE MACHINE GOES.

            depth = 5000
            print(f'Depth for next move: {depth}')
            evaluation, move = tree_search(game, depth=depth)
            print(f'Move eval: {evaluation:.2f}')
            # input()

        # Show the game.
        show_move(move)
        print()
        game.make_move(move)
        game.show_board()
        print(f'\nScore: {game.score}')
        # Get new valid moves.
        valid_moves = game.valid_moves()

    print('\nGame Over')

    # End the timer
    end_time = time.time()

    seconds = end_time - start_time
    minutes = seconds // 60
    seconds %= 60
    print(f'Time: {int(minutes)}m {int(seconds)}s')


if __name__ == "__main__":
    bot()
