use futures::TryStreamExt;
use sqlx::Result;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::collections::{HashMap, HashSet, VecDeque};

pub type UserId = i64;

pub type ChainOfFriends = Vec<UserId>;

/// A bidirectional search algorithm for finding the shortest chain of friends in a social network
/// graph
///
/// The time complexity for the algorithm is O(b^(d/2)), where b is the branching factor of the
/// graph (i.e. the average number of neighbors per node) and d is the shortest chain distance
/// between the start and end nodes.
///
/// The bidirectional search algorithm is faster than the normal breadth-first search algorithm
/// when the branching factor is large, and the shortest chain distance is small.
pub async fn find_shortest_chain(
    pool: &SqlitePool,
    start: UserId,
    end: UserId,
) -> Result<ChainOfFriends> {
    if start == end {
        return Ok(vec![start]);
    }

    let mut parents_start = HashMap::new();
    let mut parents_end = HashMap::new();
    let mut visited_start = HashSet::new();
    let mut visited_end = HashSet::new();
    let mut queue_start = VecDeque::from([start]);
    let mut queue_end = VecDeque::from([end]);

    visited_start.insert(start);

    while let (Some(user_start), Some(user_end)) = (queue_start.pop_front(), queue_end.pop_front())
    {
        let mut middle = breadth_first_search(
            pool,
            user_start,
            &mut parents_start,
            &mut visited_start,
            &visited_end,
            &mut queue_start,
        )
        .await?;

        if middle.is_none() {
            middle = breadth_first_search(
                pool,
                user_end,
                &mut parents_end,
                &mut visited_end,
                &visited_start,
                &mut queue_end,
            )
            .await?;
        }

        if let Some(middle) = middle {
            return Ok(construct_chain_of_friends(
                parents_start,
                parents_end,
                start,
                middle,
                end,
            ));
        }
    }

    Ok(vec![])
}

fn construct_chain_of_friends(
    parents_start: HashMap<UserId, UserId>,
    parents_end: HashMap<UserId, UserId>,
    start: UserId,
    middle: UserId,
    end: UserId,
) -> ChainOfFriends {
    let mut chain = vec![middle];
    let mut current_user = middle;

    while current_user != start {
        current_user = parents_start[&current_user];
        chain.push(current_user);
    }

    chain.reverse();

    current_user = middle;

    while current_user != end {
        current_user = parents_end[&current_user];
        chain.push(current_user);
    }

    chain
}

pub async fn breadth_first_search(
    pool: &SqlitePool,
    user: UserId,
    parents: &mut HashMap<UserId, UserId>,
    visited: &mut HashSet<UserId>,
    other_visited: &HashSet<UserId>,
    queue: &mut VecDeque<UserId>,
) -> Result<Option<UserId>> {
    let mut friends =
        sqlx::query_scalar!("SELECT friend_id FROM user_friends WHERE user_id = ?", user)
            .fetch(pool);

    while let Some(friend) = friends.try_next().await? {
        if visited.contains(&friend) {
            continue;
        }

        parents.insert(friend, user);
        visited.insert(friend);
        queue.push_back(friend);

        if other_visited.contains(&friend) {
            return Ok(Some(friend));
        }
    }

    Ok(None)
}

pub async fn new_in_memory_pool() -> Result<Pool<Sqlite>> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
