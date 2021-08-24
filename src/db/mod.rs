use crate::alias::{MobcCon, MobcPool};
use crate::errors::MobcError::*;
use anyhow::Result;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mobc::Pool;
use mobc_redis::redis::{AsyncCommands, FromRedisValue, ToRedisArgs};
use mobc_redis::{redis, RedisConnectionManager};
use std::time::Duration;

pub const CACHE_POOL_MAX_OPEN: u64 = 16;
pub const CACHE_POOL_MAX_IDLE: u64 = 8;
pub const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
pub const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

lazy_static! {
    pub static ref POOL: AsyncOnce<MobcPool> = AsyncOnce::new(async {
        let url = std::env::var("REDIS_URL").expect("REDIS_URL not found");
        connect(&url).await.expect("Error connecting to redis")
    });
}

pub async fn connect(url: &str) -> Result<MobcPool> {
    let client = redis::Client::open(url).map_err(RedisClientError)?;
    let manager = RedisConnectionManager::new(client);
    Ok(Pool::builder()
        .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
        .max_open(CACHE_POOL_MAX_OPEN)
        .max_idle(CACHE_POOL_MAX_IDLE)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
        .build(manager))
}

async fn get_con(pool: &MobcPool) -> Result<MobcCon> {
    pool.get().await.map_err(|e| {
        eprintln!("error connecting to redis: {}", e);
        RedisPoolError(e).into()
    })
}

pub async fn set_data<T>(pool: &MobcPool, key: &str, value: T, ttl_seconds: usize) -> Result<()>
where
    T: ToRedisArgs + Send + Sync,
{
    let mut con = get_con(&pool).await?;
    con.set(key, value).await.map_err(RedisCMDError)?;
    if ttl_seconds > 0 {
        con.expire(key, ttl_seconds).await.map_err(RedisCMDError)?;
    }
    Ok(())
}

pub async fn get_data<T>(pool: &MobcPool, key: &str) -> Result<T>
where
    T: mobc_redis::redis::FromRedisValue,
{
    let mut con = get_con(&pool).await?;
    let value = con.get(key).await.map_err(RedisCMDError)?;
    FromRedisValue::from_redis_value(&value).map_err(|e| RedisTypeError(e).into())
}
