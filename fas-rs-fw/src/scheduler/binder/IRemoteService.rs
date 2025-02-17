#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::all, clippy::pedantic)]
#![allow(warnings)]
#[allow(unused_imports)]
use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  IRemoteService["IRemoteService"] {
    native: BnRemoteService(on_transact),
    proxy: BpRemoteService {
    },
    async: IRemoteServiceAsync,
  }
}
pub trait IRemoteService: binder::Interface + Send {
    fn get_descriptor() -> &'static str
    where
        Self: Sized,
    {
        "IRemoteService"
    }
    fn sendData(
        &self,
        _arg_Pkg: &str,
        _arg_pid: i32,
        _arg_FrameTimeNanos: i64,
    ) -> binder::Result<bool>;
    fn getDefaultImpl() -> IRemoteServiceDefaultRef
    where
        Self: Sized,
    {
        DEFAULT_IMPL.lock().unwrap().clone()
    }
    fn setDefaultImpl(d: IRemoteServiceDefaultRef) -> IRemoteServiceDefaultRef
    where
        Self: Sized,
    {
        std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
    }
}
pub trait IRemoteServiceAsync<P>: binder::Interface + Send {
    fn get_descriptor() -> &'static str
    where
        Self: Sized,
    {
        "IRemoteService"
    }
    fn sendData<'a>(
        &'a self,
        _arg_Pkg: &'a str,
        _arg_pid: i32,
        _arg_FrameTimeNanos: i64,
    ) -> binder::BoxFuture<'a, binder::Result<bool>>;
}
#[::async_trait::async_trait]
pub trait IRemoteServiceAsyncServer: binder::Interface + Send {
    fn get_descriptor() -> &'static str
    where
        Self: Sized,
    {
        "IRemoteService"
    }
    async fn sendData(
        &self,
        _arg_Pkg: &str,
        _arg_pid: i32,
        _arg_FrameTimeNanos: i64,
    ) -> binder::Result<bool>;
}
impl BnRemoteService {
    /// Create a new async binder service.
    pub fn new_async_binder<T, R>(
        inner: T,
        rt: R,
        features: binder::BinderFeatures,
    ) -> binder::Strong<dyn IRemoteService>
    where
        T: IRemoteServiceAsyncServer + binder::Interface + Send + Sync + 'static,
        R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
        struct Wrapper<T, R> {
            _inner: T,
            _rt: R,
        }
        impl<T, R> binder::Interface for Wrapper<T, R>
        where
            T: binder::Interface,
            R: Send + Sync,
        {
            fn as_binder(&self) -> binder::SpIBinder {
                self._inner.as_binder()
            }
            fn dump(
                &self,
                _file: &std::fs::File,
                _args: &[&std::ffi::CStr],
            ) -> std::result::Result<(), binder::StatusCode> {
                self._inner.dump(_file, _args)
            }
        }
        impl<T, R> IRemoteService for Wrapper<T, R>
        where
            T: IRemoteServiceAsyncServer + Send + Sync + 'static,
            R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
        {
            fn sendData(
                &self,
                _arg_Pkg: &str,
                _arg_pid: i32,
                _arg_FrameTimeNanos: i64,
            ) -> binder::Result<bool> {
                self._rt.block_on(
                    self._inner
                        .sendData(_arg_Pkg, _arg_pid, _arg_FrameTimeNanos),
                )
            }
        }
        let wrapped = Wrapper {
            _inner: inner,
            _rt: rt,
        };
        Self::new_binder(wrapped, features)
    }
}
pub trait IRemoteServiceDefault: Send + Sync {
    fn sendData(
        &self,
        _arg_Pkg: &str,
        _arg_pid: i32,
        _arg_FrameTimeNanos: i64,
    ) -> binder::Result<bool> {
        Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
    }
}
pub mod transactions {
    pub const sendData: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
}
pub type IRemoteServiceDefaultRef = Option<std::sync::Arc<dyn IRemoteServiceDefault>>;
use lazy_static::lazy_static;
lazy_static! {
    static ref DEFAULT_IMPL: std::sync::Mutex<IRemoteServiceDefaultRef> =
        std::sync::Mutex::new(None);
}
impl BpRemoteService {
    fn build_parcel_sendData(
        &self,
        _arg_Pkg: &str,
        _arg_pid: i32,
        _arg_FrameTimeNanos: i64,
    ) -> binder::Result<binder::binder_impl::Parcel> {
        let mut aidl_data = self.binder.prepare_transact()?;
        aidl_data.write(_arg_Pkg)?;
        aidl_data.write(&_arg_pid)?;
        aidl_data.write(&_arg_FrameTimeNanos)?;
        Ok(aidl_data)
    }
    fn read_response_sendData(
        &self,
        _arg_Pkg: &str,
        _arg_pid: i32,
        _arg_FrameTimeNanos: i64,
        _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>,
    ) -> binder::Result<bool> {
        if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
            if let Some(_aidl_default_impl) = <Self as IRemoteService>::getDefaultImpl() {
                return _aidl_default_impl.sendData(_arg_Pkg, _arg_pid, _arg_FrameTimeNanos);
            }
        }
        let _aidl_reply = _aidl_reply?;
        let _aidl_status: binder::Status = _aidl_reply.read()?;
        if !_aidl_status.is_ok() {
            return Err(_aidl_status);
        }
        let _aidl_return: bool = _aidl_reply.read()?;
        Ok(_aidl_return)
    }
}
impl IRemoteService for BpRemoteService {
    fn sendData(
        &self,
        _arg_Pkg: &str,
        _arg_pid: i32,
        _arg_FrameTimeNanos: i64,
    ) -> binder::Result<bool> {
        let _aidl_data = self.build_parcel_sendData(_arg_Pkg, _arg_pid, _arg_FrameTimeNanos)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::sendData,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_sendData(_arg_Pkg, _arg_pid, _arg_FrameTimeNanos, _aidl_reply)
    }
}
impl<P: binder::BinderAsyncPool> IRemoteServiceAsync<P> for BpRemoteService {
    fn sendData<'a>(
        &'a self,
        _arg_Pkg: &'a str,
        _arg_pid: i32,
        _arg_FrameTimeNanos: i64,
    ) -> binder::BoxFuture<'a, binder::Result<bool>> {
        let _aidl_data = match self.build_parcel_sendData(_arg_Pkg, _arg_pid, _arg_FrameTimeNanos) {
            Ok(_aidl_data) => _aidl_data,
            Err(err) => return Box::pin(std::future::ready(Err(err))),
        };
        let binder = self.binder.clone();
        P::spawn(
            move || {
                binder.submit_transact(
                    transactions::sendData,
                    _aidl_data,
                    binder::binder_impl::FLAG_PRIVATE_LOCAL,
                )
            },
            move |_aidl_reply| async move {
                self.read_response_sendData(_arg_Pkg, _arg_pid, _arg_FrameTimeNanos, _aidl_reply)
            },
        )
    }
}
impl IRemoteService for binder::binder_impl::Binder<BnRemoteService> {
    fn sendData(
        &self,
        _arg_Pkg: &str,
        _arg_pid: i32,
        _arg_FrameTimeNanos: i64,
    ) -> binder::Result<bool> {
        self.0.sendData(_arg_Pkg, _arg_pid, _arg_FrameTimeNanos)
    }
}
fn on_transact(
    _aidl_service: &dyn IRemoteService,
    _aidl_code: binder::binder_impl::TransactionCode,
    _aidl_data: &binder::binder_impl::BorrowedParcel<'_>,
    _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>,
) -> std::result::Result<(), binder::StatusCode> {
    match _aidl_code {
        transactions::sendData => {
            let _arg_Pkg: String = _aidl_data.read()?;
            let _arg_pid: i32 = _aidl_data.read()?;
            let _arg_FrameTimeNanos: i64 = _aidl_data.read()?;
            let _aidl_return = _aidl_service.sendData(&_arg_Pkg, _arg_pid, _arg_FrameTimeNanos);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION),
    }
}
pub(crate) mod mangled {
    pub use super::IRemoteService as _14_IRemoteService;
}
