# B+ Tree Integration with `mem` Crate

This document analyzes the compatibility and integration strategy for implementing B+ Tree with the Dunes `mem` crate in a database engine context.

## Executive Summary

**B+ Tree is highly compatible with the `mem` crate** and represents the optimal choice for persistent, mmap-backed storage scenarios. The `RawMem` trait abstraction provides exactly what B+ Tree needs: dynamic memory growth, zero-cost slice access, and support for memory-mapped files.

**Recommendation:** Implement B+ Tree as a future enhancement when persistent storage requirements emerge. Current in-memory trees (SBT, Treap) are sufficient for the initial implementation phase.

---

## Why B+ Tree for Database Persistence

From docs/TREE_COMPARISON.md, B+ Tree is the #3 priority tree and **#1 for persistent storage**:

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| **Ease of Persistence** | ⭐⭐⭐⭐⭐ | Industry standard for database indexes |
| **Mmap Suitability** | ⭐⭐⭐⭐⭐ | Optimal for mmap-backed storage |
| **Range Query Support** | ⭐⭐⭐⭐⭐ | Leaves linked for sequential scans |
| **Search Performance** | ⭐⭐⭐⭐⭐ | O(log n) worst-case, excellent cache behavior |
| **In-Memory Performance** | ⭐⭐⭐ | Good but not optimal (use SBT instead) |

**Key B+ Tree Properties:**
- All data stored in leaf nodes (internal nodes only route)
- Leaf nodes linked together for efficient range scans
- High fanout reduces tree height and disk I/O
- Bulk loading support for initial construction
- Predictable performance for database workloads

---

## Current `mem` Crate Architecture

The `mem` crate provides three key abstractions:

### 1. `RawMem` Trait (crates/mem/src/raw.rs)

```rust
pub trait RawMem {
    type Item: Pod;  // bytemuck::Pod for safe transmutation

    fn as_slice(&self) -> &[Self::Item];
    fn as_mut_slice(&mut self) -> &mut [Self::Item];
    fn grow(&mut self, cap: usize) -> Result<Page<'_, Self::Item>>;
    fn shrink(&mut self, cap: usize) -> Result<()>;
}
```

**Perfect for B+ Tree because:**
- ✅ Slice-based access matches B+ Tree's array-of-keys/children storage
- ✅ Dynamic growth supports node splitting
- ✅ `Pod` constraint ensures safe memory mapping
- ✅ Zero-cost abstraction over different backends

### 2. `Alloc<T>` - Dynamically Allocated Memory (crates/mem/src/alloc.rs)

```rust
pub struct Alloc<T> {
    // Vec-like dynamically allocated memory
}
```

**Use case:** In-memory B+ Tree for hot data cache
- Fast allocation via system allocator
- No persistence
- Best for frequently accessed index

### 3. `FileMapped<T>` - Memory-Mapped Files (crates/mem/src/file.rs)

```rust
pub struct FileMapped<T> {
    // Memory-mapped file storage
}
```

**Use case:** Persistent B+ Tree for cold storage tier
- Direct file-backed storage
- OS-managed page cache
- Survives process restarts
- Enables database larger than RAM

### 4. `PreAlloc<P>` - Pre-Allocated Memory (crates/mem/src/pre.rs)

```rust
pub struct PreAlloc<P> {
    // Wraps pre-allocated slices/arrays
}
```

**Use case:** Stack-allocated or fixed-size B+ Trees
- Zero allocation overhead
- Useful for small, predictable workloads
- Embedded database scenarios

---

## B+ Tree Node Structure Design

B+ Tree requires a different node structure than binary trees. Here's how to integrate with `mem` crate:

### Proposed Node Structure

```rust
use bytemuck::{Pod, Zeroable};

/// B+ Tree internal node - routes searches to children
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct InternalNode<T, const ORDER: usize> {
    /// Number of keys currently in node (0..ORDER-1)
    pub key_count: u32,
    /// Whether this is a leaf node
    pub is_leaf: bool,
    /// Padding for alignment
    _padding: [u8; 3],
    /// Keys for routing (sorted array)
    pub keys: [T; ORDER],
    /// Child pointers (indices into node array)
    /// children[i] contains keys < keys[i]
    /// children[key_count] contains keys >= keys[key_count-1]
    pub children: [Option<u32>; ORDER + 1],
}

/// B+ Tree leaf node - stores actual data
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct LeafNode<T, V, const ORDER: usize> {
    /// Number of keys currently in node
    pub key_count: u32,
    /// Next leaf pointer for range scans
    pub next_leaf: Option<u32>,
    /// Previous leaf pointer for reverse scans
    pub prev_leaf: Option<u32>,
    /// Keys (sorted array)
    pub keys: [T; ORDER],
    /// Values corresponding to keys
    pub values: [V; ORDER],
}

/// Unified node type for storage
#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub union BPlusNode<T, V, const ORDER: usize> {
    pub internal: InternalNode<T, ORDER>,
    pub leaf: LeafNode<T, V, ORDER>,
}
```

