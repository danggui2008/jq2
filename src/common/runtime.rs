use once_cell::sync::Lazy;

pub static CRUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Runtime::new().unwrap()
});

