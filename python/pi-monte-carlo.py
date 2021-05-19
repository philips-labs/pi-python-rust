import random as r

def montecarlo_pi(trials):
  hit = 0
  r.seed()

  for i in range(0, trials):
    x = r.uniform(-1.00000, 1.00000)
    y = r.uniform(-1.00000, 1.00000)

    # Shouldn't we do a sqrt over this? 
    position_on_board = x * x + y * y

    if position_on_board < 1.0:
      hit += 1

  return float(hit) * 4.0 / float(trials)

pi_est = montecarlo_pi(5_000_000)
print(pi_est)