**Key Design Decisions:**

1. **Fixed-size nodes with ORDER parameter**
   - Enables efficient array-based storage
   - `#[repr(C)]` guarantees memory layout
   - Works perfectly with `mem` crate's slice-based API

2. **`Pod` and `Zeroable` constraints**
   - Required by `RawMem` trait
   - Enables safe memory mapping
   - Allows zero-copy serialization

3. **Index-based pointers (u32)**
   - Instead of raw pointers, use indices into node array
   - Survives memory mapping and relocation
   - Supports up to 4 billion nodes (sufficient for most databases)

4. **Linked leaf nodes**
   - `next_leaf` and `prev_leaf` enable O(k) range scans
   - Critical for database workloads (SELECT * WHERE key BETWEEN a AND b)

---

## Integration Pattern: B+ Tree with `mem` Crate

### Proposed API

```rust
use mem::{RawMem, Alloc, FileMapped};

/// B+ Tree backed by any RawMem implementation
pub struct BPlusTree<T, V, M, const ORDER: usize>
where
    T: Pod + Ord,
    V: Pod,
    M: RawMem<Item = BPlusNode<T, V, ORDER>>,
{
    mem: M,
    root: Option<u32>,  // Index of root node
    height: u32,
    node_count: u32,
}

impl<T, V, M, const ORDER: usize> BPlusTree<T, V, M, ORDER>
where
    T: Pod + Ord,
    V: Pod,
    M: RawMem<Item = BPlusNode<T, V, ORDER>>,
{
    /// Create new B+ Tree with given memory backend
    pub fn new(mem: M) -> Self {
        Self {
            mem,
            root: None,
            height: 0,
            node_count: 0,
        }
    }

    /// Search for a key
    pub fn search(&self, key: &T) -> Option<&V> {
        let root = self.root?;
        let mut current = root;
        let mut height = self.height;

        // Traverse internal nodes
        while height > 0 {
            let node = self.get_internal_node(current)?;
            let child_idx = self.find_child_index(node, key);
            current = node.children[child_idx]?;
            height -= 1;
        }

        // Search in leaf node
        let leaf = self.get_leaf_node(current)?;
        let key_idx = leaf.keys[..leaf.key_count as usize]
            .binary_search(key)
            .ok()?;
        Some(&leaf.values[key_idx])
    }

    /// Insert key-value pair
    pub fn insert(&mut self, key: T, value: V) -> mem::Result<()> {
        if self.root.is_none() {
            // Allocate first node (leaf)
            let node_idx = self.allocate_node()?;
            self.root = Some(node_idx);
        }

        let root = self.root.unwrap();
        if let Some(split_info) = self.insert_into_node(root, key, value)? {
            // Root split - grow tree height
            let new_root = self.allocate_node()?;
            // ... handle root split
            self.root = Some(new_root);
            self.height += 1;
        }

        Ok(())
    }

    /// Range scan from start_key to end_key
    pub fn range_scan(&self, start: &T, end: &T) -> RangeIter<T, V, M, ORDER> {
        // Find starting leaf
        // Iterate through linked leaves until end
        todo!()
    }

    /// Allocate a new node using mem backend
    fn allocate_node(&mut self) -> mem::Result<u32> {
        let idx = self.node_count;
        let page = self.mem.grow(1)?;

        // Initialize new node as zeroed (using Pod trait)
        page.zeroed();

        self.node_count += 1;
        Ok(idx)
    }

    /// Get node as slice
    fn get_node(&self, idx: u32) -> Option<&BPlusNode<T, V, ORDER>> {
        self.mem.as_slice().get(idx as usize)
    }

    /// Get mutable node reference
    fn get_node_mut(&mut self, idx: u32) -> Option<&mut BPlusNode<T, V, ORDER>> {
        self.mem.as_mut_slice().get_mut(idx as usize)
    }
}
```

