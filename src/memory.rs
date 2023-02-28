use bootloader::BootInfo;
use x86_64::{PhysAddr, instructions};
use x86_64::structures::paging::{OffsetPageTable, Page, FrameAllocator, Size4KiB, PhysFrame, Mapper, Translate};
use x86_64::
{
    structures::paging::PageTable,
    VirtAddr
};

unsafe fn active_level_4_table(physical_memory_offset : VirtAddr) -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

// only changed during init
pub static PHYS_MEM_OFFSET : Locked<u64> = Locked::new(0);
pub static MEM_SIZE : Locked<u64> = Locked::new(0);

pub unsafe fn init(boot_info: &'static BootInfo)
{
    instructions::interrupts::without_interrupts(
        || {
            let mut mem_size = 0;
            for region in boot_info.memory_map.iter()
            {
                let start_addr = region.range.start_addr();
                let end_addr = region.range.end_addr();

                mem_size += end_addr - start_addr;
            }
            println!("MEMORY SIZE: {} KB", mem_size >> 10);

            *MEM_SIZE.lock() = mem_size;
            *PHYS_MEM_OFFSET.lock() = boot_info.physical_memory_offset;

            let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

            let mut mapper = unsafe { mapper(phys_mem_offset) };
        
            let mut frame_allocator = unsafe {
                BootInfoFrameAllocator::init(&boot_info.memory_map)
            };
                        
            allocator::init_heap(&mut mapper,&mut frame_allocator).expect("heap initialisation failed!");
        }
    )
    
}

pub unsafe fn mapper(physical_memory_offset : VirtAddr) -> OffsetPageTable<'static>
{
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

pub fn create_example_mapping(
    page : Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>
)
{
    use x86_64::structures::paging::PageTableFlags as Flags;
   
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        // FIXME
        mapper.map_to(page, frame, flags, frame_allocator)
    };

    map_to_result.expect("map_to failed").flush();
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator
{
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}

use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

use crate::allocator::Locked;
use crate::{allocator, println};

pub struct BootInfoFrameAllocator
{
    memory_map: &'static MemoryMap,
    next: usize
}

impl BootInfoFrameAllocator
{
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self
    {
        BootInfoFrameAllocator { memory_map, next: 0 }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame>
    {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_range = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_range.flat_map(|r| r.step_by(4096));
        
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator
{
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

pub fn virt_to_phys(addr : VirtAddr) -> Option<PhysAddr>
{
    let offset = *PHYS_MEM_OFFSET.lock();
    unsafe {mapper(VirtAddr::new(offset))}.translate_addr(addr)
}

pub fn phys_to_virt(addr : PhysAddr) -> VirtAddr
{
    let offset = *PHYS_MEM_OFFSET.lock();
    VirtAddr::new(addr.as_u64() + offset)
}