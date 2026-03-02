pub fn stop() {
    if atty::is(atty::Stream::Stdout) {
        println!("Sending stop request to /tmp/swhook.sock...");
    }
    crate::server::send_stop_message_to_unix_socket();
}
