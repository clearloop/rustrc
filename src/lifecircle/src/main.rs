fn main() {
    let r;
    {
        let i = 1;
        r = &i;
    } 
}

fn skip_prefix(line: &str, prefix: &str) -> &str {
    let line = "line";
    let lang = "en";
    let v;
    {
        let p = format!("lang:{}=", lang);
        v = skip_prefix(line, p.as_str());
    }
    println!("{}", v);
}
