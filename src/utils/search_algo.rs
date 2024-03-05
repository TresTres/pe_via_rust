use std::collections::BTreeMap;



pub fn find_nearest_element_less_than<K, V>(btreemap: &BTreeMap<K, V>, key: &K) -> (K, V) 
    where K: Ord + Clone, V: Clone
{
    /*
        Find the nearest element less than the key in a BTreeMap.
    */
    if btreemap.is_empty() {
        panic!("BTreeMap is empty");
    }

    let mut result = btreemap.get_key_value(&key);
    if result.is_none() {
        let iter = btreemap.range(..key);
        result = iter.last();
        if result.is_none() {
            panic!("No element less than the key");
        }
    }
    let (k, v) = result.unwrap();
    return (k.clone(), v.clone());
}