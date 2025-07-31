use crate::telemetry::{
    config::TelemetryConfig,
    events::{CommandExecution, RecipeExecution, SessionExecution, TelemetryEvent},
};
use opentelemetry::{
    metrics::{Counter, Histogram, MeterProvider},
    trace::{Span, Tracer, TracerProvider as OtelTracerProvider},
    KeyValue,
};
use opentelemetry_sdk::{
    metrics::{PeriodicReader, SdkMeterProvider},
    trace::SdkTracerProvider,
    Resource,
};
use opentelemetry_semantic_conventions as semconv;
use std::time::Duration;

pub struct OtlpProvider {
    tracer_provider: Option<SdkTracerProvider>,
    meter_provider: Option<SdkMeterProvider>,
    recipe_counter: Option<Counter<u64>>,
    recipe_duration: Option<Histogram<f64>>,
    session_counter: Option<Counter<u64>>,
    session_duration: Option<Histogram<f64>>,
    command_counter: Option<Counter<u64>>,
    command_duration: Option<Histogram<f64>>,
    token_counter: Option<Counter<u64>>,
    tool_counter: Option<Counter<u64>>,
    initialized: bool,
}

impl OtlpProvider {
    pub fn new() -> Self {
        Self {
            tracer_provider: None,
            meter_provider: None,
            recipe_counter: None,
            recipe_duration: None,
            session_counter: None,
            session_duration: None,
            command_counter: None,
            command_duration: None,
            token_counter: None,
            tool_counter: None,
            initialized: false,
        }
    }

    async fn init_metrics(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(meter_provider) = &self.meter_provider {
            let meter = meter_provider.meter("goose");

            self.recipe_counter = Some(
                meter
                    .u64_counter("goose.recipe.executions")
                    .with_description("Number of recipe executions")
                    .build(),
            );

            self.recipe_duration = Some(
                meter
                    .f64_histogram("goose.recipe.duration")
                    .with_description("Recipe execution duration in seconds")
                    .build(),
            );

            self.session_counter = Some(
                meter
                    .u64_counter("goose.session.executions")
                    .with_description("Number of session executions")
                    .build(),
            );

            self.session_duration = Some(
                meter
                    .f64_histogram("goose.session.duration")
                    .with_description("Session execution duration in seconds")
                    .build(),
            );

            self.command_counter = Some(
                meter
                    .u64_counter("goose.command.executions")
                    .with_description("Number of command executions")
                    .build(),
            );

            self.command_duration = Some(
                meter
                    .f64_histogram("goose.command.duration")
                    .with_description("Command execution duration in seconds")
                    .build(),
            );

            self.token_counter = Some(
                meter
                    .u64_counter("goose.tokens.used")
                    .with_description("Number of tokens used")
                    .build(),
            );

            self.tool_counter = Some(
                meter
                    .u64_counter("goose.tool.calls")
                    .with_description("Number of tool calls")
                    .build(),
            );
        }

        Ok(())
    }

    fn record_recipe_execution(&self, execution: &RecipeExecution) {
        let attributes = vec![
            KeyValue::new("recipe.name", execution.recipe_name.clone()),
            KeyValue::new("recipe.version", execution.recipe_version.clone()),
            KeyValue::new("usage.type", format!("{:?}", execution.usage_type)),
            KeyValue::new(
                "result",
                format!(
                    "{:?}",
                    execution
                        .result
                        .as_ref()
                        .unwrap_or(&crate::telemetry::events::SessionResult::Success)
                ),
            ),
        ];

        if let Some(counter) = &self.recipe_counter {
            counter.add(1, &attributes);
        }

        if let (Some(histogram), Some(duration_ms)) = (&self.recipe_duration, execution.duration_ms)
        {
            histogram.record(duration_ms as f64 / 1000.0, &attributes);
        }

        if let (Some(counter), Some(token_usage)) = (&self.token_counter, &execution.token_usage) {
            let token_attributes = [
                attributes.clone(),
                vec![
                    KeyValue::new("token.type", "input"),
                    KeyValue::new("model", token_usage.model.clone().unwrap_or_default()),
                    KeyValue::new("provider", token_usage.provider.clone().unwrap_or_default()),
                ],
            ]
            .concat();

            counter.add(token_usage.input_tokens, &token_attributes);

            let output_attributes = [
                attributes.clone(),
                vec![
                    KeyValue::new("token.type", "output"),
                    KeyValue::new("model", token_usage.model.clone().unwrap_or_default()),
                    KeyValue::new("provider", token_usage.provider.clone().unwrap_or_default()),
                ],
            ]
            .concat();

            counter.add(token_usage.output_tokens, &output_attributes);
        }

        if let Some(counter) = &self.tool_counter {
            for tool_usage in &execution.tool_usage {
                let tool_attributes = [
                    attributes.clone(),
                    vec![
                        KeyValue::new("tool.name", tool_usage.tool_name.clone()),
                        KeyValue::new("tool.result", "success"),
                    ],
                ]
                .concat();

                counter.add(tool_usage.success_count, &tool_attributes);

                if tool_usage.error_count > 0 {
                    let error_attributes = [
                        attributes.clone(),
                        vec![
                            KeyValue::new("tool.name", tool_usage.tool_name.clone()),
                            KeyValue::new("tool.result", "error"),
                        ],
                    ]
                    .concat();

                    counter.add(tool_usage.error_count, &error_attributes);
                }
            }
        }
    }

