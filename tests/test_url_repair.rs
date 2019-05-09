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
http://example.com/            HTTP://EXAMPLE.COM/
http://example.com/            http://ex%61mple%2Ecom/

# An incorrect number of slashes after the scheme should be corrected.
http://example.com/            http:example.com
http://example.com/            http:/example.com
http://example.com/            http:///example.com

# Redundant port numbers for "special" schemes should be stripped.
ftp://example.com/             ftp://example.com:21/
gopher://example.com/          gopher://example.com:70/
http://example.com/            http://example.com:80/
https://example.com/           https://example.com:443/
ws://example.com/              ws://example.com:80/
wss://example.com/             wss://example.com:443/

# Non-redundant port numbers should be preserved.
ftp://example.com:1108/        ftp://example.com:1108/
gopher://example.com:1108/     gopher://example.com:1108/
http://example.com:1108/       http://example.com:1108/
https://example.com:1108/      https://example.com:1108/
ws://example.com:1108/         ws://example.com:1108/
wss://example.com:1108/        wss://example.com:1108/

# Empty username and password should be stripped.
# We don't do any other tests on these because they're so very deprecated.
http://example.com/            http://@example.com/
http://example.com/            http://:@example.com/
http://user@example.com/       http://user:@example.com/
http://:pass@example.com/      http://:pass@example.com/
http://user:pass@example.com/  http://user:pass@example.com/

# Leading and trailing dots on the hostname are to be stripped.
# Doubled dots in the hostname are to be collapsed.
#http://example.com/           http://example.com.
#http://example.com/           http://example.com./
#http://example.com/           http://.example.com./
#http://example.com/           http://example..com/

# Unicode in the hostname is to be converted to IDNA.
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

# IPv4 address literals
http://127.0.0.1/   http://127.0.0.1/

# leading zeroes should be stripped and should NOT cause the number to be
# interpreted as octal -- needs implementing manually, rust-url does
# the Wrong Thing (on purpose, apparently, blech)
#http://127.0.0.1/   http://127.000.000.001/
#http://127.0.0.8/   http://127.000.000.008/
#http://127.0.0.10/  http://127.000.000.010/

# IPv6 address literals
http://[::1]/    http://[::1]/
http://[::1]/    http://[0:0:0:0:0:0:0:1]/
http://[::1]/    http://[0000:0000:0000:0000:0000:0000:0000:0001]/

http://[2001:db8::ff00:42:8329]/    http://[2001:db8::ff00:42:8329]/
http://[2001:db8::ff00:42:8329]/    http://[2001:db8:0:0:0:ff00:42:8329]/
http://[2001:db8::ff00:42:8329]/    http://[2001:0db8:0000:0000:0000:ff00:0042:8329]/

# Scheme-relative URLs and bare hostnames should be filled in with the
# default scheme, which is 'http'.
#http://example.com/           //example.com
#http://example.com/           example.com

# The scheme and hostname are case-folded to lowercase; the path,
# query, and fragment are case-preserved.
http://example.com/ABOUT?x=Y#zED http://example.com/ABOUT?x=Y#zED
http://example.com/ABOUT?x=Y#zED HTTP://EXAMPLE.com/ABOUT?x=Y#zED

# An incorrect number of slashes after the hostname should be corrected.
http://example.com/            http://example.com
#http://example.com/           http://example.com//

# Dot, dotdot, and slash normalization within the path.  The URL
# standard doesn't require slash normalization, for no apparent reason.
#http://example.com/about      http://example.com//about

http://example.com/a/b/d       http://example.com/a/./b/c/../d
http://example.com/a/b/d       http://example.com/./a/././b/./c/.././d
http://example.com/a/b/d       http://example.com/a/b/e/../c/f/../../d
#http://example.com/a/b/d      http://example.com//a//b//d

# Trailing slashes cannot be dropped, but can be normalized.
http://example.com/a/b/d/      http://example.com/a/b/d/
#http://example.com/a/b/d/     http://example.com/a/b/d//
http://example.com/a/b/d/      http://example.com/a/b/d/./

