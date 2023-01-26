# Social Network

## Q&A

1. How did you represent the social network?

   I chose to represent the social network as a graph, where the vertices are the
   users and the edges are the friendship relationships.

2. Why did you choose this representation?

   The reason why I chose this representation is because there is good theory of
   graphs and a multitude of algorithms for calculating the shortest path between
   two nodes.

3. What algorithm did you use to compute the shortest chain of friends?

   I used a bidirectional search algorithm to compute the shortest chain of friends
   between two users in a social network graph.

   The algorithm starts by visiting the start node, and then visits all of its neighbors,
   and then visits all the neighbors of those neighbors, and so on. The same
   process is done for the end node in the backward search.

   The algorithm stops when it reaches the middle point (i.e. the point where
   the two searches meet) which is the shortest path between the start and end nodes

4. What alternatives did you consider?

   There are several alternatives to bidirectional search that can be used to find
   the shortest path between two vertices in a graph. Some of them are:

   - Dijkstra's algorithm: This algorithm is a generalization of the breadth-first
     search algorithm that can be used to find the shortest path in a graph with
     positive edge weights. It uses a priority queue to explore the vertices in
     increasing order of their distance from the source vertex.

   - A\* algorithm: This algorithm is an extension of Dijkstra's algorithm that
     uses an heuristic function to estimate the distance from a vertex to the target
     vertex. The heuristic function should be admissible, meaning that it should
     never overestimate the actual distance, and consistent, meaning that it satisfies
     the triangle inequality.

   - Bellman-Ford algorithm: This algorithm is used to find the shortest path in
     a graph with negative edge weights. It relaxes the edges of the graph repeatedly
     until no more improvements can be made.

   - Floyd-Warshall algorithm: This algorithm is an all-pairs shortest path algorithm
     that can be used to find the shortest path between all pairs of vertices in
     a graph.

5. Why did you choose this algorithm over the alternatives?

   In the case of an undirected unweighted graph, the bidirectional search algorithm
   is efficient and simple to implement, so it was the best choice for this problem.
   Dijkstra's algorithm and A\* are both designed for graphs with edge weights,
   so they are not appropriate for this case. Bellman-Ford algorithm is also not
   necessary since it's designed for graphs with negative edge weights and Floyd-Warshall
   algorithm is an all-pairs algorithm and that's not necessary in this case.

   In case the graph has weighted edges, or if it's directed, other algorithms
   should be considered and the best choice would depend on the specific
   characteristics of the graph and the use case.

## Development

In [flake.nix](flake.nix) file, a [Nix
Shell](https://nixos.wiki/wiki/Development_environment_with_nix-shell) is
defined that includes everything you need for development. You can enter the
shell by running the following command:

```sh
nix develop
```

You might want to use [direnv](https://direnv.net/) to enter the shell automatically.

To run the benchmarks, execute the following command in the terminal:

```sh
cargo bench
```

or for the expensive, ignored benchmarks:

```sh
cargo bench -- --ignored
```

To test the library run the following command in the terminal:

```sh
cargo test
```

## Database

If you want to initialize the database, run the [scripts/init-db.sh](scripts/init-db.sh)
script.

You need an open connection to the database when you compile the code because all
the queries used in the code are statically checked by the [SQLx](https://github.com/launchbadge/sqlx)
library against the database schema. But thanks to the _offline mode_ feature of
SQLx, a database connection **is not required** because all the information needed
is cached to the [sqlx-data.json](sqlx-data.json) file, generated with the following
command:

```sh
cargo sqlx prepare -- --all-targets
```