    fn create_recipe_span(&self, execution: &RecipeExecution) {
        if let Some(tracer_provider) = &self.tracer_provider {
            let tracer = tracer_provider.tracer("goose");
            let mut span = tracer
                .span_builder(format!("recipe.{}", execution.recipe_name))
                .with_attributes(vec![
                    KeyValue::new("recipe.name", execution.recipe_name.clone()),
                    KeyValue::new("recipe.version", execution.recipe_version.clone()),
                    KeyValue::new("usage.type", format!("{:?}", execution.usage_type)),
                    KeyValue::new("user.id", execution.user_id.clone()),
                ])
                .start(&tracer);

            if let Some(result) = &execution.result {
                span.set_attribute(KeyValue::new("result", format!("{:?}", result)));
            }

            if let Some(duration_ms) = execution.duration_ms {
                span.set_attribute(KeyValue::new("duration.ms", duration_ms as i64));
            }

            if let Some(error_details) = &execution.error_details {
                span.set_attribute(KeyValue::new(
                    "error.type",
                    error_details.error_type.clone(),
                ));
                span.set_attribute(KeyValue::new(
                    "error.message",
                    error_details.error_message.clone(),
                ));
                span.record_error(&std::io::Error::other(error_details.error_message.clone()));
            }

            span.end();
        }
    }

    fn record_session_execution(&self, execution: &SessionExecution) {
        let attributes = vec![
            KeyValue::new("session.type", format!("{:?}", execution.session_type)),
            KeyValue::new("usage.type", format!("{:?}", execution.usage_type)),
            KeyValue::new(
                "result",
                format!(
                    "{:?}",
                    execution
                        .result
                        .as_ref()
                        .unwrap_or(&crate::telemetry::events::SessionResult::Success)
                ),
            ),
            KeyValue::new("execution.type", "session"),
        ];

        if let Some(counter) = &self.session_counter {
            counter.add(1, &attributes);
        }

        if let (Some(histogram), Some(duration_ms)) =
            (&self.session_duration, execution.duration_ms)
        {
            histogram.record(duration_ms as f64 / 1000.0, &attributes);
        }

        if let (Some(counter), Some(token_usage)) = (&self.token_counter, &execution.token_usage) {
            let token_attributes = [
                attributes.clone(),
                vec![
                    KeyValue::new("token.type", "input"),
                    KeyValue::new("model", token_usage.model.clone().unwrap_or_default()),
                    KeyValue::new("provider", token_usage.provider.clone().unwrap_or_default()),
                ],
            ]
            .concat();

            counter.add(token_usage.input_tokens, &token_attributes);

            let output_attributes = [
                attributes.clone(),
                vec![
                    KeyValue::new("token.type", "output"),
                    KeyValue::new("model", token_usage.model.clone().unwrap_or_default()),
                    KeyValue::new("provider", token_usage.provider.clone().unwrap_or_default()),
                ],
            ]
            .concat();

            counter.add(token_usage.output_tokens, &output_attributes);
        }

        if let Some(counter) = &self.tool_counter {
            for tool_usage in &execution.tool_usage {
                let tool_attributes = [
                    attributes.clone(),
                    vec![
                        KeyValue::new("tool.name", tool_usage.tool_name.clone()),
                        KeyValue::new("tool.result", "success"),
                    ],
                ]
                .concat();

                counter.add(tool_usage.success_count, &tool_attributes);

                if tool_usage.error_count > 0 {
                    let error_attributes = [
                        attributes.clone(),
                        vec![
                            KeyValue::new("tool.name", tool_usage.tool_name.clone()),
                            KeyValue::new("tool.result", "error"),
                        ],
                    ]
                    .concat();

                    counter.add(tool_usage.error_count, &error_attributes);
                }
            }
        }
    }

