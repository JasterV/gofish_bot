use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum MobcError {
//     #[error("could not get redis connection from pool : {0}")]
//     RedisPoolError(mobc::Error<mobc_redis::redis::RedisError>),
//     #[error("error parsing string from redis result: {0}")]
//     RedisTypeError(mobc_redis::redis::RedisError),
//     #[error("error executing redis command: {0}")]
//     RedisCMDError(mobc_redis::redis::RedisError),
//     #[error("error creating Redis client: {0}")]
//     RedisClientError(mobc_redis::redis::RedisError),
// }
