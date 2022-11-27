
use enigo::*;
use x11rb::protocol::xproto::ConnectionExt;

fn main() {
    println!("Hello, world!");

    // get the current window name using x11rb.
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    
    for i  in 0..100 {
        if let Ok(window) = conn.get_window_attributes(i) {
            println!("window: {} {:?}", i, window);
            if let Ok(reply) = window.reply() {
                //reply.
            }
        }
    }
    
    
}