    fn create_session_span(&self, execution: &SessionExecution) {
        if let Some(tracer_provider) = &self.tracer_provider {
            let tracer = tracer_provider.tracer("goose");
            let mut span = tracer
                .span_builder(format!("session.{}", execution.session_id))
                .with_attributes(vec![
                    KeyValue::new("session.id", execution.session_id.clone()),
                    KeyValue::new("session.type", format!("{:?}", execution.session_type)),
                    KeyValue::new("usage.type", format!("{:?}", execution.usage_type)),
                    KeyValue::new("user.id", execution.user_id.clone()),
                    KeyValue::new("message.count", execution.message_count as i64),
                    KeyValue::new("turn.count", execution.turn_count as i64),
                ])
                .start(&tracer);

            if let Some(result) = &execution.result {
                span.set_attribute(KeyValue::new("result", format!("{:?}", result)));
            }

            if let Some(duration_ms) = execution.duration_ms {
                span.set_attribute(KeyValue::new("duration.ms", duration_ms as i64));
            }

            if let Some(error_details) = &execution.error_details {
                span.set_attribute(KeyValue::new(
                    "error.type",
                    error_details.error_type.clone(),
                ));
                span.set_attribute(KeyValue::new(
                    "error.message",
                    error_details.error_message.clone(),
                ));
                span.record_error(&std::io::Error::other(error_details.error_message.clone()));
            }

            span.end();
        }
    }

    fn record_command_execution(&self, execution: &CommandExecution) {
        let attributes = vec![
            KeyValue::new("command.name", execution.command_name.clone()),
            KeyValue::new("command.type", format!("{:?}", execution.command_type)),
            KeyValue::new("usage.type", format!("{:?}", execution.usage_type)),
            KeyValue::new(
                "result",
                format!(
                    "{:?}",
                    execution
                        .result
                        .as_ref()
                        .unwrap_or(&crate::telemetry::events::CommandResult::Success)
                ),
            ),
            KeyValue::new("execution.type", "command"),
        ];

        if let Some(counter) = &self.command_counter {
            counter.add(1, &attributes);
        }

        if let (Some(histogram), Some(duration_ms)) =
            (&self.command_duration, execution.duration_ms)
        {
            histogram.record(duration_ms as f64 / 1000.0, &attributes);
        }
    }

    fn create_command_span(&self, execution: &CommandExecution) {
        if let Some(tracer_provider) = &self.tracer_provider {
            let tracer = tracer_provider.tracer("goose");
            let mut span = tracer
                .span_builder(format!("command.{}", execution.command_name))
                .with_attributes(vec![
                    KeyValue::new("command.name", execution.command_name.clone()),
                    KeyValue::new("command.type", format!("{:?}", execution.command_type)),
                    KeyValue::new("usage.type", format!("{:?}", execution.usage_type)),
                    KeyValue::new("user.id", execution.user_id.clone()),
                ])
                .start(&tracer);

            if let Some(result) = &execution.result {
                span.set_attribute(KeyValue::new("result", format!("{:?}", result)));
            }

            if let Some(duration_ms) = execution.duration_ms {
                span.set_attribute(KeyValue::new("duration.ms", duration_ms as i64));
            }

            if let Some(error_details) = &execution.error_details {
                span.set_attribute(KeyValue::new(
                    "error.type",
                    error_details.error_type.clone(),
                ));
                span.set_attribute(KeyValue::new(
                    "error.message",
                    error_details.error_message.clone(),
                ));
                span.record_error(&std::io::Error::other(error_details.error_message.clone()));
            }

            span.end();
        }
    }
}

