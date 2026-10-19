# Deep Dive: Policy Gradient (REINFORCE) on CartPole


## Overview

I will implement REINFORCE, a policy gradient algorithm, from scratch in Python with NumPy and train it to balance the pole in Gymnasium's CartPole-v1 simulation. I will measure how reliably and quickly the agent learns across random seeds. No ML libraries are used; the simulator is only the test environment.

## Background

REINFORCE learns the probability of taking each action directly. After each episode, it makes actions that led to high reward more likely and the rest less likely. REINFORCE is a continuous domain algorithm. 

## Deliverables

A GitHub repo with the code and a README covering how the algorithm works, the results, and how to run it.

## Timeline

| Task | Hours |
| --- | --- |
| Learn REINFORCE; set up Gymnasium CartPole | 1.5 |
| Write the softmax policy and check its gradient | 1 |
| Implement REINFORCE and get it learning | 3 |
| Run 10 seeds; plot average learning curves | 1 |
| Write the README; record a demo | 1.5 |
| **Total** | **8** |

## Grading rubric

- **A:** REINFORCE reliably balances the pole, with learning curves over 10 seeds and a clear README.
- **B:** REINFORCE learns to balance the pole, with a basic README but results from only one or two runs.
