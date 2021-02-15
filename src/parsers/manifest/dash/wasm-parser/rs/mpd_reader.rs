use std::{mem, io::{self, Read}};
pub struct MPDReader {}

impl Read for MPDReader {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let actual_size;
        unsafe {
            actual_size = super::read_next(mem::transmute((*buf).as_ptr()), buf.len());
        }
        Ok(actual_size)
    }
}

// impl BufRead for MPDReader {
//     #[inline]
//     fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
//         let actual_size;
//         unsafe {
//             actual_size = read_next(mem::transmute((*self.buf).as_ptr()), self.buf.capacity());
//             self.buf.set_len(actual_size);
//             report::report_error(format!("actual {}", actual_size));
//             let ret = self.buf.as_slice();
//             onCustomEvent(
//                 CustomEventType::Error,
//                 mem::transmute(ret.as_ptr()), ret.len());
//             Ok(ret)
//         }
//     }

//     #[inline]
//     fn consume(&mut self, amt: usize) {
//         let cloned = self.buf.as_slice()[amt..].to_vec().clone();
//         self.buf = Vec::with_capacity(READING_VEC_CAPACITY);
//         self.buf.extend(cloned);
//     }
// }
