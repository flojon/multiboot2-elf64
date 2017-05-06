#[derive(Debug)]
#[repr(C)]
pub struct MemoryMapTag {
    typ: u32,
    size: u32,
    entry_size: u32,
    entry_version: u32,
    first_area: MemoryArea,
}

impl MemoryMapTag {
    pub fn memory_areas(&self) -> MemoryAreaIter {
        let self_ptr = self as *const MemoryMapTag;
        let start_area = (&self.first_area) as *const MemoryArea;
        MemoryAreaIter {
            current_area: start_area,
            last_area: (self_ptr as u64 + (self.size - self.entry_size) as u64)
                as *const MemoryArea,
            entry_size: self.entry_size,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct MemoryArea {
    base_addr: u64,
    length: u64,
    typ: u32,
    _reserved: u32,
}

unsafe impl Send for MemoryArea {}
unsafe impl Sync for MemoryArea {}

impl MemoryArea {
    pub fn start_address(&self) -> usize {
        self.base_addr as usize
    }

    pub fn end_address(&self) -> usize {
        (self.base_addr + self.length) as usize
    }

    pub fn size(&self) -> usize {
        self.length as usize
    }
}

#[derive(Clone, Debug)]
pub struct MemoryAreaIter {
    current_area: *const MemoryArea,
    last_area: *const MemoryArea,
    entry_size: u32,
}

impl Iterator for MemoryAreaIter {
    type Item = &'static MemoryArea;
    fn next(&mut self) -> Option<&'static MemoryArea> {
        if self.current_area > self.last_area {
            None
        } else {
            let area = unsafe{&*self.current_area};
            self.current_area = ((self.current_area as u64) + self.entry_size as u64)
                as *const MemoryArea;
            if area.typ == 1 {
                Some(area)
            } else {self.next()}
        }
    }
}
