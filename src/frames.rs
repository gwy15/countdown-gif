pub fn png_frame(digit: u32) -> &'static [u8] {
    if !((0..=99).contains(&digit)) {
        panic!("digit = {digit}");
    }

    match digit {
        0 => include_bytes!("frames/00.png"),
        1 => include_bytes!("frames/01.png"),
        2 => include_bytes!("frames/02.png"),
        3 => include_bytes!("frames/03.png"),
        4 => include_bytes!("frames/04.png"),
        5 => include_bytes!("frames/05.png"),
        6 => include_bytes!("frames/06.png"),
        7 => include_bytes!("frames/07.png"),
        8 => include_bytes!("frames/08.png"),
        9 => include_bytes!("frames/09.png"),
        10 => include_bytes!("frames/10.png"),
        11 => include_bytes!("frames/11.png"),
        12 => include_bytes!("frames/12.png"),
        13 => include_bytes!("frames/13.png"),
        14 => include_bytes!("frames/14.png"),
        15 => include_bytes!("frames/15.png"),
        16 => include_bytes!("frames/16.png"),
        17 => include_bytes!("frames/17.png"),
        18 => include_bytes!("frames/18.png"),
        19 => include_bytes!("frames/19.png"),
        20 => include_bytes!("frames/20.png"),
        21 => include_bytes!("frames/21.png"),
        22 => include_bytes!("frames/22.png"),
        23 => include_bytes!("frames/23.png"),
        24 => include_bytes!("frames/24.png"),
        25 => include_bytes!("frames/25.png"),
        26 => include_bytes!("frames/26.png"),
        27 => include_bytes!("frames/27.png"),
        28 => include_bytes!("frames/28.png"),
        29 => include_bytes!("frames/29.png"),
        30 => include_bytes!("frames/30.png"),
        31 => include_bytes!("frames/31.png"),
        32 => include_bytes!("frames/32.png"),
        33 => include_bytes!("frames/33.png"),
        34 => include_bytes!("frames/34.png"),
        35 => include_bytes!("frames/35.png"),
        36 => include_bytes!("frames/36.png"),
        37 => include_bytes!("frames/37.png"),
        38 => include_bytes!("frames/38.png"),
        39 => include_bytes!("frames/39.png"),
        40 => include_bytes!("frames/40.png"),
        41 => include_bytes!("frames/41.png"),
        42 => include_bytes!("frames/42.png"),
        43 => include_bytes!("frames/43.png"),
        44 => include_bytes!("frames/44.png"),
        45 => include_bytes!("frames/45.png"),
        46 => include_bytes!("frames/46.png"),
        47 => include_bytes!("frames/47.png"),
        48 => include_bytes!("frames/48.png"),
        49 => include_bytes!("frames/49.png"),
        50 => include_bytes!("frames/50.png"),
        51 => include_bytes!("frames/51.png"),
        52 => include_bytes!("frames/52.png"),
        53 => include_bytes!("frames/53.png"),
        54 => include_bytes!("frames/54.png"),
        55 => include_bytes!("frames/55.png"),
        56 => include_bytes!("frames/56.png"),
        57 => include_bytes!("frames/57.png"),
        58 => include_bytes!("frames/58.png"),
        59 => include_bytes!("frames/59.png"),
        60 => include_bytes!("frames/60.png"),
        61 => include_bytes!("frames/61.png"),
        62 => include_bytes!("frames/62.png"),
        63 => include_bytes!("frames/63.png"),
        64 => include_bytes!("frames/64.png"),
        65 => include_bytes!("frames/65.png"),
        66 => include_bytes!("frames/66.png"),
        67 => include_bytes!("frames/67.png"),
        68 => include_bytes!("frames/68.png"),
        69 => include_bytes!("frames/69.png"),
        70 => include_bytes!("frames/70.png"),
        71 => include_bytes!("frames/71.png"),
        72 => include_bytes!("frames/72.png"),
        73 => include_bytes!("frames/73.png"),
        74 => include_bytes!("frames/74.png"),
        75 => include_bytes!("frames/75.png"),
        76 => include_bytes!("frames/76.png"),
        77 => include_bytes!("frames/77.png"),
        78 => include_bytes!("frames/78.png"),
        79 => include_bytes!("frames/79.png"),
        80 => include_bytes!("frames/80.png"),
        81 => include_bytes!("frames/81.png"),
        82 => include_bytes!("frames/82.png"),
        83 => include_bytes!("frames/83.png"),
        84 => include_bytes!("frames/84.png"),
        85 => include_bytes!("frames/85.png"),
        86 => include_bytes!("frames/86.png"),
        87 => include_bytes!("frames/87.png"),
        88 => include_bytes!("frames/88.png"),
        89 => include_bytes!("frames/89.png"),
        90 => include_bytes!("frames/90.png"),
        91 => include_bytes!("frames/91.png"),
        92 => include_bytes!("frames/92.png"),
        93 => include_bytes!("frames/93.png"),
        94 => include_bytes!("frames/94.png"),
        95 => include_bytes!("frames/95.png"),
        96 => include_bytes!("frames/96.png"),
        97 => include_bytes!("frames/97.png"),
        98 => include_bytes!("frames/98.png"),
        99 => include_bytes!("frames/99.png"),
        _ => panic!("invalid parameter {digit}"),
    }
}