# KURA Target Comparison - Technical Reference

## Quick Reference Table

| Aspect | Windows x64 | Linux x64 | Linux ARM64 | macOS x64 | macOS ARM64 |
|--------|------------|----------|-----------|-----------|-----------|
| **Target Triple** | x86_64-pc-windows-msvc | x86_64-unknown-linux-gnu | aarch64-unknown-linux-gnu | x86_64-apple-darwin | aarch64-apple-darwin |
| **Linker** | lld-link | ld.lld | ld.lld | ld64.lld | ld64.lld |
| **ABI** | Microsoft x64 | System V AMD64 | ARM64 ABI | Framework ABI | ARM64e |
| **Executable Format** | PE/COFF (.exe) | ELF | ELF | Mach-O | Mach-O |
| **Word Size** | 64-bit | 64-bit | 64-bit | 64-bit | 64-bit |
| **Endianness** | Little | Little | Little | Little | Little |
| **Calling Convention** | rcx, rdx, r8, r9 | rdi, rsi, rdx, rcx, r8, r9 | x0-x7, x8+ stack | rdi, rsi, rdx, rcx | x0-x7, x8+ stack |

## LLVM Configuration by Target

### Windows x86_64 (x86_64-pc-windows-msvc)

```
Target Triple:  x86_64-pc-windows-msvc
Data Layout:    e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128
Endianness:     Little-endian (e)
Pointer Size:   64-bit (p270/p271)
Int Width:      64-bit
Float Width:    128-bit (f80 x87 extended)
Alignment:      128-bit stack (S128)
Calling Conv:   Microsoft x64
```

**Key Characteristics:**
- Windows ABI (Application Binary Interface)
- Microsoft Visual C++ compatible
- X87 floating point format
- 128-bit stack alignment
- Register passing: RCX, RDX, R8, R9 (first 4 args)
- Return value: RAX/RDX for int64, XMM0/XMM1 for floats

**Linker Configuration:**
```rust
lld-link \
    -out:program.exe \
    program.obj \
    -subsystem:console \
    -defaultlib:libcmt \      // C Runtime
    -defaultlib:kernel32.lib  // Windows Kernel
```

---

### Linux x86_64 (x86_64-unknown-linux-gnu)

```
Target Triple:  x86_64-unknown-linux-gnu
Data Layout:    e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128
Endianness:     Little-endian (e)
ELF ABI:        System V AMD64 ABI
Calling Conv:   System V ABI
```

**Key Characteristics:**
- System V AMD64 ABI (UNIX standard)
- ELF (Executable and Linkable Format)
- GCC/Clang compatible
- 128-bit stack alignment
- Register passing: RDI, RSI, RDX, RCX, R8, R9
- XMM0-XMM7 for floating point arguments
- All arguments that don't fit in registers go on stack

**Linker Configuration:**
```rust
ld.lld \
    -o program.elf \
    program.o \
    -lc \                     // C library
    -lm                       // Math library
```

**Libc Implementation:**
- GNU libc (glibc) - most common
- musl libc - lightweight alternative
- uClibc - embedded systems

---

### Linux ARM64 (aarch64-unknown-linux-gnu)

```
Target Triple:  aarch64-unknown-linux-gnu
Data Layout:    e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128
Endianness:     Little-endian (e)
Pointer Size:   64-bit
Instruction Set: ARMv8-A (64-bit)
Calling Conv:   ARM64 ABI
```

**Key Characteristics:**
- ARM 64-bit (AArch64/ARMv8)
- Most common on Raspberry Pi 4+ and embedded systems
- ARM64 ABI standard calling convention
- Register passing: X0-X7 (first 8 args)
- Extended regs: X8-X30 available
- Stack alignment: 16 bytes

**Typical Boards:**
- Raspberry Pi 4+
- Orange Pi
- N1
- NVIDIA Jetson
- Qualcomm Snapdragon dev boards

**Linker Configuration:**
```rust
ld.lld \
    -o program \
    program.o \
    -lc \
    -lm
```

