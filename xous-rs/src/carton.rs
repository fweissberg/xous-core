//! A Carton is an object that wraps another object for shipping across the kernel
//! boundary. Structs that are stored in Cartons can be sent as messages.
// extern crate alloc;
// use alloc::alloc::{alloc, dealloc, Layout};

use crate::{Error, MemoryMessage, MemoryRange, Message, CID};

#[derive(Debug)]
pub struct Carton<'a> {
    range: MemoryRange,
    slice: &'a [u8],
}

impl<'a> Carton<'a> {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let src_mem = bytes.as_ptr();
        let size = bytes.len();
        // let layout = Layout::from_size_align(size, 4096).unwrap();
        let new_mem = crate::map_memory(
            None,
            None,
            size,
            crate::MemoryFlags::R | crate::MemoryFlags::W,
        )
        .unwrap();
        // unsafe {
        //         let new_mem = alloc(layout);
        //         core::ptr::copy(src_mem, new_mem, size);
        //         new_mem
        //     };
        Carton {
            range: new_mem,
            slice: unsafe { core::slice::from_raw_parts_mut(new_mem.as_mut_ptr(), new_mem.len()) },
        }
    }

    pub fn into_message(self, id: usize) -> MemoryMessage {
        MemoryMessage {
            id,
            buf: self.range,
            offset: None,
            valid: None,
        }
    }

    /// Perform an immutable lend of this Carton to the specified server.
    /// This function will block until the server returns.
    pub fn lend(&self, connection: CID, id: usize) -> Result<(), Error> {
        let msg = MemoryMessage {
            id,
            buf: self.range,
            offset: None,
            valid: None,
        };
        crate::send_message(connection, Message::Borrow(msg))
    }

    /// Perform a mutable lend of this Carton to the server.
    pub fn lend_mut(&mut self, connection: CID, id: usize) -> Result<(), Error> {
        let msg = MemoryMessage {
            id,
            buf: self.range,
            offset: None,
            valid: None,
        };
        crate::send_message(connection, Message::MutableBorrow(msg))
    }
}

impl<'a> AsRef<MemoryRange> for Carton<'a> {
    fn as_ref(&self) -> &MemoryRange {
        &self.range
    }
}

impl<'a> AsRef<[u8]> for Carton<'a> {
    fn as_ref(&self) -> &[u8] {
        &self.slice
    }
}

impl<'a> Drop for Carton<'a> {
    fn drop(&mut self) {
        // let layout = Layout::from_size_align(self.range.len(), 4096).unwrap();
        // let ptr = self.range.as_mut_ptr();
        // unsafe { dealloc(ptr, layout) };
    }
}
