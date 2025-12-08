use GBEmu::cpu::Bus::Bus;
use GBEmu::cpu::CPU::CPU;

fn make() -> (CPU, Bus) {
    let mut cpu = CPU::new();
    let bus = Bus::new();
    // Put PC in WRAM so read_u8_from_pc/read_u16_from_pc can use bus.write_byte
    cpu.PC = 0xC000;
    (cpu, bus)
}

// Per-destination register tests for register-register opcodes
#[test]
fn test_ld_b_ops() {
    let (mut cpu, mut bus) = make();
    // ld_b_b (no-op)
    cpu.B = 0x10; assert_eq!(CPU::ld_b_b(&mut cpu, &mut bus), 4); assert_eq!(cpu.B, 0x10);
    cpu.C = 0x21; assert_eq!(CPU::ld_b_c(&mut cpu, &mut bus), 4); assert_eq!(cpu.B, 0x21);
    cpu.D = 0x22; assert_eq!(CPU::ld_b_d(&mut cpu, &mut bus), 4); assert_eq!(cpu.B, 0x22);
    cpu.E = 0x23; assert_eq!(CPU::ld_b_e(&mut cpu, &mut bus), 4); assert_eq!(cpu.B, 0x23);
    cpu.H = 0x24; assert_eq!(CPU::ld_b_h(&mut cpu, &mut bus), 4); assert_eq!(cpu.B, 0x24);
    cpu.L = 0x25; assert_eq!(CPU::ld_b_l(&mut cpu, &mut bus), 4); assert_eq!(cpu.B, 0x25);
    cpu.write_HL(0xC500); bus.write_byte(0xC500, 0x26); assert_eq!(CPU::ld_b_hl(&mut cpu, &mut bus), 8); assert_eq!(cpu.B, 0x26);
    cpu.A = 0x27; assert_eq!(CPU::ld_b_a(&mut cpu, &mut bus), 4); assert_eq!(cpu.B, 0x27);
}

#[test]
fn test_ld_c_ops() {
    let (mut cpu, mut bus) = make();
    cpu.C = 0x10; assert_eq!(CPU::ld_c_c(&mut cpu, &mut bus), 4); assert_eq!(cpu.C, 0x10);
    cpu.B = 0x31; assert_eq!(CPU::ld_c_b(&mut cpu, &mut bus), 4); assert_eq!(cpu.C, 0x31);
    cpu.D = 0x32; assert_eq!(CPU::ld_c_d(&mut cpu, &mut bus), 4); assert_eq!(cpu.C, 0x32);
    cpu.E = 0x33; assert_eq!(CPU::ld_c_e(&mut cpu, &mut bus), 4); assert_eq!(cpu.C, 0x33);
    cpu.H = 0x34; assert_eq!(CPU::ld_c_h(&mut cpu, &mut bus), 4); assert_eq!(cpu.C, 0x34);
    cpu.L = 0x35; assert_eq!(CPU::ld_c_l(&mut cpu, &mut bus), 4); assert_eq!(cpu.C, 0x35);
    cpu.write_HL(0xC510); bus.write_byte(0xC510, 0x36); assert_eq!(CPU::ld_c_hl(&mut cpu, &mut bus), 8); assert_eq!(cpu.C, 0x36);
    cpu.A = 0x37; assert_eq!(CPU::ld_c_a(&mut cpu, &mut bus), 4); assert_eq!(cpu.C, 0x37);
}

