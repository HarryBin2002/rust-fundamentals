/*
Vector
A Vector is a resizable array. It stores values in contiguous memory blocks. The predefined structure Vec can be used to create vectors. Some important features of a Vector are −

A Vector can grow or shrink at runtime.

A Vector is a homogeneous collection.

A Vector stores data as sequence of elements in a particular order. Every element in a Vector is assigned a unique index number. The index starts from 0 and goes up to n-1 where, n is the size of the collection. For example, in a collection of 5 elements, the first element will be at index 0 and the last element will be at index 4.

A Vector will only append values to (or near) the end. In other words, a Vector can be used to implement a stack.

Memory for a Vector is allocated in the heap.

Sr.No	Method	Signature & Description

1	new()	
pub fn new()->Vect
Constructs a new, empty Vec. The vector will not allocate until elements are pushed onto it.

2	push()	
pub fn push(&mut self, value: T)
Appends an element to the back of a collection.

3	remove()	
pub fn remove(&mut self, index: usize) -> T
Removes and returns the element at position index within the vector, shifting all elements after it to the left.

4	contains()	
pub fn contains(&self, x: &T) -> bool
Returns true if the slice contains an element with the given value.

5	len()	
pub fn len(&self) -> usize
Returns the number of elements in the vector, also referred to as its 'length'.
*/


/*
HashMap
A map is a collection of key-value pairs (called entries). No two entries in a map can have the same key.
The HashMap structure is defined in the std::collections module.

Sr.No	Method	Signature & Description
1	insert()	
pub fn insert(&mut self, k: K, v: V) -> Option

Inserts a key/value pair, if no key then None is returned. After update, old value is returned.

2	len()	
pub fn len(&self) -> usize

Returns the number of elements in the map.

3	get()	
pub fn get<Q: ?Sized>(&lself, k: &Q) -> Option<&V> where K:Borrow Q:Hash+ Eq

Returns a reference to the value corresponding to the key.

4	iter()	
pub fn iter(&self) -> Iter<K, V>

An iterator visiting all key-value pairs in arbitrary order. The iterator element type is (&'a K, &'a V).

5	contains_key	
pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool

Returns true if the map contains a value for the specified key.

6	remove()	
pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>

Removes a key from the map, returning the stored key and value if the key was previously in the map.
*/


/*
HashSet
HashSet is a set of unique values of type T
use std::collections::HashSet;

Syntax: Creating a HashSet
let mut hash_set_name = HashSet::new();

Sr.No	Method	Signature & Description
1	insert()	
pub fn insert(&mut self, value: T) -> bool

Adds a value to the set. If the set did not have this value present, true is returned else false.

2	len()	
pub fn len(&self) -> usize

Returns the number of elements in the set.

3	get()	
pub fn get<Q:?Sized>(&self, value: &Q) -> Option<&T> where T: Borrow,Q: Hash + Eq,

Returns a reference to the value in the set, if any that is equal to the given value.

4	iter()	
pub fn iter(&self) -> Iter

Returns an iterator visiting all elements in arbitrary order. The iterator element type is &'a T.

5	contains_key	
pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool

Returns true if the set contains a value.

6	remove()	
pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool

Removes a value from the set. Returns true if the value was present in the set.
*/


// use
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // Syntax - Creating a Vector
    // let mut instance_name = Vec::new();
    let mut v: Vec<u32> = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);
 
    println!("size of vector is :{}", v.len());
    println!("{:?}", v);

    // Or
    // let vector_name = vec![val1,val2,val3]
    let mut v1: Vec<u32> = vec![1, 2, 3];
    println!("{:?}", v1);
    v1.push(4);
    println!("{:?}", v1);



    // For example HashMap
    let mut hash_map1: HashMap<u32, String> = HashMap::new();

    hash_map1.insert(1, String::from("z"));
    hash_map1.insert(2, String::from("a"));
    hash_map1.insert(5, String::from("c"));
    hash_map1.insert(7, String::from("a"));
    hash_map1.insert(3, String::from("d"));
    hash_map1.insert(6, String::from("g"));
    hash_map1.insert(4, String::from("e"));
    hash_map1.insert(8, String::from("f"));
    hash_map1.insert(9, String::from("i"));

    println!("{:?}", hash_map1);


    // For example HashSet
    let mut programing_languages: HashSet<String> = HashSet::new();

    programing_languages.insert("Java".to_string());
    programing_languages.insert("Solidity".to_string());
    programing_languages.insert("Rust".to_string());
    programing_languages.insert("JavaScript".to_string());
    programing_languages.insert("Python".to_string());
    // insert duplicate elements => hashset does not insert.
    programing_languages.insert("Solidity".to_string());
    programing_languages.insert("Rust".to_string());

    println!("{:?}", programing_languages);

}
