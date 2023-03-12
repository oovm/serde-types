use package_key::InsensitiveKey;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_serde() {
    let s1 = serde_json::to_string(&InsensitiveKey::new("test")).unwrap();
    assert_eq!(s1, "\"TEST\"");
    let s2: InsensitiveKey = serde_json::from_str("\"test\"").unwrap();

    println!("{}", s2);
}