**Common Flags:**
```
-mcpu=cortex-a72      // Raspberry Pi 4 specific
-march=armv8-a+crc    // ARMv8 with CRC
```

---

### macOS x86_64 (x86_64-apple-darwin)

```
Target Triple:  x86_64-apple-darwin
Data Layout:    e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128
Endianness:     Little-endian (e)
Mach Object:    Mach-O 64-bit (m:o)
Calling Conv:   Apple Framework ABI
```

**Key Characteristics:**
- Mach-O executable format (native macOS)
- Apple Framework ABI
- Intel/AMD x86-64 architecture
- Supported on: Intel Macs from 2008-2020
- Register passing: RDI, RSI, RDX, RCX, R8, R9
- XMM0-XMM7 for floating point

**Linker Configuration:**
```rust
ld64.lld \
    -o program \
    program.o \
    -lSystem \         // macOS system libs
    -lm
```

**Minimum Deployment Target:**
```
macOS 10.7+
```

**Code Signature Requirements:**
- Modern macOS requires code signing
- Use `codesign -s -` for ad-hoc signing

---

### macOS ARM64 (aarch64-apple-darwin)

```
Target Triple:  aarch64-apple-darwin
Data Layout:    e-m:o-i64:64-i128:128-n32:64-S128
Endianness:     Little-endian (e)
Mach Object:    Mach-O 64-bit (m:o)
Calling Conv:   ARM64e ABI
Architecture:   ARM64 (AArch64)
```

**Key Characteristics:**
- Apple Silicon (M1, M2, M3, M4) native format
- ARM64e with additional CPU features
- Pointer authentication code (PAC) support
- Register passing: X0-X7 (first 8 args)  
- Supported on: All Apple Silicon Macs (2020+)

**Linker Configuration:**
```rust
ld64.lld \
    -o program \
    program.o \
    -lSystem \
    -lm
```

**Minimum Deployment Target:**
```
macOS 11.0+
```

**Performance:**
- Native performance on Apple Silicon
- Very efficient power consumption
- Support for NEON vector instructions
- Better battery life on MacBook Pro/Air

---

## Code Generation Differences

### Example: Passing Function Arguments

All targets ultimately pass the first few arguments in registers:

```kura
// KURA Code
fn add(a, b, c, d, e) {
    return a + b + c + d + e
}
```

**Windows x64 (RCX, RDX, R8, R9 + Stack):**
```llvm
define i64 @add(i64 %a, i64 %b, i64 %c, i64 %d, i64 %e) {
    ; RCX=a, RDX=b, R8=c, R9=d, [RSP]=e
}
```

**Linux x64 (RDI, RSI, RDX, RCX, R8, R9):**
```llvm
define i64 @add(i64 %a, i64 %b, i64 %c, i64 %d, i64 %e) {
    ; RDI=a, RSI=b, RDX=c, RCX=d, R8=e, R9 available
}
```

**ARM64 (X0-X7):**
```llvm
define i64 @add(i64 %a, i64 %b, i64 %c, i64 %d, i64 %e) {
    ; X0=a, X1=b, X2=c, X3=d, X4=e
}
```

### Example: Return Values

**Integers:**
- All targets: RAX (x86_64) or X0 (ARM64)

**Floats:**
- Windows: XMM0
- Linux x64: XMM0
- macOS: XMM0/XMM1
- ARM64: V0

**Structs:**
- Small (≤16 bytes): Registers
- Large: Return via pointer (implicit first parameter)

---

## Floating-Point Representation

### Windows x64
- X87 extended (f80)
- SSE/AVE for SIMD
- IEEE 754 compliant

### Linux x64
- Same as Windows
- SSE/AVE standard
- IEEE 754

### ARM64
- NEON vector unit (128-bit SIMD)
- IEEE 754 floating point
- 16 x 128-bit V0-V31 registers

### macOS x64
- Same as generic x64
- Supports SSE 4.2+
- Optional AVX support on newer Macs

### macOS ARM64
- Native floating point in NEON
- Better performance than x87
- Native double precision

---

## Memory Layout & Alignment

