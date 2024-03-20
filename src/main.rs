use nokhwa::{native_api_backend, query};

fn main() {
    // DirectShow をサポートしていないので、OBS Virtual Camera は検出されない
    // 参考: https://github.com/l1npengtul/nokhwa/issues/102
    let backend = native_api_backend().unwrap();
    let devices = query(backend).unwrap();
    println!("There are {} devices", devices.len());
    for device in devices {
        println!(
            "device name: {}, index: {}",
            device.human_name(),
            device.index()
        );
    }
}
