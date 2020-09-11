""" naive python to have a simple comparison """
import time
import pandas as pd

t0 = time.time()
df = pd.read_csv("uspop.csv")
t1 = time.time()

print((t1 - t0) * 1000000000)
