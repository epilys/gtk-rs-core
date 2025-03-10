// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Cancellable, DataStreamByteOrder, FilterOutputStream, OutputStream, Seekable};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, ptr};

glib::wrapper! {
    #[doc(alias = "GDataOutputStream")]
    pub struct DataOutputStream(Object<ffi::GDataOutputStream, ffi::GDataOutputStreamClass>) @extends FilterOutputStream, OutputStream, @implements Seekable;

    match fn {
        type_ => || ffi::g_data_output_stream_get_type(),
    }
}

impl DataOutputStream {
    pub const NONE: Option<&'static DataOutputStream> = None;

    #[doc(alias = "g_data_output_stream_new")]
    pub fn new(base_stream: &impl IsA<OutputStream>) -> DataOutputStream {
        unsafe {
            from_glib_full(ffi::g_data_output_stream_new(
                base_stream.as_ref().to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DataOutputStream`] objects.
    ///
    /// This method returns an instance of [`DataOutputStreamBuilder`](crate::builders::DataOutputStreamBuilder) which can be used to create [`DataOutputStream`] objects.
    pub fn builder() -> DataOutputStreamBuilder {
        DataOutputStreamBuilder::new()
    }
}

impl Default for DataOutputStream {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DataOutputStream`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DataOutputStreamBuilder {
    builder: glib::object::ObjectBuilder<'static, DataOutputStream>,
}

impl DataOutputStreamBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn byte_order(self, byte_order: DataStreamByteOrder) -> Self {
        Self {
            builder: self.builder.property("byte-order", byte_order),
        }
    }

    pub fn base_stream(self, base_stream: &impl IsA<OutputStream>) -> Self {
        Self {
            builder: self
                .builder
                .property("base-stream", base_stream.clone().upcast()),
        }
    }

    pub fn close_base_stream(self, close_base_stream: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("close-base-stream", close_base_stream),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`DataOutputStream`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> DataOutputStream {
        self.builder.build()
    }
}

pub trait DataOutputStreamExt: 'static {
    #[doc(alias = "g_data_output_stream_get_byte_order")]
    #[doc(alias = "get_byte_order")]
    fn byte_order(&self) -> DataStreamByteOrder;

    #[doc(alias = "g_data_output_stream_put_byte")]
    fn put_byte(
        &self,
        data: u8,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_data_output_stream_put_int16")]
    fn put_int16(
        &self,
        data: i16,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_data_output_stream_put_int32")]
    fn put_int32(
        &self,
        data: i32,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_data_output_stream_put_int64")]
    fn put_int64(
        &self,
        data: i64,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_data_output_stream_put_string")]
    fn put_string(
        &self,
        str: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_data_output_stream_put_uint16")]
    fn put_uint16(
        &self,
        data: u16,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_data_output_stream_put_uint32")]
    fn put_uint32(
        &self,
        data: u32,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_data_output_stream_put_uint64")]
    fn put_uint64(
        &self,
        data: u64,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_data_output_stream_set_byte_order")]
    fn set_byte_order(&self, order: DataStreamByteOrder);

    #[doc(alias = "byte-order")]
    fn connect_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DataOutputStream>> DataOutputStreamExt for O {
    fn byte_order(&self) -> DataStreamByteOrder {
        unsafe {
            from_glib(ffi::g_data_output_stream_get_byte_order(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn put_byte(
        &self,
        data: u8,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_data_output_stream_put_byte(
                self.as_ref().to_glib_none().0,
                data,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn put_int16(
        &self,
        data: i16,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_data_output_stream_put_int16(
                self.as_ref().to_glib_none().0,
                data,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn put_int32(
        &self,
        data: i32,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_data_output_stream_put_int32(
                self.as_ref().to_glib_none().0,
                data,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn put_int64(
        &self,
        data: i64,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_data_output_stream_put_int64(
                self.as_ref().to_glib_none().0,
                data,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn put_string(
        &self,
        str: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_data_output_stream_put_string(
                self.as_ref().to_glib_none().0,
                str.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn put_uint16(
        &self,
        data: u16,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_data_output_stream_put_uint16(
                self.as_ref().to_glib_none().0,
                data,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn put_uint32(
        &self,
        data: u32,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_data_output_stream_put_uint32(
                self.as_ref().to_glib_none().0,
                data,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn put_uint64(
        &self,
        data: u64,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_data_output_stream_put_uint64(
                self.as_ref().to_glib_none().0,
                data,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_byte_order(&self, order: DataStreamByteOrder) {
        unsafe {
            ffi::g_data_output_stream_set_byte_order(
                self.as_ref().to_glib_none().0,
                order.into_glib(),
            );
        }
    }

    fn connect_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_byte_order_trampoline<
            P: IsA<DataOutputStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDataOutputStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DataOutputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::byte-order\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_byte_order_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DataOutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DataOutputStream")
    }
}