#[async_trait::async_trait]
impl super::TelemetryBackend for OtlpProvider {
    async fn initialize(
        &mut self,
        config: &TelemetryConfig,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.initialized {
            return Ok(());
        }

        let resource = Resource::builder()
            .with_attributes(vec![
                KeyValue::new(semconv::resource::SERVICE_NAME, config.service_name.clone()),
                KeyValue::new(
                    semconv::resource::SERVICE_VERSION,
                    config.service_version.clone(),
                ),
                KeyValue::new(
                    "goose.usage_type",
                    format!(
                        "{:?}",
                        config
                            .usage_type
                            .as_ref()
                            .unwrap_or(&crate::telemetry::config::UsageType::Human)
                    ),
                ),
            ])
            .build();

        use opentelemetry_otlp::{WithExportConfig, WithHttpConfig, WithTonicConfig};

        let endpoint = config.get_endpoint().ok_or(
            "OTLP provider requires GOOSE_TELEMETRY_ENDPOINT or OTEL_EXPORTER_OTLP_ENDPOINT",
        )?;

        // Check if HTTP protocol is explicitly requested
        let use_http = std::env::var("GOOSE_TELEMETRY_PROTOCOL")
            .map(|p| p.to_lowercase() == "http")
            .unwrap_or(false);

        let otlp_trace_exporter = if use_http {
            eprintln!("🌐 OTLP: Using HTTP/protobuf protocol");
            let mut builder = opentelemetry_otlp::SpanExporter::builder()
                .with_http()
                .with_endpoint(format!("{}/v1/traces", endpoint.trim_end_matches('/')));

            if let Some(api_key) = &config.api_key {
                use std::collections::HashMap;

                let mut headers = HashMap::new();

                let header_name = std::env::var("GOOSE_TELEMETRY_AUTH_HEADER")
                    .unwrap_or_else(|_| "x-api-key".to_string());

                headers.insert(header_name, api_key.clone());

                builder = builder.with_headers(headers);
                tracing::info!("OTLP provider configured with API key authentication");
            }

            // Use default HTTP client for protobuf format
            eprintln!(
                "🔧 OTLP: Trace exporter endpoint: {}/v1/traces",
                endpoint.trim_end_matches('/')
            );

            builder.build()?
        } else {
            eprintln!("🚀 OTLP: Using gRPC protocol (default)");
            let mut builder = opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .with_endpoint(endpoint.clone());

            if let Some(api_key) = &config.api_key {
                use tonic::metadata::{MetadataMap, MetadataValue};

                let mut metadata = MetadataMap::new();

                let header_name = std::env::var("GOOSE_TELEMETRY_AUTH_HEADER")
                    .unwrap_or_else(|_| "x-api-key".to_string());

                let metadata_value = MetadataValue::try_from(api_key.as_str())
                    .map_err(|e| format!("Invalid API key format: {}", e))?;

                match header_name.as_str() {
                    "x-api-key" => metadata.insert("x-api-key", metadata_value),
                    "authorization" => metadata.insert("authorization", metadata_value),
                    "x-honeycomb-team" => metadata.insert("x-honeycomb-team", metadata_value),
                    "x-otlp-api-key" => metadata.insert("x-otlp-api-key", metadata_value),
                    _ => {
                        let key = tonic::metadata::MetadataKey::from_bytes(header_name.as_bytes())
                            .map_err(|e| format!("Invalid header name '{}': {}", header_name, e))?;
                        metadata.insert(key, metadata_value)
                    }
                };

                builder = builder.with_metadata(metadata);
                tracing::info!("OTLP provider configured with API key authentication");
            }

            builder.build()?
        };

        let tracer_provider = SdkTracerProvider::builder()
            .with_resource(resource.clone())
            .with_batch_exporter(otlp_trace_exporter)
            .build();

        self.tracer_provider = Some(tracer_provider);

        let otlp_metrics_exporter = if use_http {
            let mut builder = opentelemetry_otlp::MetricExporter::builder()
                .with_http()
                .with_endpoint(format!("{}/v1/metrics", endpoint.trim_end_matches('/')));

            if let Some(api_key) = &config.api_key {
                use std::collections::HashMap;

                let mut headers = HashMap::new();

                let header_name = std::env::var("GOOSE_TELEMETRY_AUTH_HEADER")
                    .unwrap_or_else(|_| "x-api-key".to_string());

                headers.insert(header_name, api_key.clone());

                builder = builder.with_headers(headers);
            }

            // Use default HTTP client for protobuf format
            eprintln!(
                "🔧 OTLP: Metrics exporter endpoint: {}/v1/metrics",
                endpoint.trim_end_matches('/')
            );

            builder.build()?
        } else {
            let mut builder = opentelemetry_otlp::MetricExporter::builder()
                .with_tonic()
                .with_endpoint(endpoint.clone());

            if let Some(api_key) = &config.api_key {
                use tonic::metadata::{MetadataMap, MetadataValue};

                let mut metadata = MetadataMap::new();

                let header_name = std::env::var("GOOSE_TELEMETRY_AUTH_HEADER")
                    .unwrap_or_else(|_| "x-api-key".to_string());

                let metadata_value = MetadataValue::try_from(api_key.as_str())
                    .map_err(|e| format!("Invalid API key format: {}", e))?;

                match header_name.as_str() {
                    "x-api-key" => metadata.insert("x-api-key", metadata_value),
                    "authorization" => metadata.insert("authorization", metadata_value),
                    "x-honeycomb-team" => metadata.insert("x-honeycomb-team", metadata_value),
                    "x-otlp-api-key" => metadata.insert("x-otlp-api-key", metadata_value),
                    _ => {
                        let key = tonic::metadata::MetadataKey::from_bytes(header_name.as_bytes())
                            .map_err(|e| format!("Invalid header name '{}': {}", header_name, e))?;
                        metadata.insert(key, metadata_value)
                    }
                };

                builder = builder.with_metadata(metadata);
            }

            builder.build()?
        };

        let meter_provider = SdkMeterProvider::builder()
            .with_resource(resource)
            .with_reader(
                PeriodicReader::builder(otlp_metrics_exporter)
                    .with_interval(Duration::from_secs(1)) // Very short interval for immediate export
                    .build(),
            )
            .build();

        self.meter_provider = Some(meter_provider);

        self.init_metrics().await?;
        self.initialized = true;

        let auth_status = if config.api_key.is_some() {
            "with authentication"
        } else {
            "without authentication"
        };
        eprintln!(
            "🔧 OTLP telemetry provider initialized with endpoint: {} ({})",
            endpoint, auth_status
        );
        tracing::info!(
            "OTLP telemetry provider initialized with endpoint: {} ({})",
            endpoint,
            auth_status
        );
        Ok(())
    }

