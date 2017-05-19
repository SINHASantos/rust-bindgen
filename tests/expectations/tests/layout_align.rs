/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>);
impl <T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl <T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl <T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __IncompleteArrayField<T> { }
#[repr(C)]
#[derive(Debug, Copy)]
pub struct rte_kni_fifo {
    /**< Next position to be written*/
    pub write: ::std::os::raw::c_uint,
    /**< Next position to be read */
    pub read: ::std::os::raw::c_uint,
    /**< Circular buffer length */
    pub len: ::std::os::raw::c_uint,
    /**< Pointer size - for 32/64 bit OS */
    pub elem_size: ::std::os::raw::c_uint,
    /**< The buffer contains mbuf pointers */
    pub buffer: __IncompleteArrayField<*mut ::std::os::raw::c_void>,
    pub __bindgen_align: [u64; 0usize],
}
#[test]
fn bindgen_test_layout_rte_kni_fifo() {
    assert_eq!(::std::mem::size_of::<rte_kni_fifo>() , 16usize , concat ! (
               "Size of: " , stringify ! ( rte_kni_fifo ) ));
    assert_eq! (::std::mem::align_of::<rte_kni_fifo>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( rte_kni_fifo ) ));
}
impl Clone for rte_kni_fifo {
    fn clone(&self) -> Self { *self }
}
impl Default for rte_kni_fifo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct rte_eth_link {
    /**< ETH_SPEED_NUM_ */
    pub link_speed: u32,
    pub _bitfield_1: u8,
    pub __bindgen_padding_0: [u8; 3usize],
    pub __bindgen_align: [u64; 0usize],
}
#[test]
fn bindgen_test_layout_rte_eth_link() {
    assert_eq!(::std::mem::size_of::<rte_eth_link>() , 8usize , concat ! (
               "Size of: " , stringify ! ( rte_eth_link ) ));
    assert_eq! (::std::mem::align_of::<rte_eth_link>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( rte_eth_link ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_eth_link ) ) . link_speed as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( rte_eth_link ) , "::" ,
                stringify ! ( link_speed ) ));
}
impl Clone for rte_eth_link {
    fn clone(&self) -> Self { *self }
}
impl rte_eth_link {
    #[inline]
    pub fn link_duplex(&self) -> u16 {
        let mask = 1usize as u8;
        let unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_link_duplex(&mut self, val: u16) {
        let mask = 1usize as u8;
        let val = val as u16 as u8;
        let mut unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn link_autoneg(&self) -> u16 {
        let mask = 2usize as u8;
        let unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 1usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_link_autoneg(&mut self, val: u16) {
        let mask = 2usize as u8;
        let val = val as u16 as u8;
        let mut unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 1usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn link_status(&self) -> u16 {
        let mask = 4usize as u8;
        let unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 2usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_link_status(&mut self, val: u16) {
        let mask = 4usize as u8;
        let val = val as u16 as u8;
        let mut unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 2usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn new_bitfield_1(link_duplex: u16, link_autoneg: u16,
                          link_status: u16) -> u8 {
        let bitfield_unit_val =
            {
                let bitfield_unit_val =
                    {
                        let bitfield_unit_val = { 0 };
                        let link_duplex = link_duplex as u16 as u8;
                        let mask = 1usize as u8;
                        let link_duplex = (link_duplex << 0usize) & mask;
                        bitfield_unit_val | link_duplex
                    };
                let link_autoneg = link_autoneg as u16 as u8;
                let mask = 2usize as u8;
                let link_autoneg = (link_autoneg << 1usize) & mask;
                bitfield_unit_val | link_autoneg
            };
        let link_status = link_status as u16 as u8;
        let mask = 4usize as u8;
        let link_status = (link_status << 2usize) & mask;
        bitfield_unit_val | link_status
    }
}