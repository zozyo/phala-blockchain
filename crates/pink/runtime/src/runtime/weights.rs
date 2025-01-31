#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

use pallet_contracts::weights::WeightInfo;

pub struct PinkWeights<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PinkWeights<T> {
    /// Storage: Contracts DeletionQueueCounter (r:1 w:0)
    /// Proof: Contracts DeletionQueueCounter (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    fn on_process_deletion_queue_batch() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `109`
        //  Estimated: `1594`
        // Minimum execution time: 2_627_000 picoseconds.
        Weight::from_parts(2_748_000, 1594)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `k` is `[0, 1024]`.
    fn on_initialize_per_trie_key(k: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `488 + k * (69 ±0)`
        //  Estimated: `478 + k * (70 ±0)`
        // Minimum execution time: 13_607_000 picoseconds.
        Weight::from_parts(8_026_118, 478)
            // Standard Error: 1_323
            .saturating_add(Weight::from_parts(980_583, 0).saturating_mul(k.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(k.into())))
            .saturating_add(T::DbWeight::get().writes(2_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(k.into())))
            .saturating_add(Weight::from_parts(0, 70).saturating_mul(k.into()))
    }
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeStorage (r:0 w:1)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// The range of component `c` is `[0, 61717]`.
    fn reinstrument(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `238 + c * (1 ±0)`
        //  Estimated: `3708 + c * (1 ±0)`
        // Minimum execution time: 30_563_000 picoseconds.
        Weight::from_parts(22_292_544, 3708)
            // Standard Error: 60
            .saturating_add(Weight::from_parts(54_541, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(c.into()))
    }
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `c` is `[0, 125952]`.
    fn call_with_code_per_byte(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `707`
        //  Estimated: `6656 + c * (1 ±0)`
        // Minimum execution time: 268_884_000 picoseconds.
        Weight::from_parts(277_799_331, 6656)
            // Standard Error: 23
            .saturating_add(Weight::from_parts(37_876, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(c.into()))
    }
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:2 w:2)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts CodeStorage (r:0 w:1)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Contracts PristineCode (r:0 w:1)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// The range of component `c` is `[0, 61717]`.
    /// The range of component `i` is `[0, 1048576]`.
    /// The range of component `s` is `[0, 1048576]`.
    fn instantiate_with_code(c: u32, i: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `270`
        //  Estimated: `8659`
        // Minimum execution time: 3_159_921_000 picoseconds.
        Weight::from_parts(594_826_134, 8659)
            // Standard Error: 290
            .saturating_add(Weight::from_parts(106_471, 0).saturating_mul(c.into()))
            // Standard Error: 17
            .saturating_add(Weight::from_parts(1_160, 0).saturating_mul(i.into()))
            // Standard Error: 17
            .saturating_add(Weight::from_parts(1_417, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(10_u64))
    }
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:2 w:2)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `i` is `[0, 1048576]`.
    /// The range of component `s` is `[0, 1048576]`.
    fn instantiate(i: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `482`
        //  Estimated: `6408`
        // Minimum execution time: 1_653_811_000 picoseconds.
        Weight::from_parts(296_038_081, 6408)
            // Standard Error: 9
            .saturating_add(Weight::from_parts(1_461, 0).saturating_mul(i.into()))
            // Standard Error: 9
            .saturating_add(Weight::from_parts(1_430, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(7_u64))
    }
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    fn call() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `759`
        //  Estimated: `6699`
        // Minimum execution time: 195_916_000 picoseconds.
        Weight::from_parts(196_706_000, 6699)
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:1 w:1)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts CodeStorage (r:0 w:1)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Contracts PristineCode (r:0 w:1)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// The range of component `c` is `[0, 61717]`.
    fn upload_code(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `109`
        //  Estimated: `3574`
        // Minimum execution time: 251_137_000 picoseconds.
        Weight::from_parts(252_985_435, 3574)
            // Standard Error: 88
            .saturating_add(Weight::from_parts(108_141, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:1 w:1)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts CodeStorage (r:0 w:1)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Contracts PristineCode (r:0 w:1)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    fn remove_code() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `255`
        //  Estimated: `3720`
        // Minimum execution time: 33_521_000 picoseconds.
        Weight::from_parts(34_039_000, 3720)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:2 w:2)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    fn set_code() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `570`
        //  Estimated: `8985`
        // Minimum execution time: 33_477_000 picoseconds.
        Weight::from_parts(33_890_000, 8985)
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(6_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_caller(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `781 + r * (6 ±0)`
        //  Estimated: `6722 + r * (6 ±0)`
        // Minimum execution time: 239_374_000 picoseconds.
        Weight::from_parts(246_017_099, 6722)
            // Standard Error: 539
            .saturating_add(Weight::from_parts(323_826, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1601 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_is_contract(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `839 + r * (240 ±0)`
        //  Estimated: `6743 + r * (2715 ±0)`
        // Minimum execution time: 240_656_000 picoseconds.
        Weight::from_parts(87_361_934, 6743)
            // Standard Error: 5_912
            .saturating_add(Weight::from_parts(3_329_840, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 2715).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1601 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_code_hash(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `831 + r * (244 ±0)`
        //  Estimated: `6747 + r * (2719 ±0)`
        // Minimum execution time: 243_026_000 picoseconds.
        Weight::from_parts(76_953_007, 6747)
            // Standard Error: 6_640
            .saturating_add(Weight::from_parts(4_132_521, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 2719).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_own_code_hash(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `788 + r * (6 ±0)`
        //  Estimated: `6730 + r * (6 ±0)`
        // Minimum execution time: 242_736_000 picoseconds.
        Weight::from_parts(243_136_007, 6730)
            // Standard Error: 912
            .saturating_add(Weight::from_parts(414_717, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_caller_is_origin(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `778 + r * (3 ±0)`
        //  Estimated: `6723 + r * (3 ±0)`
        // Minimum execution time: 240_130_000 picoseconds.
        Weight::from_parts(244_517_187, 6723)
            // Standard Error: 384
            .saturating_add(Weight::from_parts(167_431, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 3).saturating_mul(r.into()))
    }
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_caller_is_root(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `668 + r * (3 ±0)`
        //  Estimated: `6608 + r * (3 ±0)`
        // Minimum execution time: 228_022_000 picoseconds.
        Weight::from_parts(232_385_198, 6608)
            // Standard Error: 300
            .saturating_add(Weight::from_parts(145_143, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 3).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_address(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `782 + r * (6 ±0)`
        //  Estimated: `6724 + r * (6 ±0)`
        // Minimum execution time: 240_250_000 picoseconds.
        Weight::from_parts(240_268_824, 6724)
            // Standard Error: 945
            .saturating_add(Weight::from_parts(329_577, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_gas_left(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `778 + r * (6 ±0)`
        //  Estimated: `6719 + r * (6 ±0)`
        // Minimum execution time: 242_370_000 picoseconds.
        Weight::from_parts(242_389_500, 6719)
            // Standard Error: 712
            .saturating_add(Weight::from_parts(518_380, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:2 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_balance(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `922 + r * (6 ±0)`
        //  Estimated: `6846 + r * (6 ±0)`
        // Minimum execution time: 238_563_000 picoseconds.
        Weight::from_parts(253_511_314, 6846)
            // Standard Error: 1_571
            .saturating_add(Weight::from_parts(1_454_089, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_value_transferred(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `792 + r * (6 ±0)`
        //  Estimated: `6741 + r * (6 ±0)`
        // Minimum execution time: 242_995_000 picoseconds.
        Weight::from_parts(240_061_456, 6741)
            // Standard Error: 2_650
            .saturating_add(Weight::from_parts(326_813, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_minimum_balance(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `790 + r * (6 ±0)`
        //  Estimated: `6739 + r * (6 ±0)`
        // Minimum execution time: 241_342_000 picoseconds.
        Weight::from_parts(240_875_314, 6739)
            // Standard Error: 669
            .saturating_add(Weight::from_parts(324_519, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_block_number(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `787 + r * (6 ±0)`
        //  Estimated: `6737 + r * (6 ±0)`
        // Minimum execution time: 238_954_000 picoseconds.
        Weight::from_parts(242_269_896, 6737)
            // Standard Error: 1_453
            .saturating_add(Weight::from_parts(317_998, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_now(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `778 + r * (6 ±0)`
        //  Estimated: `6723 + r * (6 ±0)`
        // Minimum execution time: 240_935_000 picoseconds.
        Weight::from_parts(242_938_271, 6723)
            // Standard Error: 792
            .saturating_add(Weight::from_parts(316_782, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
    /// Proof: TransactionPayment NextFeeMultiplier (max_values: Some(1), max_size: Some(16), added: 511, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_weight_to_fee(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `852 + r * (14 ±0)`
        //  Estimated: `6785 + r * (14 ±0)`
        // Minimum execution time: 240_142_000 picoseconds.
        Weight::from_parts(241_386_730, 6785)
            // Standard Error: 2_116
            .saturating_add(Weight::from_parts(1_387_202, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 14).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_gas(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `745 + r * (4 ±0)`
        //  Estimated: `6687 + r * (4 ±0)`
        // Minimum execution time: 165_617_000 picoseconds.
        Weight::from_parts(170_794_127, 6687)
            // Standard Error: 209
            .saturating_add(Weight::from_parts(127_931, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 4).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_input(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `780 + r * (6 ±0)`
        //  Estimated: `6724 + r * (6 ±0)`
        // Minimum execution time: 238_832_000 picoseconds.
        Weight::from_parts(237_110_694, 6724)
            // Standard Error: 539
            .saturating_add(Weight::from_parts(280_610, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_input_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `784`
        //  Estimated: `6724`
        // Minimum execution time: 241_070_000 picoseconds.
        Weight::from_parts(242_162_279, 6724)
            // Standard Error: 1
            .saturating_add(Weight::from_parts(595, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1]`.
    fn seal_return(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `768 + r * (45 ±0)`
        //  Estimated: `6708 + r * (45 ±0)`
        // Minimum execution time: 236_337_000 picoseconds.
        Weight::from_parts(238_883_828, 6708)
            // Standard Error: 188_978
            .saturating_add(Weight::from_parts(926_671, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 45).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_return_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `778`
        //  Estimated: `6731`
        // Minimum execution time: 239_103_000 picoseconds.
        Weight::from_parts(240_382_910, 6731)
            // Standard Error: 0
            .saturating_add(Weight::from_parts(181, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:4 w:4)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts DeletionQueueCounter (r:1 w:1)
    /// Proof: Contracts DeletionQueueCounter (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts DeletionQueue (r:0 w:1)
    /// Proof: Contracts DeletionQueue (max_values: None, max_size: Some(142), added: 2617, mode: Measured)
    /// The range of component `r` is `[0, 1]`.
    fn seal_terminate(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `810 + r * (356 ±0)`
        //  Estimated: `6750 + r * (7781 ±0)`
        // Minimum execution time: 238_739_000 picoseconds.
        Weight::from_parts(241_041_330, 6750)
            // Standard Error: 176_820
            .saturating_add(Weight::from_parts(115_332_869, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((6_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((8_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 7781).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
    /// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_random(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `825 + r * (10 ±0)`
        //  Estimated: `6769 + r * (10 ±0)`
        // Minimum execution time: 240_888_000 picoseconds.
        Weight::from_parts(259_901_113, 6769)
            // Standard Error: 5_935
            .saturating_add(Weight::from_parts(1_764_269, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 10).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_deposit_event(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `778 + r * (10 ±0)`
        //  Estimated: `6723 + r * (10 ±0)`
        // Minimum execution time: 237_478_000 picoseconds.
        Weight::from_parts(264_915_436, 6723)
            // Standard Error: 4_644
            .saturating_add(Weight::from_parts(3_452_918, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 10).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:6 w:6)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `t` is `[0, 4]`.
    /// The range of component `n` is `[0, 16384]`.
    fn seal_deposit_event_per_topic_and_byte(t: u32, n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `797 + t * (32 ±0)`
        //  Estimated: `6744 + t * (2508 ±0)`
        // Minimum execution time: 255_720_000 picoseconds.
        Weight::from_parts(247_945_758, 6744)
            // Standard Error: 73_390
            .saturating_add(Weight::from_parts(2_483_239, 0).saturating_mul(t.into()))
            // Standard Error: 20
            .saturating_add(Weight::from_parts(756, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(t.into())))
            .saturating_add(Weight::from_parts(0, 2508).saturating_mul(t.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_debug_message(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `777 + r * (7 ±0)`
        //  Estimated: `6721 + r * (7 ±0)`
        // Minimum execution time: 172_214_000 picoseconds.
        Weight::from_parts(177_306_567, 6721)
            // Standard Error: 839
            .saturating_add(Weight::from_parts(230_558, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 7).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `i` is `[0, 1048576]`.
    fn seal_debug_message_per_byte(i: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `125728`
        //  Estimated: `131670`
        // Minimum execution time: 354_105_000 picoseconds.
        Weight::from_parts(360_649_854, 131670)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(737, 0).saturating_mul(i.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_set_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `845 + r * (292 ±0)`
        //  Estimated: `843 + r * (293 ±0)`
        // Minimum execution time: 239_637_000 picoseconds.
        Weight::from_parts(136_431_436, 843)
            // Standard Error: 10_238
            .saturating_add(Weight::from_parts(6_070_221, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 293).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_set_storage_per_new_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1304`
        //  Estimated: `1280`
        // Minimum execution time: 256_198_000 picoseconds.
        Weight::from_parts(289_972_802, 1280)
            // Standard Error: 54
            .saturating_add(Weight::from_parts(438, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(6_u64))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_set_storage_per_old_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1167 + n * (1 ±0)`
        //  Estimated: `1167 + n * (1 ±0)`
        // Minimum execution time: 255_519_000 picoseconds.
        Weight::from_parts(257_668_217, 1167)
            // Standard Error: 19
            .saturating_add(Weight::from_parts(105, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_clear_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `841 + r * (288 ±0)`
        //  Estimated: `845 + r * (289 ±0)`
        // Minimum execution time: 239_461_000 picoseconds.
        Weight::from_parts(131_630_528, 845)
            // Standard Error: 10_483
            .saturating_add(Weight::from_parts(5_910_066, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 289).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_clear_storage_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1163 + n * (1 ±0)`
        //  Estimated: `1163 + n * (1 ±0)`
        // Minimum execution time: 254_904_000 picoseconds.
        Weight::from_parts(261_213_399, 1163)
            // Standard Error: 178
            .saturating_add(Weight::from_parts(125, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_get_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `835 + r * (296 ±0)`
        //  Estimated: `840 + r * (297 ±0)`
        // Minimum execution time: 239_995_000 picoseconds.
        Weight::from_parts(151_326_508, 840)
            // Standard Error: 8_960
            .saturating_add(Weight::from_parts(4_937_728, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 297).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_get_storage_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1179 + n * (1 ±0)`
        //  Estimated: `1179 + n * (1 ±0)`
        // Minimum execution time: 254_515_000 picoseconds.
        Weight::from_parts(256_728_817, 1179)
            // Standard Error: 22
            .saturating_add(Weight::from_parts(706, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_contains_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `856 + r * (288 ±0)`
        //  Estimated: `857 + r * (289 ±0)`
        // Minimum execution time: 240_601_000 picoseconds.
        Weight::from_parts(154_476_561, 857)
            // Standard Error: 8_872
            .saturating_add(Weight::from_parts(4_805_043, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 289).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_contains_storage_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1166 + n * (1 ±0)`
        //  Estimated: `1166 + n * (1 ±0)`
        // Minimum execution time: 253_654_000 picoseconds.
        Weight::from_parts(257_288_586, 1166)
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_take_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `829 + r * (296 ±0)`
        //  Estimated: `836 + r * (297 ±0)`
        // Minimum execution time: 239_869_000 picoseconds.
        Weight::from_parts(135_258_204, 836)
            // Standard Error: 10_378
            .saturating_add(Weight::from_parts(6_144_770, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 297).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_take_storage_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1180 + n * (1 ±0)`
        //  Estimated: `1180 + n * (1 ±0)`
        // Minimum execution time: 258_153_000 picoseconds.
        Weight::from_parts(260_068_186, 1180)
            // Standard Error: 25
            .saturating_add(Weight::from_parts(744, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: System Account (r:1602 w:1601)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_transfer(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1373 + r * (45 ±0)`
        //  Estimated: `7270 + r * (2520 ±0)`
        // Minimum execution time: 243_189_000 picoseconds.
        Weight::from_parts(243_465_000, 7270)
            // Standard Error: 30_961
            .saturating_add(Weight::from_parts(35_376_623, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 2520).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:801 w:801)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:2 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:803 w:803)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_call(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1140 + r * (276 ±0)`
        //  Estimated: `9332 + r * (2752 ±0)`
        // Minimum execution time: 243_656_000 picoseconds.
        Weight::from_parts(244_221_000, 9332)
            // Standard Error: 69_762
            .saturating_add(Weight::from_parts(216_905_619, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 2752).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:736 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:737 w:737)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_delegate_call(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0 + r * (502 ±0)`
        //  Estimated: `6727 + r * (2572 ±10)`
        // Minimum execution time: 242_632_000 picoseconds.
        Weight::from_parts(243_068_000, 6727)
            // Standard Error: 126_218
            .saturating_add(Weight::from_parts(213_096_291, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 2572).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:3 w:2)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:2 w:2)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:2 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:4 w:4)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `t` is `[0, 1]`.
    /// The range of component `c` is `[0, 1048576]`.
    fn seal_call_per_transfer_clone_byte(t: u32, c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1154 + t * (204 ±0)`
        //  Estimated: `12044 + t * (5154 ±0)`
        // Minimum execution time: 421_691_000 picoseconds.
        Weight::from_parts(394_587_369, 12044)
            // Standard Error: 1_104_014
            .saturating_add(Weight::from_parts(30_461_758, 0).saturating_mul(t.into()))
            // Standard Error: 1
            .saturating_add(Weight::from_parts(601, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(10_u64))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(t.into())))
            .saturating_add(T::DbWeight::get().writes(6_u64))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(t.into())))
            .saturating_add(Weight::from_parts(0, 5154).saturating_mul(t.into()))
    }
    /// Storage: System Account (r:1602 w:1602)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:801 w:801)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:801 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:800 w:800)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:802 w:802)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[1, 800]`.
    fn seal_instantiate(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1322 + r * (254 ±0)`
        //  Estimated: `7146 + r * (5205 ±0)`
        // Minimum execution time: 581_252_000 picoseconds.
        Weight::from_parts(582_275_000, 7146)
            // Standard Error: 279_771
            .saturating_add(Weight::from_parts(349_770_967, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((6_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(6_u64))
            .saturating_add(T::DbWeight::get().writes((5_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 5205).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:4 w:4)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:2 w:2)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:2 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `t` is `[0, 1]`.
    /// The range of component `i` is `[0, 983040]`.
    /// The range of component `s` is `[0, 983040]`.
    fn seal_instantiate_per_transfer_input_salt_byte(t: u32, i: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1071 + t * (187 ±0)`
        //  Estimated: `9492 + t * (2634 ±2)`
        // Minimum execution time: 1_623_241_000 picoseconds.
        Weight::from_parts(317_076_173, 9492)
            // Standard Error: 4_549_416
            .saturating_add(Weight::from_parts(125_360_446, 0).saturating_mul(t.into()))
            // Standard Error: 7
            .saturating_add(Weight::from_parts(1_183, 0).saturating_mul(i.into()))
            // Standard Error: 7
            .saturating_add(Weight::from_parts(1_352, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(13_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t.into())))
            .saturating_add(T::DbWeight::get().writes(10_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(t.into())))
            .saturating_add(Weight::from_parts(0, 2634).saturating_mul(t.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_hash_sha2_256(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `777 + r * (8 ±0)`
        //  Estimated: `6718 + r * (8 ±0)`
        // Minimum execution time: 238_262_000 picoseconds.
        Weight::from_parts(243_093_288, 6718)
            // Standard Error: 870
            .saturating_add(Weight::from_parts(573_939, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 8).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_hash_sha2_256_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `785`
        //  Estimated: `6725`
        // Minimum execution time: 239_888_000 picoseconds.
        Weight::from_parts(242_849_333, 6725)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(3_949, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_hash_keccak_256(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `779 + r * (8 ±0)`
        //  Estimated: `6721 + r * (8 ±0)`
        // Minimum execution time: 237_288_000 picoseconds.
        Weight::from_parts(242_510_631, 6721)
            // Standard Error: 977
            .saturating_add(Weight::from_parts(742_726, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 8).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_hash_keccak_256_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `787`
        //  Estimated: `6729`
        // Minimum execution time: 240_006_000 picoseconds.
        Weight::from_parts(233_802_510, 6729)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_161, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_hash_blake2_256(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `779 + r * (8 ±0)`
        //  Estimated: `6724 + r * (8 ±0)`
        // Minimum execution time: 237_532_000 picoseconds.
        Weight::from_parts(243_087_565, 6724)
            // Standard Error: 656
            .saturating_add(Weight::from_parts(417_850, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 8).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_hash_blake2_256_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `787`
        //  Estimated: `6733`
        // Minimum execution time: 241_429_000 picoseconds.
        Weight::from_parts(233_528_258, 6733)
            // Standard Error: 1
            .saturating_add(Weight::from_parts(913, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_hash_blake2_128(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `779 + r * (8 ±0)`
        //  Estimated: `6725 + r * (8 ±0)`
        // Minimum execution time: 237_622_000 picoseconds.
        Weight::from_parts(240_476_401, 6725)
            // Standard Error: 795
            .saturating_add(Weight::from_parts(416_869, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 8).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_hash_blake2_128_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `787`
        //  Estimated: `6727`
        // Minimum execution time: 241_134_000 picoseconds.
        Weight::from_parts(234_043_271, 6727)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(919, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 125697]`.
    fn seal_sr25519_verify_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `912 + n * (1 ±0)`
        //  Estimated: `6849 + n * (1 ±0)`
        // Minimum execution time: 292_699_000 picoseconds.
        Weight::from_parts(301_523_608, 6849)
            // Standard Error: 14
            .saturating_add(Weight::from_parts(4_676, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 160]`.
    fn seal_sr25519_verify(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `727 + r * (112 ±0)`
        //  Estimated: `6666 + r * (112 ±0)`
        // Minimum execution time: 241_126_000 picoseconds.
        Weight::from_parts(248_796_458, 6666)
            // Standard Error: 21_501
            .saturating_add(Weight::from_parts(48_091_265, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 112).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 160]`.
    fn seal_ecdsa_recover(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `822 + r * (76 ±0)`
        //  Estimated: `6717 + r * (77 ±0)`
        // Minimum execution time: 242_379_000 picoseconds.
        Weight::from_parts(261_355_525, 6717)
            // Standard Error: 18_862
            .saturating_add(Weight::from_parts(37_603_073, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 77).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 160]`.
    fn seal_ecdsa_to_eth_address(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `792 + r * (42 ±0)`
        //  Estimated: `6731 + r * (42 ±0)`
        // Minimum execution time: 241_270_000 picoseconds.
        Weight::from_parts(245_135_291, 6731)
            // Standard Error: 10_757
            .saturating_add(Weight::from_parts(9_344_876, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 42).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1536 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:1536 w:1536)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:1538 w:1538)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_set_code_hash(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0 + r * (964 ±0)`
        //  Estimated: `8190 + r * (3090 ±7)`
        // Minimum execution time: 240_506_000 picoseconds.
        Weight::from_parts(241_653_000, 8190)
            // Standard Error: 46_785
            .saturating_add(Weight::from_parts(22_107_816, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 3090).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_reentrance_count(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `773 + r * (3 ±0)`
        //  Estimated: `6723 + r * (3 ±0)`
        // Minimum execution time: 241_539_000 picoseconds.
        Weight::from_parts(245_471_045, 6723)
            // Standard Error: 416
            .saturating_add(Weight::from_parts(159_577, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 3).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_account_reentrance_count(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1975 + r * (39 ±0)`
        //  Estimated: `7805 + r * (40 ±0)`
        // Minimum execution time: 242_702_000 picoseconds.
        Weight::from_parts(274_518_595, 7805)
            // Standard Error: 1_138
            .saturating_add(Weight::from_parts(256_973, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 40).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_instantiation_nonce(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `776 + r * (3 ±0)`
        //  Estimated: `6723 + r * (3 ±0)`
        // Minimum execution time: 239_360_000 picoseconds.
        Weight::from_parts(245_990_810, 6723)
            // Standard Error: 3_188
            .saturating_add(Weight::from_parts(143_408, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 3).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64const(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_617_000 picoseconds.
        Weight::from_parts(1_900_268, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(2_950, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64load(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_739_000 picoseconds.
        Weight::from_parts(2_109_373, 0)
            // Standard Error: 43
            .saturating_add(Weight::from_parts(6_586, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64store(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_726_000 picoseconds.
        Weight::from_parts(2_268_507, 0)
            // Standard Error: 4
            .saturating_add(Weight::from_parts(6_022, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_select(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_628_000 picoseconds.
        Weight::from_parts(2_042_521, 0)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(7_935, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_if(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_648_000 picoseconds.
        Weight::from_parts(1_902_691, 0)
            // Standard Error: 7
            .saturating_add(Weight::from_parts(10_572, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_br(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_626_000 picoseconds.
        Weight::from_parts(1_891_843, 0)
            // Standard Error: 14
            .saturating_add(Weight::from_parts(4_612, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_br_if(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_581_000 picoseconds.
        Weight::from_parts(1_139_823, 0)
            // Standard Error: 74
            .saturating_add(Weight::from_parts(8_008, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_br_table(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_591_000 picoseconds.
        Weight::from_parts(1_258_400, 0)
            // Standard Error: 34
            .saturating_add(Weight::from_parts(9_706, 0).saturating_mul(r.into()))
    }
    /// The range of component `e` is `[1, 256]`.
    fn instr_br_table_per_entry(e: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_701_000 picoseconds.
        Weight::from_parts(1_876_118, 0)
            // Standard Error: 23
            .saturating_add(Weight::from_parts(4, 0).saturating_mul(e.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_call(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_617_000 picoseconds.
        Weight::from_parts(1_565_613, 0)
            // Standard Error: 629
            .saturating_add(Weight::from_parts(19_575, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_call_indirect(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_875_000 picoseconds.
        Weight::from_parts(4_549_584, 0)
            // Standard Error: 278
            .saturating_add(Weight::from_parts(24_336, 0).saturating_mul(r.into()))
    }
    /// The range of component `l` is `[0, 1024]`.
    fn instr_call_per_local(l: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_742_000 picoseconds.
        Weight::from_parts(2_087_387, 0)
            // Standard Error: 26
            .saturating_add(Weight::from_parts(1_041, 0).saturating_mul(l.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_local_get(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_861_000 picoseconds.
        Weight::from_parts(3_552_428, 0)
            // Standard Error: 23
            .saturating_add(Weight::from_parts(2_339, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_local_set(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_866_000 picoseconds.
        Weight::from_parts(3_151_948, 0)
            // Standard Error: 4
            .saturating_add(Weight::from_parts(3_667, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_local_tee(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_919_000 picoseconds.
        Weight::from_parts(3_214_587, 0)
            // Standard Error: 5
            .saturating_add(Weight::from_parts(3_867, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_global_get(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_764_000 picoseconds.
        Weight::from_parts(1_815_683, 0)
            // Standard Error: 123
            .saturating_add(Weight::from_parts(8_733, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_global_set(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_783_000 picoseconds.
        Weight::from_parts(2_437_152, 0)
            // Standard Error: 13
            .saturating_add(Weight::from_parts(8_839, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_memory_current(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_745_000 picoseconds.
        Weight::from_parts(2_018_078, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_756, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 16]`.
    fn instr_memory_grow(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_648_000 picoseconds.
        Weight::from_parts(648_059, 0)
            // Standard Error: 142_299
            .saturating_add(Weight::from_parts(13_313_060, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64clz(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_652_000 picoseconds.
        Weight::from_parts(1_953_179, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_828, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64ctz(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_607_000 picoseconds.
        Weight::from_parts(1_924_759, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_762, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64popcnt(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_687_000 picoseconds.
        Weight::from_parts(1_959_683, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_754, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64eqz(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_641_000 picoseconds.
        Weight::from_parts(1_975_838, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_681, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64extendsi32(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_689_000 picoseconds.
        Weight::from_parts(1_980_109, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_880, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64extendui32(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_671_000 picoseconds.
        Weight::from_parts(1_912_089, 0)
            // Standard Error: 29
            .saturating_add(Weight::from_parts(3_896, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i32wrapi64(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_643_000 picoseconds.
        Weight::from_parts(1_951_485, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_725, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64eq(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_649_000 picoseconds.
        Weight::from_parts(1_937_598, 0)
            // Standard Error: 4
            .saturating_add(Weight::from_parts(6_045, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64ne(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_651_000 picoseconds.
        Weight::from_parts(2_202_977, 0)
            // Standard Error: 313
            .saturating_add(Weight::from_parts(6_299, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64lts(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_589_000 picoseconds.
        Weight::from_parts(1_946_304, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(6_019, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64ltu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_614_000 picoseconds.
        Weight::from_parts(1_933_375, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(6_020, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64gts(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_678_000 picoseconds.
        Weight::from_parts(2_003_850, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_816, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64gtu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_651_000 picoseconds.
        Weight::from_parts(1_971_321, 0)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(6_114, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64les(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_647_000 picoseconds.
        Weight::from_parts(2_017_232, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_990, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64leu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_635_000 picoseconds.
        Weight::from_parts(3_232_848, 0)
            // Standard Error: 105
            .saturating_add(Weight::from_parts(5_816, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64ges(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_623_000 picoseconds.
        Weight::from_parts(1_996_165, 0)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(5_964, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64geu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_668_000 picoseconds.
        Weight::from_parts(1_973_238, 0)
            // Standard Error: 20
            .saturating_add(Weight::from_parts(6_021, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64add(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_674_000 picoseconds.
        Weight::from_parts(1_981_762, 0)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(5_898, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64sub(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_632_000 picoseconds.
        Weight::from_parts(1_935_700, 0)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(6_154, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64mul(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_607_000 picoseconds.
        Weight::from_parts(1_942_734, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_797, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64divs(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_611_000 picoseconds.
        Weight::from_parts(2_960_454, 0)
            // Standard Error: 177
            .saturating_add(Weight::from_parts(11_666, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64divu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_641_000 picoseconds.
        Weight::from_parts(2_104_200, 0)
            // Standard Error: 5
            .saturating_add(Weight::from_parts(10_540, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64rems(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_643_000 picoseconds.
        Weight::from_parts(2_602_908, 0)
            // Standard Error: 24
            .saturating_add(Weight::from_parts(11_900, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64remu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_584_000 picoseconds.
        Weight::from_parts(2_056_817, 0)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(10_722, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64and(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_652_000 picoseconds.
        Weight::from_parts(1_988_892, 0)
            // Standard Error: 4
            .saturating_add(Weight::from_parts(5_683, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64or(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_660_000 picoseconds.
        Weight::from_parts(2_148_537, 0)
            // Standard Error: 38
            .saturating_add(Weight::from_parts(5_756, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64xor(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_629_000 picoseconds.
        Weight::from_parts(1_955_010, 0)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(5_931, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64shl(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_569_000 picoseconds.
        Weight::from_parts(1_982_403, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_867, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64shrs(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_615_000 picoseconds.
        Weight::from_parts(1_989_920, 0)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(6_137, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64shru(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_646_000 picoseconds.
        Weight::from_parts(2_020_935, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_863, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64rotl(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_661_000 picoseconds.
        Weight::from_parts(2_320_710, 0)
            // Standard Error: 27
            .saturating_add(Weight::from_parts(5_922, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64rotr(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_674_000 picoseconds.
        Weight::from_parts(2_044_188, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_855, 0).saturating_mul(r.into()))
    }
}
