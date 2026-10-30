import numpy as np

from policy import choose_action, grad_log_prob


def play_episode(env, W, rng, seed=None):
    """Play one game of CartPole.

    For each step, choose an action based on current state and softmax probs, record the action, probs, state, reward.
    The game stops when the pole falls over or reaches 500 steps.

    Args:
        env: the CartPole environment (from gym.make("CartPole-v1")).
        W: 2x4 NumPy array of weights. Row 0 is left, row 1 is right.
        rng: NumPy random generator, used for the coin flips.
        seed: optional seed for the starting position of the pole.

    Returns:
        states: array of shape (T, 4), the state seen at each step.
        actions: array of shape (T,), the action taken at each step (0 or 1).
        probs: array of shape (T, 2), [P(left), P(right)] at each step.
        rewards: array of shape (T,), the reward at each step (+1 in CartPole).
        T is the number of steps the game lasted.
    """
    states, actions, probs, rewards = [], [], [], []

    state, _ = env.reset(seed=seed)
    done = False
    while not done:
        action, p = choose_action(W, state, rng)
        states.append(state)
        actions.append(action)
        probs.append(p)

        state, reward, terminated, truncated, _ = env.step(action)
        rewards.append(reward)
        done = terminated or truncated

    return np.array(states), np.array(actions), np.array(probs), np.array(rewards)


def compute_returns(rewards, gamma):
    """Compute the discounted return G_t for every step of a game.

    Loops backwards from the last step, since each return is that step's
    reward plus gamma times the next step's return.

    Args:
        rewards: array of shape (T,), the reward at each step.
        gamma: discount factor between 0 and 1 (e.g. 0.99).

    Returns:
        returns: array of shape (T,), where returns[t] is G_t.
    """
    returns = np.zeros(len(rewards))
    G = 0.0
    for t in reversed(range(len(rewards))):
        G = rewards[t] + gamma * G
        returns[t] = G
    return returns


def update_weights(W, states, actions, probs, returns, lr):
    """Change W after one game so good actions get more likely and bad ones less.

    Uses the returns, the gradients and the learning rate to change W. Each
    step's gradient is scaled by its normalized return, these are added up
    over the game, and W moves by lr times the total.

    Args:
        W: 2x4 NumPy array of weights. Row 0 is left, row 1 is right.
        states: array of shape (T, 4), the state seen at each step.
        actions: array of shape (T,), the action taken at each step.
        probs: array of shape (T, 2), [P(left), P(right)] at each step.
        returns: array of shape (T,), G_t for each step (from compute_returns).
        lr: learning rate, how big a step to take (e.g. 0.01).

    Returns:
        New 2x4 NumPy array of weights.
    """
    # 1e-8 avoids dividing by 0 when the game lasted only 1 step
    returns = (returns - returns.mean()) / (returns.std() + 1e-8)

    total = np.zeros_like(W, dtype=float)
    for t in range(len(returns)):
        total += returns[t] * grad_log_prob(states[t], actions[t], probs[t])
    return W + lr * total
