""" naive python to have a simple comparison """
import time
import numpy as np

t0 = time.time()
df = np.loadtxt("uspop.csv", delimiter=',', skiprows=1, usecols=(2, 3, 4))
t1 = time.time()

print((t1 - t0) * 1000000000)
