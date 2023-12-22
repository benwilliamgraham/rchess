/* This is the main file of the library.
 * It's goal is to be the bridge between the Rust code and the JavaScript code.
 */

mod chess;

#[no_mangle]
pub fn test_fn() -> bool {
    true
}
