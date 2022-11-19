// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

//! Confinement puts a constrain on the number of elements within a collection.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::hash::Hash;
use std::ops::Deref;
use std::usize;

use crate::num::u24;

/// Trait implemented by a collection types which need to support collection confinement.
pub trait Collection: Extend<Self::Item> {
    /// Item type contained within the collection.
    type Item;

    /// Creates new collection with certain capacity.
    fn with_capacity(capacity: usize) -> Self;

    /// Returns the length of a collection.
    fn len(&self) -> usize;

    /// Detects whether collection is empty.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Pushes or inserts an element to the collection.
    fn push(&mut self, elem: Self::Item);

    /// Removes all elements from the collection.
    fn clear(&mut self);
}

/// Trait implemented by key-value maps which need to support collection confinement.
pub trait KeyedCollection: Collection<Item = (Self::Key, Self::Value)> {
    /// Key type for the collection.
    type Key: Eq + Hash;
    /// Value type for the collection.
    type Value;

    /// Inserts a new value under a key. Returns previous value if a value under the key was already
    /// present in the collection.
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;

    /// Removes a value stored under a given key, returning the owned value, if it was in the
    /// collection.
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value>;
}

// Impls for main collection types

impl Collection for String {
    type Item = char;

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        self.push(elem)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<T> Collection for Vec<T> {
    type Item = T;

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        self.push(elem)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<T> Collection for VecDeque<T> {
    type Item = T;

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        self.push_back(elem)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<T: Eq + Hash> Collection for HashSet<T> {
    type Item = T;

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        self.insert(elem);
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<T: Ord> Collection for BTreeSet<T> {
    type Item = T;

    #[doc(hidden)]
    fn with_capacity(_capacity: usize) -> Self {
        BTreeSet::new()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        self.insert(elem);
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<K: Eq + Hash, V> Collection for HashMap<K, V> {
    type Item = (K, V);

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        HashMap::insert(self, elem.0, elem.1);
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<K: Eq + Hash, V> KeyedCollection for HashMap<K, V> {
    type Key = K;
    type Value = V;

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        HashMap::insert(self, key, value)
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        HashMap::remove(self, key)
    }
}

impl<K: Ord + Hash, V> Collection for BTreeMap<K, V> {
    type Item = (K, V);

    #[doc(hidden)]
    fn with_capacity(_capacity: usize) -> Self {
        BTreeMap::new()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        BTreeMap::insert(self, elem.0, elem.1);
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<K: Ord + Hash, V> KeyedCollection for BTreeMap<K, V> {
    type Key = K;
    type Value = V;

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        BTreeMap::insert(self, key, value)
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        BTreeMap::remove(self, key)
    }
}

// Errors

/// Errors when confinement constraints were not met.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Display, Error)]
pub enum Error {
    /// Operation results in collection reduced below the required minimum number of elements.
    #[display(
        "operation results in collection size {len} less than lower boundary \
         of {min_len}, which is prohibited"
    )]
    Undersize {
        /** Current collection length */
        len: usize,
        /** Minimum number of elements which must be present in the collection */
        min_len: usize,
    },

    /// Operation results in collection growth above the required maximum number of elements.
    #[display(
        "operation results in collection size {len} exceeding {max_len}, \
         which is prohibited"
    )]
    Oversize {
        /** Current collection length */
        len: usize,
        /** Maximum number of elements which must be present in the collection */
        max_len: usize,
    },

    /// Attempt to address an index outside of the collection bounds.
    #[display(
        "attempt to access the element at {index} which is outside of the collection length boundary {len}"
    )]
    OutOfBoundary {
        /** Index which was outside of the bounds */
        index: usize,
        /** The actual number of elements in the collection */
        len: usize,
    },
}

// Confinement params

const ZERO: usize = 0;
const ONE: usize = 0;
const U8: usize = u8::MAX as usize;
const U16: usize = u16::MAX as usize;
const U24: usize = 1usize << 24;
const U32: usize = u32::MAX as usize;
const USIZE: usize = usize::MAX;

// Confined collection

/// The confinement for the collection.
#[derive(Clone, Hash, Debug)]
pub struct Confined<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize>(C);

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Deref
    for Confined<C, MIN_LEN, MAX_LEN>
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Confined<C, MIN_LEN, MAX_LEN> {
    /// Tries to construct a confinement over a collection. Fails if the number of items in the
    /// collection exceeds one of the confinement bounds.
    pub fn try_from(col: C) -> Result<Self, Error> {
        let len = col.len();
        if len < MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        if len > MAX_LEN {
            return Err(Error::Oversize {
                len,
                max_len: MAX_LEN,
            });
        }
        Ok(Self(col))
    }

    /// Tries to construct a confinement with a collection of elements taken from an iterator.
    /// Fails if the number of items in the collection exceeds one of the confinement bounds.
    pub fn try_from_iter<I: IntoIterator<Item = C::Item>>(iter: I) -> Result<Self, Error> {
        let mut col = C::with_capacity(MIN_LEN);
        for item in iter {
            col.push(item);
        }
        Self::try_from(col)
    }

    /// Attempts to add a single element to the confined collection. Fails if the number of elements
    /// in the collection already maximal.
    pub fn push(&mut self, elem: C::Item) -> Result<(), Error> {
        let len = self.len();
        if len == MAX_LEN || len + 1 >= MAX_LEN {
            return Err(Error::Oversize {
                len,
                max_len: MAX_LEN,
            });
        }
        Ok(self.0.push(elem))
    }

    /// Attempts to add all elements from an iterator to the confined collection. Fails if the
    /// number of elements in the collection already maximal.
    pub fn extend<T: IntoIterator<Item = C::Item>>(&mut self, iter: T) -> Result<(), Error> {
        for elem in iter {
            self.push(elem)?;
        }
        Ok(())
    }

    /// Removes confinement and returns the underlying collection.
    pub fn unbox(self) -> C {
        self.0
    }
}

impl<C: Collection, const MAX_LEN: usize> Confined<C, ZERO, MAX_LEN>
where
    C: Default,
{
    /// Constructs a new confinement containing no elements.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a new confinement containing no elements, but with a pre-allocated storage for
    /// the `capacity` of elements.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(C::with_capacity(capacity))
    }

