// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! rustc compiler intrinsics.
//!
//! The corresponding definitions are in librustc_trans/intrinsic.rs.
//!
//! # Volatiles
//!
//! The volatile intrinsics provide operations intended to act on I/O
//! memory, which are guaranteed to not be reordered by the compiler
//! across other volatile intrinsics. See the LLVM documentation on
//! [[volatile]].
//!
//! [volatile]: http://llvm.org/docs/LangRef.html#volatile-memory-accesses
//!
//! # Atomics
//!
//! The atomic intrinsics provide common atomic operations on machine
//! words, with multiple possible memory orderings. They obey the same
//! semantics as C++11. See the LLVM documentation on [[atomics]].
//!
//! [atomics]: http://llvm.org/docs/Atomics.html
//!
//! A quick refresher on memory ordering:
//!
//! * Acquire - a barrier for acquiring a lock. Subsequent reads and writes
//!   take place after the barrier.
//! * Release - a barrier for releasing a lock. Preceding reads and writes
//!   take place before the barrier.
//! * Sequentially consistent - sequentially consistent operations are
//!   guaranteed to happen in order. This is the standard mode for working
//!   with atomic types and is equivalent to Java's `volatile`.

#![unstable(feature = "core_intrinsics",
            reason = "intrinsics are unlikely to ever be stabilized, instead \
                      they should be used through stabilized interfaces \
                      in the rest of the standard library",
            issue = "0")]
#![allow(missing_docs)]

extern "rust-intrinsic" {

    // NB: These intrinsics take raw pointers because they mutate aliased
    // memory, which is not valid for either `&` or `&mut`.

    pub fn atomic_cxchg<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_acq<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_rel<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_acqrel<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_relaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_failrelaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_failacq<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_acq_failrelaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_acqrel_failrelaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);

    pub fn atomic_cxchgweak<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_acq<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_rel<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_acqrel<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_relaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_failrelaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_failacq<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_acq_failrelaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_acqrel_failrelaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);

    pub fn atomic_load<T>(src: *const T) -> T;
    pub fn atomic_load_acq<T>(src: *const T) -> T;
    pub fn atomic_load_relaxed<T>(src: *const T) -> T;
    pub fn atomic_load_unordered<T>(src: *const T) -> T;

    pub fn atomic_store<T>(dst: *mut T, val: T);
    pub fn atomic_store_rel<T>(dst: *mut T, val: T);
    pub fn atomic_store_relaxed<T>(dst: *mut T, val: T);
    pub fn atomic_store_unordered<T>(dst: *mut T, val: T);

    pub fn atomic_xchg<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xchg_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xchg_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xchg_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xchg_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_xadd<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xadd_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xadd_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xadd_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xadd_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_xsub<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xsub_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xsub_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xsub_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xsub_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_and<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_and_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_and_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_and_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_and_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_nand<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_nand_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_nand_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_nand_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_nand_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_or<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_or_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_or_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_or_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_or_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_xor<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xor_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xor_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xor_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xor_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_max<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_max_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_max_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_max_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_max_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_min<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_min_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_min_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_min_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_min_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_umin<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umin_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umin_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umin_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umin_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_umax<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umax_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umax_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umax_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umax_relaxed<T>(dst: *mut T, src: T) -> T;
}