### Usage Examples

#### Example 1: In-Memory B+ Tree

```rust
use mem::Alloc;

// Create in-memory B+ Tree for hot data
let mem = Alloc::<BPlusNode<u64, u64, 64>>::new();
let mut tree = BPlusTree::<u64, u64, _, 64>::new(mem);

tree.insert(42, 100)?;
tree.insert(17, 200)?;

let value = tree.search(&42); // Some(&100)
```

#### Example 2: Memory-Mapped B+ Tree

```rust
use mem::FileMapped;

// Create persistent B+ Tree backed by file
let mem = FileMapped::<BPlusNode<u64, u64, 128>>::from_path("index.db")?;
let mut tree = BPlusTree::<u64, u64, _, 128>::new(mem);

// Insertions persist to disk
tree.insert(42, 100)?;

// Survives process restart - just reopen the file
let mem = FileMapped::<BPlusNode<u64, u64, 128>>::from_path("index.db")?;
let tree = BPlusTree::<u64, u64, _, 128>::new(mem);
let value = tree.search(&42); // Still Some(&100)
```

#### Example 3: Hybrid Cache + Persistent Design

```rust
use mem::{Alloc, FileMapped};

struct Database {
    // Hot cache: SBT for frequently accessed keys
    hot_cache: Store<u64>,  // Existing SBT implementation

    // Cold storage: B+ Tree for bulk data
    cold_storage: BPlusTree<u64, u64, FileMapped<...>, 256>,
}

impl Database {
    fn get(&self, key: u64) -> Option<u64> {
        // Try hot cache first
        if let Some(val) = self.hot_cache.search(key) {
            return Some(val);
        }

        // Fall back to cold storage
        self.cold_storage.search(&key).copied()
    }
}
```

---

## Implementation Complexity and Effort

### Lines of Code Estimate (from TREE_COMPARISON.md)
- **~600 LOC** for full B+ Tree implementation
- **~200 LOC** for tests
- **~100 LOC** for benchmarks
- **Total: ~900 LOC**

### Implementation Phases

**Phase 1: Core B+ Tree Operations (300 LOC)**
- Node structure definition with `Pod` + `Zeroable`
- Search operation
- Insert operation with node splitting
- Basic leaf linking

**Phase 2: Deletion and Rebalancing (200 LOC)**
- Delete operation
- Node merging and rebalancing
- Maintain minimum occupancy

**Phase 3: Advanced Features (100 LOC)**
- Range scan iterators
- Bulk loading for initial construction
- Node defragmentation

**Phase 4: Integration and Testing (200 LOC)**
- Integration with `mem` crate backends
- Property-based tests with proptest
- Benchmark suite

**Phase 5: Optimization (100 LOC)**
- Simd-accelerated key search
- Prefix compression for keys
- Cache-aware node layout

---

## Performance Characteristics

### Memory Layout

```
Order 64 B+ Tree node (u64 keys, u64 values):
- key_count: 4 bytes
- is_leaf: 1 byte
- padding: 3 bytes
- keys: 64 * 8 = 512 bytes
- children/values: 65 * 8 = 520 bytes (internal) or 64 * 8 = 512 bytes (leaf)
Total: ~1040 bytes per node
```

**Cache behavior:**
- Modern CPUs have 64-byte cache lines
- Order 64 node spans ~16 cache lines
- Binary search within node: ~6 cache line accesses
- Total tree search: ~6 * height cache line accesses

### Fanout vs. Height Trade-off

| Order | Keys/Node | Height (1M keys) | Height (1B keys) | Node Size |
|-------|-----------|------------------|------------------|-----------|
| 16    | 16        | 5                | 8                | ~256 B    |
| 32    | 32        | 4                | 7                | ~512 B    |
| 64    | 64        | 4                | 6                | ~1 KB     |
| 128   | 128       | 3                | 5                | ~2 KB     |
| 256   | 256       | 3                | 5                | ~4 KB     |

**Recommended orders:**
- **Order 64**: In-memory workloads (1 KB nodes, cache-friendly)
- **Order 256**: Disk/mmap workloads (4 KB nodes, matches page size)

### Comparison with SBT (Size-Balanced Tree)