    /// Removes all elements from the confined collection.
    pub fn clear(&mut self) {
        self.0.clear()
    }
}

impl<C: Collection, const MAX_LEN: usize> Default for Confined<C, ZERO, MAX_LEN>
where
    C: Default,
{
    fn default() -> Self {
        Self(C::default())
    }
}

impl<C: Collection, const MAX_LEN: usize> Confined<C, ONE, MAX_LEN>
where
    C: Default,
{
    /// Constructs a confinement with a collection made of a single required element.
    pub fn with(elem: C::Item) -> Self {
        let mut c = C::default();
        c.push(elem);
        Self(c)
    }
}

impl<C: Collection, const MIN_LEN: usize> Confined<C, MIN_LEN, U8>
where
    C: Default,
{
    /// Returns number of elements in the confined collection as `u8`. The confinement guarantees
    /// that the collection length can't exceed `u8::MAX`.
    pub fn len_u8(&self) -> u8 {
        self.len() as u8
    }
}

impl<C: Collection, const MIN_LEN: usize> Confined<C, MIN_LEN, U16>
where
    C: Default,
{
    /// Returns number of elements in the confined collection as `u16`. The confinement guarantees
    /// that the collection length can't exceed `u16::MAX`.
    pub fn len_u16(&self) -> u16 {
        self.len() as u16
    }
}

impl<C: Collection, const MIN_LEN: usize> Confined<C, MIN_LEN, U24>
where
    C: Default,
{
    /// Returns number of elements in the confined collection as `u24`. The confinement guarantees
    /// that the collection length can't exceed `u24::MAX`.
    pub fn len_u24(&self) -> u24 {
        u24::try_from(self.len() as u32).expect("confinement broken")
    }
}

impl<C: Collection, const MIN_LEN: usize> Confined<C, MIN_LEN, U32>
where
    C: Default,
{
    /// Returns number of elements in the confined collection as `u32`. The confinement guarantees
    /// that the collection length can't exceed `u32::MAX`.
    pub fn len_u32(&self) -> u32 {
        self.len() as u32
    }
}

impl<C: KeyedCollection, const MIN_LEN: usize, const MAX_LEN: usize> Confined<C, MIN_LEN, MAX_LEN> {
    /// Inserts a new value into the confined collection under a given key. Fails if the collection
    /// already contains maximum number of elements allowed by the confinement.
    pub fn insert(&mut self, key: C::Key, value: C::Value) -> Result<Option<C::Value>, Error> {
        let len = self.len();
        if len == MAX_LEN || len + 1 >= MAX_LEN {
            return Err(Error::Oversize {
                len,
                max_len: MAX_LEN,
            });
        }
        Ok(self.0.insert(key, value))
    }
}

impl<C: KeyedCollection, const MAX_LEN: usize> Confined<C, ONE, MAX_LEN>
where
    C: Default,
{
    /// Constructs a confinement with a collection made of a single required key-value pair.
    pub fn with_key_value(key: C::Key, value: C::Value) -> Self {
        let mut c = C::default();
        c.insert(key, value);
        Self(c)
    }
}

impl<const MIN_LEN: usize, const MAX_LEN: usize> Confined<String, MIN_LEN, MAX_LEN> {
    /// Removes a single character from the confined string, unless the string doesn't shorten more
    /// than the confinement requirement. Errors otherwise.
    pub fn remove(&mut self, index: usize) -> Result<char, Error> {
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        if index >= len {
            return Err(Error::OutOfBoundary { index, len });
        }
        Ok(self.0.remove(index))
    }
}

