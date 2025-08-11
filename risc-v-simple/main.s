warning: function `modulo_m_extension` is never used
  --> src/main.rs:66:4
   |
66 | fn modulo_m_extension(a: u32, b: u32) -> u32 {
   |    ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default


risc-v-simple:	file format elf32-littleriscv

Disassembly of section .text:

000110d4 <_start>:
   110d4: 20000537     	lui	a0, 0x20000
   110d8: 00052503     	lw	a0, 0x0(a0)
   110dc: 00051663     	bnez	a0, 0x110e8 <_start+0x14>
   110e0: 00a00513     	li	a0, 0xa
   110e4: 0200006f     	j	0x11104 <_start+0x30>
   110e8: ccccd5b7     	lui	a1, 0xccccd
   110ec: ccd58593     	addi	a1, a1, -0x333
   110f0: 02b535b3     	mulhu	a1, a0, a1
   110f4: 0045d593     	srli	a1, a1, 0x4
   110f8: 01400613     	li	a2, 0x14
   110fc: 02c585b3     	mul	a1, a1, a2
   11100: 40b50533     	sub	a0, a0, a1
   11104: ff010113     	addi	sp, sp, -0x10
   11108: 00112623     	sw	ra, 0xc(sp)
   1110c: 00000097     	auipc	ra, 0x0
   11110: 038080e7     	jalr	0x38(ra) <risc_v_simple::fibonacci::h58cebde3e6757506>
   11114: 00151593     	slli	a1, a0, 0x1
   11118: aaaab637     	lui	a2, 0xaaaab
   1111c: aab60613     	addi	a2, a2, -0x555
   11120: 02c5b633     	mulhu	a2, a1, a2
   11124: 00165613     	srli	a2, a2, 0x1
   11128: 100006b7     	lui	a3, 0x10000
   1112c: 00a6a023     	sw	a0, 0x0(a3)
   11130: 00b6a223     	sw	a1, 0x4(a3)
   11134: 00c6a423     	sw	a2, 0x8(a3)
   11138: 00c12083     	lw	ra, 0xc(sp)
   1113c: 01010113     	addi	sp, sp, 0x10
   11140: 0000006f     	j	0x11140 <_start+0x6c>

00011144 <risc_v_simple::fibonacci::h58cebde3e6757506>:
   11144: 00200613     	li	a2, 0x2
   11148: 04c56c63     	bltu	a0, a2, 0x111a0 <risc_v_simple::fibonacci::h58cebde3e6757506+0x5c>
   1114c: ff010113     	addi	sp, sp, -0x10
   11150: 00a12223     	sw	a0, 0x4(sp)
   11154: 00410513     	addi	a0, sp, 0x4
   11158: 00412583     	lw	a1, 0x4(sp)
   1115c: 00100513     	li	a0, 0x1
   11160: 02c5ee63     	bltu	a1, a2, 0x1119c <risc_v_simple::fibonacci::h58cebde3e6757506+0x58>
   11164: 00000713     	li	a4, 0x0
   11168: 00200613     	li	a2, 0x2
   1116c: 00100513     	li	a0, 0x1
   11170: 00810693     	addi	a3, sp, 0x8
   11174: 00c10793     	addi	a5, sp, 0xc
   11178: 00a70833     	add	a6, a4, a0
   1117c: 00a12423     	sw	a0, 0x8(sp)
   11180: 00812703     	lw	a4, 0x8(sp)
   11184: 01012623     	sw	a6, 0xc(sp)
   11188: 00c12503     	lw	a0, 0xc(sp)
   1118c: 00b67863     	bgeu	a2, a1, 0x1119c <risc_v_simple::fibonacci::h58cebde3e6757506+0x58>
   11190: 00b63833     	sltu	a6, a2, a1
   11194: 01060633     	add	a2, a2, a6
   11198: fec5f0e3     	bgeu	a1, a2, 0x11178 <risc_v_simple::fibonacci::h58cebde3e6757506+0x34>
   1119c: 01010113     	addi	sp, sp, 0x10
   111a0: 00008067     	ret
