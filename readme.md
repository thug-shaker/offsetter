# Offset macro
This crate defines multiple macros that make specifying structs with fields at specific offsets easy.

### Macrotypes
This crate currently contains two different macros.
- offset!
- offset_debug!

#### offset!
offset! just defines a struct with members at specific offsets and with a given type, name, and, visibility.
#### offset_debug!
Same as offset! except that Debug is also automatically implemented, this can also be done by adding a derive however this also prints the padding fields.
offset_debug's Debug implementation behaves like derive Debug except it ommits the generated padding fields.

### features
This crate has a feature named "checked", which inserts compile time assertions that all fields are placed at the correct offsets this feature is only available on nightly compilers, and, with the offset_of feature enabled.

### Examples
#### DRIVER_OBJECT as seen in windows drivers.
```rust
offset_debug!(
    pub struct DRIVER_OBJECT {
        0x0  pub type_: u16,
        0x2  pub size: u16,
        0x8  pub device_object: *mut DEVICE_OBJECT,
        0x10 pub flags: u32,
        0x18 pub driver_start: usize,
        0x20 pub driver_size: u32,
        0x28 pub driver_section: usize,
        0x30 pub driver_extension: usize,
        0x38 pub driver_name: UNICODE_STRING,
        0x48 pub hardware_database: *mut UNICODE_STRING,
        0x50 pub fast_io_dispatch: usize,
        0x58 pub driver_init: usize,
        0x60 pub driver_start_io: usize,
        0x68 pub driver_unload: usize,
        0x70 pub major_function: [usize; 28],
    }
);
```
#### VMCB State save area as seen in SVM based hypervisors.
```rust
offset_debug!(
    pub struct SegmentRegister {
        0x0 pub selector: u16,
        0x2 pub attrib: u16,
        0x4 pub limit: u32,
        0x8 pub base: u64,
    }
);

offset_debug!(
    pub struct VmcbSave {
        0x0   pub es: SegmentRegister,
        0x10  pub cs: SegmentRegister,
        0x20  pub ss: SegmentRegister,
        0x30  pub ds: SegmentRegister,
        0x40  pub fs: SegmentRegister,
        0x50  pub gs: SegmentRegister,
        0x60  pub gdtr: SegmentRegister,
        0x70  pub ldtr: SegmentRegister,
        0x80  pub idtr: SegmentRegister,
        0x90  pub tr: SegmentRegister,
        0xcb  pub cpl: u8,
        0xd0  pub efer: u64,
        0x148 pub cr4: u64,
        0x150 pub cr3: u64,
        0x158 pub cr0: u64,
        0x160 pub dr7: u64,
        0x168 pub dr6: u64,
        0x170 pub rflags: u64,
        0x178 pub rip: u64,
        0x1d8 pub rsp: u64,
        0x1e0 pub s_cet: u64,
        0x1e8 pub ssp: u64,
        0x1f0 pub isst_addr: u64,
        0x1f8 pub rax: u64,
        0x200 pub star: u64,
        0x208 pub lstar: u64,
        0x210 pub cstar: u64,
        0x218 pub sfmask: u64,
        0x220 pub kernel_gs_base: u64,
        0x228 pub sysenter_cs: u64,
        0x230 pub sysenter_esp: u64,
        0x238 pub sysenter_eip: u64,
        0x240 pub cr2: u64,
        0x268 pub g_pat: u64,
        0x270 pub dbgctl: u64,
        0x278 pub br_from: u64,
        0x280 pub br_to: u64,
        0x288 pub last_excp_from: u64,
        0x298 pub dbg_extn_cfg: u64,
        0x2e0 pub spec_ctrl: u64,
        // FIXME Better to keep this from printing at all by implementing Display
        // 0x670 lbr_stack: [u8; 0x100],
        0x770 pub lbr_select: u64,
        0x778 pub ibs_fetch_ctl: u64,
        0x780 pub ibs_fetch_linaddr: u64,
        0x788 pub ibs_op_ctl: u64,
        0x790 pub ibs_op_rip: u64,
        0x798 pub ibs_op_data: u64,
        0x7a0 pub ibs_op_data2: u64,
        0x7a8 pub ibs_op_data3: u64,
        0x7b0 pub ibs_dc_linaddr: u64,
        0x7b8 pub bp_ibstgt_rip: u64,
        0x7c0 pub ic_ibs_extd_ctl: u64,
    }
);
```
