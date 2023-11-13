import numpy as np
import matplotlib.pyplot as plt

with open('code-files/supporting-code/rgb_values.txt', 'r') as f:
    rgb_values_255 = [eval(line.strip()) for line in f]

rgb_array = np.array(rgb_values_255)[np.newaxis, :, :]

plt.imshow(rgb_array.astype(np.uint8), aspect='auto')
plt.axis('off')
plt.show()
