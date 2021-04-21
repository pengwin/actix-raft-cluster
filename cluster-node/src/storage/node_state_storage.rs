use sled::{Config, Db, Error};

use {
    byteorder::BigEndian,
    zerocopy::{
        ByteSlice,
        byteorder::U64, AsBytes, FromBytes, LayoutVerified, Unaligned,
    },
};
use std::path::Path;


// We use `BigEndian` for key types because
// they preserve lexicographic ordering,
// which is nice if we ever want to iterate
// over our items in order. We use the
// `U64` type from zerocopy because it
// does not have alignment requirements.
// sled does not guarantee any particular
// value alignment as of now.
#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
struct NodeKey {
    id: U64<BigEndian>,
}

impl NodeKey {
    fn new(id: u64) -> NodeKey {
        NodeKey{
            id: U64::new(id),
        }
    }
}

#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
struct NodeStateValue {
    last_applied_index: U64<BigEndian>,
}

impl NodeStateValue {
    fn new(last_applied_index: u64) -> NodeStateValue {
        NodeStateValue{
            last_applied_index: U64::new(last_applied_index),
        }
    }

    pub fn parse<B: ByteSlice>(bytes: B) -> Option<LayoutVerified<B, NodeStateValue>> {
        let header = LayoutVerified::<B, NodeStateValue>::new(bytes)?;
        Some(header)
    }
}

pub struct NodeInfoStorage {
    db: Db
}

impl NodeInfoStorage {
    
    pub fn new<P: AsRef<Path>>(p: P) -> Result<NodeInfoStorage, Error> {
        let config = Config::default()
            .path(p)
            .cache_capacity(10_000_000_000)
            .flush_every_ms(Some(100));
        
        let db = config.open()?;
        
        Ok(NodeInfoStorage{
            db,
        })
    }
    
    pub fn store_apllied_index(&self, id: u64, last_applied_index: u64) -> Result<(), Error> {
        let key = NodeKey::new(id);
        let value = NodeStateValue::new(last_applied_index);
        self.db.insert(key.as_bytes(), value.as_bytes())?;
        Ok(())
    }

    pub fn load_apllied_index(&self, id: u64) -> Result<Option<u64>, Error> {
        let key = NodeKey::new(id);
        let res = self.db.get(key.as_bytes())?;

        return match res {
            Some(r) => {
                let s = NodeStateValue::parse(r.as_bytes());
                return match s {
                    Some(q) => Ok(Some(q.last_applied_index.get())),
                    None => Ok(None),
                };
            },
            None => {
                Ok(None)
            }
        };
    }
}
