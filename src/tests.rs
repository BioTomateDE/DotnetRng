use super::DotnetRng;

#[test]
fn size() {
    assert_eq!(size_of::<DotnetRng>(), 228);
}

#[test]
fn seed0() {
    let mut rng = DotnetRng::new(0);
    assert_eq!(rng.next(), 1_559_595_546);
    assert_eq!(rng.next(), 1_755_192_844);
    assert_eq!(rng.next(), 1_649_316_166);
    assert_eq!(rng.next(), 1_198_642_031);
    assert_eq!(rng.next(), 442_452_829);
    assert_eq!(rng.next(), 1_200_195_957);
    assert_eq!(rng.next(), 1_945_678_308);
    assert_eq!(rng.next(), 949_569_752);
    assert_eq!(rng.next(), 2_099_272_109);
    assert_eq!(rng.next(), 587_775_847);
}

#[test]
fn seed1000() {
    let mut rng = DotnetRng::new(1000);
    assert_eq!(rng.next(), 325_467_165);
    assert_eq!(rng.next(), 506_683_626);
    assert_eq!(rng.next(), 1_623_525_913);
    assert_eq!(rng.next(), 2_344_573);
    assert_eq!(rng.next(), 1_485_571_032);
    assert_eq!(rng.next(), 980_737_479);
    assert_eq!(rng.next(), 2_067_435_452);
    assert_eq!(rng.next(), 271_829_958);
    assert_eq!(rng.next(), 1_490_890_881);
    assert_eq!(rng.next(), 53_262_104);
}

#[test]
fn seed_min() {
    let mut rng = DotnetRng::new(i32::MIN);
    assert_eq!(rng.next(), 1_559_595_546);
    assert_eq!(rng.next(), 1_755_192_844);
    assert_eq!(rng.next(), 1_649_316_172);
    assert_eq!(rng.next(), 1_198_642_031);
    assert_eq!(rng.next(), 442_452_829);
    assert_eq!(rng.next(), 1_200_195_955);
    assert_eq!(rng.next(), 1_945_678_308);
    assert_eq!(rng.next(), 949_569_752);
    assert_eq!(rng.next(), 2_099_272_109);
    assert_eq!(rng.next(), 587_775_835);
}

#[test]
fn doubles() {
    let mut rng = DotnetRng::new(1225);
    assert_eq!(rng.next_f64().to_bits(), 4_604_455_517_613_236_172);
    assert_eq!(rng.next_f64().to_bits(), 4_598_267_667_770_550_314);
    assert_eq!(rng.next_f64().to_bits(), 4_583_817_318_630_030_113);
    assert_eq!(rng.next_f64().to_bits(), 4_583_079_487_540_083_974);
    assert_eq!(rng.next_f64().to_bits(), 4_598_644_753_433_843_220);
    assert_eq!(rng.next_f64().to_bits(), 4_590_695_495_783_602_806);
    assert_eq!(rng.next_f64().to_bits(), 4_605_160_136_941_609_342);
    assert_eq!(rng.next_f64().to_bits(), 4_573_073_318_620_275_688);
    assert_eq!(rng.next_f64().to_bits(), 4_602_953_608_649_110_487);
    assert_eq!(rng.next_f64().to_bits(), 4_607_126_565_782_788_711);
}

#[test]
fn ranged_small() {
    let mut rng = DotnetRng::new(187_187_187);
    assert_eq!(rng.next_ranged(69, 420), 317);
    assert_eq!(rng.next_ranged(69, 420), 229);
    assert_eq!(rng.next_ranged(69, 420), 227);
    assert_eq!(rng.next_ranged(69, 420), 181);
    assert_eq!(rng.next_ranged(69, 420), 350);
    assert_eq!(rng.next_ranged(69, 420), 320);
    assert_eq!(rng.next_ranged(69, 420), 330);
    assert_eq!(rng.next_ranged(69, 420), 267);
    assert_eq!(rng.next_ranged(69, 420), 108);
    assert_eq!(rng.next_ranged(69, 420), 400);
}

#[test]
fn ranged_large() {
    let mut rng = DotnetRng::new(20_230_807);
    assert_eq!(rng.next_ranged(-42069, i32::MAX), 1_534_921_242);
    assert_eq!(rng.next_ranged(-42069, i32::MAX), 2_038_703_413);
    assert_eq!(rng.next_ranged(-42069, i32::MAX), 998_041_784);
    assert_eq!(rng.next_ranged(-42069, i32::MAX), 1_589_122_846);
    assert_eq!(rng.next_ranged(-42069, i32::MAX), 1_789_172_735);
    assert_eq!(rng.next_ranged(-42069, i32::MAX), 629_506_782);
    assert_eq!(rng.next_ranged(-42069, i32::MAX), 723_391_659);
    assert_eq!(rng.next_ranged(-42069, i32::MAX), 1_828_598_720);
    assert_eq!(rng.next_ranged(-42069, i32::MAX), 1_160_835_804);
    assert_eq!(rng.next_ranged(-42069, i32::MAX), 1_544_888_066);
    assert_eq!(rng.next_ranged(-42069, i32::MAX), 1_039_720_213);
    assert_eq!(rng.next_ranged(-42069, i32::MAX), 280_809_207);
}

#[test]
fn bytes() {
    let mut rng = DotnetRng::new(-1337);
    let bytes: [u8; 100] = rng.next_bytes();
    assert_eq!(
        bytes,
        [
            104, 0, 244, 199, 67, 94, 2, 170, 194, 124, 79, 217, 39, 252, 34, 39, 106, 137, 84,
            178, 229, 18, 239, 30, 154, 247, 34, 126, 240, 54, 227, 40, 165, 116, 212, 193, 23,
            186, 227, 105, 199, 86, 230, 13, 79, 164, 218, 69, 90, 187, 243, 186, 246, 89, 36, 85,
            16, 214, 45, 76, 60, 132, 185, 139, 152, 38, 51, 179, 39, 97, 15, 176, 166, 235, 234,
            143, 44, 226, 206, 246, 29, 221, 35, 52, 67, 41, 50, 76, 79, 127, 177, 65, 141, 150,
            44, 67, 156, 90, 117, 41
        ]
    );
}
