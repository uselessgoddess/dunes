# Tree Structure Comparison for Graph Database Integer Key Indexing

This document compares various balanced tree structures for storing 64-bit integer keys in a graph database context where relations are indexed as u8/u16/u32/u64 keys. Trees are sorted by priority for the Dunes project requirements.

## Quick Comparison Summary

| Priority | Tree Type | Complexity | Memory/Node | Guarantees | Best For |
|----------|-----------|------------|-------------|------------|----------|
| **1** | **Size-Balanced Tree** | Moderate | 24 bytes | O(log n) worst-case | **In-memory + order stats** |
| **2** | **Red-Black Tree** | Moderate | 25 bytes | O(log n) worst-case | **In-memory general purpose** |
| **3** | **B+ Tree** | High | Variable | O(log n) worst-case | **Disk/mmap persistent** |
| 4 | AVL Tree | Moderate | 24 bytes | O(log n) worst-case | Fast searches |
| 5 | Skip List | Low | 16-32 bytes | O(log n) expected | Simple concurrent |
| 6 | B-Tree | Moderate-High | Variable | O(log n) worst-case | Disk storage |
| 7 | Weight-Balanced Tree | Moderate | 24 bytes | O(log n) worst-case | Functional style |
| 8 | WAVL Tree | High | 24 bytes | O(log n) worst-case | Theoretical optimal |
| 9 | Treap | Low | 24-32 bytes | O(log n) expected | Simple persistent |
| 10 | Scapegoat Tree | Low-Moderate | 16 bytes | O(log n) amortized | Minimal memory |
| - | Lock-Free Skip List | High | 24-48 bytes | O(log n) expected | High concurrency |
| - | Splay Tree | Moderate | 16 bytes | O(log n) amortized | Temporal locality |
| - | vEB Tree | Very High | O(√u) space | O(log log n) | u16 only |
| - | Fusion Tree | Extreme | High | O(log_w n) | Theoretical only |

## Top Priority Trees

### 1. Size-Balanced Tree (SBT) - **RECOMMENDED FOR DUNES**

| Criterion | Rating | Details |
|-----------|--------|---------|
| **Search Performance** | ⭐⭐⭐⭐⭐ | O(log n) worst-case, very good cache behavior |
| **Insertion Performance** | ⭐⭐⭐⭐⭐ | O(log n) worst-case, efficient rebalancing with size field |
| **Memory Footprint** | ⭐⭐⭐⭐⭐ | 2 pointers + size field = ~24 bytes |
| **Ease of Persistence** | ⭐⭐⭐⭐ | Good - size field makes incremental updates efficient |
| **Raw Integer Suitability** | ⭐⭐⭐⭐⭐ | Excellent - size enables order statistics (rank/select) |
| **Range Query Support** | ⭐⭐⭐⭐⭐ | Excellent - supports efficient rank/select operations |
| **In-Memory Performance** | ⭐⭐⭐⭐⭐ | Excellent cache locality and predictable performance |
| **Mmap Suitability** | ⭐⭐⭐⭐ | Size field benefits mmap scenarios |
| **Implementation Complexity** | ⭐⭐⭐ | Moderate - straightforward rotation rules based on size |
| **Worst-Case Consistency** | ⭐⭐⭐⭐⭐ | Guaranteed O(log n) operations, no amortization |
| **Randomization** | ⭐⭐⭐⭐⭐ | None - fully deterministic |

**Why SBT is best for Dunes:**
- Guaranteed O(log n) operations without amortization
- Minimal memory overhead with built-in order statistics support
- Simple, clean implementation suitable for Rust
- Excellent cache locality for in-memory workloads
- No randomization needed
- Well-suited for u8/u16/u32/u64 keys
- Matches the trees-rs inspiration from the issue

**Tradeoffs:**
- Slightly more complex than scapegoat tree
- Size field requires maintenance on each update
- Not optimal for disk-based storage (use B+ tree instead)

---

### 2. Red-Black Tree