#[test]
fn test_ld_d_ops() {
    let (mut cpu, mut bus) = make();
    cpu.D = 0x10; assert_eq!(CPU::ld_d_d(&mut cpu, &mut bus), 4); assert_eq!(cpu.D, 0x10);
    cpu.B = 0x41; assert_eq!(CPU::ld_d_b(&mut cpu, &mut bus), 4); assert_eq!(cpu.D, 0x41);
    cpu.C = 0x42; assert_eq!(CPU::ld_d_c(&mut cpu, &mut bus), 4); assert_eq!(cpu.D, 0x42);
    cpu.E = 0x43; assert_eq!(CPU::ld_d_e(&mut cpu, &mut bus), 4); assert_eq!(cpu.D, 0x43);
    cpu.H = 0x44; assert_eq!(CPU::ld_d_h(&mut cpu, &mut bus), 4); assert_eq!(cpu.D, 0x44);
    cpu.L = 0x45; assert_eq!(CPU::ld_d_l(&mut cpu, &mut bus), 4); assert_eq!(cpu.D, 0x45);
    cpu.write_HL(0xC520); bus.write_byte(0xC520, 0x46); assert_eq!(CPU::ld_d_hl(&mut cpu, &mut bus), 8); assert_eq!(cpu.D, 0x46);
    cpu.A = 0x47; assert_eq!(CPU::ld_d_a(&mut cpu, &mut bus), 4); assert_eq!(cpu.D, 0x47);
}

#[test]
fn test_ld_e_ops() {
    let (mut cpu, mut bus) = make();
    cpu.E = 0x10; assert_eq!(CPU::ld_e_e(&mut cpu, &mut bus), 4); assert_eq!(cpu.E, 0x10);
    cpu.B = 0x51; assert_eq!(CPU::ld_e_b(&mut cpu, &mut bus), 4); assert_eq!(cpu.E, 0x51);
    cpu.C = 0x52; assert_eq!(CPU::ld_e_c(&mut cpu, &mut bus), 4); assert_eq!(cpu.E, 0x52);
    cpu.D = 0x53; assert_eq!(CPU::ld_e_d(&mut cpu, &mut bus), 4); assert_eq!(cpu.E, 0x53);
    cpu.H = 0x54; assert_eq!(CPU::ld_e_h(&mut cpu, &mut bus), 4); assert_eq!(cpu.E, 0x54);
    cpu.L = 0x55; assert_eq!(CPU::ld_e_l(&mut cpu, &mut bus), 4); assert_eq!(cpu.E, 0x55);
    cpu.write_HL(0xC530); bus.write_byte(0xC530, 0x56); assert_eq!(CPU::ld_e_hl(&mut cpu, &mut bus), 8); assert_eq!(cpu.E, 0x56);
    cpu.A = 0x57; assert_eq!(CPU::ld_e_a(&mut cpu, &mut bus), 4); assert_eq!(cpu.E, 0x57);
}

#[test]
fn test_ld_h_ops() {
    let (mut cpu, mut bus) = make();
    cpu.H = 0x10; assert_eq!(CPU::ld_h_h(&mut cpu, &mut bus), 4); assert_eq!(cpu.H, 0x10);
    cpu.B = 0x61; assert_eq!(CPU::ld_h_b(&mut cpu, &mut bus), 4); assert_eq!(cpu.H, 0x61);
    cpu.C = 0x62; assert_eq!(CPU::ld_h_c(&mut cpu, &mut bus), 4); assert_eq!(cpu.H, 0x62);
    cpu.D = 0x63; assert_eq!(CPU::ld_h_d(&mut cpu, &mut bus), 4); assert_eq!(cpu.H, 0x63);
    cpu.E = 0x64; assert_eq!(CPU::ld_h_e(&mut cpu, &mut bus), 4); assert_eq!(cpu.H, 0x64);
    cpu.L = 0x65; assert_eq!(CPU::ld_h_l(&mut cpu, &mut bus), 4); assert_eq!(cpu.H, 0x65);
    cpu.write_HL(0xC540); bus.write_byte(0xC540, 0x66); assert_eq!(CPU::ld_h_hl(&mut cpu, &mut bus), 8); assert_eq!(cpu.H, 0x66);
    cpu.A = 0x67; assert_eq!(CPU::ld_h_a(&mut cpu, &mut bus), 4); assert_eq!(cpu.H, 0x67);
}

