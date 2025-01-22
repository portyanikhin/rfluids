use crate::error::CoolPropError;
use crate::native::AbstractState;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

trait Remember<S, K> {
    type Error;

    fn remember(&mut self, src: S, key: K) -> Result<f64, Self::Error>;
}

impl<K> Remember<&AbstractState, K> for HashMap<K, f64>
where
    K: Into<u8> + Copy + Eq + Hash,
{
    type Error = CoolPropError;

    fn remember(&mut self, src: &AbstractState, key: K) -> Result<f64, CoolPropError> {
        Ok(match self.entry(key) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => *entry.insert(src.keyed_output(key)?),
        })
    }
}
