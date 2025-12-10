use rand::rng;
use rand::RngCore;
use uuid::Uuid;

/// Generate a version 1 UUID (timestamp + node id). Node ID is randomly generated.
pub fn generate_v1() -> Uuid {
    let mut node = [0u8; 6];
    rng().fill_bytes(&mut node);
    Uuid::now_v1(&node)
}

/// Generate a version 4 (random) UUID.
pub fn generate_v4() -> Uuid {
    Uuid::new_v4()
}

/// Generate a version 5 (name-based, SHA-1) UUID using a namespace and name.
/// `namespace` accepts: "dns", "url", "oid", "x500" (case-insensitive). Defaults to "dns".
pub fn generate_v5(name: &str, namespace: &str) -> Uuid {
    let ns = match namespace.to_ascii_lowercase().as_str() {
        "url" => Uuid::NAMESPACE_URL,
        "oid" => Uuid::NAMESPACE_OID,
        "x500" => Uuid::NAMESPACE_X500,
        _ => Uuid::NAMESPACE_DNS,
    };
    Uuid::new_v5(&ns, name.as_bytes())
}

/// Generate a version 6 UUID (timestamp-ordered). Node ID is randomly generated.
pub fn generate_v6() -> Uuid {
    let mut node = [0u8; 6];
    rng().fill_bytes(&mut node);
    Uuid::now_v6(&node)
}

/// Generate a version 7 (time-ordered) UUID.
pub fn generate_v7() -> Uuid {
    Uuid::now_v7()
}

/// Format a `Uuid` into a string with options.
pub fn format_uuid(u: Uuid, no_hyphen: bool, braced: bool, upper: bool) -> String {
    let s = if no_hyphen {
        u.as_simple().to_string()
    } else if braced {
        u.braced().to_string()
    } else {
        u.to_string()
    };

    if upper {
        s.to_uppercase()
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_and_format_v7() {
        let u = generate_v7();
        let s = format_uuid(u, false, false, false);
        assert_eq!(s.len(), 36);
        let version_char = s.chars().nth(14).unwrap();
        assert_eq!(version_char, '7');
    }

    #[test]
    fn gen_v4_and_v5() {
        let u4 = generate_v4();
        assert_eq!(u4.get_version_num(), 4);

        let u5 = generate_v5("example", "dns");
        assert_eq!(u5.get_version_num(), 5);
    }

    #[test]
    fn format_variants_lengths_and_case() {
        let u = generate_v4();

        let normal = format_uuid(u, false, false, false);
        assert_eq!(normal.len(), 36);

        let no_hyphen = format_uuid(u, true, false, false);
        assert_eq!(no_hyphen.len(), 32);

        let braced = format_uuid(u, false, true, false);
        assert_eq!(braced.len(), 38);
        assert!(braced.starts_with('{') && braced.ends_with('}'));

        let upper = format_uuid(u, false, false, true);
        assert_eq!(upper, normal.to_uppercase());
    }

    #[test]
    fn no_hyphen_takes_precedence_over_braced() {
        let u = generate_v4();
        let s = format_uuid(u, true, true, false);
        // no_hyphen branch is evaluated first -> expect simple form (32 chars)
        assert_eq!(s.len(), 32);
        assert!(!s.starts_with('{'));
    }

    #[test]
    fn gen_v1_and_v6_versions() {
        let u1 = generate_v1();
        assert_eq!(u1.get_version_num(), 1);

        let u6 = generate_v6();
        assert_eq!(u6.get_version_num(), 6);
    }

    #[test]
    fn generate_v5_is_deterministic_and_namespace_case_insensitive() {
        let a = generate_v5("example", "dns");
        let b = generate_v5("example", "DNS"); // case-insensitive namespace
        assert_eq!(a, b);

        let c = generate_v5("example", "url");
        assert_ne!(a, c);
    }

    #[test]
    fn generate_v4_likely_unique() {
        let a = generate_v4();
        let b = generate_v4();
        // Extremely unlikely to collide; assert they are different to catch obvious issues.
        assert_ne!(a, b);
    }
}