| Criterion | SBT | B+ Tree (Order 64) |
|-----------|-----|-------------------|
| **Node size** | 24 bytes | 1040 bytes |
| **Height (1M keys)** | ~20 | ~4 |
| **Height (1B keys)** | ~30 | ~6 |
| **Cache lines per search** | 20-30 | 24 (4 nodes * 6 lines) |
| **Range scan** | O(k log n) | O(k) - linked leaves! |
| **Memory overhead** | Low | Medium-High |
| **Best use case** | In-memory indexing | Persistent storage, range queries |

**Conclusion:** SBT wins for in-memory, B+ Tree wins for persistent and range-heavy workloads.

---

## mem Crate Compatibility Analysis

### ✅ Perfect Compatibility

1. **`RawMem` trait is ideal for B+ Tree**
   - Slice-based access matches node array storage
   - `grow()` supports dynamic node allocation during splits
   - `shrink()` could support node deletion (optional)
   - `Pod` constraint ensures memory mapping safety

2. **Multiple backends supported**
   - `Alloc<T>`: In-memory B+ Tree
   - `FileMapped<T>`: Persistent B+ Tree
   - `PreAlloc<P>`: Fixed-size embedded B+ Tree
   - Easy to switch backends without changing B+ Tree code

3. **Zero-cost abstraction**
   - No runtime overhead from trait-based design
   - Monomorphization eliminates virtual dispatch
   - Direct slice access for node reads/writes

### ⚠️ Design Considerations

1. **Node size constraints**
   - Nodes must implement `Pod` trait
   - No heap allocations within nodes
   - Fixed-size arrays only (no `Vec`)
   - Solution: Use const generic ORDER parameter

2. **Pointer relocation**
   - Memory-mapped files may map at different addresses
   - Solution: Use indices (u32) instead of raw pointers ✅

3. **Growth strategy**
   - B+ Tree grows one node at a time during splits
   - `mem` crate's `grow(1)` is perfect for this
   - No need for exponential growth like Vec

4. **Alignment requirements**
   - `#[repr(C)]` ensures predictable layout
   - `Pod` trait guarantees safe byte representation
   - Padding added automatically for alignment

### 🔧 Potential mem Crate Enhancements

**Optional enhancements that would help B+ Tree (but not required):**

1. **Batch allocation:**
   ```rust
   fn grow_batch(&mut self, count: usize) -> Result<&mut [Self::Item]>
   ```
   Useful for bulk loading, but `grow(1)` in loop works fine.

2. **Node recycling:**
   ```rust
   fn free(&mut self, index: usize) -> Result<()>
   ```
   For reusing deleted nodes, but can track free list in B+ Tree itself.

3. **Persistent metadata:**
   ```rust
   fn metadata(&self) -> &mut [u8; 4096]  // First page for tree metadata
   ```
   Store root pointer, height, etc. Can workaround by reserving node 0.

**None of these are blockers - current `mem` crate API is sufficient.**

---

## Comparison with Existing Trees

### Current Implementation Status

| Tree | Status | Complexity | Use Case |
|------|--------|------------|----------|
| **Size-Balanced Tree** | ✅ Implemented | ⭐⭐⭐ Moderate | In-memory indexing (PRIMARY) |
| **Treap** | ✅ Implemented | ⭐⭐⭐⭐ Low | Simple persistent structures |
| **B+ Tree** | ❌ Not implemented | ⭐⭐ High | Persistent mmap-backed storage |

### Speed/Difficulty Balance

From docs/TREE_COMPARISON.md:

| Tree | Implementation | Performance | Balance Score |
|------|---------------|-------------|---------------|
| **Treap** | ⭐⭐⭐⭐ Low (~200 LOC) | ⭐⭐⭐⭐ Good | **Excellent** ✅ |
| **SBT** | ⭐⭐⭐ Moderate (~300 LOC) | ⭐⭐⭐⭐⭐ Excellent | **Very Good** ✅ |
| **B+ Tree** | ⭐⭐ High (~600 LOC) | ⭐⭐⭐⭐⭐ Excellent (persistent) | **Good** 🟡 |

**Interpretation for issue #10:**
- **"Better speed/difficulty balance"** = Treap (already implemented!)
- **"Research B+ Tree compatibility"** = This document

**Current state is good:**
- ✅ Treap provides easy-to-implement tree option
- ✅ SBT provides high-performance in-memory option
- ✅ B+ Tree reserved for future persistent storage needs

---

## Migration Path: When to Implement B+ Tree

