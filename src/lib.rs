extern crate quinn;

use std::mem;

pub struct EndpointFFI(quinn::Endpoint);
pub struct EndpointBuilderFFI<'a>(quinn::EndpointBuilder<'a>);

#[no_mangle]
pub extern "C" fn new_endpoint_builder<'a>() -> *mut EndpointBuilderFFI<'a> {
    let builder = EndpointBuilderFFI(quinn::Endpoint::new());
    Box::into_raw(Box::new(builder))
}

#[no_mangle]
pub extern "C" fn listen<'a>(builder: *mut EndpointBuilderFFI<'a>) {
    assert!(!builder.is_null());
    let builder: &mut EndpointBuilderFFI = unsafe { &mut *builder };

    // builder.listen
}

#[no_mangle]
pub extern "C" fn bind<'a>(builder: *mut EndpointBuilderFFI<'a>) {
    assert!(!builder.is_null());
    let builder: &mut EndpointBuilderFFI = unsafe { &mut *builder };

    // builder.bind
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
