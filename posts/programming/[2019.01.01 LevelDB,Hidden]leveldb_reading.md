# LevelDB 源码阅读

### 1. 文件结构

| Files | Descriptions |
| ----- | ------------ |
| ./benchmarks/db\_bench.cc | Benchmark |
| ./benchmarks/db\_bench\_sqlite3.cc | Benchmark |
| ./benchmarks/db\_bench\_tree\_db.cc | Benchmark |
| ./build/include/port/port\_config.h | Generated |
| ./db/autocompact\_test.cc | 测试（还没细读） |
| ./db/builder.cc | 根据 Iter 构建 Table 并写文件 |
| ./db/builder.h | BuildTable 接口 |
| ./db/c.cc | C 封装 |
| ./db/corruption\_test.cc | 测试（还没细读） |
| ./db/db\_impl.cc | |
| ./db/db\_impl.h | |
| ./db/db\_iter.cc | |
| ./db/db\_iter.h | |
| ./db/db\_test.cc | 2000 多行的测试（还没细读） |
| ./db/dbformat.cc | InternalKey、LookupKey 封装函数 |
| ./db/dbformat.h | 定一了 DB 内部的核心 Config，以及 InternalKey 和 LookupKey 包装，在 db 目录下均使用封装后的 key 值 |
| ./db/dbformat\_test.cc | DBFormat 测试 |
| ./db/dumpfile.cc | DumpFile 实现（还没细读） |
| ./db/fault\_injection\_test.cc | 测试（还没细读） |
| ./db/filename.cc | DBName FileName 互转 |
| ./db/filename.h | DBName FileName 互转 |
| ./db/filename\_test.cc | DBName FileName 互转 |
| ./db/leveldbutil.cc | Dump File CLI |
| ./db/log\_format.h | Log 中的 RecordType 各项参数 |
| ./db/log\_reader.cc | 与之对应的 Reader（还没细读） |
| ./db/log\_reader.h | 与之对应的 Reader（还没细读） |
| ./db/log\_test.cc | Log 测试（还没细读） |
| ./db/log\_writer.cc | 神来之笔：dest_->Append(Slice("\x00\x00\x00\x00\x00\x00", leftover)); |
| ./db/log\_writer.h | 使用文件构造 Writer，使用 AddRecord 写入记录。按照 kBlockSize 大小安排 Record |
| ./db/memtable.cc | MemTableIterator 的实现，SkipList Wrapper |
| ./db/memtable.h | MemTable 封装，使用 SkipList 和 Arena，实现 key-value 对的 Get 和 Add 方法，以及迭代器。其中 Key 使用 Internal Key，包含了 SequenceNumber 和 ValueType，存储到 MemTable 时会在头部加入长度编码，Value 也是如此。查找时由于排序的关系，会找到最近更新的值。 |
| ./db/recovery\_test.cc | 恢复测试（还没细读） |
| ./db/repair.cc | RepairDB 的实现（没细读） |
| ./db/skiplist.h | SkipList，支持遍历、快速查找，无删除。同步机制还需要再研究下 |
| ./db/skiplist\_test.cc | SkipList 测试 |
| ./db/snapshot.h | Snapshot 双向链表 |
| ./db/table\_cache.cc | Table LRU Cache 实现 |
| ./db/table\_cache.h | Table LRU Cache 接口 |
| ./db/version\_edit.cc | Version Meta 编解码 |
| ./db/version\_edit.h | 可 Edit 的 Version Meta |
| ./db/version\_edit\_test.cc | Version Meta 编解码测试 |
| ./db/version\_set.cc |  |
| ./db/version\_set.h | Version、VersionSet 和 Compaction 的定义 |
| ./db/version\_set\_test.cc | 一半是 FindFile 和 SomeFileOverlapsRange 的测试 |
| ./db/write\_batch.cc | WriteBatch 构造、合并及写入 MemTable |
| ./db/write\_batch\_internal.h | WriteBatch 内部接口 |
| ./db/write\_batch\_test.cc | WriteBatch 测试（还没细读） |
| ./helpers/memenv/memenv.cc | Env in Memory |
| ./helpers/memenv/memenv.h | Env in Memory |
| ./helpers/memenv/memenv\_test.cc | Env in Memory |
| ./include/leveldb/c.h | 暴露的 C 接口，解决 API 问题 |
| ./include/leveldb/cache.h | 缓存接口，附带 `LRUCache` 工厂函数 |
| ./include/leveldb/comparator.h | 比较器接口，仅 `dbformat.h` 中 `InternalKeyComparator` 继承该类 |
| ./include/leveldb/db.h | LevelDB 核心 DB 接口，及 `Snapshot` 和 `Range` 接口 |
| ./include/leveldb/dumpfile.h | 一个 `DumpFile` 接口，功能暂不明确 |
| ./include/leveldb/env.h | `Env` 环境，封装了文件系统、`Log` 和 `Thread` 操作 |
| ./include/leveldb/export.h | 跨平台 `EXPORT` 实现 |
| ./include/leveldb/filter\_policy.h | `Filter` 策略对象，附带 `BloomFilterPolicy` 工厂函数 |
| ./include/leveldb/iterator.h | `Iterator` 接口 |
| ./include/leveldb/options.h | `Option` 接口 |
| ./include/leveldb/slice.h | `Slice` 字节流封装 |
| ./include/leveldb/status.h | 状态码接口 |
| ./include/leveldb/table.h | `Table` 接口 |
| ./include/leveldb/table\_builder.h | `TableBuilder` 接口 |
| ./include/leveldb/write\_batch.h | `WriteBatch` 接口，存储一组待更新的键值对 |
| ./issues/issue178\_test.cc | Issue |
| ./issues/issue200\_test.cc | Issue |
| ./issues/issue320\_test.cc | Issue |
| ./port/port.h | 平台适配 |
| ./port/port\_example.h | 平台适配 |
| ./port/port\_stdcxx.h | 平台适配 |
| ./port/thread\_annotations.h | 平台适配 |
| ./table/block.cc | Block Iterator 的实现，包括共享 key 下的二分查找 |
| ./table/block.h | Block 的定义，最后 4 位存储 NumRestarts |
| ./table/block\_builder.cc | BlockBuilder 实现，注释中描述了 Block 的 Form |
| ./table/block\_builder.h | BlockBuilder 接口 |
| ./table/filter\_block.cc | FilterBlockBuilder 和 FilterBlockReader 的实现 |
| ./table/filter\_block.h | FilterBlockBuilder 和 FilterBlockReader 的接口 |
| ./table/filter\_block\_test.cc | FilterBlock 的测试 |
| ./table/format.cc | BlockHandle 和 Footer 的 Encode 和 Decode，以及 ReadBlock 实现 |
| ./table/format.h | BlockHandle、Footer 的定义。BlockHandle 包含 offset 和 size，Footer 包含 MetaIndex 和 Index 的 BlockHandle。Block 的尾部包含 Block Type 和 CRC 校验值。 |
| ./table/iterator.cc | Iterator 的函数实现，包括 EmptyIterator |
| ./table/iterator\_wrapper.h | Iterator Wrapper，缓存 Key 值和 Valid，avoid virtual function calls |
| ./table/merger.cc | Iterator Merge，Sharded 实现 |
| ./table/merger.h | Iterator Merge 接口 |
| ./table/table.cc | Table Reader |
| ./table/table\_builder.cc | Table Writer |
| ./table/table\_test.cc | Table 测试（还没细读） |
| ./table/two\_level\_iterator.cc | TwoLevelIterator 实现（还需要细读） |
| ./table/two\_level\_iterator.h | TwoLevelIterator 接口 |
| ./util/arena.cc | `MemoryPool` 的实现。提供 `Allocate` 和 `AllocateAligned` 两种接口，后者保证申请的内存起始地址对齐。默认申请 4k 的 Block，每次消费 Block 中的剩余空间。析构时依次删除每个 Block。 |
| ./util/arena.h | `MemoryPool` 接口 |
| ./util/arena\_test.cc | `Arena` 测试 |
| ./util/bloom.cc | `BloomFilterPolicy`，提供 filter 的生成和 key in filter 的检查 |
| ./util/bloom\_test.cc | `Bloom` 过滤器测试 |
| ./util/cache.cc | `LRUCache` 实现。每个节点定义为 `LRUHandle`，存储键值和前后节点地址信息；`HandleTable` 实现了闭式哈希表（哈希表中使用 `next_hash` 构建链表），支持 `Resize`；`LRUCache` 使用双向链表实现了 `LRU` 的功能，`Lookup` 时将节点先删除，再添加到链表末端，保持更新，`Insert` 时删除旧的节点；`ShardedLRUCache` 实现了根据 `Hash` 值高位分片的 `LRU` |
| ./util/cache\_test.cc | `Cache` 测试 |
| ./util/coding.cc | 定长/变长整型编解码实现 |
| ./util/coding.h | 定长/变长整型编解码接口 |
| ./util/coding\_test.cc | 测试 |
| ./util/comparator.cc | `BytewiseComparator` 工厂实现 |
| ./util/crc32c.cc | `CRC32` 实现 |
| ./util/crc32c.h | `CRC32` 接口 |
| ./util/crc32c\_test.cc | `CRC32` 测试 |
| ./util/env.cc | 环境 |
| ./util/env\_posix.cc | 环境 |
| ./util/env\_posix\_test.cc | 环境 |
| ./util/env\_posix\_test\_helper.h | 环境 |
| ./util/env\_test.cc | 环境 |
| ./util/env\_windows.cc | 环境 |
| ./util/env\_windows\_test.cc | 环境 |
| ./util/env\_windows\_test\_helper.h | 环境 |
| ./util/filter\_policy.cc | `FilterPolicy` 空析构函数 |
| ./util/hash.cc | 字节流哈希 |
| ./util/hash.h | 字节流哈希 |
| ./util/hash\_test.cc | 字节流哈希 |
| ./util/histogram.cc | 朴素的直方图统计 |
| ./util/histogram.h | 朴素的直方图统计 |
| ./util/logging.cc | 数值字符串转换辅助函数 |
| ./util/logging.h | 数值字符串转换辅助函数 |
| ./util/logging\_test.cc | 数值字符串转换辅助函数 |
| ./util/mutexlock.h | 锁的 RAII 封装 |
| ./util/no\_destructor.h | NoDestructor |
| ./util/no\_destructor\_test.cc | NoDestructor |
| ./util/options.cc | Options 默认构造函数 |
| ./util/posix\_logger.h | Log 函数实现 |
| ./util/random.h | 线性同余随机数生成器 |
| ./util/status.cc | 状态码实现 |
| ./util/status\_test.cc | 状态码测试 |
| ./util/testharness.cc | 单元测试辅助 |
| ./util/testharness.h | 单元测试辅助，宏 |
| ./util/testutil.cc | 测试辅助函数 |
| ./util/testutil.h | 测试辅助函数 |
| ./util/windows\_logger.h | Log 函数实现 |

### 2. 实现细节

1. 装饰器模式。代码用多次出现 B 类继承 A 类、同时包含一个 A 类私有对象，比如 `EnvWrapper`、`InternalKeyComparator`、`InternalFilterPolicy`。继承的同时，可以选择性的修改部分功能，可以理解为装饰器模式
2. LRUCache（待整理）
3. SkipList（待整理）

