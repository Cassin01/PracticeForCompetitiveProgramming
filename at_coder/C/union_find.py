class UnionFind:
    def __init__(self, n):
        #親要素のノード番号を格納。par[x] == xの時そのノードは根
        self.par = [i for i in range(n+1)]
    def find(self, x):
        # 根ならその番号を返す
        if self.par[x] == x:
            return x