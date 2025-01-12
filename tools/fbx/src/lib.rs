#[link(name = "rpd_fbx_plugin", kind = "static")]
extern "C" {
    fn fileToSceneBuffer(filename: *mut u8, pathlen: i32, size_len: *mut i32) -> *mut u8;
    fn sceneBufferToFile(filename: *mut u8, pathlen: i32, buffer: *mut u8, size_len: i32) -> bool;
    fn printHello();
}

pub fn fileToScene(mut filename: String) -> Option<Vec<u8>> {
    let mut size = 0i32;
    unsafe {
        let data = fileToSceneBuffer(filename.as_mut_ptr(), filename.len() as i32, &mut size);
        let dataref = std::slice::from_raw_parts(data, size as usize);
        let ret = Vec::from(dataref);
        return Some(ret);
    }
}

pub fn sceneToFile(mut filename: String, mut buf: Vec<u8>) -> bool {
    unsafe {
        let dataref = buf.as_mut_slice();
        let ret = sceneBufferToFile(filename.as_mut_ptr(), filename.len() as i32, dataref.as_mut_ptr(), dataref.len() as i32);
        return ret;
    }
}