use mobc::{Connection, Pool};
use mobc_redis::RedisConnectionManager;
use teloxide::{adaptors::AutoSend, prelude::*, Bot};

pub type Cx = UpdateWithCx<AutoSend<Bot>, Message>;
pub type MobcPool = Pool<RedisConnectionManager>;
pub type MobcCon = Connection<RedisConnectionManager>;
