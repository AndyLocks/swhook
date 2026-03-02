pub fn reload() {
    if atty::is(atty::Stream::Stdout) {
        println!("Sending reload request to /tmp/swhook.sock...");
    }
    crate::server::send_reload_message_to_unix_socket();
}
