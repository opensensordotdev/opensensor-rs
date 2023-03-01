thread 'arrow::round_trip_parquet' panicked at 'called `Result::unwrap()` on an `Err` value: OutOfSpec("The children must have an equal number of values.\n                         However, the values at index 1 have a length of 9, which is different from values at index 0, 6.")', /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/arrow2-0.14.2/src/array/struct_/mod.rs:120:52
stack backtrace:
   0: rust_begin_unwind
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panicking.rs:575:5
   1: core::panicking::panic_fmt
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/panicking.rs:64:14
   2: core::result::unwrap_failed
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/result.rs:1791:5
   3: core::result::Result<T,E>::unwrap
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/result.rs:1113:23
   4: arrow2::array::struct_::StructArray::new
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/arrow2-0.14.2/src/array/struct_/mod.rs:120:9
   5: arrow2::array::struct_::StructArray::from_data
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/arrow2-0.14.2/src/array/struct_/mod.rs:129:9
   6: <arrow2::io::parquet::read::deserialize::struct_::StructIterator as core::iter::traits::iterator::Iterator>::next
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/arrow2-0.14.2/src/io/parquet/read/deserialize/struct_.rs:50:22
   7: <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/boxed.rs:1923:9
   8: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::next
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/adapters/map.rs:103:9
   9: <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/boxed.rs:1923:9
  10: <arrow2::io::parquet::read::deserialize::struct_::StructIterator as core::iter::traits::iterator::Iterator>::next::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/arrow2-0.14.2/src/io/parquet/read/deserialize/struct_.rs:26:25
  11: core::iter::adapters::map::map_fold::{{closure}}
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/adapters/map.rs:84:28
  12: core::iter::traits::iterator::Iterator::fold
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/traits/iterator.rs:2414:21
  13: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/adapters/map.rs:124:9
  14: core::iter::traits::iterator::Iterator::for_each
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/traits/iterator.rs:831:9
  15: alloc::vec::Vec<T,A>::extend_trusted
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/mod.rs:2880:17
  16: <alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/spec_extend.rs:26:9
  17: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/spec_from_iter_nested.rs:62:9
  18: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/spec_from_iter.rs:33:9
  19: <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/mod.rs:2748:9
  20: core::iter::traits::iterator::Iterator::collect
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/traits/iterator.rs:1836:9
  21: <arrow2::io::parquet::read::deserialize::struct_::StructIterator as core::iter::traits::iterator::Iterator>::next
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/arrow2-0.14.2/src/io/parquet/read/deserialize/struct_.rs:23:22
  22: <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/boxed.rs:1923:9
  23: <arrow2::io::parquet::read::deserialize::struct_::StructIterator as core::iter::traits::iterator::Iterator>::next::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/arrow2-0.14.2/src/io/parquet/read/deserialize/struct_.rs:26:25
  24: core::iter::adapters::map::map_fold::{{closure}}
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/adapters/map.rs:84:28
  25: core::iter::traits::iterator::Iterator::fold
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/traits/iterator.rs:2414:21
  26: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/adapters/map.rs:124:9
  27: core::iter::traits::iterator::Iterator::for_each
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/traits/iterator.rs:831:9
  28: alloc::vec::Vec<T,A>::extend_trusted
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/mod.rs:2880:17
  29: <alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/spec_extend.rs:26:9
  30: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/spec_from_iter_nested.rs:62:9
  31: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/spec_from_iter.rs:33:9
  32: <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/mod.rs:2748:9
  33: core::iter::traits::iterator::Iterator::collect
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/traits/iterator.rs:1836:9
  34: <arrow2::io::parquet::read::deserialize::struct_::StructIterator as core::iter::traits::iterator::Iterator>::next
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/arrow2-0.14.2/src/io/parquet/read/deserialize/struct_.rs:23:22
  35: <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/boxed.rs:1923:9
  36: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::next
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/adapters/map.rs:103:9
  37: <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/boxed.rs:1923:9
  38: <arrow2::io::parquet::read::row_group::RowGroupDeserializer as core::iter::traits::iterator::Iterator>::next::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/arrow2-0.14.2/src/io/parquet/read/row_group.rs:69:25
  39: core::iter::adapters::map::map_try_fold::{{closure}}
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/adapters/map.rs:91:28
  40: core::iter::traits::iterator::Iterator::try_fold
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/traits/iterator.rs:2238:21
  41: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/adapters/map.rs:117:9
  42: <core::iter::adapters::GenericShunt<I,R> as core::iter::traits::iterator::Iterator>::try_fold
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/adapters/mod.rs:195:9
  43: core::iter::traits::iterator::Iterator::try_for_each
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/traits/iterator.rs:2299:9
  44: <core::iter::adapters::GenericShunt<I,R> as core::iter::traits::iterator::Iterator>::next
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/adapters/mod.rs:178:9
  45: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/spec_from_iter_nested.rs:26:32
  46: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/spec_from_iter.rs:33:9
  47: <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/mod.rs:2748:9
  48: core::iter::traits::iterator::Iterator::collect
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/traits/iterator.rs:1836:9
  49: <core::result::Result<V,E> as core::iter::traits::collect::FromIterator<core::result::Result<A,E>>>::from_iter::{{closure}}
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/result.rs:2075:49
  50: core::iter::adapters::try_process
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/adapters/mod.rs:164:17
  51: <core::result::Result<V,E> as core::iter::traits::collect::FromIterator<core::result::Result<A,E>>>::from_iter
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/result.rs:2075:9
  52: core::iter::traits::iterator::Iterator::collect
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/traits/iterator.rs:1836:9
  53: <arrow2::io::parquet::read::row_group::RowGroupDeserializer as core::iter::traits::iterator::Iterator>::next
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/arrow2-0.14.2/src/io/parquet/read/row_group.rs:66:21
  54: <arrow2::io::parquet::read::file::FileReader<R> as core::iter::traits::iterator::Iterator>::next
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/arrow2-0.14.2/src/io/parquet/read/file.rs:77:19
  55: <arrow2::io::parquet::read::file::FileReader<R> as core::iter::traits::iterator::Iterator>::next
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/arrow2-0.14.2/src/io/parquet/read/file.rs:97:21
  56: parquet_round_trip::arrow::round_trip_parquet
             at ./src/arrow.rs:324:24
  57: parquet_round_trip::arrow::round_trip_parquet::{{closure}}
             at ./src/arrow.rs:260:28
  58: core::ops::function::FnOnce::call_once
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/ops/function.rs:507:5
  59: core::ops::function::FnOnce::call_once
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/ops/function.rs:507:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test arrow::round_trip_parquet ... FAILED

failures:

failures:
    arrow::round_trip_parquet