#[test]
fn test_ld_l_ops() {
    let (mut cpu, mut bus) = make();
    cpu.L = 0x10; assert_eq!(CPU::ld_l_l(&mut cpu, &mut bus), 4); assert_eq!(cpu.L, 0x10);
    cpu.B = 0x71; assert_eq!(CPU::ld_l_b(&mut cpu, &mut bus), 4); assert_eq!(cpu.L, 0x71);
    cpu.C = 0x72; assert_eq!(CPU::ld_l_c(&mut cpu, &mut bus), 4); assert_eq!(cpu.L, 0x72);
    cpu.D = 0x73; assert_eq!(CPU::ld_l_d(&mut cpu, &mut bus), 4); assert_eq!(cpu.L, 0x73);
    cpu.E = 0x74; assert_eq!(CPU::ld_l_e(&mut cpu, &mut bus), 4); assert_eq!(cpu.L, 0x74);
    cpu.H = 0x75; assert_eq!(CPU::ld_l_h(&mut cpu, &mut bus), 4); assert_eq!(cpu.L, 0x75);
    cpu.write_HL(0xC550); bus.write_byte(0xC550, 0x76); assert_eq!(CPU::ld_l_hl(&mut cpu, &mut bus), 8); assert_eq!(cpu.L, 0x76);
    cpu.A = 0x77; assert_eq!(CPU::ld_l_a(&mut cpu, &mut bus), 4); assert_eq!(cpu.L, 0x77);
}

#[test]
fn test_ld_a_ops() {
    let (mut cpu, mut bus) = make();
    cpu.A = 0x10; assert_eq!(CPU::ld_a_a(&mut cpu, &mut bus), 4); assert_eq!(cpu.A, 0x10);
    cpu.B = 0x81; assert_eq!(CPU::ld_a_b(&mut cpu, &mut bus), 4); assert_eq!(cpu.A, 0x81);
    cpu.C = 0x82; assert_eq!(CPU::ld_a_c(&mut cpu, &mut bus), 4); assert_eq!(cpu.A, 0x82);
    cpu.D = 0x83; assert_eq!(CPU::ld_a_d(&mut cpu, &mut bus), 4); assert_eq!(cpu.A, 0x83);
    cpu.E = 0x84; assert_eq!(CPU::ld_a_e(&mut cpu, &mut bus), 4); assert_eq!(cpu.A, 0x84);
    cpu.H = 0x85; assert_eq!(CPU::ld_a_h(&mut cpu, &mut bus), 4); assert_eq!(cpu.A, 0x85);
    cpu.L = 0x86; assert_eq!(CPU::ld_a_l(&mut cpu, &mut bus), 4); assert_eq!(cpu.A, 0x86);
    cpu.write_HL(0xC560); bus.write_byte(0xC560, 0x87); assert_eq!(CPU::ld_a_hl(&mut cpu, &mut bus), 8); assert_eq!(cpu.A, 0x87);
}

