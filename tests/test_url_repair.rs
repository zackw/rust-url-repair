#[macro_use] extern crate galvanic_test;

// Test cases expected to be successful.  Each line within the string
// has two fields, with the result of repair on the left, and the
// input to repair_url on the right.  (It's this way so we can easily
// sort by all the inputs that repair to the same value.)  Blank lines
// and lines beginning with # are ignored.
// #commented-out test cases are not yet implemented.
const SUCCESSFUL_CASES: &'static str = r#"
# Equivalent forms of the scheme and hostname.
http://example.com/            http://example.com/
http://example.com/            http://example.com
http://example.com/            HTTP://EXAMPLE.COM/
http://example.com/            HTTP://EXAMPLE.COM
http://example.com/            HTTP:EXAMPLE.COM
http://example.com/            http:\\example.com\

# Redundant port numbers should be stripped.
http://example.com/            http://example.com:80/
http://example.com:443/        http://example.com:443/
https://example.com/           https://example.com:443/
https://example.com:80/        https://example.com:80/

# Empty username and password should be stripped.
# We don't do any other tests on these because they're so very deprecated.
http://example.com/            http://@example.com/
#http://example.com/           http://%@example.com/
#http://user@example.com/      http://user%@example.com/
http://%pass@example.com/      http://%pass@example.com/
http://user%pass@example.com/  http://user%pass@example.com/

# Leading and trailing dots on the hostname are to be stripped.
# Doubled dots in the hostname are to be collapsed.
#http://example.com/           http://example.com.
#http://example.com/           http://example.com./
#http://example.com/           http://.example.com./
#http://example.com/           http://example..com/

# Unicode is to be converted to IDNA.
http://xn--hxajbheg2az3al.example/    http://παράδειγμα.example/

# RFC-nonconformant hostnames are to be tolerated.
# url::Url enforces RFC rules upon .set_hostname() so we have to
# smuggle unacceptable hostnames through as usernames instead.
# mail.163.com example from https://github.com/servo/rust-url/issues/489
# caution: right-to-left characters below
http://under_score.example/                  http://under_score.example/
#http://hyphen-.example@domain.invalid/      http://hyphen-.example/
#http://-hyphen.example@domain.invalid/      http://-hyphen.example/
#http://mail.163.com.xn----9mcjf9b4dbm09f.com@domain.invalid/ http://mail.163.com.xn----9mcjf9b4dbm09f.com/
#http://mail.163.com.xn----9mcjf9b4dbm09f.com@domain.invalid/ http://mail.163.com.روغن-کنجد.com/

# Canonicalization of IPv4 address literals
http://127.0.0.1/   http://127.0.0.1/




# leading zeroes are OK but should not cause the number to be
# interpreted as octal -- needs implementing manually, rust-url does
# the Wrong Thing (on purpose, apparently, blech)
#http://127.0.0.1/   http://127.000.000.001/
#http://127.0.0.8/   http://127.000.000.008/
#http://127.0.0.10/  http://127.000.000.010/

# Scheme-relative URLs and bare hostnames should be filled in with the
# default scheme, which is 'http'.
#http://example.com/           //example.com
#http://example.com/           example.com

# The scheme and hostname are case-folded to lowercase; the path,
# query, and fragment are case-preserved.
http://example.com/ABOUT?x=Y#zED http://example.com/ABOUT?x=Y#zED
http://example.com/ABOUT?x=Y#zED HTTP://EXAMPLE.com/ABOUT?x=Y#zED

"#;

#[allow(dead_code)] // unsuccessful tests not yet implemented
const UNSUCCESSFUL_CASES: &'static str = r#"
# IPv4 address literals
# individual components must be in the range 0..255
InvalidIpv4Address    http://256.1.1.1/
InvalidIpv4Address    http://1.256.1.1/
InvalidIpv4Address    http://1.1.256.1/
InvalidIpv4Address    http://1.1.1.256/

# these are network addresses, not host addresses
#InvalidIpv4Address    http://0.0.0.0/
#InvalidIpv4Address    http://1.0.0.0/
#InvalidIpv4Address    http://1.2.0.0/
#InvalidIpv4Address    http://1.2.3.0/

# legacy inet_aton formats should NOT be accepted
#InvalidIpv4Address   http://127.0.1/
#InvalidIpv4Address   http://127.1/
#InvalidIpv4Address   http://0177.0.1/
#InvalidIpv4Address   http://0177.0.0.1/
#InvalidIpv4Address   http://0177.1/
#InvalidIpv4Address   http://0x7f.0.1/
#InvalidIpv4Address   http://0x7f.0.0.1/
#InvalidIpv4Address   http://0x7f.1/
#InvalidIpv4Address   http://2130706433/
#InvalidIpv4Address   http://0x7f000001/
#InvalidIpv4Address   http://017700000001/

"#;

test_suite! {
    name test_url_repair;

    use url_repair::repair_url;

    fixture successful_case(parsed: &'static str, input: &'static str) -> () {
        params {
            use crate::SUCCESSFUL_CASES;
            SUCCESSFUL_CASES.lines()
                .filter_map(|line| {
                    if line.is_empty() || line.starts_with("#") {
                        None
                    } else {
                        let mut fields = line.split_ascii_whitespace();
                        let parsed = fields.next().expect("parsed url missing");
                        let input = fields.next().expect("input text missing");
                        assert_eq!(fields.next(), None, "too many fields");
                        Some((parsed, input))
                    }
                })
        }
        setup(&mut self) {}
    }
    test successful_repairs(successful_case) {
        let input = successful_case.params.input;
        let parsed = successful_case.params.parsed;

        match repair_url(input) {
            Ok(url) => {
                assert_eq!(url.as_str(), *parsed);
            }
            Err(err) => {
                assert!(false, "{}: unsuccessful repair: {}", input, err);
            }
        }
    }
}
