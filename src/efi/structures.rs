use core::sync::atomic::{AtomicPtr, Ordering};

pub const EFI_PAGE_SIZE: u64 = 4096;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
#[repr(u64)]
pub enum Error {
    /// There was no memory area found that is available after
    /// exiting boot services
    NoValidMemoryArea,
    CouldntAccessSystemTable,
    CouldntRegisterSystemTable,
    CouldntExitBootService(EfiStatus),
    CouldntGetMemoryMap(EfiStatus),
    Unknown(u64),
}

/// Global EFI system table which is saved upon entry of the kernel.
pub static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(core::ptr::null_mut());

pub unsafe fn register_system_table(system_table: *mut EfiSystemTable) -> Result<()> {
    EFI_SYSTEM_TABLE
        .compare_exchange(
            core::ptr::null_mut(),
            system_table,
            Ordering::SeqCst,
            Ordering::SeqCst,
        ).map_or(Err(Error::CouldntRegisterSystemTable), |_| Ok(()))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct EfiGuid(pub u32, pub u16, pub u16, pub [u8; 8]);

/// Collection fo related interfaces
/// Type: `void *`
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct EfiHandle(u64);

/// Collection fo related interfaces
/// Type: `void *`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct EfiStatus(pub u64);

/// The structure defining all boot services
/// supplied by the UEFI firmware.
///
/// Is destroyed when `ExitBootServices` is called
/// on the `EfiSystemTable`
#[repr(C)]
pub struct EfiBootServices {
    header: EfiTableHeader,
    // raises the task priority level.
    _raise_tpl: usize,
    _restore_tpl: usize,
    _allocate_pages: usize,
    _free_pages: usize,
    pub get_memory_map: unsafe fn(
        mmap_size: &mut u64,
        mmap: *mut u8,
        map_key: &mut u64,
        descriptor_size: &mut u64,
        descriptor_version: &mut u32,
    ) -> EfiStatus,
    _allocate_pool: usize,
    _free_pool: usize,
    _create_event: usize,
    _set_timer: usize,
    _wait_for_event: usize,
    _signal_event: usize,
    _close_event: usize,
    _check_event: usize,
    _install_protocol_interface: usize,
    _reinstall_protocol_interface: usize,
    _uninstall_protocol_interface: usize,
    _handle_protocol: usize,
    _reserved: usize,
    _register_protocol_notify: usize,
    _locate_handle: usize,
    _locate_device_path: usize,
    _install_configuration_table: usize,
    _load_image: usize,
    _start_image: usize,
    // exits the image's entry point
    _exit: usize,
    // unloads an image
    _unload_image: usize,
    pub exit_boot_services: unsafe fn(image_handle: EfiHandle, map_key: u64) -> EfiStatus,
}

#[repr(C)]
struct EfiSimpleTextInputProtocol {
    /// Resets the input device hardware.
    reset: unsafe fn(
        this: *const EfiSimpleTextInputProtocol,
        extended_verification: bool,
    ) -> EfiStatus,
    /// Reads the next keystroke from the input device.
    read_keystroke:
        unsafe fn(this: *const EfiSimpleTextInputProtocol, key: *mut EfiInputKey) -> EfiStatus,
    /// Event to use with EFI_BOOT_SERVICES.WaitForEvent() to wait for a key to
    /// be available
    _wait_for_key: usize,
}
#[repr(C)]
struct EfiSimpleTextOutputProtocol {
    /// Resets the output device hardware.
    reset: unsafe fn(
        this: *const EfiSimpleTextOutputProtocol,
        extended_verification: bool,
    ) -> EfiStatus,
    /// Writes a string to the device.
    output_string:
        unsafe fn(this: *const EfiSimpleTextOutputProtocol, string: *const u16) -> EfiStatus,
    /// Verifies that all chars in a string can be output
    /// to the target device.
    test_string:
        unsafe fn(this: *const EfiSimpleTextOutputProtocol, string: *const u16) -> EfiStatus,
    /// Returns information for an available text mode that the output
    /// device(s) supports.
    _query_mode: usize,

    /// Sets the output device(s) to a specified mode.
    _set_mode: usize,

    _set_attribute: usize,

    _clear_screen: usize,

    _set_cursor_position: usize,

    _enable_cursor: usize,

    _mode: usize,
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
struct EfiInputKey {
    scan_code: u16,
    unicode_char: u16,
}

#[repr(C)]
pub struct EfiSystemTable {
    header: EfiTableHeader,

    firmware_vendor: *const u16,

    firmware_revision: u32,

    console_in_handle: EfiHandle,

    console_in: *const EfiSimpleTextInputProtocol,

    console_out_handle: EfiHandle,

    console_out: *const EfiSimpleTextOutputProtocol,

    console_err_handle: EfiHandle,

    console_err: *const EfiSimpleTextOutputProtocol,

    _runtime_services: usize,

    pub boot_services: *const EfiBootServices,
    pub number_of_tables: usize,
    pub tables: *const EfiConfigurationTable,
}

#[derive(Debug)]
#[repr(C)]
pub struct EfiConfigurationTable {
    pub guid: EfiGuid,
    /// a pointer to the `VendorTable`
    pub table: usize,
}


#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}


#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct EfiMemoryDescriptor {
    pub typ: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum EfiMemoryType {
    /// This enum variant is not used.
    Reserved = 0,
    /// The code portions of a loaded UEFI application.
    LoaderCode = 1,
    /// The data portions of a loaded UEFI applications,
    /// as well as any memory allocated by it.
    #[allow(dead_code)]
    LoaderData = 2,
    /// Code of the boot drivers.
    ///
    /// Can be reused after OS is loaded.
    BootServicesCode = 3,
    /// Memory used to store boot drivers' data.
    ///
    /// Can be reused after OS is loaded.
    BootServicesData = 4,
    /// Runtime drivers' code.
    RuntimeServicesCode = 5,
    /// Runtime services' code.
    RuntimeServicesData = 6,
    /// Free usable memory.
    Conventional = 7,
    /// Memory in which errors have been detected.
    Unusable = 8,
    /// Memory that holds ACPI tables.
    /// Can be reclaimed after they are parsed.
    AcpiReclaim = 9,
    /// Firmware-reserved addresses.
    AcpiNonVolatile = 10,
    /// A region used for memory-mapped I/O.
    Mmio = 11,
    /// Address space used for memory-mapped port I/O.
    MmioPortSpace = 12,
    /// Address space which is part of the processor.
    PalCode = 13,
    /// Memory region which is usable and is also non-volatile.
    PersistentMemory = 14,
}

impl From<u32> for EfiMemoryType {
    fn from(v: u32) -> Self {
        use EfiMemoryType::*;
        match v {
            0 => Reserved,
            1 => LoaderCode,
            2 => LoaderCode,
            3 => BootServicesCode,
            4 => BootServicesData,
            5 => RuntimeServicesCode,
            6 => RuntimeServicesData,
            7 => Conventional,
            8 => Unusable,
            9 => AcpiReclaim,
            10 => AcpiNonVolatile,
            11 => Mmio,
            12 => MmioPortSpace,
            13 => PalCode,
            14 => PersistentMemory,
            _ => {
                panic!("Unsupported memory type supplied!")
            }
        }
    }
}

impl EfiMemoryType {
    /// Returns whether or not the memory type is available
    /// for general purpose use after boot services have been exited (brexit).
    pub fn available_post_exit_boot_services(&self) -> bool {
        use EfiMemoryType::*;
        matches!(
            self,
            BootServicesCode | BootServicesData | Conventional | PersistentMemory
        )
    }
}

pub fn load_system_table() -> Result<&'static EfiSystemTable> {
     let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);
     if st.is_null() {
         Err(Error::CouldntAccessSystemTable)
     } else { 
         // If non-null deference the pointer and return a reference to it.
         unsafe { 
             Ok(&*st) 
         } 
     }
}

pub fn destroy_system_table() {
     EFI_SYSTEM_TABLE.store(core::ptr::null_mut(), Ordering::SeqCst);
}

pub fn exit_boot_service_int(st: &EfiSystemTable, handle: EfiHandle, key: u64) -> Result<()> {
    // Exit and check success
    let status = unsafe { ((*st.boot_services).exit_boot_services)(handle, key) };
    if status.0 != 0 {
        return Err(Error::CouldntExitBootService(status));
    }

    Ok(())
}