use anyhow::Result;
use zzingo_db::{
    buffer::{BufferPool, BufferPoolManager},
    disk::DiskManager,
};

fn main() -> Result<()> {
    let disk = DiskManager::open("btree.btr")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    Ok(())
}
