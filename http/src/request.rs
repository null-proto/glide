




// #[cfg(test)]
// mod request_unittest {
//
//   #[test]
//   fn test_request_uri() {
//     let sample = "POST /index.html/e?y=6&x=0 HTTP/1.1\r
// Host: [::]:8000\r
// User-Agent: curl/8.x.x\r
// Accept: */*\r
// Content-Length: 1\r
// Content-Type: application/x-www-form-urlencoded\r
// \r
// w
// ";
//     let req = Request::parse(sample.as_bytes()).unwrap();
//     assert_eq!(req.get_uri(), "/index.html/e");
//   }
//
//   #[test]
//   fn test_request_query() {
//     let sample = "POST /index.html/e?y=6&x=0 HTTP/1.1\r
// Host: [::]:8000\r
// User-Agent: curl/8.x.x\r
// Accept: */*\r
// Content-Length: 1\r
// Content-Type: application/x-www-form-urlencoded\r
// \r
// w
// ";
//     let req = Request::parse(sample.as_bytes()).unwrap();
//     assert_eq!(*req.get_query().unwrap().get("x").unwrap(), "0");
//   }
//
//   #[test]
//   fn test_request_header_map() {
//     let sample = "POST /index.html/e?y=6&x=0 HTTP/1.1\r
// Host: [::]:8000\r
// User-Agent: curl/8.x.x\r
// Accept: */*\r
// Content-Length: 1\r
// Content-Type: application/x-www-form-urlencoded\r
// \r
// w
// ";
//     let req = Request::parse(sample.as_bytes()).unwrap();
//     assert_eq!(
//       *req.header.map.unwrap().get(field::CONTENT_LENGTH).unwrap(),
//       "1"
//     );
//   }
//
//   #[test]
//   fn test_request_g() {
//     let sample = "\
// GET /d/er.git/info/refs?service=git-upload-pack HTTP/1.1\r
// Host: [::1]:8000\r
// User-Agent: git/2.51.0\r
// Accept: */*\r
// Accept-Encoding: deflate, gzip, br, zstd\r
// Accept-Language: en-US, *;q=0.9\r
// Pragma: no-cacher\r
// Git-Protocol: version=2\r
// \r
// ";
//     let req = Request::parse(sample.as_bytes()).unwrap();
//     assert_eq!(req.get_uri(), "/d/er.git/info/refs");
//     assert_eq!(
//       *req.get_query().unwrap().get("service").unwrap(),
//       "git-upload-pack"
//     );
//   }
// }