impl<T, const MIN_LEN: usize, const MAX_LEN: usize> Confined<Vec<T>, MIN_LEN, MAX_LEN> {
    /// Removes an element from the vector at a given index. Errors if the index exceeds the number
    /// of elements in the vector, of if the new vector length will be less than the confinement
    /// requirement. Returns the removed element otherwise.
    pub fn remove(&mut self, index: usize) -> Result<T, Error> {
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        if index >= len {
            return Err(Error::OutOfBoundary { index, len });
        }
        Ok(self.0.remove(index))
    }
}

impl<T, const MIN_LEN: usize, const MAX_LEN: usize> Confined<VecDeque<T>, MIN_LEN, MAX_LEN> {
    /// Removes an element from the deque at a given index. Errors if the index exceeds the number
    /// of elements in the deque, of if the new deque length will be less than the confinement
    /// requirement. Returns the removed element otherwise.
    pub fn remove(&mut self, index: usize) -> Result<T, Error> {
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        if index >= len {
            return Err(Error::OutOfBoundary { index, len });
        }
        Ok(self.0.remove(index).expect("element within the length"))
    }
}

impl<T: Hash + Eq, const MIN_LEN: usize, const MAX_LEN: usize>
    Confined<HashSet<T>, MIN_LEN, MAX_LEN>
{
    /// Removes an element from the set. Errors if the index exceeds the number of elements in the
    /// set, of if the new collection length will be less than the confinement requirement. Returns
    /// if the element was present in the set.
    pub fn remove(&mut self, elem: &T) -> Result<bool, Error> {
        if !self.0.contains(elem) {
            return Ok(false);
        }
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.remove(elem))
    }

    /// Removes an element from the set. Errors if the index exceeds the number of elements in the
    /// set, of if the new collection length will be less than the confinement requirement. Returns
    /// the removed element otherwise.
    pub fn take(&mut self, elem: &T) -> Result<Option<T>, Error> {
        if !self.0.contains(elem) {
            return Ok(None);
        }
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.take(elem))
    }
}

impl<T: Ord, const MIN_LEN: usize, const MAX_LEN: usize> Confined<BTreeSet<T>, MIN_LEN, MAX_LEN> {
    /// Removes an element from the set. Errors if the index exceeds the number of elements in the
    /// set, of if the new collection length will be less than the confinement requirement. Returns
    /// if the element was present in the set.
    pub fn remove(&mut self, elem: &T) -> Result<bool, Error> {
        if !self.0.contains(elem) {
            return Ok(false);
        }
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.remove(elem))
    }

    /// Removes an element from the set. Errors if the index exceeds the number of elements in the
    /// set, of if the new collection length will be less than the confinement requirement. Returns
    /// the removed element otherwise.
    pub fn take(&mut self, elem: &T) -> Result<Option<T>, Error> {
        if !self.0.contains(elem) {
            return Ok(None);
        }
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.take(elem))
    }
}

impl<K: Hash + Eq, V, const MIN_LEN: usize, const MAX_LEN: usize>
    Confined<HashMap<K, V>, MIN_LEN, MAX_LEN>
{
    /// Removes an element from the map. Errors if the index exceeds the number of elements in the
    /// map, of if the new collection length will be less than the confinement requirement. Returns
    /// the removed value otherwise.
    pub fn remove(&mut self, key: &K) -> Result<Option<V>, Error> {
        if !self.0.contains_key(key) {
            return Ok(None);
        }
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.remove(key))
    }
}

impl<K: Ord + Hash, V, const MIN_LEN: usize, const MAX_LEN: usize>
    Confined<BTreeMap<K, V>, MIN_LEN, MAX_LEN>
{
    /// Removes an element from the map. Errors if the index exceeds the number of elements in the
    /// map, of if the new collection length will be less than the confinement requirement. Returns
    /// the removed value otherwise.
    pub fn remove(&mut self, key: &K) -> Result<Option<V>, Error> {
        if !self.0.contains_key(key) {
            return Ok(None);
        }
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.remove(key))
    }
}

// Type aliases

/// [`String`] with maximum 255 characters.
pub type TinyString = Confined<String, ZERO, U8>;
/// [`String`] with maximum 2^16-1 characters.
pub type SmallString = Confined<String, ZERO, U16>;
/// [`String`] with maximum 2^24-1 characters.
pub type MediumString = Confined<String, ZERO, U24>;
/// [`String`] with maximum 2^32-1 characters.
pub type LargeString = Confined<String, ZERO, U32>;
/// [`String`] which contains at least a single character.
pub type NonEmptyString = Confined<String, ONE, USIZE>;

