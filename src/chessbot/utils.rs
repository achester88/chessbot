pub fn bit_scan(num: u64) -> usize {
    if num != 0 {
      return num.trailing_zeros() as usize;
    }
    return 0;
  }
  
pub fn bit_scan_neg(num: u64) -> usize {
    (num.leading_zeros() ^ 63) as usize
  }