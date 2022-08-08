pub fn sanitize(s: &str) -> String {
    let s = sanitize_filename::sanitize(s);
    s.replace('.', "") // remove dots,  gives trouble when appending .yaml with `with_extension`
}
