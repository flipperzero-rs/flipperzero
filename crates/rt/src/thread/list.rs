use core::{
    ffi::CStr,
    iter::FusedIterator,
    num::NonZeroUsize,
    ops::{Index, IndexMut},
    ptr::NonNull,
};

use flipperzero_sys::{
    self as sys, FuriThreadId, FuriThreadList, FuriThreadListItem, FuriThreadPriority,
};

pub struct ThreadList(NonNull<FuriThreadList>);
impl ThreadList {
    /// Creates a new thread list.
    ///
    /// The list is empty initially but can be populated by a call to [`x`].
    pub fn new() -> Self {
        // SAFETY: allocation is always safe
        let raw = unsafe { sys::furi_thread_list_alloc() };
        // SAFETY: the pointer is guaranteed to be non-null
        let raw = unsafe { NonNull::new_unchecked(raw) };
        Self(raw)
    }

    /// Obtains the raw non-null pointer to the underlying thread list.
    #[inline(always)]
    pub fn as_ptr(&self) -> NonNull<FuriThreadList> {
        self.0
    }

    /// Gets the size of this thread list.
    pub fn size(&self) -> usize {
        let raw = self.0.as_ptr();
        // SAFETY: `raw` is a valid pointer
        unsafe { sys::furi_thread_list_size(raw) }
    }

    pub fn try_enumerate_into(&mut self) -> Result<(), ()> {
        let raw = self.0.as_ptr();
        // SAFETY: `raw` is a valid pointer
        let success = unsafe { sys::furi_thread_enumerate(raw) };
        if success {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_raw(&self, position: usize) -> *mut FuriThreadListItem {
        let raw = self.0.as_ptr();
        // SAFETY: `raw` is a valid pointer
        // and out-of-bounds `index` access is checked internally
        unsafe { sys::furi_thread_list_get_at(raw, position) }
    }

    pub fn get(&self, position: usize) -> Option<&ThreadItem> {
        let size = self.size();
        if position >= size {
            return None;
        }

        Some(&self[position])
    }

    pub fn get_mut(&mut self, position: usize) -> Option<&mut ThreadItem> {
        let size = self.size();
        if position >= size {
            return None;
        }

        Some(&mut self[position])
    }
}
impl Drop for ThreadList {
    fn drop(&mut self) {
        let Self(raw) = self;
        let raw = raw.as_ptr();
        // SAFETY: `raw` is a valid pointer allocated by  a call to `furi_thread_list_alloc`
        unsafe {
            sys::furi_thread_list_free(raw);
        };
    }
}
impl Index<usize> for ThreadList {
    type Output = ThreadItem;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        let item = self.get_raw(index).cast();
        // SAFETY: `ThreadItem` is a transparent wrapper around `FuriThreadListItem`
        // and there are no `mut` item references thanks to this being `&self`
        unsafe { &*item }
    }
}
impl IndexMut<usize> for ThreadList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let item = self.get_raw(index).cast();
        // SAFETY: `ThreadItem` is a transparent wrapper around `FuriThreadListItem`
        // and the item reference is exclusive thanks to this being `&self mut`
        unsafe { &mut *item }
    }
}
impl<'a> IntoIterator for &'a ThreadList {
    type Item = &'a ThreadItem;

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let size = self.size();
        Self::IntoIter {
            list: self,
            index: 0,
            size,
        }
    }
}

pub struct Iter<'a> {
    list: &'a ThreadList,
    index: usize,
    size: usize,
}
impl<'a> Iterator for Iter<'a> {
    type Item = &'a ThreadItem;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { list, index, size } = self;

        if index == size {
            return None;
        }

        let item = &list[*index];
        *index += 1;

        Some(item)
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.size - self.index;
        (remaining, Some(remaining))
    }
}
impl FusedIterator for Iter<'_> {}
impl ExactSizeIterator for Iter<'_> {}
// TODO: impl DoubleEndedIterator for Iter<'_> {}

#[repr(transparent)]
pub struct ThreadItem(FuriThreadListItem);
impl ThreadItem {
    /// Gets the thread ID of this thread.
    ///
    ///# Safety
    ///
    /// This should only be called while the corresponding thread is running.
    pub unsafe fn id(&self) -> FuriThreadId {
        let thread = self.0.thread;
        // SAFETY: while the thread is running, `thread` is a valid pointer
        unsafe { sys::furi_thread_get_id(thread) }
    }

