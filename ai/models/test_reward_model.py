from reward_model import calculate_reward

def test_calculate_reward():
    assert calculate_reward('click', 1.0) == 1.0
    assert calculate_reward('scroll', 0.8) == 0.4
    assert calculate_reward('visit', 1.2) == 2.4
    print('All tests passed!')

if __name__ == "__main__":
    test_calculate_reward()
