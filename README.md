# url-repair: repair and canonicalize absolute URLs

This crate wraps around the [`url` crate][], and provides a function
that parses `url::Url` objects from strings, just like `Url::parse`.
However, it anticipates and corrects a variety of common abbreviations
and typos that occur when URLs are written down by hand.  For example:

    >>> repair_url("example.com").as_str()
    "http://example.com/"

    >>> repair_url("example.com/about").as_str()
    "http://example.com/about"

    >>> repair_url(r"HTTP:\\EXAMPLE.COM\ABOUT").as_str()
    "http://example.com/ABOUT"

You can set a different default scheme using `url_repair::UrlRepairOptions`:

    >>> let repair_opts = UrlRepairOptions().default_scheme("https");
    >>> repair_opts.repair_url("example.com").as_str()
    "https://example.com"

You can also use this to set the encoding of query parameters, like
`url::ParseOptions`.  The other options controlled by `url::ParseOptions`
are not currently exposed.

    >>> # FIXME write an example

This crate does not provide a wrapper for `Url::join`, because it is
unclear how to reconcile the normal treatment of relative URLs with
the heuristics for deciding that a scheme is missing.

The URL crate’s parser enforces the RFC requirements for domain names
quite strictly.  Unfortunately, DNS operators and top-level domain
registrars have not been nearly as strict, so URLs referring to sites
that exist in the real world may be rejected.  Currently, whenever
`repair_url` encounters such a URL, it will preserve the original
hostname as a username, and change the hostname to `domain.invalid`:

    >>> repair_url("http://mail.163.com.xn----9mcjf9b4dbm09f.com/").as_str()
    "http://mail.163.com.xn----9mcjf9b4dbm09f.com@domain.invalid/"

[`url` crate]: https://crates.io/crates/url