| Criterion | Rating | Details |
|-----------|--------|---------|
| **Search Performance** | ⭐⭐⭐⭐ | O(log n) worst-case, good cache locality |
| **Insertion Performance** | ⭐⭐⭐⭐⭐ | O(log n) worst-case, 1-2 rotations typically |
| **Memory Footprint** | ⭐⭐⭐⭐ | 3 pointers + 1 bit color = ~25 bytes |
| **Ease of Persistence** | ⭐⭐⭐⭐ | Good - incremental changes affect path to root only |
| **Raw Integer Suitability** | ⭐⭐⭐⭐⭐ | Excellent - color bit can be packed into pointer |
| **Range Query Support** | ⭐⭐⭐⭐ | Good - ordered traversal is straightforward |
| **In-Memory Performance** | ⭐⭐⭐⭐ | Very good, well-balanced |
| **Mmap Suitability** | ⭐⭐⭐⭐ | Fixed-size nodes benefit from memory mapping |
| **Implementation Complexity** | ⭐⭐⭐ | Moderate - rotation logic and color rules well-documented |
| **Worst-Case Consistency** | ⭐⭐⭐⭐⭐ | Guaranteed O(log n), no amortization |
| **Randomization** | ⭐⭐⭐⭐⭐ | None - deterministic |

**Strengths:**
- Battle-tested in production (Linux kernel, C++ std::map)
- Fewer rotations on average than AVL
- Well-documented algorithms and implementations
- Good balance between performance and simplicity

**Tradeoffs:**
- Slightly more complex than SBT
- Color bit adds minimal overhead
- No built-in order statistics (requires augmentation)

---

### 3. B+ Tree - **RECOMMENDED FOR PERSISTENT STORAGE**

| Criterion | Rating | Details |
|-----------|--------|---------|
| **Search Performance** | ⭐⭐⭐⭐⭐ | O(log n) worst-case, excellent cache behavior |
| **Insertion Performance** | ⭐⭐⭐⭐ | O(log n) worst-case, efficient bulk loading |
| **Memory Footprint** | ⭐⭐⭐ | Variable per node, all data in leaves |
| **Ease of Persistence** | ⭐⭐⭐⭐⭐ | Excellent - optimal for database storage |
| **Raw Integer Suitability** | ⭐⭐⭐⭐⭐ | Excellent - compact leaf storage |
| **Range Query Support** | ⭐⭐⭐⭐⭐ | Best - leaves linked for sequential scans |
| **In-Memory Performance** | ⭐⭐⭐ | Good but not optimal |
| **Mmap Suitability** | ⭐⭐⭐⭐⭐ | Optimal for mmap-backed storage |
| **Implementation Complexity** | ⭐⭐ | High - separate internal/leaf node handling |
| **Worst-Case Consistency** | ⭐⭐⭐⭐⭐ | Guaranteed O(log n) |
| **Randomization** | ⭐⭐⭐⭐⭐ | None - deterministic |

**Why B+ Tree for persistent storage:**
- Industry standard for database indexes
- Excellent sequential scan performance (linked leaves)
- Bulk loading support
- Minimizes disk I/O through high fanout
- Designed specifically for disk-based storage

**When to use:**
- Database needs to exceed memory capacity
- Mmap-backed persistent indexing
- Range scans are frequent
- Hybrid cache + disk design (for cold storage tier)

---

## Secondary Priority Trees

### 4. AVL Tree

| Criterion | Rating | Details |
|-----------|--------|---------|
| Search | ⭐⭐⭐⭐⭐ | O(log n) worst-case, stricter balancing than RB |
| Insertion | ⭐⭐⭐ | O(log n) but may need multiple rotations (up to log n) |
| Memory | ⭐⭐⭐⭐⭐ | 2 pointers + 2-bit height = ~24 bytes |
| Implementation | ⭐⭐⭐ | Moderate - 4 rotation cases |

**Use case:** When search performance is critical and insertions are infrequent.

---

### 5. Skip List

| Criterion | Rating | Details |
|-----------|--------|---------|
| Search | ⭐⭐⭐⭐ | O(log n) expected, good cache locality on lower levels |
| Insertion | ⭐⭐⭐⭐ | O(log n) expected, no rotations |
| Memory | ⭐⭐⭐⭐ | Variable - avg 2 pointers, ~16-32 bytes |
| Implementation | ⭐⭐⭐⭐⭐ | Low - very simple to implement correctly |
| Concurrency | ⭐⭐⭐⭐⭐ | Excellent - natural support for lock-free operations |

**Use case:** Simple implementation needed, or concurrent access required.

---

### 6. B-Tree

