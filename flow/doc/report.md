# Flow

## Results

Our implementation successfully computes a flow of 163 and the minimum cut on the input file, confirming the analysis of the American enemy.

We have analyzed the possibilities of decreasing the capacities near Minsk.
Our analysis is summaries in the following table

| case | 4W-48 | 4w-49 | effect on flow |
| ---- | ----- | ----- | -------------- |
| 1    | 30    | 20    | same           |
| 2    | 20    | 30    | same           |
| 3    | 20    | 20    | same           |
| 4    | 10    | 10    | -20            |
| 5    | 1     | 1     | -38            |

Lets just take one of the cases as an example.

We can see that the bottleneck for case 5 is

```
52 -> 51 with capacity: 2
7 -> 50 with capacity: 34
7 -> 51 with capacity: 28
5 -> 49 with capacity: 12
46 -> 47 with capacity: 20
4W -> 48 with capacity: 1
4W -> 49 with capacity: 1
R -> b with capacity: 8
b -> H with capacity: 19
Max flow: 125
```

## Implementation details

We use a implementation called Edmonds-Karp Algorithm. The Edmonds–Karp algorithm is an implementation of the Ford–Fulkerson with breath first search, where BFS always picks a path with minimum number of edges.
When BFS is used in together with Ford-Fulkerson Algorithm, the worst case time complexity can be reduced to `O(VE^2)`

The implementation is strongly inspired by this page: https://www.geeksforgeeks.org/ford-fulkerson-algorithm-for-maximum-flow-problem/

All edges are bidirectional. When a edge has an infinite capacity we set the capacity `i32::MAX / 2` representing a really high capacity, (we add the `/2`, because otherwise we risk the number to overflow when running the algorithm).
