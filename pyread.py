import time
import numpy as np

t0 = time.time()

with open("uspop.csv") as f:
    next(f)
    list_lat = []
    list_long = []
    for line in f:
        line = line.rstrip()
        l_split = line.split(",")
        l0 = l_split[0]
        l3 = l_split[3]
        l4 = l_split[4]
        list_lat.append(l3)
        list_long.append(l4)

np_lat = np.array(list_lat, dtype=float)
np_long = np.array(list_long, dtype=float)

t1 = time.time()

print((t1 - t0) * 1000000)