| Criterion | Rating | Details |
|-----------|--------|---------|
| Search | ⭐⭐⭐⭐⭐ | O(log n), excellent for disk |
| Insertion | ⭐⭐⭐⭐ | O(log n), bulk operations efficient |
| Memory | ⭐⭐⭐ | High per node but fewer nodes overall |
| Persistence | ⭐⭐⭐⭐⭐ | Designed for persistence |
| Implementation | ⭐⭐ | Moderate-high - node splitting/merging |

**Use case:** Point queries dominate and persistence needed (simpler than B+ tree).

---

### 7. Weight-Balanced Tree (WBT)

| Criterion | Rating | Details |
|-----------|--------|---------|
| Performance | ⭐⭐⭐⭐ | Similar to SBT |
| Memory | ⭐⭐⭐⭐⭐ | 2 pointers + weight = ~24 bytes |
| Functional Style | ⭐⭐⭐⭐⭐ | Excellent for persistent structures |
| Implementation | ⭐⭐⭐ | Moderate - weight ratio maintenance |

**Use case:** Functional/persistent structure needs, similar benefits to SBT.

---

## Lower Priority / Specialized Trees

### 8-10. Other Binary Trees

| Tree | Best Feature | Main Drawback |
|------|--------------|---------------|
| **WAVL Tree** | Theoretically optimal (combines RB + AVL benefits) | Very complex implementation |
| **Treap** | Simple persistent structure | Probabilistic guarantees only |
| **Scapegoat Tree** | Minimal memory (16 bytes/node) | Occasional O(n) rebuilds |

---

### Specialized Concurrent Trees

| Tree | Use Case | Complexity |
|------|----------|------------|
| **Lock-Free Skip List** | High concurrency in-memory | High - subtle correctness |
| **Ctrie / B-link Tree** | Concurrent database operations | Very high |

**Note:** Only consider these if high concurrency is a hard requirement.

---

### Trees Not Recommended

| Tree | Why Not Recommended |
|------|---------------------|
| **Splay Tree** | Amortized only, poor for persistence, unstable structure |
| **Finger Tree** | Overkill for simple keys, very high complexity |
| **Jump List** | Skip list variant, randomized, no advantages over standard skip list |
| **vEB Tree** | O(√u) space impractical for u64, only viable for u16 |
| **Fusion Tree** | Theoretical interest only, extremely complex bit-level tricks |
| **Order Statistic Tree** | This is an augmentation technique, not a base tree (use SBT instead which has this built-in) |

---

## Detailed Comparison Tables

### Performance Characteristics

| Tree | Search | Insert | Delete | Guarantee |
|------|--------|--------|--------|-----------|
| Size-Balanced Tree | O(log n) | O(log n) | O(log n) | Worst-case |
| Red-Black Tree | O(log n) | O(log n) | O(log n) | Worst-case |
| AVL Tree | O(log n) | O(log n) | O(log n) | Worst-case |
| WAVL Tree | O(log n) | O(log n) | O(log n) | Worst-case |
| B+ Tree | O(log n) | O(log n) | O(log n) | Worst-case |
| B-Tree | O(log n) | O(log n) | O(log n) | Worst-case |
| Skip List | O(log n) | O(log n) | O(log n) | Expected |
| Treap | O(log n) | O(log n) | O(log n) | Expected |
| Splay Tree | O(log n) | O(log n) | O(log n) | Amortized |
| Scapegoat Tree | O(log n) | O(log n) | O(log n) | Amortized |

### Memory Footprint Comparison

| Tree | Per-Node Size | Additional Metadata | Total |
|------|---------------|---------------------|-------|
| Scapegoat Tree | 2 ptrs | None | ~16 bytes |
| Splay Tree | 2 ptrs | None | ~16 bytes |
| Size-Balanced Tree | 2 ptrs | size field | ~24 bytes |
| AVL Tree | 2 ptrs | height (2 bits) | ~24 bytes |
| Treap | 2 ptrs | priority | ~24-32 bytes |
| Weight-Balanced | 2 ptrs | weight | ~24 bytes |
| Red-Black Tree | 3 ptrs | color (1 bit) | ~25 bytes |
| Skip List | Variable | level pointers | ~16-32 bytes avg |
| Lock-Free Skip List | Variable | atomic ptrs | ~24-48 bytes |
| B-Tree / B+ Tree | Order-dependent | Multiple keys | Variable |

### Storage Medium Suitability