    /// Gets the app ID of the thread.
    ///
    ///# Safety
    ///
    /// This should only be called while the corresponding thread is running.
    pub unsafe fn app_id(&self) -> &CStr {
        let app_id = self.0.app_id;
        // SAFETY: while the thread is running, `app_id` is a valid Nul-terminated string
        unsafe { CStr::from_ptr(app_id) }
    }

    /// Gets the name of the thread.
    ///
    ///# Safety
    ///
    /// This should only be called while the corresponding thread is running.
    pub unsafe fn name(&self) -> &CStr {
        let name = self.0.name;
        // SAFETY: while the thread is running, `name` is a valid Nul-terminated string
        unsafe { CStr::from_ptr(name) }
    }

    pub fn priority(&self) -> ThreadPriority {
        let priority = self.0.priority;
        let priority = ThreadPriority::try_from(priority);

        // SAFETY: `self` must have all fields valid
        unsafe { priority.unwrap_unchecked() }
    }

    pub const fn stack_address(&self) -> u32 {
        self.0.stack_address
    }

    pub const fn heap(&self) -> Option<NonZeroUsize> {
        NonZeroUsize::new(self.0.heap)
    }

    pub const fn stack_size(&self) -> u32 {
        self.0.stack_size
    }

    pub const fn stack_min_free(&self) -> u32 {
        self.0.stack_min_free
    }

    pub fn state(&self) -> ThreadState {
        let state = self.0.state;
        // TODO: are there any more invariants?
        // SAFETY: `state` is a valid pointer to a Nul-terminated string
        let state = unsafe { CStr::from_ptr(state) };
        state.into()
    }

    pub const fn cpu(&self) -> f32 {
        self.0.cpu
    }

    pub const fn counter_previous(&self) -> u32 {
        self.0.counter_previous
    }

    pub const fn counter_current(&self) -> u32 {
        self.0.counter_current
    }

    pub const fn tick(&self) -> u32 {
        self.0.tick
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ThreadPriority {
    None = sys::FuriThreadPriority_FuriThreadPriorityNone,
    Idle = sys::FuriThreadPriority_FuriThreadPriorityIdle,
    Lowest = sys::FuriThreadPriority_FuriThreadPriorityLowest,
    Low = sys::FuriThreadPriority_FuriThreadPriorityLow,
    Normal = sys::FuriThreadPriority_FuriThreadPriorityNormal,
    High = sys::FuriThreadPriority_FuriThreadPriorityHigh,
    Highest = sys::FuriThreadPriority_FuriThreadPriorityHighest,
    Isr = sys::FuriThreadPriority_FuriThreadPriorityIsr,
}
impl From<ThreadPriority> for FuriThreadPriority {
    fn from(value: ThreadPriority) -> Self {
        value as Self
    }
}
impl TryFrom<FuriThreadPriority> for ThreadPriority {
    type Error = ThreadPriorityFromSysError;

    fn try_from(value: FuriThreadPriority) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::FuriThreadPriority_FuriThreadPriorityNone => Self::None,
            sys::FuriThreadPriority_FuriThreadPriorityIdle => Self::Idle,
            sys::FuriThreadPriority_FuriThreadPriorityLowest => Self::Lowest,
            sys::FuriThreadPriority_FuriThreadPriorityLow => Self::Low,
            sys::FuriThreadPriority_FuriThreadPriorityNormal => Self::Normal,
            sys::FuriThreadPriority_FuriThreadPriorityHigh => Self::High,
            sys::FuriThreadPriority_FuriThreadPriorityHighest => Self::Highest,
            sys::FuriThreadPriority_FuriThreadPriorityIsr => Self::Isr,
            invalid => Err(ThreadPriorityFromSysError(invalid))?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ThreadPriorityFromSysError(u8);

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ThreadState<'a> {
    Running,
    Ready,
    Blocked,
    Suspended,
    Deleted,
    Invalid,
    Other(&'a CStr),
}
impl<'a> From<&'a CStr> for ThreadState<'a> {
    fn from(value: &'a CStr) -> Self {
        match value.to_bytes() {
            b"Running" => Self::Running,
            b"Ready" => Self::Ready,
            b"Blocked" => Self::Blocked,
            b"Suspended" => Self::Suspended,
            b"Deleted" => Self::Deleted,
            b"Invalid" => Self::Invalid,
            _ => Self::Other(value),
        }
    }
}