// Memory/Immediate/16-bit tests
#[test]
fn test_hl_memory_and_immediates() {
    let (mut cpu, mut bus) = make();
    // ld_bc_a / ld_de_a
    cpu.write_BC(0xC600); cpu.A = 0x9A; assert_eq!(CPU::ld_bc_a(&mut cpu, &mut bus), 8); assert_eq!(bus.read_byte(0xC600), 0x9A);
    cpu.write_DE(0xC601); cpu.A = 0x8B; assert_eq!(CPU::ld_de_a(&mut cpu, &mut bus), 8); assert_eq!(bus.read_byte(0xC601), 0x8B);

    // ld_a_bc / ld_a_de
    cpu.write_BC(0xC602); bus.write_byte(0xC602, 0x7C); assert_eq!(CPU::ld_a_bc(&mut cpu, &mut bus), 8); assert_eq!(cpu.A, 0x7C);
    cpu.write_DE(0xC603); bus.write_byte(0xC603, 0x6D); assert_eq!(CPU::ld_a_de(&mut cpu, &mut bus), 8); assert_eq!(cpu.A, 0x6D);

    // ld_hli_a and ld_a_hli
    cpu.write_HL(0xC604); cpu.A = 0x5E; assert_eq!(CPU::ld_hli_a(&mut cpu, &mut bus), 8); assert_eq!(bus.read_byte(0xC604), 0x5E); assert_eq!(cpu.HL(), 0xC605);
    bus.write_byte(0xC605, 0x4F); assert_eq!(CPU::ld_a_hli(&mut cpu, &mut bus), 8); assert_eq!(cpu.A, 0x4F);

    // ld_hld_a and ld_a_hld
    cpu.write_HL(0xC700); cpu.A = 0x3A; assert_eq!(CPU::ld_hld_a(&mut cpu, &mut bus), 8); assert_eq!(bus.read_byte(0xC700), 0x3A); assert_eq!(cpu.HL(), 0xC6FF);
    bus.write_byte(0xC6FF, 0x2B); assert_eq!(CPU::ld_a_hld(&mut cpu, &mut bus), 8); assert_eq!(cpu.A, 0x2B);

    // ld_hl_n8 and ld_a_n8 etc (immediates)
    cpu.PC = 0xC000;
    cpu.write_HL(0xC710);
    bus.write_byte(0xC000, 0xAA);
    assert_eq!(CPU::ld_hl_n8(&mut cpu, &mut bus), 12);
    assert_eq!(bus.read_byte(0xC710), 0xAA);

    cpu.PC = 0xC000;
    bus.write_byte(0xC000, 0xBB);
    assert_eq!(CPU::ld_a_n8(&mut cpu, &mut bus), 8);
    assert_eq!(cpu.A, 0xBB);
}

#[test]
fn test_16bit_loads() {
    let (mut cpu, mut bus) = make();
    cpu.PC = 0xC800; bus.write_byte(0xC800, 0x34); bus.write_byte(0xC801, 0x12); assert_eq!(CPU::ld_bc_n16(&mut cpu, &mut bus), 12); assert_eq!(cpu.BC(), 0x1234);
    cpu.PC = 0xC802; bus.write_byte(0xC802, 0x01); bus.write_byte(0xC803, 0x02); assert_eq!(CPU::ld_de_n16(&mut cpu, &mut bus), 12); assert_eq!(cpu.DE(), 0x0201);
    cpu.PC = 0xC804; bus.write_byte(0xC804, 0xFE); bus.write_byte(0xC805, 0xDC); assert_eq!(CPU::ld_hl_n16(&mut cpu, &mut bus), 12); assert_eq!(cpu.HL(), 0xDCFE);
    cpu.PC = 0xC806; bus.write_byte(0xC806, 0xAA); bus.write_byte(0xC807, 0xBB); assert_eq!(CPU::ld_sp_n16(&mut cpu, &mut bus), 12); assert_eq!(cpu.SP, 0xBBAA);

    // ld_n16_sp
    cpu.PC = 0xC808; cpu.SP = 0xBEEF; bus.write_byte(0xC808, 0x00); bus.write_byte(0xC809, 0xC8); assert_eq!(CPU::ld_n16_sp(&mut cpu, &mut bus), 20);
    let addr = 0xC800u16; assert_eq!(bus.read_byte(addr), (cpu.PC & 0xFF) as u8); assert_eq!(bus.read_byte(addr + 1), (cpu.PC >> 8) as u8);

    // ld_hl_sp_e8 / ld_sp_hl
    cpu.SP = 0x0100; cpu.PC = 0xC810; bus.write_byte(0xC810, 0x02); assert_eq!(CPU::ld_hl_sp_e8(&mut cpu, &mut bus), 12); assert_eq!(cpu.HL(), 0x0102);
    cpu.write_HL(0xDEAD); assert_eq!(CPU::ld_sp_hl(&mut cpu, &mut bus), 12); assert_eq!(cpu.SP, 0xDEAD);
}

