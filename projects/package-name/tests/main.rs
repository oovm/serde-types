use package_key::PackageKey;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_serde() {
    let s1 = serde_json::to_string(&PackageKey::new("test")).unwrap();
    assert_eq!(s1, "\"TEST\"");
    let s2: PackageKey = serde_json::from_str("\"test\"").unwrap();

    println!("{}", s2);
}
