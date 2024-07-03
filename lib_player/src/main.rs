use std::{
    thread::{self, sleep},
    time::{Duration, Instant},
};
use lib_player::{Synth, Parser};

fn main() {
    let mml_1 = "t82v8o5C8C16<B16>C16<G8G16>D16C8D16E8r16C16G16E16E16E16D8D16E16D8C16<B16>C8r16<G16>C16C16C16C16C16C16<G16G16>D16E16D16C16D16E16D16C16F8E16E16D8C16G8.&G32.r64E16D8r8C16C16C16C16C8r16<G16>D8C16D16E8r16<G16>C8C8C16<G8G8A16G16G16G8G8>C16C16C16C16C16<G16r16>D8D16C16D16E16<G16G16G16>C16C16C16C16C16C16C16C16D16E16D16C16C8C8G8E8D16C16C16C16A8D16E16D16C16r16C16B8A8B16>C8C8.&C32.r64<C16E16D16C16D16C8.&C32.r64G8.&G32.r64E16D16D8.&D32.r8&r64D8C8<B8&B32.r64>C4&C32.r4&r64G8&G32.r64G8C8&C32.r64D8E8C8r8G8G16G8C16C8A8G8E8D16C16G8G16G16G16C16C16C16D8E8D16C16D16C16C16C16D8r8C8C16D16E16D16C8.&C32.r64G16G16G16G16G16C8C16D8E8C8r16<G16>D16G16G16G8C16C16C8A16G8E8D16C16G8G8v9G16C16C16C16D8D16E16D8v10C8D8C8D8v11E8<G8.&G32.:B8.&B32.:>G8.&G32.v12r4&r64<G8:>E8:G8E16<G8:>E8:G8E8<G8&G32.:>E8&E32.:G8&G32.r64C8:>C8o4B8:>B8<G8:>G8<A8:>A8<G16:>G16<G8:>G8<G8:>E8<G8&G32.:>E8&E32.r64<A8:>A8<G8:>G8<G8:>E8<E8:G8:>C8r8D8E16D8C16C8<A8>C8r4C16D16E16D8C16C8<A8>C8<G8:>E8:G8E16<G8:>E8:G8E8<G8&G32.:>E8&E32.:G8&G32.r64C8:>C8o4B8:>B8<G8:>G8<A8:>A8<G16:>G16<G8:>G8<G8:>E8E16<A8:>A8<G8:>G8<G16:>E16D16<G16:>C16D16<E8:G8:>C8r8C16D16E16D8C16C8<A8>C8r8<G8>C16E8D8C16C8F8E8v11r8.<G16>C16D16E16D8C16C16<A16v10A16>C16C8<G8>E16D8v9C16D16C32D32C8F8E8v8C8r8<G8E8:G8:>C8r4E16F16C8:G8C8r8<G8D8:B8r4>C16<G16>C16D16E8r8<G8E8:G8:>C8v7r4C16:E16D16:F16E16.:G16.r32F16.:A16.v8r8&r32G64A16.r64G8F8E16&E64E64F64r64G32.F32.E32D8&D32r16.C16C16<B8>C16<G8G16>D16C8D16E8r16C16G16E16E16E16D8D16D+16D8C16<B16>C8r16<G16>C16C16C16C16C16C16<G16G16>D16E16D16C16D16E16D16C16F8E16E16D16&D64C32.C16<G16>E8&E32.r64F32E32D16C16r16<G16>C16C16C16C16C8r16<G16>D8C16D16E8r16<G16>C16C16C16C16C16<G16G16G16G16A16G16G16G8G8>C16C16C16C16C16<G16r16>G8C16C16D16E8r16<G16>C16C16C16C16C16C16C16C16D16E16D16C16C8C8G8E8D16C16C16C16A8D16E16D16C16r16C16B8A8B8&B32.r64>C4&C32.r64<E16D16C16D16C8&C32.r64<G16>G8.&G32.r64E16D16D8.&D32.r16&r64<G16>D8C8<B16>C16C4&C16.&C64r8&r64C8G8G16G8C16C16C16D16D16E8C8r16<G16>C16G16G16G16C16C16C16C16A16A16G8E8D16C16G8G16G16G16C16C8D16D16E8D16C16C8<B16B16r4>C8C16D16E16D16C8.&C32.r64G16:C16:E16G8G16C16C8C16v6<B16:v8>D16D16v6C8:v8E8v6<A16:v8>C16C16<B16>C16D16:<B16>G16G16G16G16C16C8C16A16G8E8D16C16G16G16G16G16v9G8C8D8E8D16v10C16D16C16D8C8D8v11E8<G8:B8:>G8D16G16B16>D16v12<G8:>G8o4G8:>E8:G8E16<G8:>E8:G8E8<G8&G32.:>E8&E32.:G8&G32.r64C8:A8:>C8o4B8:>G8:B8<G8:>E8:G8<A8:>D8:A8<G16:>G16<G8:>G8<G8:>E8<G8&G32.:>E8&E32.r64<A8:>A8<G8:>G8<G8:>E8<E8:G8:>C8r8D8E16<F8:>D8C16<G8:>C8<A8>C8r8<G16G16>C16D16E16<G8:>D8C16<E8:>C8<A8>C8<G8:>E8:G8E16<G8:>E8:G8E8<G8&G32.:>E8&E32.:G8&G32.r64C8:>C8o4B8:>B8<G8:>G8<A8:>F8:A8<G16:>E16:G16<G8:>E8:G8<G8:>E8<A8&A32.:>A8&A32.r64<G8:>G8<G16:>E16D16<G16:>C16D16<E8:G8:>C8r8C16D16E16<F8&F32.:>D8&D32.r64<G8:>C8<A8>C8r8<G16G16>C16D16E16<G8:>D8C16C16C16<A8:>F8<G8:>E8v11r8.<G16>C16D16E16D8C16C16<A16v10A16>C16C8<G8>E16D8v9C16D16C32D32C8F8E8v8C8<C16<B16>C16E8C16<B16>C16E8.&E32.r64F8.&F32.r64C16<B16>C16E8C16<B16>C16F8E8C8.&C32.r64<A8.&A32.:>D8.&D32.:F8.&F32.r64D8C8<B8.&B32.r64>E8D8D8C8<B8>C8E8.&E32.r64D8.&D32.r64C16<B16>C16E8C16<B16>C16E8.&E32.r64F8.&F32.r64C16<B16>C16E8C16<B16>C16E8.&E32.r64G8.&G32.r64<A4&A16.&A64:>D4&D16.&D64:F4&F16.&F64r64C8C8.&C32.r64<B8&B32.r64o5C16<G8:>G8E16G8G16A8C8G8E16D16C8D16C8&C32.r64<B8:>E8C8<G8.&G32.:>C8.&C32.r64<B8.&B32.r64G8:>G8E16G8&G32.r64A8C8G8E16D16C8D16C8&C32.r64C8:D8E8<F8:>C8C8<B8G8G8:>G8E16G8E16A8C8:G8G8r8.C16D8&D32.:B8&B32.r64B8>C16C8<E8.&E32.r64E16D16C8D16D16C16D8D8&D32.r64D16C16C8E8C8C8C16C16<B8>C16C4&C32.v11r2&r64C+16v12D+16F16v13F+16v14<G+8:>F8:G+8F16<G+8:>F8:G+8F8<G+8&G+32.:>F8&F32.:G+8&G+32.r64C+8:>C+8<C8:>C8o4G+8:>G+8<A+8:>A+8<G+16:>G+16<G+8:>G+8<G+8:>F8<G+8&G+32.:>F8&F32.r64<A+8:>A+8<G+8:>G+8<G+8:>F8<F8:G+8:>C+8r8D+8F16D+8C+16C+8<A+8>C+8C8:>C8<C+16:>C+16<C+16:>C+16<C+16:>C+16<C+8:G+8D+8&D+32.:>D+8&D+32.r64<F8:>F8<D+16:F+16:>D+16<C+16:F16:>C+16<C+8:F8:>C+8o4G+8:>F8:G+8F16<G+8:>F8:G+8F8<G+8&G+32.:>F8&F32.:G+8&G+32.r64C+8:>C+8<C8:>C8<C+8:>C+8<C+8.&C+32.:F8.&F32.:>C+8.&C+32.r64<G+16A+16>C+16D+16<F8&F32.:G+8&G+32.:>F8&F32.r64<F16D+16C+8<A+16F8:G+8:>C+8<A+16>C+8D+8<G+8&G+32.:>F8&F32.r64<A+8:>F+8<G+8:>F8<F8:>C+8C+8.&C+32.:>C+8.&C+32.r64<C8.&C32.:>C8.&C32.r64<C8:>C8<C+8:>C+8<D+8:>D+8<G+8:>G+8o4F8:G+8:>C+8r8C+16D+16F16<F+8:>D+8<F16:>C+16<F8:>C+8<F8:A+8F8:>C+8v13r4C+16D+16F16<G+8:>D+8<G+16:>C+16<G+8:>C+8v12<A+8:>F+8<G+8:>F8<F8&F32.:G+8&G+32.:>C+8&C+32.r64<G+16>C+16D+16F16D+8C+16C+8<A+8>C+8v14<G+8>F16D+8C+8C+8<G+16G+8F+8:>D+8<F8:>C+8v15F16:>F16<F+16:>F+16<G+16:>G+16<F+8:>F+8<F8:>F8<D+16:>D+16<C+8.&C+32.:>C+8.&C+32.r64o4A+8.&A+32.:>A+8.&A+32.r64F16:>F16<F+16:>F+16<G+16:>G+16<F+8:>F+8<F8:>F8<F+16:>F+16<F8.&F32.:>F8.&F32.r64o4F+8F8>F16:>F16<F+16:>F+16<G+16:>G+16<F+8:>F+8<F8:>F8<D+16:>D+16<C+8.&C+32.:>C+8.&C+32.r64<D+8:>D+8<C+8:>C+8<C+8.&C+32.:>C+8.&C+32.r64<C8.&C32.:>C8.&C32.r64<F+8.&F+32.:>F+8.&F+32.r64<F8.&F32.:>F8.&F32.r64<F16:>F16<F+16:>F+16<G+16:>G+16<F+8:>F+8<F8:>F8<D+16:>D+16<C+8.&C+32.:>C+8.&C+32.r64o4A+8.&A+32.:>A+8.&A+32.r64F16:>F16<F+16:>F+16<G+16:>G+16<F+8:>F+8<F8:>F8v14<F+16:>F+16<F8:>F8<D+8:>D+8<F8:>F8v13r8<F16:>F16<F+16:>F+16<G+16:>G+16<F+8:>F+8<F8:>F8v12<D+16:>D+16<C+8.&C+32.:>C+8.&C+32.r64<D+8.&D+32.:F+8.&F+32.:A+8.&A+32.:>C+8.&C+32.v11r64<C8<G+8F+8v10F16&F64C+64r64D+64F4.&F16";
    let mml_2 = "t82v8o3E8G8>C8:E8r8<F8A8>C8:G8r8<G8B8>D8:G8r8o2A8>E8:A8>G16C8&C32.r64<E8G8>C8:G8r8<F8A8>C8:G8r8<G8B8>D8:G8r8o2A8>E8:A8>E16C16G8<E8G8>C8:E8r8<F8A8>C8:G8r8<G8B8>D8:G8r8o2A8>E8:A8>E16C8&C32.r64<E8G8>C8:G8r8<F8A8>C8:G8r8<G8B8>D8:G8r8o2A8>E8:A8>E16C16G8<E8G8>C8:G8r8<F8A8>C8:G8r8<G8B8>D8:G8r8o2A8>E8:A8>C8E8<E8G8>C8.&C64:G8.&G64r32.<F8A8>C8.&C64:G8.&G64r32.<G8B8>D8.&D64:G8.&G64r32.<C8G8v7>C8:v8E8v7C8:v8D8<E8:>C8<G8>C8.&C64:G8.&G64r32.<F8A8>C8:E8:G8C8<G16.:B16.:>D16.r32<G16.:B16.:>D16.r32D8:G8r8o2A8>E8:A8>C8E8<E8G8>C8:G8r8<F8A8>C8:G8r8<G8>D8:G8r4o2A8>E8:A8>D16C16G8<E8:>C8:E8<E8:>C8:E8C8:G8r8<F8A8>C8:G8r8<G16.:B16.:>D16.r32<G16.:B16.:>D16.r32D8:G8r8o2A8>E8:A8>G8C8<E8G8v9>C8:G8r8<F8A8>C8:G8v10r8<G8:B8:>D8v3<G8v4G8G8v11G8.&G32.:>D8.&D32.r64o2G8:>G8v12<G8v11E8:>E8C8E8&E32.:G8&G32.:>C8&C32.r64o2F8:>F8F16:A16:>C16<F8F8:A8:>C8<F8<G8:>G8D8G8&G32.:B8&B32.:>D8&D32.r64o2A8:>A8E16:A16:>C16o2A8>E8:A8:>C8o2A8E8:>E8C8E8&E32.:G8&G32.:>C8&C32.r64o2F8:>F8F16:A16:>C16<F8F8:A8:>C8<F8<G8:>G8D8G8&G32.:B8&B32.:>D8&D32.r64o2A8:>A8E16:A16:>C16o2A8>E8:A8:>C8o2A8E8:>E8C8E8&E32.:G8&G32.:>C8&C32.r64o2F8:>F8A16:>C16<F8A8:>C8<F8<G8:>G8D8G8&G32.:B8&B32.:>D8&D32.r64o2A8:>A8E16:A16:>C16o2A8>E8:A8:>C8o2A8E8:>E8C8E8&E32.:G8&G32.:>C8&C32.r64o2F8:>F8F16:A16:>C16<F8F8:A8:>C8<F8<G8:>G8D8G8&G32.:B8&B32.:>D8&D32.r64o2A8:>A8E16:A16:>C16o2A8>E8:A8:>C8o2A8E8:>E8C8E8&E32.:G8&G32.:>C8&C32.r64o2F8:>F8v10F16:A16:>C16<F8F8:A8:>C8<F8v9r2&r16G16>F8E8v8C8<E8G8>C8r8<F8A8>C8:G8r8<G8v7G8v8G8r8<A8A8A8.&A32.:>A8.&A32.r64E8G8>C8r8<F8A8>C8:G8r8<G8B8>D8.&D64:G8.&G64r32.o2G8:>G8B8v7>D8:v8G8v7<G8v8E8G8>C8:E8r8<F8A8>C8:G8r8<G8B8>D8:G8r8o2A8>E8:A8>G16C8&C32.r64<E8G8>C8:G8r8<F8A8>C8:G8r8<G8B8>D8:G8r8o2A8>E8:A8>E16C16G16r16<E8G8>C8:E8r8<F8A8>C8:G8r8<G8B8>D8:G8r8o2A8>E8:A8>E16C8&C32.r64<E8G8>C8:G8r8<F8A8>C8:G8r8<G8B8>D8:G8r8o2A8>E8:A8>E16C16G8<E4.&E16.:G4.&G16.:>C4.&C16.r32<F4.&F16.:A4.&A16.:>C4.&C16.r32<G8.&G32.:B8.&B32.:>D8.&D32.r64<G+8.&G+32.:B8.&B32.:>E8.&E32.r64o2A8>E8:A8>C8E8o2E8:>E8G8>C16.:G16.r8&r32o2F8:>F8A8>C16.:G16.v6r16.<F16v8<G8:>G8B8>D16.:G16.r8&r32<C8G8v7>C8:v8E8v7C8:v8D8<E8:>C8<G8>C8.&C64:G8.&G64r32.<F8A8>C8:E8:G8C8<G16.r32B16.r32>D8:G8r8o2A8>E8:A8>C8:G8o2A8>E8G8>C8:G8<E8F8A8>C8:G8<F8<G8:>G8>D8:G8r4&r16o2A16>E8:A8>C8<B8<E8:>E8<E8:>E8E8:G8:>C8r16<E16<F8:>F8<F8:>F8F8:A8:>C8<F8<G8:>G8<G8:>G8G8:B8:>D8r16<G16<A8:>A8<A8:>A8A8:>C8:E8<E8E8G8v9>C8:G8<E8F8A8>C8:G8v10<F8<G8:>G8D8G8v11>C8<G8:>D8o2G8G8:>G8v12<G8v11E8:>E8C8E8&E32.:G8&G32.:>C8&C32.r64o2F8:>F8F16:A16:>C16<F8F8:A8:>C8<F8<G8:>G8D8G8&G32.:B8&B32.:>D8&D32.r64o2A8:>A8E16:A16:>C16o2A8>E8:A8:>C8o2A8E8:>E8C8E8&E32.:G8&G32.:>C8&C32.r64o2F8:>F8F16:A16:>C16<F8F8:A8:>C8<F8<G8:>G8D8G8&G32.:B8&B32.:>D8&D32.r64o2A8:>A8E16:A16:>C16o2A8>E8:A8:>C8o2A8E8:>E8C8E8&E32.:G8&G32.:>C8&C32.r64o2F8:>F8A16:>C16<F8A8:>C8<F8<G8:>G8D8G8&G32.:B8&B32.:>D8&D32.r64o2A8:>A8E16:A16:>C16o2A8>E8:A8:>C8o2A8E8:>E8C8E8&E32.:G8&G32.:>C8&C32.r64o2F8:>F8F16:A16:>C16<F8F8:A8:>C8<F8<G8:>G8D8G8&G32.:B8&B32.:>D8&D32.r64o2A8:>A8E16:A16:>C16o2A8>E8:A8:>C8o2A8E8:>E8C8E8&E32.:G8&G32.:>C8&C32.r64o2F8:>F8v10F16:A16:>C16<F8F8:A8:>C8<F8v9r2&r16G16>F8E8v8C8<F2.&F8.&F64r32.E2.&E8.&E64r32.D4.&D16.r32<G+4.&G+16.:>E4.&E16.r32<A4.&A16.r32G16v7>A16B16B16>C8<B16v8A16F4.&F16.r8&r32F8G8A8E4.&E16.r8&r32E8G8E8F4.&F16.r32<G8.&G32.:>C8.&C32.r64<B8.&B32.r64>E4.&E16.r32F4.&F16.r32G4.&G16.r32A4.&A16.r32E4.&E16.r32F4.&F16.r32G4.&G16.r32A4.&A16.r32E8G8>C8.&C32.r64<F8A8>C8.&C32.r64<G8.&G32.r64G+8.&G+32.r64A8>C8E8C8<E8G8>C8.&C32.r64<F8A8>C8.&C32.v10r1&r8.&r64<D+16v11<G+8:>G+8v12o1G+8:>G+8v13<F8:>F8>C+8F8&F32.:G+8&G+32.:>C+8&C+32.r64o1F+8:>F+8>F+8&F+32.:A+8&A+32.:>C+8&C+32.r64<F+8:A+8:>C+8<F+8o1G+8:>G+8>D+8G+8&G+32.:>C8&C32.:D+8&D+32.r64o1A+8:>A+8>F8&F32.:A+8&A+32.:>C+8&C+32.r64<F8:A+8:>C+8o2A+8F8:>F8C+8F8&F32.:G+8&G+32.:>C+8&C+32.r64o2F+8:>F+8F+16:A+16:>C+16<F+8F+8:A+8:>C+8<F+8<G+8:>G+8D+8G+8&G+32.:>C8&C32.:D+8&D+32.r64o2A+8:>A+8F16:A+16:>C+16o2A+8>F8:A+8:>C+8o2A+8F8:>F8C+8F8&F32.:G+8&G+32.:>C+8&C+32.r64o2F+8:>F+8A+16:>C+16<F+8A+8:>C+8<G+8<A+8:>A+8v14F8v13D+8:G+8:v14>C8o2G+8<A+8:>A+8v13A+8>F8:A+8:>C+8o2A+8F8:>F8C+8F8&F32.:G+8&G+32.:>C+8&C+32.r64o2F+8:>F+8F+16:A+16:>C+16<F+8F+8:A+8:>C+8<F+8<G+8:>G+8D+8G+8&G+32.:>C8&C32.:D+8&D+32.r64o2A+16:>A+16v14<A+16v13>F16:A+16:>C+16o2A+8>F8:A+8:>C+8o2A+8F8:>F8C+8F8&F32.:G+8&G+32.:>C+8&C+32.r64o2F+8:>F+8F+16:A+16:>C+16<F+8F+8:A+8:>C+8<F+8<G+8:>G+8D+8G+8&G+32.:>C8&C32.:D+8&D+32.v12r64o2A+8:>A+8F16:A+16:>C+16o2A+8>F8:A+8:>C+8o2A+8v11F4.&F16.:>F4.&F16.r32<F+4.&F+16.:>F+4.&F+16.v14r2&r8.&r32F+16<G+8:>G+8o1G+8:>G+8v15<F8:>F8>C+8F8&F32.:G+8&G+32.:>C+8&C+32.r64o1F+8:>F+8>F+16:A+16:>C+16o2F+8>F+8:A+8:>C+8<F+8o1G+8:>G+8>D+8G+8&G+32.:>C8&C32.:D+8&D+32.r64o1A+8:>A+8>F16:A+16:>C+16o2A+16r16>A+8:>C+8o2A+8F8:>F8C+8F8&F32.:G+8&G+32.:>C+8&C+32.r64o2F+8:>F+8F+16:A+16:>C+16<F+8F+8:A+8:>C+8<F+8<G+8:>G+8D+8G+8&G+32.:>C8&C32.:D+8&D+32.r64o2A+8:>A+8F16:A+16:>C+16o2A+8>F8:A+8:>C+8o2A+8F8:>F8C+8F8&F32.:G+8&G+32.:>C+8&C+32.r64o2F+8:>F+8F+16:A+16:>C+16<F+8F+8:A+8:>C+8<F+8<G+8:>G+8D+8v14G+8&G+32.:>C8&C32.:D+8&D+32.r64o2A+8:>A+8F16:A+16:>C+16o2A+8v13>F8:A+8:>C+8o2A+8F8:>F8C+8v12F8&F32.:G+8&G+32.:>C+8&C+32.r64o2F+8:>F+8F+16:A+16:>C+16<F+8v11F+8:A+8:>C+8<F+8<G+8>D+8G+8.&G+32.v10r64C+4.&C+16.:G+4.&G+16.:>C+4.&C+16.";

    let time = Instant::now();
    let parsed_1 = Parser::parse(mml_1.to_string());
    println!("parse 1: {}", time.elapsed().as_millis());

    let parsed_2 = Parser::parse(mml_2.to_string());

    let synth = Synth::new(String::from("./test_resouces/YDP-GrandPiano-SF2-20160804/YDP-GrandPiano-20160804.sf2"));
    let (_stream, connection) = synth.new_stream();

    let mut conn1 = connection.clone();
    let mut conn2 = connection.clone();

    thread::spawn(move || parsed_1.play(&mut conn1, 0));
    thread::spawn(move || parsed_2.play(&mut conn2, 1));

    sleep(Duration::from_secs(200));
}