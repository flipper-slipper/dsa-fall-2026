import gymnasium as gym
import matplotlib.pyplot as plt
import numpy as np

from reinforce import compute_returns, play_episode, update_weights


def train(num_games=1000, gamma=0.99, lr=0.01, seed=0):
    """Train a CartPole policy with REINFORCE.

    Each game: play with the current W, compute the returns, update W, and
    record how long the game lasted. Longer games mean a better policy.

    Args:
        num_games: how many games to play.
        gamma: discount factor between 0 and 1.
        lr: learning rate, how big a step each update takes.
        seed: seed for the starting weights, the random action choices and
            the games' starting positions.

    Returns:
        W: the trained 2x4 NumPy array of weights.
        lengths: list of how many steps each game lasted.
    """
    env = gym.make("CartPole-v1")
    rng = np.random.default_rng(seed)
    W = rng.normal(0, 0.01, size=(2, 4))  # small random starting weights

    lengths = []
    for game in range(num_games):
        states, actions, probs, rewards = play_episode(
            env, W, rng, seed=seed if game == 0 else None)
        returns = compute_returns(rewards, gamma)
        W = update_weights(W, states, actions, probs, returns, lr)
        lengths.append(len(rewards))

        if (game + 1) % 50 == 0:
            print(f"game {game + 1}: average length of last 50 = {np.mean(lengths[-50:]):.1f}")

    env.close()
    return W, lengths


def moving_average(values, n):
    """Average of the last n values at each point."""
    return [np.mean(values[max(0, i - n + 1):i + 1]) for i in range(len(values))]


if __name__ == "__main__":
    W, lengths = train()
    print("final W:\n", np.round(W, 3))

    plt.plot(lengths, alpha=0.3, label="each game")
    plt.plot(moving_average(lengths, 50), linewidth=2, label="average of last 50")
    plt.xlabel("game")
    plt.ylabel("steps the pole stayed up")
    plt.title("REINFORCE on CartPole")
    plt.legend()
    plt.savefig("training.png")
    plt.show()
