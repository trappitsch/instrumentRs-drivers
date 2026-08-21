fn calculate_crc(pkg: &Vec<u8>) -> [u8; 2] {
    let mut crc = u16::MAX;

    for b in pkg {
        crc ^= *b as u16;

        for _ in 0..8 {
            let ex_or_a001 = crc & 1 == 1;
            crc >>= 1;

            if ex_or_a001 {
                crc ^= 0xa001;
            }
        }
    }

    crc.to_le_bytes()
}

/// Extend all `Vec<u8>` in a mut slice with CRCs.
///
/// This function helps to extend `expected_reads` and `expected_wrties`.
pub fn extend_with_crc(pkg: &mut [Vec<u8>]) {
    for v in pkg {
        let crc = calculate_crc(v);
        v.extend_from_slice(&crc);
    }
}
