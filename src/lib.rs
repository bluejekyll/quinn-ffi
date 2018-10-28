use std::mem;

use quinn::{Endpoint, EndpointBuilder};

#[no_mangle]
pub extern "C" fn new_endpoint_builder<'a>() -> *mut EndpointBuilder<'a> {
    Box::into_raw(Box::new(Endpoint::new()))
}

#[no_mangle]
pub extern "C" fn listen<'a>(builder: *mut EndpointBuilder<'a>) {
    assert!(!builder.is_null());
    let builder: &mut EndpointBuilder = unsafe { &mut *builder };

    // builder.listen
}

#[no_mangle]
pub extern "C" fn bind<'a>(builder: *mut EndpointBuilder<'a>) {
    assert!(!builder.is_null());
    let builder: &mut EndpointBuilder = unsafe { &mut *builder };

    // builder.bind
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
