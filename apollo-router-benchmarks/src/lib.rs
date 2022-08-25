#[cfg(test)]
pub mod tests {
    include!("shared.rs");

    #[test]
    #[tracing_test::traced_test]
    fn test() {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let builder = setup();
        let router = runtime.block_on(builder.build()).unwrap();
        runtime.block_on(async move { basic_composition_benchmark(router).await });
    }
}
