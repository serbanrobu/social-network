use social_network::{find_shortest_chain, new_in_memory_pool, UserId};
use sqlx::Result;

#[async_std::test]
async fn test_find_shortest_chain_in_a_basic_graph() -> Result<()> {
    let pool = new_in_memory_pool().await?;

    sqlx::query!("INSERT INTO users VALUES (1), (2), (3), (4), (5), (6), (7), (8), (9)")
        .execute(&pool)
        .await?;

    sqlx::query!(
        "INSERT INTO friendships VALUES (1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (4, 7), (5, 8), (6, 9)"
    )
    .execute(&pool)
    .await?;

    // This test case is relevant because it shows that the algorithm is able to find the shortest
    // path when it's not the direct path.
    let expected_chain = vec![1, 2, 5, 8];
    let result = find_shortest_chain(&pool, 1, 8).await?;
    assert_eq!(expected_chain, result);

    // This test case is relevant because it shows that the algorithm returns an empty path when
    // the starting or ending vertex doesn't exist in the graph.
    let expected_chain: Vec<UserId> = vec![];
    let result = find_shortest_chain(&pool, 10, 8).await?;
    assert_eq!(expected_chain, result);

    // Tests the case where the starting and ending vertices are the same, and it should return the
    // vertex itself, in this case [1].
    let expected_chain = vec![1];
    let result = find_shortest_chain(&pool, 1, 1).await?;
    assert_eq!(expected_chain, result);

    // This test case is relevant because it shows that the algorithm is able to find the shortest
    // path when it's not the direct path
    let expected_chain = vec![1, 3, 6, 9];
    let result = find_shortest_chain(&pool, 1, 9).await?;
    assert_eq!(expected_chain, result);

    Ok(())
}

// Tests the case where the graph is disconnected, in this case, since there is no chain between
// vertices 1 and 8, the algorithm should return an empty chain [].
#[async_std::test]
async fn test_find_shortest_chain_in_a_disconnected_graph() -> Result<()> {
    let pool = new_in_memory_pool().await?;

    sqlx::query!("INSERT INTO users VALUES (1), (2), (3), (4), (5)")
        .execute(&pool)
        .await?;

    sqlx::query!("INSERT INTO friendships VALUES (1, 2), (1, 3), (2, 4), (2, 5)")
        .execute(&pool)
        .await?;

    let expected_chain: Vec<UserId> = vec![];
    let result = find_shortest_chain(&pool, 1, 8).await?;
    assert_eq!(expected_chain, result);

    Ok(())
}

// Tests the case where the graph is large, in this case, the graph has 1000 vertices, and the
// expected chain is [1, 2, 3, ..., 1000], this test case is useful to check the performance of the
// algorithm when the graph is large.
#[async_std::test]
async fn test_find_shortest_chain_in_a_large_graph() -> Result<()> {
    let pool = new_in_memory_pool().await?;

    for i in 1..1_001 {
        sqlx::query!("INSERT INTO users VALUES (?)", i)
            .execute(&pool)
            .await?;

        if i > 1 {
            let j = i - 1;

            sqlx::query!("INSERT INTO friendships VALUES (?, ?)", j, i)
                .execute(&pool)
                .await?;
        }
    }

    let expected_chain = (1..1_001).collect::<Vec<_>>();
    let result = find_shortest_chain(&pool, 1, 1_000).await?;
    assert_eq!(expected_chain, result);

    Ok(())
}

// Tests the case where the graph is dense. In this case, all vertices are connected to all other
// vertices, the expected chain is [1, 100], this test case is useful to check the performance of
// the algorithm when the graph is dense.
#[async_std::test]
async fn test_find_shortest_chain_in_a_dense_graph() -> Result<()> {
    let pool = new_in_memory_pool().await?;

    for i in 1..101 {
        sqlx::query!("INSERT INTO users VALUES (?)", i)
            .execute(&pool)
            .await?;

        for j in 1..i {
            sqlx::query!("INSERT INTO friendships VALUES (?, ?)", j, i)
                .execute(&pool)
                .await?;
        }
    }

    let expected_chain = vec![1, 100];
    let result = find_shortest_chain(&pool, 1, 100).await?;
    assert_eq!(expected_chain, result);

    Ok(())
}

// Tests the case where the graph has cycles. In this case, there is a cycle between all vertices,
// the expected chain is [1, 100], this test case is useful to check the algorithm handle loops and
// cycles.
#[async_std::test]
async fn test_find_shortest_chain_in_a_cyclic_graph() -> Result<()> {
    let pool = new_in_memory_pool().await?;

    for i in 1..101 {
        sqlx::query!("INSERT INTO users VALUES (?)", i)
            .execute(&pool)
            .await?;

        if i > 1 {
            let j = i - 1;

            sqlx::query!("INSERT INTO friendships VALUES (?, ?)", j, i)
                .execute(&pool)
                .await?;
        }
    }

    sqlx::query!("INSERT INTO friendships VALUES (?, ?)", 1, 100)
        .execute(&pool)
        .await?;

    let expected_chain = vec![1, 100];
    let result = find_shortest_chain(&pool, 1, 100).await?;
    assert_eq!(expected_chain, result);

    Ok(())
}