# Backslashes in any component up to and including the path are to be
# normalized to slashes.  Mixtures are allowed.
http://example.com/                 http:\\example.com\
http://example.com/about/backslash  http:\\example.com\about\backslash
http://example.com/a/b/d            http:/\example.com/.\a/.\./b\.\c/../.\d
http://example.com/a/b/d            http://example.com/a/b/e\..\c/f/..\..\d
#http://example.com/a/b/d/          http://example.com\\a//b\/d/\

# Backslashes are not to be normalized to slashes within query and fragment.
http://example.com/?a=b\c           http://example.com/?a=b\c
http://example.com/?a\b=c           http://example.com/?a\b=c
http://example.com/#a=b\c           http://example.com/#a=b\c
http://example.com/#a\b=c           http://example.com/#a\b=c

# TODO write test cases for:
# as many layers of %-quotation as possible are to be stripped
# necessary %-quotation should be preserved in all components
# characters that _don't_ need to be %-quoted should be unquoted
# characters outside printable ASCII should be %-quoted in path,
#  query, and fragment

"#;

// Same as above, but these are expected to parse unsuccessfully.
// The left-hand side is the expected Debug serialization of the
// url::ParseError that is returned.
const UNSUCCESSFUL_CASES: &'static str = r#"
# IPv4 address literals
# individual components must be in the range 0..255
InvalidIpv4Address    http://256.1.1.1/
InvalidIpv4Address    http://1.256.1.1/
InvalidIpv4Address    http://1.1.256.1/
InvalidIpv4Address    http://1.1.1.256/

# these are network addresses, not host addresses
# ??? unclear whether we should try to detect them
# (where does one draw the line between usable and non-usable?)
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

# IPv6 address literals
# inappropriate characters within the square brackets
InvalidIpv6Address    http://[xyz::1]/
InvalidIpv6Address    http://[%4a::1]/

# missing one or other square bracket
InvalidIpv6Address    http://[::1/
EmptyHost             http://::1]/

# cannot use :: twice (it's ambiguous)
InvalidIpv6Address    http://[2001:db8::ff00::42:8329]/
"#;

test_suite! {
    name test_url_repair;

    use url_repair::repair_url;
    use std::iter::Iterator;

    fn parse_test_list (s: &str) -> impl Iterator<Item=(&str, &str)> {
        s.lines().filter_map(|line| {
            if line.is_empty() || line.starts_with("#") {
                None
            } else {
                let mut fields = line.split_whitespace();
                let result = fields.next().expect("expected result missing");
                let input = fields.next().expect("input text missing");
                assert_eq!(fields.next(), None, "too many fields");
                Some((result, input))
            }
        })
    }

    fixture successful_case(parsed: &'static str, input: &'static str) -> () {
        params {
            parse_test_list(crate::SUCCESSFUL_CASES)
        }
        setup(&mut self) {}
    }
    test successful_repairs(successful_case) {
        let input = *successful_case.params.input;
        let parsed = *successful_case.params.parsed;

        match repair_url(input) {
            Ok(url)  => assert_eq!(url.as_str(), parsed),
            Err(err) => assert!(false, "{}: unsuccessful repair: {}",
                                input, err)
        }
    }

    fixture unsuccessful_case(xerr: &'static str, input: &'static str) -> () {
        params {
            parse_test_list(crate::UNSUCCESSFUL_CASES)
        }
        setup(&mut self) {}
    }
    test unsuccessful_repairs(unsuccessful_case) {
        let input = *unsuccessful_case.params.input;
        let xerr = *unsuccessful_case.params.xerr;

        match repair_url(input) {
            Err(err) => assert_eq!(format!("{:?}", err), xerr),
            Ok(url)  => assert!(false,
                                "{}: should not parse (got {}, expected {})",
                                input, url, xerr)
        }
    }
}
