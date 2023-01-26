#![feature(test)]

extern crate test;

use async_std::task;
use social_network::{find_shortest_chain, new_in_memory_pool};
use sqlx::Result;
use test::Bencher;

#[bench]
fn bench_find_shortest_chain_in_a_small_graph(b: &mut Bencher) -> Result<()> {
    let pool = task::block_on(async {
        let pool = new_in_memory_pool().await?;

        sqlx::query!("INSERT INTO users VALUES (1), (2), (3), (4), (5), (6), (7), (8), (9)")
            .execute(&pool)
            .await?;

        sqlx::query!(
            "INSERT INTO friendships VALUES (1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (4, 7), (5, 8), (6, 9)"
        )
        .execute(&pool)
        .await?;

        Ok(pool) as Result<_>
    })?;

    b.iter(|| task::block_on(find_shortest_chain(&pool, 1, 8)));

    Ok(())
}

#[bench]
fn bench_find_shortest_chain_in_a_medium_graph(b: &mut Bencher) -> Result<()> {
    let pool = task::block_on(async {
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

        Ok(pool) as Result<_>
    })?;

    b.iter(|| task::block_on(find_shortest_chain(&pool, 1, 100)));

    Ok(())
}

#[ignore]
#[bench]
fn bench_find_shortest_chain_in_a_large_graph(b: &mut Bencher) -> Result<()> {
    let pool = task::block_on(async {
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

        Ok(pool) as Result<_>
    })?;

    println!("test");

    b.iter(|| task::block_on(find_shortest_chain(&pool, 1, 1_000)));

    Ok(())
}

#[bench]
fn bench_find_shortest_chain_in_a_dense_graph(b: &mut Bencher) -> Result<()> {
    let pool = task::block_on(async {
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

        Ok(pool) as Result<_>
    })?;

    b.iter(|| task::block_on(find_shortest_chain(&pool, 1, 100)));

    Ok(())
}

#[bench]
fn bench_find_shortest_chain_in_a_cyclic_graph(b: &mut Bencher) -> Result<()> {
    let pool = task::block_on(async {
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

        Ok(pool) as Result<_>
    })?;

    b.iter(|| task::block_on(find_shortest_chain(&pool, 1, 100)));

    Ok(())
}
