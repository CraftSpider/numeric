use alloc::boxed::Box;
use core::ptr;
use numeric_utils::intern::Interned;
use numeric_utils::Interner;

#[macro_use]
mod macros;

pub const fn arr_size<T>() -> usize {
    (size_of::<T>() / size_of::<usize>()) + 1
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub enum Side {
    Above,
    Below,
}

/// The error for when you try to convert a `IBig` with a value that is too large or small for
/// the type being converted into.
#[derive(Debug)]
pub struct OutOfRangeError(pub(crate) Side);

impl OutOfRangeError {
    pub(crate) fn above() -> Self {
        Self(Side::Above)
    }

    pub(crate) fn below() -> Self {
        Self(Side::Below)
    }

    pub fn side(self) -> Side {
        self.0
    }
}

pub type InternedInt = Interned<Box<[usize]>>;
pub static INT_STORE: Interner<Box<[usize]>> = Interner::new();

pub enum TaggedVal<'a> {
    Inline(usize),
    Slice(&'a InternedInt),
}

impl PartialEq for TaggedVal<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Inline(a), Self::Inline(b)) if a == b => true,
            (Self::Slice(a), Self::Slice(b)) if ptr::addr_eq(*a, *b) => true,
            _ => false,
        }
    }
}

/// An offset containing a `Tag` in its lower two bits
#[derive(Copy, Clone)]
pub union TaggedOffset<const T: usize> {
    val: usize,
    ptr: *const InternedInt,
}

impl<const T: usize> TaggedOffset<T> {
    const MASK: usize = const { (1 << T) - 1 };

    #[must_use]
    #[inline]
    pub const fn new(offset: usize, tags: usize) -> TaggedOffset<T> {
        assert!(offset <= usize::MAX >> T);
        TaggedOffset {
            val: (offset << T) | 1 | tags << 1,
        }
    }

    #[must_use]
    #[inline]
    pub fn new_ptr(r: *const InternedInt, tags: usize) -> TaggedOffset<T> {
        assert_eq!(r.addr() % (2 << T), 0, "Pointer has insufficient alignment");
        TaggedOffset {
            ptr: r.map_addr(|r| r | tags << 1),
        }
    }

    #[inline(always)]
    const fn val(self) -> usize {
        // SAFETY: Always sound to access internals as a usize
        unsafe { self.val }
    }

    #[inline(always)]
    const fn ptr(self) -> *const InternedInt {
        // SAFETY: Always sound to access internals as a pointer, if not derefed
        unsafe { self.ptr }
    }

    #[must_use]
    #[inline]
    pub fn invert_tag<const IDX: usize>(self) -> TaggedOffset<T> {
        TaggedOffset {
            ptr: self.ptr().map_addr(|a| a ^ (const { 1 << IDX })),
        }
    }

    #[must_use]
    #[inline]
    pub fn get(self) -> (TaggedVal<'static>, usize) {
        (self.offset(), self.tags())
    }

    #[must_use]
    #[inline]
    pub fn offset(self) -> TaggedVal<'static> {
        if self.inline() {
            TaggedVal::Inline(self.val() >> T)
        } else {
            // SAFETY: If tag isn't inline, internal value is guaranteed to be a valid pointer
            TaggedVal::Slice(unsafe { &*self.ptr().map_addr(|a| a & !Self::MASK) })
        }
    }

    #[must_use]
    #[inline]
    pub const fn inline(self) -> bool {
        self.val() & 0b1 != 0
    }

    #[must_use]
    #[inline]
    pub const fn tags(self) -> usize {
        (self.val() & Self::MASK) >> 1
    }

    #[inline]
    fn to_enum(&self) -> MaybeInline<'_> {
        match self.offset() {
            TaggedVal::Inline(val) => MaybeInline::Inline(val),
            TaggedVal::Slice(val) => MaybeInline::Slice(val.get()),
        }
    }
}

impl<const T: usize> PartialEq for TaggedOffset<T> {
    fn eq(&self, other: &Self) -> bool {
        self.val() == other.val()
    }
}

impl<const T: usize> Eq for TaggedOffset<T> {}

// SAFETY: TaggedOffset pointee is guaranteed Send + Sync
unsafe impl<const T: usize> Send for TaggedOffset<T> {}
// SAFETY: TaggedOffset pointee is guaranteed Send + Sync
unsafe impl<const T: usize> Sync for TaggedOffset<T> {}

pub enum MaybeInline<'a> {
    Inline(usize),
    Slice(&'a [usize]),
}

impl MaybeInline<'_> {
    #[inline]
    pub const fn slice(&self) -> &[usize] {
        match self {
            MaybeInline::Inline(i) => core::slice::from_ref(i),
            MaybeInline::Slice(s) => s,
        }
    }
}
