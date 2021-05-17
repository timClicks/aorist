use siphasher::sip128::{Hasher128, SipHasher};
use std::collections::BTreeSet;
use std::hash::Hasher;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub trait AoristConcept {
    fn get_uuid(&self) -> Uuid;
    fn get_children_uuid(&self) -> Vec<Uuid>;
    fn get_tag(&self) -> Option<String>;

    fn get_uuid_from_children_uuid(&self) -> Uuid {
        let child_uuids = self.get_children_uuid();
        if child_uuids.len() > 0 {
            let uuids = child_uuids.into_iter().collect::<BTreeSet<Uuid>>();
            let mut hasher = SipHasher::new();
            for uuid in uuids {
                hasher.write(uuid.as_bytes());
            }
            let bytes: [u8; 16] = hasher.finish128().as_bytes();
            Uuid::from_bytes(bytes)
        } else {
            // TODO: this should just be created from the hash
            Uuid::new_v4()
        }
    }
    fn compute_uuids(&mut self);
}
pub trait AoristConceptChildren {
    fn get_child_concepts<'a, 'b>(&'a self) -> Vec<Concept<'b>>
    where
        'a: 'b;
}
include!(concat!(env!("OUT_DIR"), "/concepts.rs"));
