error_chain!{
    foreign_links {
        Serde(::serde_json::Error);
        Io(::std::io::Error);
    }
}
