#!/usr/bin/env python3

import sys

def paths(graph, node, path, twice):
    path.append(node)
    for n in graph[node]:
        if n == "end":
            yield 1
        elif n.isupper() or n not in path:
            yield from paths(graph, n, path, twice)
        elif twice and n != "start":
            yield from paths(graph, n, path, False)
    path.pop()

def main():
    graph = {}
    for line in sys.stdin:
        m, n = line.strip().split("-")
        graph.setdefault(m, []).append(n)
        graph.setdefault(n, []).append(m)
    print(sum(paths(graph, "start", [], False)))
    print(sum(paths(graph, "start", [], True)))

if __name__ == "__main__":
    main()