| Tree | In-Memory | Mmap-backed | Disk-based | Notes |
|------|-----------|-------------|------------|-------|
| **Size-Balanced Tree** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ | Best for in-memory |
| **Red-Black Tree** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ | General purpose |
| **B+ Tree** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | Best for disk |
| **B-Tree** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | Good for disk |
| AVL Tree | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ | Search-heavy |
| Skip List | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | Simple concurrent |
| Lock-Free Skip List | ⭐⭐⭐⭐⭐ | ⭐ | ⭐ | In-memory only |
| Splay Tree | ⭐⭐⭐ | ⭐ | ⭐ | Poor for mmap |
| vEB Tree | ⭐⭐ | ⭐ | ⭐ | Impractical for u64 |

### Implementation Complexity

| Tree | Complexity | Lines of Code (est.) | Debug Difficulty |
|------|------------|----------------------|------------------|
| Scapegoat Tree | ⭐⭐⭐⭐⭐ Low | ~200 | Low |
| Skip List | ⭐⭐⭐⭐⭐ Low | ~150 | Low |
| Treap | ⭐⭐⭐⭐ Low | ~200 | Low |
| Size-Balanced Tree | ⭐⭐⭐ Moderate | ~300 | Moderate |
| Red-Black Tree | ⭐⭐⭐ Moderate | ~400 | Moderate |
| AVL Tree | ⭐⭐⭐ Moderate | ~350 | Moderate |
| Weight-Balanced | ⭐⭐⭐ Moderate | ~350 | Moderate |
| B-Tree | ⭐⭐ Moderate-High | ~500 | High |
| B+ Tree | ⭐⭐ High | ~600 | High |
| WAVL Tree | ⭐ High | ~500 | Very High |
| Lock-Free Skip List | ⭐ High | ~400 | Very High |
| vEB Tree | ⭐ Very High | ~600 | Very High |
| Fusion Tree | ⭐ Extreme | ~1000+ | Extreme |

---

## Recommendations by Use Case

### For Dunes Graph Database - In-Memory Indexing

**Primary Choice: Size-Balanced Tree**

**Implementation Priority:**
1. **Start with SBT** - Clean Rust implementation using `&mut [Node]` slice approach
2. **Add proptest coverage** - Property tests for insert/delete/search invariants
3. **Benchmark with criterion** - Measure real-world performance
4. **Optimize for cache hits** - Profile with perf/cachegrind

**Secondary Choice: Red-Black Tree** (if SBT proves too complex)

---

### For Persistent Mmap-Backed Indexing

**Primary Choice: B+ Tree**

**Key features needed:**
- High fanout (order 64-256)
- Linked leaf nodes for sequential scans
- Bulk loading support
- Fixed-size node layout for mmap

---

### For Hybrid Design (Cache + On-Disk)

**Two-tier approach:**
1. **Hot cache:** Size-Balanced Tree for frequently accessed keys
2. **Cold storage:** B+ Tree for bulk data

**Benefits:**
- Each structure optimized for its storage medium
- Clear migration path as data moves between tiers
- SBT provides fast in-memory operations
- B+ tree optimized for sequential disk access

**Alternative: Adaptive Radix Tree (ART)**
- Excellent for integer keys
- Cache-friendly through node path compression
- Space-efficient for sparse key spaces
- Growing adoption in modern databases (PostgreSQL, HyPer)

---

## Implementation Notes for Dunes

Given the project context from issue #7:

```rust
// Desired API pattern
struct Database {
    memory: Vec<Node<u64>>,
}

impl Database {
    fn grow_memory(&mut self) {
        // 1. Database grows its memory
        self.memory.extend(...);

        // 2. Capture new memory slice into trees
        let slice = &mut self.memory[old_len..];
        let tree = SizeBalanced::new(slice);
    }
}
```

**Key design goals:**
- Use trait-based abstraction (`Tree` trait) for multiple implementations
- Support generic index types (u8/u16/u32/u64, NonZero variants via `Idx` trait)
- Slice-based memory management (`&mut [Node<T>]`)
- Zero-cost abstractions where possible
- Property-based testing with proptest
- Performance benchmarking with criterion

---

## References

- **Trees-rs implementations:**
  - [Feature v2 branch](https://github.com/linksplatform/trees-rs/tree/feature/v2)
  - [Main branch](https://github.com/linksplatform/trees-rs/tree/main)
- **Size-Balanced Tree:** Chinese student's tree, optimal for order statistics
- **Database indexing:** B-tree variants remain industry standard for disk-based storage
- **Modern trends:** Adaptive Radix Trees gaining adoption in high-performance databases
