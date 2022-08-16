//! Tracing configuration for apollo telemetry.
// This entire file is license key functionality
use opentelemetry::sdk::trace::Builder;
use tower::BoxError;

use crate::plugins::telemetry::apollo::Config;
use crate::plugins::telemetry::config::Trace;
use crate::plugins::telemetry::tracing::{apollo_telemetry, TracingConfigurator};

impl TracingConfigurator for Config {
    fn apply(&self, builder: Builder, trace_config: &Trace) -> Result<Builder, BoxError> {
        tracing::debug!("configuring Apollo tracing {:?}", self);
        Ok(match self {
            Config {
                endpoint: Some(endpoint),
                apollo_key: Some(key),
                apollo_graph_ref: Some(reference),
                client_name_header,
                client_version_header,
                schema_id,
                buffer_size,
                ..
            } => {
                tracing::debug!("configuring exporter to Spaceport");
                let exporter = apollo_telemetry::Exporter::builder()
                    .trace_config(trace_config.clone())
                    .endpoint(endpoint.clone())
                    .apollo_key(key)
                    .apollo_graph_ref(reference)
                    .client_name_header(client_name_header)
                    .client_version_header(client_version_header)
                    .schema_id(schema_id)
                    .buffer_size(*buffer_size)
                    .build();
                builder.with_batch_exporter(exporter, opentelemetry::runtime::Tokio)
            }
            _ => builder,
        })
    }
}
