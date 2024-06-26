import dataclasses


@dataclasses.dataclass
class Edge:
    to: int
    """残余グラフ上での辺の行き先"""
    cap: int
    """残余グラフ上での辺の容量"""
    rev: int
    """辺u->graph[u][i].toの逆辺G[u][i].to->uがG[G[u][i].to]の何番目に存在するか"""


class MaximumFlow:
    """Ford-Fulkerson法による最大流問題を解く"""

    def __init__(self, n: int) -> None:
        self.size = n
        self.graph = [[] for _ in range(n)]
        self.used = [False] * n

    def add_edge(self, a: int, b: int, c: int) -> None:
        """頂点aからbに向かう上限c リットル/秒の辺を追加. 残余グラフを構築する."""
        # a-bがaの何番目の辺か、bの何番目の辺かを追加
        current_graph_a_ln = len(self.graph[a])
        current_graph_b_ln = len(self.graph[b])
        self.graph[a].append(Edge(to=b, cap=c, rev=current_graph_b_ln))
        self.graph[b].append(Edge(to=a, cap=0, rev=current_graph_a_ln))

    def dfs(self, pos: int, goal: int, flow: int) -> int:
        """深さ優先探索（Fはスタートからposに到達する過程での"残余グラフの辺の容量"の最小値）
        返り値は流したフローの量（流せない場合は0を返す）
        """
        if pos == goal:
            return flow

        self.used[pos] = True

        # 探索する
        for i in range(len(self.graph[pos])):
            # 容量0の辺は使えない
            if self.graph[pos][i].cap == 0:
                continue

            # すでに訪問した頂点に行っても意味がない
            if self.used[self.graph[pos][i].to]:
                continue

            # 目的地までのパスを探す
            flow_i = self.dfs(
                pos=self.graph[pos][i].to,  # いまみる頂点
                goal=goal,  # 目的地
                flow=min(flow, self.graph[pos][i].cap),  # 残余グラフの辺の容量の最小値
            )

            # フローを流せる場合、残余グラフの容量をflowだけ増減させる
            if flow_i > 0:
                self.graph[pos][i].cap -= flow_i
                self.graph[self.graph[pos][i].to][self.graph[pos][i].rev].cap += flow_i
                return flow_i

        # すべての辺を探索しても見つからない
        return 0

    def max_flow(self, s: int, t: int) -> int:
        total_flow = 0
        while True:
            for i in range(self.size):
                self.used[i] = False
            flow = self.dfs(s, t, 10**9)
            if flow == 0:
                break
            total_flow += flow
        return total_flow


# i番目の青色頂点: i
# j番目の赤色頂点: n + j
# スタート地点s : 2 * n + 1

n = int(input())

mf = MaximumFlow(2 * n + 2)

c = [[] for _ in range(n)]
for i in range(n):
    c[i] = list(map(int, input().split()))


for i in range(1, n + 1):
    mf.add_edge(2 * n + 1, i, 1)  # s -> 青色の辺
    mf.add_edge(n + i, 2 * n + 2, 1)  # 赤色の辺 -> t

print(mf.max_flow(s=2 * n + 1, t=2 * n + 2))
