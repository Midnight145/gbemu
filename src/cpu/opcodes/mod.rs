use crate::cpu::Bus::Bus;
use crate::cpu::CPU::CPU;

mod load;
mod math;
mod jump;
mod ctrl;
mod bitops;
mod prefix;

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

const PREFIXED_OPCODES: [fn(&mut CPU, &mut Bus) -> u8;256] = [
    CPU::rlc_b, CPU::rlc_c, CPU::rlc_d, CPU::rlc_e, CPU::rlc_h, CPU::rlc_l, CPU::rlc_hl, CPU::rlc_a,
    CPU::rrc_b, CPU::rrc_c, CPU::rrc_d, CPU::rrc_e, CPU::rrc_h, CPU::rrc_l, CPU::rrc_hl, CPU::rrc_a,

    CPU::rl_b, CPU::rl_c, CPU::rl_d, CPU::rl_e, CPU::rl_h, CPU::rl_l, CPU::rl_hl, CPU::rl_a,
    CPU::rr_b, CPU::rr_c, CPU::rr_d, CPU::rr_e, CPU::rr_h, CPU::rr_l, CPU::rr_hl, CPU::rr_a,

    CPU::sla_b, CPU::sla_c, CPU::sla_d, CPU::sla_e, CPU::sla_h, CPU::sla_l, CPU::sla_hl, CPU::sla_a,
    CPU::sra_b, CPU::sra_c, CPU::sra_d, CPU::sra_e, CPU::sra_h, CPU::sra_l, CPU::sra_hl, CPU::sra_a,

    CPU::swap_b, CPU::swap_c, CPU::swap_d, CPU::swap_e, CPU::swap_h, CPU::swap_l, CPU::swap_hl, CPU::swap_a,
    CPU::srl_b, CPU::srl_c, CPU::srl_d, CPU::srl_e, CPU::srl_h, CPU::srl_l, CPU::srl_hl, CPU::srl_a,

    CPU::bit0_b, CPU::bit0_c, CPU::bit0_d, CPU::bit0_e, CPU::bit0_h, CPU::bit0_l, CPU::bit0_hl, CPU::bit0_a,
    CPU::bit1_b, CPU::bit1_c, CPU::bit1_d, CPU::bit1_e, CPU::bit1_h, CPU::bit1_l, CPU::bit1_hl, CPU::bit1_a,
    CPU::bit2_b, CPU::bit2_c, CPU::bit2_d, CPU::bit2_e, CPU::bit2_h, CPU::bit2_l, CPU::bit2_hl, CPU::bit2_a,
    CPU::bit3_b, CPU::bit3_c, CPU::bit3_d, CPU::bit3_e, CPU::bit3_h, CPU::bit3_l, CPU::bit3_hl, CPU::bit3_a,
    CPU::bit4_b, CPU::bit4_c, CPU::bit4_d, CPU::bit4_e, CPU::bit4_h, CPU::bit4_l, CPU::bit4_hl, CPU::bit4_a,
    CPU::bit5_b, CPU::bit5_c, CPU::bit5_d, CPU::bit5_e, CPU::bit5_h, CPU::bit5_l, CPU::bit5_hl, CPU::bit5_a,
    CPU::bit6_b, CPU::bit6_c, CPU::bit6_d, CPU::bit6_e, CPU::bit6_h, CPU::bit6_l, CPU::bit6_hl, CPU::bit6_a,
    CPU::bit7_b, CPU::bit7_c, CPU::bit7_d, CPU::bit7_e, CPU::bit7_h, CPU::bit7_l, CPU::bit7_hl, CPU::bit7_a,

    CPU::res0_b, CPU::res0_c, CPU::res0_d, CPU::res0_e, CPU::res0_h, CPU::res0_l, CPU::res0_hl, CPU::res0_a,
    CPU::res1_b, CPU::res1_c, CPU::res1_d, CPU::res1_e, CPU::res1_h, CPU::res1_l, CPU::res1_hl, CPU::res1_a,
    CPU::res2_b, CPU::res2_c, CPU::res2_d, CPU::res2_e, CPU::res2_h, CPU::res2_l, CPU::res2_hl, CPU::res2_a,
    CPU::res3_b, CPU::res3_c, CPU::res3_d, CPU::res3_e, CPU::res3_h, CPU::res3_l, CPU::res3_hl, CPU::res3_a,
    CPU::res4_b, CPU::res4_c, CPU::res4_d, CPU::res4_e, CPU::res4_h, CPU::res4_l, CPU::res4_hl, CPU::res4_a,
    CPU::res5_b, CPU::res5_c, CPU::res5_d, CPU::res5_e, CPU::res5_h, CPU::res5_l, CPU::res5_hl, CPU::res5_a,
    CPU::res6_b, CPU::res6_c, CPU::res6_d, CPU::res6_e, CPU::res6_h, CPU::res6_l, CPU::res6_hl, CPU::res6_a,
    CPU::res7_b, CPU::res7_c, CPU::res7_d, CPU::res7_e, CPU::res7_h, CPU::res7_l, CPU::res7_hl, CPU::res7_a,

    CPU::set0_b, CPU::set0_c, CPU::set0_d, CPU::set0_e, CPU::set0_h, CPU::set0_l, CPU::set0_hl, CPU::set0_a,
    CPU::set1_b, CPU::set1_c, CPU::set1_d, CPU::set1_e, CPU::set1_h, CPU::set1_l, CPU::set1_hl, CPU::set1_a,
    CPU::set2_b, CPU::set2_c, CPU::set2_d, CPU::set2_e, CPU::set2_h, CPU::set2_l, CPU::set2_hl, CPU::set2_a,
    CPU::set3_b, CPU::set3_c, CPU::set3_d, CPU::set3_e, CPU::set3_h, CPU::set3_l, CPU::set3_hl, CPU::set3_a,
    CPU::set4_b, CPU::set4_c, CPU::set4_d, CPU::set4_e, CPU::set4_h, CPU::set4_l, CPU::set4_hl, CPU::set4_a,
    CPU::set5_b, CPU::set5_c, CPU::set5_d, CPU::set5_e, CPU::set5_h, CPU::set5_l, CPU::set5_hl, CPU::set5_a,
    CPU::set6_b, CPU::set6_c, CPU::set6_d, CPU::set6_e, CPU::set6_h, CPU::set6_l, CPU::set6_hl, CPU::set6_a,
    CPU::set7_b, CPU::set7_c, CPU::set7_d, CPU::set7_e, CPU::set7_h, CPU::set7_l, CPU::set7_hl, CPU::set7_a,
];