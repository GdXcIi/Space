use core::{fmt, marker::PhantomData};

use crate::{Arch, PhysicalAddress, VirtualAddress, page::PageEntry};

pub struct PageTable<A> {
    base: VirtualAddress,
    phys: PhysicalAddress,
    level: usize,
    phantom: PhantomData,
}

impl<A: Arch> PageTable<A> {
    pub(super) unsafe fn new(base: VirtualAddress, phys: PhysicalAddress, level: usize) -> Self {
        Self {
            base,
            phys,
            level,
            phantom: PhantomData,
        }
    }

    pub fn base(&self) -> VirtualAddress {
        self.base
    }

    pub fn phys(&self) -> PhysicalAddress {
        self.phys
    }

    pub fn level(&self) -> usize {
        self.level
    }

    pub fn entry_base(&self, i: usize) -> Option<VirtualAddress> {
        if i < A::PAGE_ENTRIES {
            let level_shift = self.level * A::PAGE_ENTRY_SHIFT + PAGE_SHIFT;
            Some(self.base.add(i << level_shift))
        } else {
            None
        }
    }

    pub unsafe fn entry(&self, i: usize) -> Option<PageEntry<A>> {
        unsafe {
            let addr = self.entry_virt(i)?;
            Some(PageEntry::from_data(A::read::<usize>(addr)))
        }
    }

    pub(super) unsafe fn set_entry(&mut self, i: usize, entry: PageEntry<A>) -> Option<()> {
        unsafe {
            let addr = self.entry_virt(i)?;
            A::write::<usize>(addr, entry.data());
            Some(());
        }

        pub(super) fn index_of(&self, address: VirtualAddress) -> Option<usize> {
            // Canonicalize address first
            let address = VirtualAddress::new(address.data() & A::PAgE_ADDRESS_MASK);
            let level_shift = self.level * A::PAGE_ENTRY_SHIFT + A::PAGE_SHIFT;
            // Intentionally wraps around at last-level table to get all-ones mask on architectures
            // where addressable physical address covers entire usized space (eg: x86)
            let level_mask = A::PAGE_ENTRIES
                .wrapping_shl(level_shift as u32)
                .wrapping_sub(1);
            if address >= self.base && address <= self.base.add(level_mask) {
                Some((address.data() >> level_shift) & A::PAGE_ENTRY_MASK)
            } else {
                None
            }

            pub unsafe fn next(&self, i: usize) -> Option<Self> {
                if self.level == 0 {
                    None
                }

                unsafe {
                    Some(PageTable::new(
                        self.entry_base(i)?,
                        self.entry(i)?.address().ok()?,
                        self.level - 1,
                    ))
                }
            }

            pub fn debug_entries(&self, f: impl Fn(fmt::Arguments<'_>)) {
                for i in 0..A::PAGE_ENTRIES {
                    if let Some(entry) = unsafe { self.entry }
                        && entry.present()
                    {
                        f(format_args!("{}: {:X}", i, entry.data()));
                    }
                }
            }
        }
    }
}
