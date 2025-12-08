use crate::cpu::Bus::Bus;
use crate::cpu::CPU::CPU;

mod load;
mod math;
mod jump;
mod ctrl;
mod bitops;

const OPCODES: [fn(&mut CPU, &mut Bus) -> u8;256] = [
    CPU::nop, CPU::ld_bc_n16, CPU::ld_bc_a, CPU::inc_bc, CPU::inc_b, CPU::dec_b, CPU::ld_b_n8, CPU::rlca,
    CPU::ld_n16_sp, CPU::add_hl_bc, CPU::ld_a_bc, CPU::dec_bc, CPU::inc_c, CPU::dec_c, CPU::ld_c_n8, CPU::rrca,

    CPU::stop_n8, CPU::ld_de_n16, CPU::ld_de_a, CPU::inc_de, CPU::inc_d, CPU::dec_d, CPU::ld_d_n8, CPU::rla,
    CPU::jr_e8, CPU::add_hl_de, CPU::ld_a_de, CPU::dec_de, CPU::inc_e, CPU::dec_e, CPU::ld_e_n8, CPU::rra,

    CPU::jr_nz_e8, CPU::ld_hl_n16, CPU::ld_hli_a, CPU::inc_hl, CPU::inc_h, CPU::dec_h, CPU::ld_h_n8, CPU::daa,
    CPU::jr_z_e8, CPU::add_hl_hl, CPU::ld_a_hli, CPU::dec_hl, CPU::inc_l, CPU::dec_l, CPU::ld_l_n8, CPU::cpl,

    CPU::jr_nc_e8, CPU::ld_sp_n16, CPU::ld_hld_a, CPU::inc_sp, CPU::inc_hl_ptr, CPU::dec_hl_ptr, CPU::ld_hl_n8, CPU::scf,
    CPU::jr_c_e8, CPU::add_hl_sp, CPU::ld_a_hld, CPU::dec_sp, CPU::inc_a, CPU::dec_a, CPU::ld_a_n8, CPU::ccf,

    CPU::ld_b_b, CPU::ld_b_c, CPU::ld_b_d, CPU::ld_b_e, CPU::ld_b_h, CPU::ld_b_l, CPU::ld_b_hl, CPU::ld_b_a,
    CPU::ld_c_b, CPU::ld_c_c, CPU::ld_c_d, CPU::ld_c_e, CPU::ld_c_h, CPU::ld_c_l, CPU::ld_c_hl, CPU::ld_c_a,

    CPU::ld_d_b, CPU::ld_d_c, CPU::ld_d_d, CPU::ld_d_e, CPU::ld_d_h, CPU::ld_d_l, CPU::ld_d_hl, CPU::ld_d_a,
    CPU::ld_e_b, CPU::ld_e_c, CPU::ld_e_d, CPU::ld_e_e, CPU::ld_e_h, CPU::ld_e_l, CPU::ld_e_hl, CPU::ld_e_a,

    CPU::ld_h_b, CPU::ld_h_c, CPU::ld_h_d, CPU::ld_h_e, CPU::ld_h_h, CPU::ld_h_l, CPU::ld_h_hl, CPU::ld_h_a,
    CPU::ld_l_b, CPU::ld_l_c, CPU::ld_l_d, CPU::ld_l_e, CPU::ld_l_h, CPU::ld_l_l, CPU::ld_l_hl, CPU::ld_l_a,

    CPU::ld_hl_b, CPU::ld_hl_c, CPU::ld_hl_d, CPU::ld_hl_e, CPU::ld_hl_h, CPU::ld_hl_l, CPU::halt, CPU::ld_hl_a,
    CPU::ld_a_b, CPU::ld_a_c, CPU::ld_a_d, CPU::ld_a_e, CPU::ld_a_h, CPU::ld_a_l, CPU::ld_a_hl, CPU::ld_a_a,

    CPU::add_a_b, CPU::add_a_c, CPU::add_a_d, CPU::add_a_e, CPU::add_a_h, CPU::add_a_l, CPU::add_a_hl, CPU::add_a_a,
    CPU::adc_a_b, CPU::adc_a_c, CPU::adc_a_d, CPU::adc_a_e, CPU::adc_a_h, CPU::adc_a_l, CPU::adc_a_hl, CPU::adc_a_a,

    CPU::sub_a_b, CPU::sub_a_c, CPU::sub_a_d, CPU::sub_a_e, CPU::sub_a_h, CPU::sub_a_l, CPU::sub_a_hl, CPU::sub_a_a,
    CPU::sbc_a_b, CPU::sbc_a_c, CPU::sbc_a_d, CPU::sbc_a_e, CPU::sbc_a_h, CPU::sbc_a_l, CPU::sbc_a_hl, CPU::sbc_a_a,

    CPU::and_a_b, CPU::and_a_c, CPU::and_a_d, CPU::and_a_e, CPU::and_a_h, CPU::and_a_l, CPU::and_a_hl, CPU::and_a_a,
    CPU::xor_a_b, CPU::xor_a_c, CPU::xor_a_d, CPU::xor_a_e, CPU::xor_a_h, CPU::xor_a_l, CPU::xor_a_hl, CPU::xor_a_a,

    CPU::or_a_b, CPU::or_a_c, CPU::or_a_d, CPU::or_a_e, CPU::or_a_h, CPU::or_a_l, CPU::or_a_hl, CPU::or_a_a,
    CPU::cp_a_b, CPU::cp_a_c, CPU::cp_a_d, CPU::cp_a_e, CPU::cp_a_h, CPU::cp_a_l, CPU::cp_a_hl, CPU::cp_a_a,

    CPU::ret_nz, CPU::pop_bc, CPU::jp_nz_a16, CPU::jp_a16, CPU::call_nz_a16, CPU::push_bc, CPU::add_a_u8, CPU::rst_00,
    CPU::ret_z, CPU::ret, CPU::jp_z_a16, CPU::prefix, CPU::call_z_a16, CPU::call_a16, CPU::adc_a_u8, CPU::rst_08,

    CPU::ret_nc, CPU::pop_de, CPU::jp_nc_a16, CPU::invalid, CPU::call_nc_a16, CPU::push_de, CPU::sub_a_u8, CPU::rst_10,
    CPU::ret_c, CPU::reti, CPU::jp_c_a16, CPU::invalid, CPU::call_c_a16, CPU::invalid, CPU::sbc_a_u8, CPU::rst_18,

    CPU::ldh_a8_a, CPU::pop_hl, CPU::ldh_c_a, CPU::invalid, CPU::invalid, CPU::push_hl, CPU::and_a_u8, CPU::rst_20,
    CPU::add_sp_e8, CPU::jp_hl, CPU::ld_a16_a, CPU::invalid, CPU::invalid, CPU::invalid, CPU::xor_a_u8, CPU::rst_28,

    CPU::ldh_a_a8, CPU::pop_af, CPU::ldh_a_c, CPU::di, CPU::invalid, CPU::push_af, CPU::or_a_u8, CPU::rst_30,
    CPU::ld_hl_sp_e8, CPU::ld_sp_hl, CPU::ld_a_a16, CPU::ei, CPU::invalid, CPU::invalid, CPU::cp_a_u8, CPU::rst_38
];