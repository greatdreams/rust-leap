pub fn is_leap(year: i32) -> bool {
   if year % 400 == 0 ||(year % 4 == 0 && year % 100 != 0) {
        true
   }else {
        false
   }
}