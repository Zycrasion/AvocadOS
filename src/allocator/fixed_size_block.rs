use core::alloc::Layout;

const BLOCK_SIZES: &[usize] = &[8,16,32,64,128,256,512,1_024,2_048, 4_096, 8_192, 16_384];

struct ListNode
{
    next: Option<&'static mut ListNode>,
}

pub struct FixedSizeAllocator
{
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    fallback_allocator: linked_list_allocator::Heap
}

impl FixedSizeAllocator
{
    pub const fn new() -> Self
    {
        const EMPTY: Option<&'static mut ListNode> = None;
        FixedSizeAllocator { list_heads: [EMPTY; BLOCK_SIZES.len()], fallback_allocator: linked_list_allocator::Heap::empty() }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_end: usize)
    {
        self.fallback_allocator.init(heap_start, heap_end);
    }

    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8
    {
        match self.fallback_allocator.allocate_first_fit(layout)
        {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => core::ptr::null_mut()
        } 
    }

    fn list_index(layout : &Layout) -> Option<usize>
    {
        let required_block_size = layout.size().max(layout.align());
        BLOCK_SIZES.iter().position(|&s| s >= required_block_size)
    }   
}

use crate::println;

use super::Locked;
use alloc::alloc::GlobalAlloc;

unsafe impl GlobalAlloc for Locked<FixedSizeAllocator>
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();
        match FixedSizeAllocator::list_index(&layout)
        {
            Some(index) => {
                match allocator.list_heads[index].take()
                {
                    Some(node) => {
                        allocator.list_heads[index] = node.next.take();
                        node as *mut ListNode as *mut u8
                    },
                    None => {
                        let block_size = BLOCK_SIZES[index];
                        let block_align = block_size;
                        let layout = Layout::from_size_align(block_size, block_align)
                            .unwrap();

                        allocator.fallback_alloc(layout)
                    }
                }
            },
            None => allocator.fallback_alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        use core::{mem, ptr::{NonNull}};

        let mut allocator = self.lock();
        match FixedSizeAllocator::list_index(&layout)
        {
            Some(index) => {
                let new_node = ListNode {
                    next: allocator.list_heads[index].take()
                };

                assert!(mem::size_of::<ListNode>() <= BLOCK_SIZES[index]);
                assert!(mem::align_of::<ListNode>() <= BLOCK_SIZES[index]);

                let new_node_ptr = ptr as *mut ListNode;
                new_node_ptr.write(new_node);

                allocator.list_heads[index] = Some(&mut *new_node_ptr);
            },
            None => {
                let ptr = NonNull::new(ptr).unwrap();
                allocator.fallback_allocator.deallocate(ptr, layout);
            }   
        }
    }
}