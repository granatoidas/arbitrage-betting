use reqwest::{
    blocking::{RequestBuilder, Response},
    header::{HeaderMap, HeaderValue},
    Result,
};

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    headers.insert("Cache-Control", HeaderValue::from_static("max-age=0"));
    headers.insert(
        "sec-ch-ua",
        HeaderValue::from_static(
            r#"" Not A;Brand";v="99", "Chromium";v="99", "Google Chrome";v="99""#,
        ),
    );
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert(
        "sec-ch-ua-platform",
        HeaderValue::from_static(r#""Windows""#),
    );
    headers.insert("Upgrade-Insecure-Requests", HeaderValue::from_static("1"));
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.51 Safari/537.36"));
    headers.insert("Accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("none"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));
    headers.insert("Sec-Fetch-User", HeaderValue::from_static("?1"));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("document"));
    headers.insert(
        "Accept-Encoding",
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert(
        "Accept-Language",
        HeaderValue::from_static("en-US,en;q=0.9"),
    );

    headers
}

pub(crate) trait DefaultChromeHeaders {
    fn default_chrome_headers(self) -> RequestBuilder;
}

impl DefaultChromeHeaders for RequestBuilder {
    fn default_chrome_headers(self) -> RequestBuilder {
        self.headers(construct_headers())
    }
}