### Stack Alignment
```
Windows x64:     16 bytes (after return address)
Linux x64:       16 bytes
Linux ARM64:     16 bytes
macOS x64:       16 bytes
macOS ARM64:     16 bytes
```

### Pointer Size
```
All 64-bit targets: 8 bytes
```

### Structure Packing (Example)
```kura
struct Point {
    x: 0,      // i64 = 8 bytes
    y: 0,      // i64 = 8 bytes
    active: 0, // i1 = 1 byte (+ 7 bytes padding)
}
```

- Size: 24 bytes (common across all targets)
- Alignment: 8 bytes

---

## Performance Characteristics by Target

### Windows x64 (Relative Baseline: 1.0x)
- **Speed**: ~1.0x baseline
- **Power**: High
- **Common on**: Desktops, Servers, Workstations
- **Best for**: Enterprise desktop apps

### Linux x64 (Relative Baseline: 1.0x) 
- **Speed**: ~1.0x baseline (identical arch)
- **Power**: High
- **Common on**: Servers, Cloud VMs, Desktops
- **Best for**: Backend services, cloud computing

### Linux ARM64 (Relative Baseline: 0.7-0.8x vs x64)
- **Speed**: Competitive with x64 on modern cores
- **Power**: 40-60% lower than x64 at same workload
- **Common on**: Raspberry Pi, IoT, embedded
- **Best for**: Power-constrained environments

### macOS x86_64 (Relative Baseline: 1.0x)
- **Speed**: ~1.0x (Intel)
- **Power**: High (~20-40W at full load)
- **Common on**: Intel MacBooks, iMacs (pre-2020)
- **Best for**: Desktop development

### macOS ARM64 (Relative Baseline: 1.1-1.2x perf-per-watt)
- **Speed**: Similar to x64, sometimes faster
- **Power**: 5-10W at full load (extremely efficient)
- **Common on**: Apple Silicon Macs (M-series chips)
- **Best for**: Laptops, best power efficiency

---

## Selecting the Right Target

### Use Windows x86_64 if:
- Targeting Windows-only environment
- Enterprise compatibility required
- Legacy system support needed

### Use Linux x86_64 if:
- Targeting cloud/servers
- Multi-platform targeting (can cross-compile)
- Maximum compatibility in Linux ecosystem

### Use Linux ARM64 if:
- Targeting Raspberry Pi or similar devices
- IoT/embedded applications
- Power efficiency critical
- ARM-based cloud instances (AWS Graviton)

### Use macOS x86_64 if:
- Supporting older Intel Macs
- Maintaining legacy Mac support
- Not ready for Apple Silicon migration

### Use macOS ARM64 if:
- Targeting modern Apple Silicon Macs
- Maximum performance on macOS
- Optimal battery life on MacBooks
- Interested in future-proofing

---

## Compilation Matrix: All Possible Cross-compilations

```
From \ To      Windows   Linux    Linux    macOS    macOS
               x86_64    x86_64   ARM64    x86_64   ARM64
─────────────────────────────────────────────────────────
Windows        ✅ native  ✅        ✅      ⚠️*      ⚠️*
Linux x86_64   ✅         ✅ native ✅      ⚠️*      ⚠️*
Linux ARM64    ✅         ✅        ✅      ⚠️*      ⚠️*
macOS x86      ✅         ✅        ✅      ✅ native ✅
macOS ARM64    ✅         ✅        ✅      ✅       ✅ native
```

✅ = Full support
⚠️ = Requires apple-specific linker (ld64.lld)
    May need Apple SDK (code signing, frameworks)

---

## Future Considerations

### Not Yet Supported
- Windows ARM64 (arm64-pc-windows-msvc)
- Android (aarch64-linux-android)
- WebAssembly (wasm32-unknown-unknown)
- RISC-V (riscv64-unknown-linux-gnu)
- 32-bit targets (x86, ARM32)

### Potential Additions
Based on community demand:
- Android NDK support
- WebAssembly for browser + Node.js
- RISC-V for open-source boards
- More ARM variants for embedded

---

**Document Version**: 1.0  
**Updated**: March 24, 2026  
**Status**: Reference Documentation
