import numpy as np


def softmax(scores):
    """Turn a list of scores into probabilities that add up to 1.

    Uses the exponential softmax fucntion to turn scores into probabilities.

    Args:
        scores: NumPy array of scores, one per action (e.g. [left, right]).

    Returns:
        NumPy array of probabilities, same length as scores, all positive,
        summing to 1.
    """
    # Subtract the max score for computer safety with large numbers. This does not actually change the final output.
    e = np.exp(scores - np.max(scores))
    return e / e.sum()


def choose_action(W, state, rng):
    """Pick left or right based on softmax probabilities.

    Computes one score per action as a weighted sum of the state, turns the
    scores into probabilities with softmax, then samples an action.

    Args:
        W: 2x4 NumPy array of weights. Row 0 is left, row 1 is right.
        state: NumPy array of the 4 CartPole numbers
            [cart position, cart speed, pole angle, pole spin].
        rng: NumPy random generator

    Returns:
        action: 0 (left) or 1 (right).
        probs: NumPy array [P(left), P(right)], needed later for the update.
    """
    scores = W @ state
    probs = softmax(scores)
    action = rng.choice(2, p=probs)
    return action, probs


def grad_log_prob(state, action, probs):
    """Calculate how to make our action more likely given our state, which we
    might use if we later realize it was a good move.

    This is the gradient of log P(action | state) with respect to W. Each row
    gets (1 if it is the action taken, else 0) - probs[row], multiplied by the
    state. So the taken action's score goes up, the other goes down, weights
    with bigger state numbers change more, and the push is bigger when the
    action was unlikely.

    Args:
        state: NumPy array of the 4 CartPole numbers the action was chosen in.
        action: the action taken, 0 (left) or 1 (right).
        probs: NumPy array [P(left), P(right)] at that state.

    Returns:
        grad: 2x4 NumPy array, same shape as W. Adding a small multiple of it
            to W makes the action more likely; subtracting makes it less likely.
    """
    taken = np.zeros(2)
    taken[action] = 1
    return np.outer(taken - probs, state)