/// [`Vec`] with maximum 255 items of type `T`.
pub type TinyVec<T> = Confined<Vec<T>, ZERO, U8>;
/// [`Vec`] with maximum 2^16-1 items of type `T`.
pub type SmallVec<T> = Confined<Vec<T>, ZERO, U16>;
/// [`Vec`] with maximum 2^24-1 items of type `T`.
pub type MediumVec<T> = Confined<Vec<T>, ZERO, U24>;
/// [`Vec`] with maximum 2^32-1 items of type `T`.
pub type LargeVec<T> = Confined<Vec<T>, ZERO, U32>;
/// [`Vec`] which contains at least a single item.
pub type NonEmptyVec<T> = Confined<Vec<T>, ONE, USIZE>;

/// [`VecDeque`] with maximum 255 items of type `T`.
pub type TinyDeque<T> = Confined<VecDeque<T>, ZERO, U8>;
/// [`VecDeque`] with maximum 2^16-1 items of type `T`.
pub type SmallDeque<T> = Confined<VecDeque<T>, ZERO, U16>;
/// [`VecDeque`] with maximum 2^24-1 items of type `T`.
pub type MediumDeque<T> = Confined<VecDeque<T>, ZERO, U24>;
/// [`VecDeque`] with maximum 2^32-1 items of type `T`.
pub type LargeDeque<T> = Confined<VecDeque<T>, ZERO, U32>;
/// [`VecDeque`] which contains at least a single item.
pub type NonEmptyDeque<T> = Confined<VecDeque<T>, ONE, USIZE>;

/// [`HashSet`] with maximum 255 items of type `T`.
pub type TinyHashSet<T> = Confined<HashSet<T>, ZERO, U8>;
/// [`HashSet`] with maximum 2^16-1 items of type `T`.
pub type SmallHashSet<T> = Confined<HashSet<T>, ZERO, U16>;
/// [`HashSet`] with maximum 2^24-1 items of type `T`.
pub type MediumHashSet<T> = Confined<HashSet<T>, ZERO, U24>;
/// [`HashSet`] with maximum 2^32-1 items of type `T`.
pub type LargeHashSet<T> = Confined<HashSet<T>, ZERO, U32>;
/// [`HashSet`] which contains at least a single item.
pub type NonEmptyHashSet<T> = Confined<HashSet<T>, ONE, USIZE>;

/// [`BTreeSet`] with maximum 255 items of type `T`.
pub type TinyOrdSet<T> = Confined<BTreeSet<T>, ZERO, U8>;
/// [`BTreeSet`] with maximum 2^16-1 items of type `T`.
pub type SmallOrdSet<T> = Confined<BTreeSet<T>, ZERO, U16>;
/// [`BTreeSet`] with maximum 2^24-1 items of type `T`.
pub type MediumOrdSet<T> = Confined<BTreeSet<T>, ZERO, U24>;
/// [`BTreeSet`] with maximum 2^32-1 items of type `T`.
pub type LargeOrdSet<T> = Confined<BTreeSet<T>, ZERO, U32>;
/// [`BTreeSet`] which contains at least a single item.
pub type NonEmptyOrdSet<T> = Confined<BTreeSet<T>, ONE, USIZE>;

/// [`HashMap`] with maximum 255 items.
pub type TinyHashMap<K, V> = Confined<HashMap<K, V>, ZERO, U8>;
/// [`HashMap`] with maximum 2^16-1 items.
pub type SmallHashMap<K, V> = Confined<HashMap<K, V>, ZERO, U16>;
/// [`HashMap`] with maximum 2^24-1 items.
pub type MediumHashMap<K, V> = Confined<HashMap<K, V>, ZERO, U24>;
/// [`HashMap`] with maximum 2^32-1 items.
pub type LargeHashMap<K, V> = Confined<HashMap<K, V>, ZERO, U32>;
/// [`HashMap`] which contains at least a single item.
pub type NonEmptyHashMap<K, V> = Confined<HashMap<K, V>, ONE, USIZE>;

/// [`BTreeMap`] with maximum 255 items.
pub type TinyOrdMap<K, V> = Confined<BTreeMap<K, V>, ZERO, U8>;
/// [`BTreeMap`] with maximum 2^16-1 items.
pub type SmallOrdMap<K, V> = Confined<BTreeMap<K, V>, ZERO, U16>;
/// [`BTreeMap`] with maximum 2^24-1 items.
pub type MediumOrdMap<K, V> = Confined<BTreeMap<K, V>, ZERO, U24>;
/// [`BTreeMap`] with maximum 2^32-1 items.
pub type LargeOrdMap<K, V> = Confined<BTreeMap<K, V>, ZERO, U32>;
/// [`BTreeMap`] which contains at least a single item.
pub type NonEmptyOrdMap<K, V> = Confined<BTreeMap<K, V>, ONE, USIZE>;