extern "rust-intrinsic" {

    pub fn atomic_fence();
    pub fn atomic_fence_acq();
    pub fn atomic_fence_rel();
    pub fn atomic_fence_acqrel();

    /// A compiler-only memory barrier.
    ///
    /// Memory accesses will never be reordered across this barrier by the
    /// compiler, but no instructions will be emitted for it. This is
    /// appropriate for operations on the same thread that may be preempted,
    /// such as when interacting with signal handlers.
    pub fn atomic_singlethreadfence();
    pub fn atomic_singlethreadfence_acq();
    pub fn atomic_singlethreadfence_rel();
    pub fn atomic_singlethreadfence_acqrel();

    /// Magic intrinsic that derives its meaning from attributes
    /// attached to the function.
    ///
    /// For example, dataflow uses this to inject static assertions so
    /// that `rustc_peek(potentially_uninitialized)` would actually
    /// double-check that dataflow did indeed compute that it is
    /// uninitialized at that point in the control flow.
    pub fn rustc_peek<T>(_: T) -> T;

    /// Aborts the execution of the process.
    pub fn abort() -> !;

    /// Tells LLVM that this point in the code is not reachable,
    /// enabling further optimizations.
    ///
    /// NB: This is very different from the `unreachable!()` macro!
    pub fn unreachable() -> !;

    /// Informs the optimizer that a condition is always true.
    /// If the condition is false, the behavior is undefined.
    ///
    /// No code is generated for this intrinsic, but the optimizer will try
    /// to preserve it (and its condition) between passes, which may interfere
    /// with optimization of surrounding code and reduce performance. It should
    /// not be used if the invariant can be discovered by the optimizer on its
    /// own, or if it does not enable any significant optimizations.
    pub fn assume(b: bool);

    /// Executes a breakpoint trap, for inspection by a debugger.
    pub fn breakpoint();

    /// The size of a type in bytes.
    ///
    /// More specifically, this is the offset in bytes between successive
    /// items of the same type, including alignment padding.
    pub fn size_of<T>() -> usize;

    /// Moves a value to an uninitialized memory location.
    ///
    /// Drop glue is not run on the destination.
    pub fn move_val_init<T>(dst: *mut T, src: T);

    pub fn min_align_of<T>() -> usize;
    pub fn pref_align_of<T>() -> usize;

    pub fn size_of_val<T: ?Sized>(_: &T) -> usize;
    pub fn min_align_of_val<T: ?Sized>(_: &T) -> usize;

    /// Executes the destructor (if any) of the pointed-to value.
    ///
    /// This has two use cases:
    ///
    /// * It is *required* to use `drop_in_place` to drop unsized types like
    ///   trait objects, because they can't be read out onto the stack and
    ///   dropped normally.
    ///
    /// * It is friendlier to the optimizer to do this over `ptr::read` when
    ///   dropping manually allocated memory (e.g. when writing Box/Rc/Vec),
    ///   as the compiler doesn't need to prove that it's sound to elide the
    ///   copy.
    ///
    /// # Undefined Behavior
    ///
    /// This has all the same safety problems as `ptr::read` with respect to
    /// invalid pointers, types, and double drops.
    #[stable(feature = "drop_in_place", since = "1.8.0")]
    pub fn drop_in_place<T: ?Sized>(to_drop: *mut T);

    /// Gets a static string slice containing the name of a type.
    pub fn type_name<T: ?Sized>() -> &'static str;

    /// Gets an identifier which is globally unique to the specified type. This
    /// function will return the same value for a type regardless of whichever
    /// crate it is invoked in.
    pub fn type_id<T: ?Sized + 'static>() -> u64;

    /// Creates a value initialized to zero.
    ///
    /// `init` is unsafe because it returns a zeroed-out datum,
    /// which is unsafe unless T is `Copy`.  Also, even if T is
    /// `Copy`, an all-zero value may not correspond to any legitimate
    /// state for the type in question.
    pub fn init<T>() -> T;

    /// Creates an uninitialized value.
    ///
    /// `uninit` is unsafe because there is no guarantee of what its
    /// contents are. In particular its drop-flag may be set to any
    /// state, which means it may claim either dropped or
    /// undropped. In the general case one must use `ptr::write` to
    /// initialize memory previous set to the result of `uninit`.
    pub fn uninit<T>() -> T;

    /// Moves a value out of scope without running drop glue.
    pub fn forget<T>(_: T) -> ();

    /// Reinterprets the bits of a value of one type as another type; both types
    /// must have the same size. Neither the original, nor the result, may be an
    /// [invalid value] (../../nomicon/meet-safe-and-unsafe.html).
    ///
    /// `transmute` is semantically equivalent to a bitwise move of one type
    /// into another. It copies the bits from the destination type into the
    /// source type, then forgets the original. It's equivalent to C's `memcpy`
    /// under the hood, just like `transmute_copy`.
    ///
    /// `transmute` is incredibly unsafe. There are a vast number of ways to
    /// cause undefined behavior with this function. `transmute` should be
    /// the absolute last resort.
    ///
    /// The [nomicon](../../nomicon/transmutes.html) has additional
    /// documentation.
    ///
    /// # Examples
    ///
    /// There are a few things that `transmute` is really useful for.
    ///
    /// Getting the bitpattern of a floating point type (or, more generally,
    /// type punning, when `T` and `U` aren't pointers):
    ///
    /// ```
    /// let bitpattern = unsafe {
    ///     std::mem::transmute::<f32, u32>(1.0)
    /// };
    /// assert_eq!(bitpattern, 0x3F800000);
    /// ```
    ///
    /// Turning a pointer into a function pointer:
    ///
    /// ```
    /// fn foo() -> i32 {
    ///     0
    /// }
    /// let pointer = foo as *const ();
    /// let function = unsafe {
    ///     std::mem::transmute::<*const (), fn() -> i32>(pointer)
    /// };
    /// assert_eq!(function(), 0);
    /// ```
    ///
    /// Extending a lifetime, or shortening an invariant lifetime; this is
    /// advanced, very unsafe rust:
    ///
    /// ```
    /// struct R<'a>(&'a i32);
    /// unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    ///     std::mem::transmute::<R<'b>, R<'static>>(r)
    /// }
    ///
    /// unsafe fn shorten_invariant_lifetime<'b, 'c>(r: &'b mut R<'static>)
    ///                                              -> &'b mut R<'c> {
    ///     std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)
    /// }
    /// ```
    ///
    /// # Alternatives
    ///
    /// However, many uses of `transmute` can be achieved through other means.
    /// `transmute` can transform any type into any other, with just the caveat
    /// that they're the same size, and often interesting results occur. Below
    /// are common applications of `transmute` which can be replaced with safe
    /// applications of `as`:
    ///
    /// Turning a pointer into a `usize`:
    ///
    /// ```
    /// let ptr = &0;
    /// let ptr_num_transmute = unsafe {
    ///     std::mem::transmute::<&i32, usize>(ptr)
    /// };
    /// // Use an `as` cast instead
    /// let ptr_num_cast = ptr as *const i32 as usize;
    /// ```
    ///
    /// Turning a `*mut T` into an `&mut T`:
    ///
    /// ```
    /// let ptr: *mut i32 = &mut 0;
    /// let ref_transmuted = unsafe {
    ///     std::mem::transmute::<*mut i32, &mut i32>(ptr)
    /// };
    /// // Use a reborrow instead
    /// let ref_casted = unsafe { &mut *ptr };
    /// ```
    ///
    /// Turning an `&mut T` into an `&mut U`:
    ///
    /// ```
    /// let ptr = &mut 0;
    /// let val_transmuted = unsafe {
    ///     std::mem::transmute::<&mut i32, &mut u32>(ptr)
    /// };
    /// // Now, put together `as` and reborrowing - note the chaining of `as`
    /// // `as` is not transitive
    /// let val_casts = unsafe { &mut *(ptr as *mut i32 as *mut u32) };
    /// ```
    ///
    /// Turning an `&str` into an `&[u8]`:
    ///
    /// ```
    /// // this is not a good way to do this.
    /// let slice = unsafe { std::mem::transmute::<&str, &[u8]>("Rust") };
    /// assert_eq!(slice, &[82, 117, 115, 116]);
    /// // You could use `str::as_bytes`
    /// let slice = "Rust".as_bytes();
    /// assert_eq!(slice, &[82, 117, 115, 116]);
    /// // Or, just use a byte string, if you have control over the string
    /// // literal
    /// assert_eq!(b"Rust", &[82, 117, 115, 116]);
    /// ```
    ///
    /// Turning a `Vec<&T>` into a `Vec<Option<&T>>`:
    ///
    /// ```
    /// let store = [0, 1, 2, 3];
    /// let mut v_orig = store.iter().collect::<Vec<&i32>>();
    /// // Using transmute: this is Undefined Behavior, and a bad idea.
    /// // However, it is no-copy.
    /// let v_transmuted = unsafe {
    ///     std::mem::transmute::<Vec<&i32>, Vec<Option<&i32>>>(
    ///         v_orig.clone())
    /// };
    /// // This is the suggested, safe way.
    /// // It does copy the entire Vector, though, into a new array.
    /// let v_collected = v_orig.clone()
    ///                         .into_iter()
    ///                         .map(|r| Some(r))
    ///                         .collect::<Vec<Option<&i32>>>();
    /// // The no-copy, unsafe way, still using transmute, but not UB.
    /// // This is equivalent to the original, but safer, and reuses the
    /// // same Vec internals. Therefore the new inner type must have the
    /// // exact same size, and the same or lesser alignment, as the old
    /// // type. The same caveats exist for this method as transmute, for
    /// // the original inner type (`&i32`) to the converted inner type
    /// // (`Option<&i32>`), so read the nomicon pages linked above.
    /// let v_from_raw = unsafe {
    ///     Vec::from_raw_parts(v_orig.as_mut_ptr(),
    ///                         v_orig.len(),
    ///                         v_orig.capacity())
    /// };
    /// std::mem::forget(v_orig);
    /// ```
    ///
    /// Implementing `split_at_mut`:
    ///
    /// ```
    /// use std::{slice, mem};
    /// // There are multiple ways to do this; and there are multiple problems
    /// // with the following, transmute, way.
    /// fn split_at_mut_transmute<T>(slice: &mut [T], mid: usize)
    ///                              -> (&mut [T], &mut [T]) {
    ///     let len = slice.len();
    ///     assert!(mid <= len);
    ///     unsafe {
    ///         let slice2 = mem::transmute::<&mut [T], &mut [T]>(slice);
    ///         // first: transmute is not typesafe; all it checks is that T and
    ///         // U are of the same size. Second, right here, you have two
    ///         // mutable references pointing to the same memory.
    ///         (&mut slice[0..mid], &mut slice2[mid..len])
    ///     }
    /// }
    /// // This gets rid of the typesafety problems; `&mut *` will *only* give
    /// // you an `&mut T` from an `&mut T` or `*mut T`.
    /// fn split_at_mut_casts<T>(slice: &mut [T], mid: usize)
    ///                          -> (&mut [T], &mut [T]) {
    ///     let len = slice.len();
    ///     assert!(mid <= len);
    ///     unsafe {
    ///         let slice2 = &mut *(slice as *mut [T]);
    ///         // however, you still have two mutable references pointing to
    ///         // the same memory.
    ///         (&mut slice[0..mid], &mut slice2[mid..len])
    ///     }
    /// }
    /// // This is how the standard library does it. This is the best method, if
    /// // you need to do something like this
    /// fn split_at_stdlib<T>(slice: &mut [T], mid: usize)
    ///                       -> (&mut [T], &mut [T]) {
    ///     let len = slice.len();
    ///     assert!(mid <= len);
    ///     unsafe {
    ///         let ptr = slice.as_mut_ptr();
    ///         // This now has three mutable references pointing at the same
    ///         // memory. `slice`, the rvalue ret.0, and the rvalue ret.1.
    ///         // `slice` is never used after `let ptr = ...`, and so one can
    ///         // treat it as "dead", and therefore, you only have two real
    ///         // mutable slices.
    ///         (slice::from_raw_parts_mut(ptr, mid),
    ///          slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    ///     }
    /// }
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn transmute<T, U>(e: T) -> U;

    /// Returns `true` if the actual type given as `T` requires drop
    /// glue; returns `false` if the actual type provided for `T`
    /// implements `Copy`.
    ///
    /// If the actual type neither requires drop glue nor implements
    /// `Copy`, then may return `true` or `false`.
    pub fn needs_drop<T>() -> bool;

    /// Calculates the offset from a pointer.
    ///
    /// This is implemented as an intrinsic to avoid converting to and from an
    /// integer, since the conversion would throw away aliasing information.
    ///
    /// # Safety
    ///
    /// Both the starting and resulting pointer must be either in bounds or one
    /// byte past the end of an allocated object. If either pointer is out of
    /// bounds or arithmetic overflow occurs then any further use of the
    /// returned value will result in undefined behavior.
    pub fn offset<T>(dst: *const T, offset: isize) -> *const T;

    /// Calculates the offset from a pointer, potentially wrapping.
    ///
    /// This is implemented as an intrinsic to avoid converting to and from an
    /// integer, since the conversion inhibits certain optimizations.
    ///
    /// # Safety
    ///
    /// Unlike the `offset` intrinsic, this intrinsic does not restrict the
    /// resulting pointer to point into or one byte past the end of an allocated
    /// object, and it wraps with two's complement arithmetic. The resulting
    /// value is not necessarily valid to be used to actually access memory.
    pub fn arith_offset<T>(dst: *const T, offset: isize) -> *const T;

    /// Copies `count * size_of<T>` bytes from `src` to `dst`. The source
    /// and destination may *not* overlap.
    ///
    /// `copy_nonoverlapping` is semantically equivalent to C's `memcpy`.
    ///
    /// # Safety
    ///
    /// Beyond requiring that the program must be allowed to access both regions
    /// of memory, it is Undefined Behavior for source and destination to
    /// overlap. Care must also be taken with the ownership of `src` and
    /// `dst`. This method semantically moves the values of `src` into `dst`.
    /// However it does not drop the contents of `dst`, or prevent the contents
    /// of `src` from being dropped or used.
    ///
    /// # Examples
    ///
    /// A safe swap function:
    ///
    /// ```
    /// use std::mem;
    /// use std::ptr;
    ///
    /// # #[allow(dead_code)]
    /// fn swap<T>(x: &mut T, y: &mut T) {
    ///     unsafe {
    ///         // Give ourselves some scratch space to work with
    ///         let mut t: T = mem::uninitialized();
    ///
    ///         // Perform the swap, `&mut` pointers never alias
    ///         ptr::copy_nonoverlapping(x, &mut t, 1);
    ///         ptr::copy_nonoverlapping(y, x, 1);
    ///         ptr::copy_nonoverlapping(&t, y, 1);
    ///
    ///         // y and t now point to the same thing, but we need to completely forget `tmp`
    ///         // because it's no longer relevant.
    ///         mem::forget(t);
    ///     }
    /// }
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize);

    /// Copies `count * size_of<T>` bytes from `src` to `dst`. The source
    /// and destination may overlap.
    ///
    /// `copy` is semantically equivalent to C's `memmove`.
    ///
    /// # Safety
    ///
    /// Care must be taken with the ownership of `src` and `dst`.
    /// This method semantically moves the values of `src` into `dst`.
    /// However it does not drop the contents of `dst`, or prevent the contents of `src`
    /// from being dropped or used.
    ///
    /// # Examples
    ///
    /// Efficiently create a Rust vector from an unsafe buffer:
    ///
    /// ```
    /// use std::ptr;
    ///
    /// # #[allow(dead_code)]
    /// unsafe fn from_buf_raw<T>(ptr: *const T, elts: usize) -> Vec<T> {
    ///     let mut dst = Vec::with_capacity(elts);
    ///     dst.set_len(elts);
    ///     ptr::copy(ptr, dst.as_mut_ptr(), elts);
    ///     dst
    /// }
    /// ```
    ///
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn copy<T>(src: *const T, dst: *mut T, count: usize);

    /// Invokes memset on the specified pointer, setting `count * size_of::<T>()`
    /// bytes of memory starting at `dst` to `val`.
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn write_bytes<T>(dst: *mut T, val: u8, count: usize);

    /// Equivalent to the appropriate `llvm.memcpy.p0i8.0i8.*` intrinsic, with
    /// a size of `count` * `size_of::<T>()` and an alignment of
    /// `min_align_of::<T>()`
    ///
    /// The volatile parameter is set to `true`, so it will not be optimized out.
    pub fn volatile_copy_nonoverlapping_memory<T>(dst: *mut T, src: *const T,
                                                  count: usize);
    /// Equivalent to the appropriate `llvm.memmove.p0i8.0i8.*` intrinsic, with
    /// a size of `count` * `size_of::<T>()` and an alignment of
    /// `min_align_of::<T>()`
    ///
    /// The volatile parameter is set to `true`, so it will not be optimized out.
    pub fn volatile_copy_memory<T>(dst: *mut T, src: *const T, count: usize);
    /// Equivalent to the appropriate `llvm.memset.p0i8.*` intrinsic, with a
    /// size of `count` * `size_of::<T>()` and an alignment of
    /// `min_align_of::<T>()`.
    ///
    /// The volatile parameter is set to `true`, so it will not be optimized out.
    pub fn volatile_set_memory<T>(dst: *mut T, val: u8, count: usize);

    /// Perform a volatile load from the `src` pointer.
    pub fn volatile_load<T>(src: *const T) -> T;
    /// Perform a volatile store to the `dst` pointer.
    pub fn volatile_store<T>(dst: *mut T, val: T);

    /// Returns the square root of an `f32`
    pub fn sqrtf32(x: f32) -> f32;
    /// Returns the square root of an `f64`
    pub fn sqrtf64(x: f64) -> f64;

    /// Raises an `f32` to an integer power.
    pub fn powif32(a: f32, x: i32) -> f32;
    /// Raises an `f64` to an integer power.
    pub fn powif64(a: f64, x: i32) -> f64;

    /// Returns the sine of an `f32`.
    pub fn sinf32(x: f32) -> f32;
    /// Returns the sine of an `f64`.
    pub fn sinf64(x: f64) -> f64;

    /// Returns the cosine of an `f32`.
    pub fn cosf32(x: f32) -> f32;
    /// Returns the cosine of an `f64`.
    pub fn cosf64(x: f64) -> f64;

    /// Raises an `f32` to an `f32` power.
    pub fn powf32(a: f32, x: f32) -> f32;
    /// Raises an `f64` to an `f64` power.
    pub fn powf64(a: f64, x: f64) -> f64;

    /// Returns the exponential of an `f32`.
    pub fn expf32(x: f32) -> f32;
    /// Returns the exponential of an `f64`.
    pub fn expf64(x: f64) -> f64;

    /// Returns 2 raised to the power of an `f32`.
    pub fn exp2f32(x: f32) -> f32;
    /// Returns 2 raised to the power of an `f64`.
    pub fn exp2f64(x: f64) -> f64;

    /// Returns the natural logarithm of an `f32`.
    pub fn logf32(x: f32) -> f32;
    /// Returns the natural logarithm of an `f64`.
    pub fn logf64(x: f64) -> f64;

    /// Returns the base 10 logarithm of an `f32`.
    pub fn log10f32(x: f32) -> f32;
    /// Returns the base 10 logarithm of an `f64`.
    pub fn log10f64(x: f64) -> f64;

    /// Returns the base 2 logarithm of an `f32`.
    pub fn log2f32(x: f32) -> f32;
    /// Returns the base 2 logarithm of an `f64`.
    pub fn log2f64(x: f64) -> f64;

    /// Returns `a * b + c` for `f32` values.
    pub fn fmaf32(a: f32, b: f32, c: f32) -> f32;
    /// Returns `a * b + c` for `f64` values.
    pub fn fmaf64(a: f64, b: f64, c: f64) -> f64;

    /// Returns the absolute value of an `f32`.
    pub fn fabsf32(x: f32) -> f32;
    /// Returns the absolute value of an `f64`.
    pub fn fabsf64(x: f64) -> f64;

    /// Copies the sign from `y` to `x` for `f32` values.
    pub fn copysignf32(x: f32, y: f32) -> f32;
    /// Copies the sign from `y` to `x` for `f64` values.
    pub fn copysignf64(x: f64, y: f64) -> f64;

    /// Returns the largest integer less than or equal to an `f32`.
    pub fn floorf32(x: f32) -> f32;
    /// Returns the largest integer less than or equal to an `f64`.
    pub fn floorf64(x: f64) -> f64;

    /// Returns the smallest integer greater than or equal to an `f32`.
    pub fn ceilf32(x: f32) -> f32;
    /// Returns the smallest integer greater than or equal to an `f64`.
    pub fn ceilf64(x: f64) -> f64;

    /// Returns the integer part of an `f32`.
    pub fn truncf32(x: f32) -> f32;
    /// Returns the integer part of an `f64`.
    pub fn truncf64(x: f64) -> f64;

    /// Returns the nearest integer to an `f32`. May raise an inexact floating-point exception
    /// if the argument is not an integer.
    pub fn rintf32(x: f32) -> f32;
    /// Returns the nearest integer to an `f64`. May raise an inexact floating-point exception
    /// if the argument is not an integer.
    pub fn rintf64(x: f64) -> f64;

    /// Returns the nearest integer to an `f32`.
    pub fn nearbyintf32(x: f32) -> f32;
    /// Returns the nearest integer to an `f64`.
    pub fn nearbyintf64(x: f64) -> f64;

    /// Returns the nearest integer to an `f32`. Rounds half-way cases away from zero.
    pub fn roundf32(x: f32) -> f32;
    /// Returns the nearest integer to an `f64`. Rounds half-way cases away from zero.
    pub fn roundf64(x: f64) -> f64;

    /// Float addition that allows optimizations based on algebraic rules.
    /// May assume inputs are finite.
    pub fn fadd_fast<T>(a: T, b: T) -> T;

    /// Float subtraction that allows optimizations based on algebraic rules.
    /// May assume inputs are finite.
    pub fn fsub_fast<T>(a: T, b: T) -> T;

    /// Float multiplication that allows optimizations based on algebraic rules.
    /// May assume inputs are finite.
    pub fn fmul_fast<T>(a: T, b: T) -> T;

    /// Float division that allows optimizations based on algebraic rules.
    /// May assume inputs are finite.
    pub fn fdiv_fast<T>(a: T, b: T) -> T;

    /// Float remainder that allows optimizations based on algebraic rules.
    /// May assume inputs are finite.
    pub fn frem_fast<T>(a: T, b: T) -> T;


    /// Returns the number of bits set in an integer type `T`
    pub fn ctpop<T>(x: T) -> T;

    /// Returns the number of leading bits unset in an integer type `T`
    pub fn ctlz<T>(x: T) -> T;

    /// Returns the number of trailing bits unset in an integer type `T`
    pub fn cttz<T>(x: T) -> T;

    /// Reverses the bytes in an integer type `T`.
    pub fn bswap<T>(x: T) -> T;

    /// Performs checked integer addition.
    pub fn add_with_overflow<T>(x: T, y: T) -> (T, bool);

    /// Performs checked integer subtraction
    pub fn sub_with_overflow<T>(x: T, y: T) -> (T, bool);

    /// Performs checked integer multiplication
    pub fn mul_with_overflow<T>(x: T, y: T) -> (T, bool);

    /// Performs an unchecked division, resulting in undefined behavior
    /// where y = 0 or x = `T::min_value()` and y = -1
    pub fn unchecked_div<T>(x: T, y: T) -> T;
    /// Returns the remainder of an unchecked division, resulting in
    /// undefined behavior where y = 0 or x = `T::min_value()` and y = -1
    pub fn unchecked_rem<T>(x: T, y: T) -> T;

    /// Returns (a + b) mod 2^N, where N is the width of T in bits.
    pub fn overflowing_add<T>(a: T, b: T) -> T;
    /// Returns (a - b) mod 2^N, where N is the width of T in bits.
    pub fn overflowing_sub<T>(a: T, b: T) -> T;
    /// Returns (a * b) mod 2^N, where N is the width of T in bits.
    pub fn overflowing_mul<T>(a: T, b: T) -> T;

    /// Returns the value of the discriminant for the variant in 'v',
    /// cast to a `u64`; if `T` has no discriminant, returns 0.
    pub fn discriminant_value<T>(v: &T) -> u64;

    /// Rust's "try catch" construct which invokes the function pointer `f` with
    /// the data pointer `data`.
    ///
    /// The third pointer is a target-specific data pointer which is filled in
    /// with the specifics of the exception that occurred. For examples on Unix
    /// platforms this is a `*mut *mut T` which is filled in by the compiler and
    /// on MSVC it's `*mut [usize; 2]`. For more information see the compiler's
    /// source as well as std's catch implementation.
    pub fn try(f: fn(*mut u8), data: *mut u8, local_ptr: *mut u8) -> i32;
}
