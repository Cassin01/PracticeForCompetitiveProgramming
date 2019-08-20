import matplotlib.pyplot as plt
import networkx as nx

options = {
    'node_color': 'black',
    'node_size': 100,
    'width': 3,
}

G = nx.DiGraph()
G.add_nodes_from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])

G.add_edges_from([(1, 3), (1, 3), (1, 3), (1, 3)])
G.add_edges_from([(1, 2), (4, 2), (4, 5), (6, 7)])
G.add_edges_from([(6, 9), (9, 5), (7, 3), (4, 6)])
G.add_edges_from([(6, 8), (8, 4), (2, 10), (10, 6)])
G.add_edges_from([(6, 2)])
G.add_edges_from([(5, 3), (7, 3), (11, 3), (5, 11)])

nx.draw_networkx(G)
#nx.draw_shell(G, nlist=[range(5,10), range(5)], **options)
plt.show()
