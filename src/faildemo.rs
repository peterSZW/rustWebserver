
pub fn crash_demo() {
   fail_point!("name1");
} 


pub fn fail_main() {
   crash_demo( );
} 