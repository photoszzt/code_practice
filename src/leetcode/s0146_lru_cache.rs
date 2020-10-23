/**
 * [146] LRU Cache
 *
 * Design a data structure that follows the constraints of a <a href="https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU" target="_blank">Least Recently Used (LRU) cache</a>.
 * Implement the LRUCache class:
 *
 * 	LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
 * 	int get(int key) Return the value of the key if the key exists, otherwise return -1.
 * 	void put(int key, int value) Update the value of the key if the key exists. Otherwise, add the key-value pair to the cache. If the number of keys exceeds the capacity from this operation, evict the least recently used key.
 *
 * Follow up:<br />
 * Could you do get and put in O(1) time complexity?
 *
 * Example 1:
 *
 * Input
 * ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
 * [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
 * Output
 * [null, null, null, 1, null, -1, null, -1, 3, 4]
 * Explanation
 * LRUCache lRUCache = new LRUCache(2);
 * lRUCache.put(1, 1); // cache is {1=1}
 * lRUCache.put(2, 2); // cache is {1=1, 2=2}
 * lRUCache.get(1);    // return 1
 * lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
 * lRUCache.get(2);    // returns -1 (not found)
 * lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
 * lRUCache.get(1);    // return -1 (not found)
 * lRUCache.get(3);    // return 3
 * lRUCache.get(4);    // return 4
 *
 *
 * Constraints:
 *
 * 	1 <= capacity <= 3000
 * 	0 <= key <= 3000
 * 	0 <= value <= 10^4
 * 	At most 3 * 10^4 calls will be made to get and put.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/lru-cache/
// discuss: https://leetcode.com/problems/lru-cache/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct LRUCache {
    map: std::collections::HashMap<i32, i32>,
    list: std::collections::VecDeque<(i32, u32)>,
    access_time: std::collections::HashMap<i32, u32>,
    capacity: i32,
    id: u32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            map: std::collections::HashMap::new(),
            list: std::collections::VecDeque::new(),
            access_time: std::collections::HashMap::new(),
            capacity,
            id: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        self.access(key.clone());
        self.map.get(&key).unwrap_or(&-1).clone()
    }

    fn put(&mut self, key: i32, value: i32) {
        self.map.insert(key, value);
        self.access(key);
        while self.map.len() > self.capacity as usize {
            if let Some(key) = self.remove_lru() {
                self.map.remove(&key);
            }
        }
    }

    fn access(&mut self, key: i32) {
        self.id += 1;
        self.access_time.insert(key, self.id);
        self.list.push_back((key, self.id));
    }

    fn remove_lru(&mut self) -> Option<i32> {
        loop {
            if let Some((key, t)) = self.list.pop_front() {
                if t == self.access_time.get(&key).unwrap().clone() {
                    return Some(key.clone());
                }
            } else {
                return None;
            }
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_146() {
        println!("init cache");
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        println!("return 1");
        assert_eq!(lru_cache.get(1), 1); // returns 1
        println!("evict key 2");
        lru_cache.put(3, 3); // evicts key 2
        println!("return -1");
        assert_eq!(lru_cache.get(2), -1); // returns -1 (not found)
        lru_cache.put(4, 4); // evicts key 1
        assert_eq!(lru_cache.get(1), -1); // returns -1 (not found)
        assert_eq!(lru_cache.get(3), 3); // returns 3
        assert_eq!(lru_cache.get(4), 4); // returns 4
    }
}
