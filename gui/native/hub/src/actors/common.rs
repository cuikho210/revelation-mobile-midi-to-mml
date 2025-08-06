use messages::prelude::{Address, Handler};
use rinf::{DartSignal, DartSignalBinary, RustSignal};
use tracing::{debug, error, info};

use crate::{error::send_error_signal, signals::error::ErrorFrom};

pub trait ActorName {
    fn get_name() -> &'static str;
}

pub trait ListenDartSignal {
    async fn listen_with_response<T: DartSignal + Send + 'static, A, U>(
        mut self_addr: Address<A>,
        signal_name: &'static str,
        error_from: Option<ErrorFrom>,
    ) where
        A: Handler<T> + ActorName,
        <A as Handler<T>>::Result: Into<anyhow::Result<U>>,
        <A as Handler<T>>::Result: Send,
        <A as Handler<T>>::Result: Sync,
        <A as Handler<T>>::Result: 'static,
        U: RustSignal,
    {
        let actor_name = A::get_name();
        info!("[{actor_name}] spawning listen {signal_name}");
        let receiver = T::get_dart_signal_receiver();
        while let Some(signal_pack) = receiver.recv().await {
            debug!("[{actor_name}] Received a signal {signal_name} request from Dart",);
            match self_addr.send(signal_pack.message).await {
                Ok(res) => match res.into() {
                    Ok(res) => {
                        debug!("[{actor_name}] {signal_name} successful");
                        res.send_signal_to_dart();
                    }
                    Err(err) => {
                        error!("[{actor_name}] {err}");
                        send_error_signal(err, format!("{signal_name} Error"), error_from.clone());
                    }
                },
                Err(err) => error!(
                    "[{actor_name}] Error sending signal {signal_name} to {actor_name}: {err}"
                ),
            };
        }
    }

    async fn listen_without_response<T: DartSignal + Send + 'static, A>(
        mut self_addr: Address<A>,
        signal_name: &'static str,
    ) where
        A: Handler<T> + ActorName,
        <A as Handler<T>>::Result: Send,
        <A as Handler<T>>::Result: Sync,
        <A as Handler<T>>::Result: 'static,
    {
        let actor_name = A::get_name();
        info!("[{actor_name}] spawning listen {signal_name}");
        let receiver = T::get_dart_signal_receiver();
        while let Some(signal_pack) = receiver.recv().await {
            debug!("[{actor_name}] Received a signal {signal_name} request from Dart",);
            if let Err(err) = self_addr.send(signal_pack.message).await {
                error!("[{actor_name}] Error sending signal {signal_name} to {actor_name}: {err}");
            };
        }
    }

    async fn listen_binary<T: DartSignalBinary + Send + 'static, R, A>(
        mut self_addr: Address<A>,
        signal_name: &'static str,
    ) where
        R: From<Vec<u8>> + Send + 'static,
        A: Handler<R> + ActorName,
        <A as Handler<R>>::Result: Send,
        <A as Handler<R>>::Result: Sync,
        <A as Handler<R>>::Result: 'static,
    {
        let actor_name = A::get_name();
        info!("[{actor_name}] spawning listen binary {signal_name}");
        let receiver = T::get_dart_signal_receiver();
        while let Some(signal_pack) = receiver.recv().await {
            debug!("[{actor_name}] Received a binary signal {signal_name} request from Dart",);
            if let Err(err) = self_addr.send(R::from(signal_pack.binary)).await {
                error!("[{actor_name}] Error sending signal {signal_name} to {actor_name}: {err}");
            };
        }
    }
}
