// Copyright 2019 Zack Weinberg <zackw@panix.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use url::Url;
pub use url::ParseError;

/// Repair an absolute URL; that is, parse it as `url::Url::parse` would,
/// but correcting a number of common abbreviations and typos that occur
/// when URLs are written down by hand.
pub fn repair_url(input: &str) -> Result<Url, ParseError> {
    // corrections are not yet implemented
    Url::parse(input)
}

#[cfg(test)]
mod tests {
    use crate::repair_url;
    #[test]
    fn smoke_test() {
        assert_eq!(repair_url("http://example.com/").unwrap().as_str(),
                   "http://example.com/");
        assert_eq!(repair_url("HTTP://EXAMPLE.COM/").unwrap().as_str(),
                   "http://example.com/");
        assert_eq!(repair_url("hTtPs://example.com").unwrap().as_str(),
                   "https://example.com/");
        assert_eq!(repair_url("http://παράδειγμα.example/").unwrap().as_str(),
                   "http://xn--hxajbheg2az3al.example/");
    }
}
