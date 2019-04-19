import numpy as np
import matplotlib.pyplot as plt

x = np.arange(0, 6, 0.1)

a = 2
y = a * x

plt.legend(loc='upper left', fontsize=10, title='Function')
plt.title('y = a * x | a = 2', loc='left', fontsize=10)
plt.plot(x, y, label = 'y = ax | a = 2')

plt.show()
plt.savefig("graph.png")