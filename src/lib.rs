//! A small library for creating boxed slices `Box<[T]>`.

use std::mem::MaybeUninit;

/// Assumes all elements of the elements in `ts` are initialized, with the same semantics as
/// [`MaybeUninit::assume_init`].
///
/// # Example
/// ```
/// # use boxchop::assume_all_init;
/// # use std::mem::MaybeUninit;
/// #
/// let numbers = Box::new([
///     MaybeUninit::new(0),
///     MaybeUninit::new(12),
///     MaybeUninit::new(42)
/// ]);
/// let numbers = unsafe { assume_all_init(numbers) };
///
/// assert_eq!(
///     numbers,
///     Box::from([0, 12, 42])
/// );
/// ```
pub unsafe fn assume_all_init<T>(ts: Box<[MaybeUninit<T>]>) -> Box<[T]> {
    // TODO: what is the right way to do this?

    // Justification:
    // - `MaybeUninit<T>` is guaranteed to have the same ABI as `T`, it just may be uninitialized;
    // - This function is told to assume the data *is* initialized; therefore
    // - The data can be safely transmuted "out" of `MaybeUninit`
    std::mem::transmute(ts)
}

/// Creates a boxed slice of uninitialized memory.
///
/// Use [`MaybeUninit`] to initialize the values and then [`assume_all_init`] to assert all values
/// have been initialized.
///
/// # Example
/// ```
/// # use boxchop::new_uninit;
/// #
/// let nothings = new_uninit::<usize>(3);
///
/// assert_eq!(nothings.len(), 3);
/// // all 3 values are uninitialized
/// ```
pub fn new_uninit<T>(len: usize) -> Box<[MaybeUninit<T>]> {
    unsafe {
        // Create the slice
        let slice_ref_mut = if std::mem::size_of::<T>() == 0 {
            std::slice::from_raw_parts_mut(std::ptr::NonNull::dangling().as_ptr(), len)
        } else {
            // Allocate the memory for `len` count of `MaybeUninit<T>`s
            let layout = std::alloc::Layout::array::<MaybeUninit<T>>(len).unwrap();
            let mem = std::alloc::alloc(layout) as *mut MaybeUninit<T>;

            // Make slice reference from the pointer of memory
            std::slice::from_raw_parts_mut(mem, len)
        };

        // And put it in a box
        Box::from_raw(slice_ref_mut)
    }
}

/// Creates a boxed slice of zeroed memory.
///
/// # Example
/// ```
/// # use boxchop::{assume_all_init, new_zeroed};
/// #
/// let xs = new_zeroed::<usize>(4);
///
/// assert_eq!(xs.len(), 4);
///
/// // This is safe since a `usize` with all-zero bit pattern is valid
/// let zeroes = unsafe { assume_all_init(xs) };
///
/// assert_eq!(
///     zeroes,
///     Box::from([0, 0, 0, 0])
/// );
/// ```
pub fn new_zeroed<T>(len: usize) -> Box<[MaybeUninit<T>]> {
    unsafe {
        // Create the slice
        let slice_ref_mut = if std::mem::size_of::<T>() == 0 {
            std::slice::from_raw_parts_mut(std::ptr::NonNull::dangling().as_ptr(), len)
        } else {
            // Allocate the memory for `len` count of `MaybeUninit<T>`s
            let layout = std::alloc::Layout::array::<MaybeUninit<T>>(len).unwrap();
            let mem = std::alloc::alloc_zeroed(layout) as *mut MaybeUninit<T>;

            // Make slice reference from the pointer of memory
            std::slice::from_raw_parts_mut(mem, len)
        };

        // And put it in a box
        Box::from_raw(slice_ref_mut)
    }
}

// TODO: new_consts

/// Creates a boxed slice of `len` [copies](Copy) of `val`.
///
/// # Example
/// ```
/// # use boxchop::new_copies;
/// #
/// let twelves = new_copies(2, 12);
///
/// assert_eq!(
///     twelves,
///     Box::from([12, 12])
/// );
/// ```
pub fn new_copies<T>(len: usize, val: T) -> Box<[T]>
where
    T: Copy,
{
    let mut ts = new_uninit(len);

    if std::mem::size_of::<T>() != 0 {
        for t in ts.iter_mut() {
            let ptr: *mut T = t.as_mut_ptr();
            unsafe { ptr.write(val) }
        }
    }

    unsafe { assume_all_init(ts) }
}

/// Creates a boxed slice of `len` [clones](Clone) of `val`.
///
/// # Example
/// ```
/// # use boxchop::new_clones;
/// #
/// #[derive(Clone, Eq, PartialEq, Debug)]
/// enum Bread { Wheat, White, Other }
///
/// let loaf = new_clones(18, Bread::Wheat);
///
/// assert_eq!(
///     loaf,
///     Box::from([
///         Bread::Wheat,
///         Bread::Wheat,
///         // ... 15 more
/// #       Bread::Wheat,
/// #       Bread::Wheat,
/// #       Bread::Wheat,
/// #       Bread::Wheat,
/// #       Bread::Wheat,
/// #       Bread::Wheat,
/// #       Bread::Wheat,
/// #       Bread::Wheat,
/// #       Bread::Wheat,
/// #       Bread::Wheat,
/// #       Bread::Wheat,
/// #       Bread::Wheat,
/// #       Bread::Wheat,
/// #       Bread::Wheat,
/// #       Bread::Wheat,
///         Bread::Wheat,
///     ])
/// );
/// ```
pub fn new_clones<T>(len: usize, val: T) -> Box<[T]>
where
    T: Clone,
{
    let mut ts = new_uninit(len);

    if std::mem::size_of::<T>() != 0 {
        for t in ts.iter_mut() {
            let ptr: *mut T = t.as_mut_ptr();
            unsafe { ptr.write(val.clone()) }
        }
    }

    unsafe { assume_all_init(ts) }
}

/// Creates a boxed slice of `len` elements using [`Default`].
///
/// # Example
/// ```
/// # use boxchop::new_defaults;
/// #
/// #[derive(Default, Eq, PartialEq, Debug)]
/// struct Counter(usize);
///
/// let counters = new_defaults::<Counter>(2);
///
/// assert_eq!(
///     counters,
///     Box::from([Counter(0), Counter(0)])
/// );
/// ```
pub fn new_defaults<T>(len: usize) -> Box<[T]>
where
    T: Default,
{
    let mut ts = new_uninit(len);

    if std::mem::size_of::<T>() != 0 {
        for t in ts.iter_mut() {
            let ptr: *mut T = t.as_mut_ptr();
            unsafe { ptr.write(T::default()) }
        }
    }

    unsafe { assume_all_init(ts) }
}

/// Creates a boxed slice of `len` elements using the closure `gen` to generate each element, given
/// the element's index.
///
/// # Example
/// ```
/// # use boxchop::new_with;
/// #
/// let nums = new_with(5, |x| x + 1);
///
/// assert_eq!(
///     nums,
///     Box::from([1, 2, 3, 4, 5])
/// );
/// ```
pub fn new_with<T>(len: usize, mut gen: impl FnMut(usize) -> T) -> Box<[T]> {
    let mut ts = new_uninit(len);

    if std::mem::size_of::<T>() != 0 {
        for (idx, t) in ts.iter_mut().enumerate() {
            let ptr: *mut T = t.as_mut_ptr();
            unsafe { ptr.write(gen(idx)) }
        }
    }

    unsafe { assume_all_init(ts) }
}
