use capstone::arch;
use capstone::arch::BuildsCapstone;
use capstone::Capstone;
use std::fmt::Write;

pub fn print_disassembly(code: &[u8]) {
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .build()
        .unwrap();

    println!("Disassembly of {} bytes:", code.len());
    let insns = cs.disasm_all(&code, 0x0).unwrap();
    for i in insns.iter() {
        let mut line = String::new();

        write!(&mut line, "{:4x}:\t", i.address()).unwrap();

        let mut bytes_str = String::new();
        let mut len = 0;
        let mut first = true;
        for b in i.bytes() {
            if !first {
                write!(&mut bytes_str, " ").unwrap();
            }
            write!(&mut bytes_str, "{:02x}", b).unwrap();
            len += 1;
            first = false;
        }
        write!(&mut line, "{:21}\t", bytes_str).unwrap();
        if len > 8 {
            write!(&mut line, "\n\t\t\t\t").unwrap();
        }

        if let Some(s) = i.mnemonic() {
            write!(&mut line, "{}\t", s).unwrap();
        }

        if let Some(s) = i.op_str() {
            write!(&mut line, "{}", s).unwrap();
        }

        println!("{}", line);
    }
}
