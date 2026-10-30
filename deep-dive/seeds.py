from multiprocessing import Pool

import matplotlib.pyplot as plt
import numpy as np

from train import moving_average, train

NUM_SEEDS = 10
SOLVED_STEPS = 475


def run_seed(seed):
    """Train with one seed and return how long each game lasted."""
    _, lengths = train(seed=seed)
    return lengths


def first_solved(averages):
    """First game (0-indexed) where the 50-game average reaches SOLVED_STEPS, or None."""
    for game, avg in enumerate(averages):
        if game >= 49 and avg >= SOLVED_STEPS:
            return game
    return None


if __name__ == "__main__":
    # Each seed is independent, so train them all at the same time
    with Pool() as pool:
        all_lengths = np.array(pool.map(run_seed, range(NUM_SEEDS)))

    averages = np.array([moving_average(lengths, 50) for lengths in all_lengths])

    print("seed  final avg  best avg  solved at  games at 500")
    for seed in range(NUM_SEEDS):
        solved = first_solved(averages[seed])
        print(f"{seed:4}  {averages[seed][-1]:9.1f}  {averages[seed].max():8.1f}  "
              f"{str(solved):>9}  {np.sum(all_lengths[seed] == 500):12}")
    print(f"mean final avg: {averages[:, -1].mean():.1f}")

    for seed in range(NUM_SEEDS):
        plt.plot(averages[seed], color="gray", alpha=0.4, linewidth=1)
    plt.plot(averages.mean(axis=0), color="C0", linewidth=2.5, label="mean of 10 seeds")
    plt.plot([], color="gray", alpha=0.4, label="one seed (average of last 50)")
    plt.axhline(500, color="black", linestyle=":", linewidth=1)
    plt.xlabel("game")
    plt.ylabel("steps the pole stayed up")
    plt.title("REINFORCE on CartPole, 10 seeds")
    plt.legend(loc="upper left")
    plt.savefig("seeds.png")
