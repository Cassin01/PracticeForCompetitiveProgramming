import matplotlib.pyplot as plt
import networkx as nx

G = nx.DiGraph()
G.add_nodes_from([1, 2, 3, 4, 5, 6, 7])
G.add_edges_from([
    (1, 2),
    (1, 3),
    (4, 2),
    (4, 3),
    (4, 5),
    (4, 6),
    (7, 5),
    (7, 6),
    (4, 7)])

nx.draw_networkx(G)
plt.show()
