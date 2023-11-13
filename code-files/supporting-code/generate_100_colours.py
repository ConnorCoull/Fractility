import numpy as np
import matplotlib.pyplot as plt

cmap = plt.get_cmap('gist_rainbow')

values = np.linspace(0, 1, 100)

rgb_values = [cmap(value)[:3] for value in values]

rgb_values_255 = [(int(r*255), int(g*255), int(b*255)) for r, g, b in rgb_values]

with open('rgb_values.txt', 'w') as f:
    for rgb in rgb_values_255:
        f.write(str(rgb) + '\n')