    async fn send_event(
        &self,
        event: &TelemetryEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !self.initialized {
            return Err("OTLP provider not initialized".into());
        }

        match event {
            TelemetryEvent::RecipeExecution(execution) => {
                eprintln!(
                    "📊 OTLP: Recording recipe execution: {}",
                    execution.recipe_name
                );
                self.record_recipe_execution(execution);
                self.create_recipe_span(execution);
                eprintln!("✅ OTLP: Recipe span created and recorded");
            }
            TelemetryEvent::SessionExecution(execution) => {
                eprintln!(
                    "📊 OTLP: Recording session execution: {}",
                    execution.session_id
                );
                self.record_session_execution(execution);
                self.create_session_span(execution);
                eprintln!("✅ OTLP: Session span created and recorded");
            }
            TelemetryEvent::CommandExecution(execution) => {
                eprintln!(
                    "📊 OTLP: Recording command execution: {}",
                    execution.command_name
                );
                self.record_command_execution(execution);
                self.create_command_span(execution);
                eprintln!("✅ OTLP: Command span created and recorded");
            }
            TelemetryEvent::SystemMetrics(_metrics) => {
                // System metrics could be implemented in the future
            }
            TelemetryEvent::UserSession(_session) => {
                // User session metrics could be implemented in the future
            }
        }

        // Let the standard OTLP library handle the export
        eprintln!("✅ OTLP: Event recorded successfully (will be exported periodically by PeriodicReader)");

        Ok(())
    }

    async fn shutdown(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.initialized {
            eprintln!("🛑 OTLP: Shutting down telemetry provider...");

            // Use proper async context for shutdown flush
            if let Ok(handle) = tokio::runtime::Handle::try_current() {
                eprintln!("✅ OTLP: Found active Tokio runtime for shutdown flush");

                if let Some(tracer_provider) = &self.tracer_provider {
                    eprintln!("🔄 OTLP: Shutdown flushing traces with runtime context...");
                    let tracer_provider = tracer_provider.clone();
                    let result = handle
                        .spawn(async move { tracer_provider.force_flush() })
                        .await;

                    match result {
                        Ok(Ok(_)) => eprintln!("✅ OTLP: Shutdown traces flush successful"),
                        Ok(Err(e)) => eprintln!("❌ OTLP: Shutdown traces flush error: {:?}", e),
                        Err(e) => eprintln!("❌ OTLP: Shutdown traces spawn error: {:?}", e),
                    }
                }

                if let Some(meter_provider) = &self.meter_provider {
                    eprintln!("🔄 OTLP: Shutdown flushing metrics with runtime context...");
                    let meter_provider = meter_provider.clone();
                    let result = handle
                        .spawn(async move { meter_provider.force_flush() })
                        .await;

                    match result {
                        Ok(Ok(_)) => eprintln!("✅ OTLP: Shutdown metrics flush successful"),
                        Ok(Err(e)) => eprintln!("❌ OTLP: Shutdown metrics flush error: {:?}", e),
                        Err(e) => eprintln!("❌ OTLP: Shutdown metrics spawn error: {:?}", e),
                    }
                }
            } else {
                eprintln!("⚠️ OTLP: No active Tokio runtime for shutdown flush");
                eprintln!("   (This is expected during shutdown - data should have been exported periodically)");
            }

            eprintln!("✅ OTLP: Shutdown complete");
            tracing::info!("OTLP telemetry provider shutdown successfully");
        }
        Ok(())
    }
}
