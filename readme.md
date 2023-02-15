# Offset macro
This crate defines multiple macros that make specifying structs with fields at specific offsets easy.

### Macrotypes
This crate currently contains three different macros.
- offset!
- offset_debug!
- offset_export!

#### offset!
offset! just defines a struct with members at specific offsets and with a given type and name, all members, and the struct itself, will be private.
#### offset_debug!
Same as offset! except that Debug is also automatically implemented.
#### offset_export!
Same as offset_debug! except that all fields other than inserted padding are public. As is the struct itself.

### Examples
#### DRIVER_OBJECT as seen in windows drivers.
```rust
offset_export!(
    struct DRIVER_OBJECT {
        0x0 type_: u16,
        0x2 size: u16,
        0x8 device_object: *mut DEVICE_OBJECT,
        0x10 flags: u32,
        0x18 driver_start: usize,
        0x20 driver_size: u32,
        0x28 driver_section: usize,
        0x30 driver_extension: usize,
        0x38 driver_name: UNICODE_STRING,
        0x48 hardware_database: *mut UNICODE_STRING,
        0x50 fast_io_dispatch: usize,
        0x58 driver_init: usize,
        0x60 driver_start_io: usize,
        0x68 driver_unload: usize,
        0x70 major_function: [usize; 28],
    }
);
```
#### VMCB State save area as seen in SVM based hypervisors.
```rust
offset_export!(
    struct VmcbSave {
        0x0   es: SegmentRegister,
        0x10  cs: SegmentRegister,
        0x20  ss: SegmentRegister,
        0x30  ds: SegmentRegister,
        0x40  fs: SegmentRegister,
        0x50  gs: SegmentRegister,
        0x60  gdtr: SegmentRegister,
        0x70  ldtr: SegmentRegister,
        0x80  idtr: SegmentRegister,
        0x90  tr: SegmentRegister,
        0xcb  cpl: u8,
        0xd0  efer: u64,
        0x148 cr4: u64,
        0x150 cr3: u64,
        0x158 cr0: u64,
        0x160 dr7: u64,
        0x168 dr6: u64,
        0x170 rflags: u64,
        0x178 rip: u64,
        0x1d8 rsp: u64,
        0x1e0 s_cet: u64,
        0x1e8 ssp: u64,
        0x1f0 isst_addr: u64,
        0x1f8 rax: u64,
        0x200 star: u64,
        0x208 lstar: u64,
        0x210 cstar: u64,
        0x218 sfmask: u64,
        0x220 kernel_gs_base: u64,
        0x228 sysenter_cs: u64,
        0x230 sysenter_esp: u64,
        0x238 sysenter_eip: u64,
        0x240 cr2: u64,
        0x268 g_pat: u64,
        0x270 dbgctl: u64,
        0x278 br_from: u64,
        0x280 br_to: u64,
        0x288 last_excp_from: u64,
        0x298 dbg_extn_cfg: u64,
        0x2e0 spec_ctrl: u64,
        0x670 lbr_stack: [u8; 0x100],
        0x770 lbr_select: u64,
        0x778 ibs_fetch_ctl: u64,
        0x780 ibs_fetch_linaddr: u64,
        0x788 ibs_op_ctl: u64,
        0x790 ibs_op_rip: u64,
        0x798 ibs_op_data: u64,
        0x7a0 ibs_op_data2: u64,
        0x7a8 ibs_op_data3: u64,
        0x7b0 ibs_dc_linaddr: u64,
        0x7b8 bp_ibstgt_rip: u64,
        0x7c0 ic_ibs_extd_ctl: u64,
    }
);
```