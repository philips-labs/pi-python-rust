import random as r
#import math as m

def montecarlo_pi( trials):
  count = 0
  r.seed()

  for i in range(0, trials):
    x = r.uniform(-1.00000, 1.00000)
    y = r.uniform(-1.00000, 1.00000)
    p = x * x + y * y
    
    #if m.sqrt(p) < 1.0:
    if p < 1.0:
      count += 1

  return float(count) * 4.0 / float(trials)

pi_est = montecarlo_pi(5_000_000)
print(pi_est)