### Current Phase: In-Memory Only
**Use SBT** - Best performance, moderate complexity, already implemented.

### Phase 2: Persistence Needed
**Triggers for B+ Tree implementation:**
- Database size exceeds available RAM
- Durability requirements (survive crashes)
- Range scan queries become frequent
- Need to support mmap-backed indexes

### Phase 3: Hybrid Design
**Two-tier architecture:**
- **Hot tier:** SBT for frequently accessed keys (in-memory)
- **Cold tier:** B+ Tree for bulk storage (mmap-backed)
- Automatic promotion/demotion based on access patterns

---

## Implementation Roadmap

### Minimal B+ Tree Implementation (MVP)

**Goal:** Basic B+ Tree working with `mem` crate in ~400 LOC

**Scope:**
1. ✅ Node structure with `Pod` + `Zeroable`
2. ✅ Search operation
3. ✅ Insert with splits
4. ✅ Basic tests
5. ❌ Delete (defer to later)
6. ❌ Rebalancing (defer to later)

**Estimated effort:** 3-4 days

### Full B+ Tree Implementation

**Goal:** Production-ready B+ Tree with all features

**Scope:**
1. All MVP features
2. Delete with node merging
3. Bulk loading
4. Range scan iterators
5. Comprehensive tests (unit + proptest)
6. Benchmarks vs. SBT
7. Documentation

**Estimated effort:** 1-2 weeks

---

## Code Skeleton Example

Here's a minimal skeleton showing integration with `mem` crate:

```rust
use bytemuck::{Pod, Zeroable};
use mem::{RawMem, Result};

const ORDER: usize = 64;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct Node<T: Pod> {
    key_count: u32,
    is_leaf: bool,
    _pad: [u8; 3],
    keys: [T; ORDER],
    children: [Option<u32>; ORDER + 1],
}

struct BPlusTree<T: Pod + Ord, M: RawMem<Item = Node<T>>> {
    mem: M,
    root: Option<u32>,
}

impl<T: Pod + Ord, M: RawMem<Item = Node<T>>> BPlusTree<T, M> {
    pub fn new(mem: M) -> Self {
        Self { mem, root: None }
    }

    pub fn search(&self, key: &T) -> bool {
        let Some(root) = self.root else { return false };
        let nodes = self.mem.as_slice();
        let mut current = root as usize;

        loop {
            let node = &nodes[current];
            let pos = node.keys[..node.key_count as usize]
                .binary_search(key);

            match pos {
                Ok(_) => return !node.is_leaf, // Found in internal
                Err(idx) => {
                    if node.is_leaf {
                        return false;
                    }
                    current = node.children[idx]?.as_usize();
                }
            }
        }
    }

    pub fn insert(&mut self, key: T) -> Result<()> {
        if self.root.is_none() {
            let root_idx = self.allocate_node()?;
            self.root = Some(root_idx);
        }
        // ... rest of insert logic
        Ok(())
    }

    fn allocate_node(&mut self) -> Result<u32> {
        let nodes = self.mem.as_slice();
        let idx = nodes.len() as u32;
        self.mem.grow(1)?.zeroed();
        Ok(idx)
    }
}
```

This skeleton demonstrates:
- ✅ `Pod` + `Zeroable` node structure
- ✅ Slice-based node access
- ✅ Dynamic node allocation via `grow()`
- ✅ Generic over `RawMem` backend

---

## Conclusion

**B+ Tree is highly compatible with the `mem` crate** and should be implemented when:
1. Database needs to exceed RAM capacity
2. Persistent storage is required
3. Range queries become frequent

**Current state (SBT + Treap) is excellent for:**
- ✅ In-memory graph database indexing
- ✅ Fast lookups and updates
- ✅ Simple implementation and maintenance

**Recommendation:**
- **Short term:** Continue using SBT for in-memory indexing
- **Medium term:** Document B+ Tree design (this document)
- **Long term:** Implement B+ Tree when persistence needs arise

The `mem` crate's abstraction is perfectly suited for B+ Tree implementation, requiring no changes to the existing API.

---

## References

- **docs/TREE_COMPARISON.md**: Comprehensive tree analysis
- **crates/mem/**: Memory management abstraction crate
- **crates/trees/**: Current tree implementations (SBT, Treap)
- **Issue #7**: Original tree implementation issue
- **Issue #10**: B+ Tree research (this document)

---

*Document created: 2025-12-02*
*Dunes Database Engine*
