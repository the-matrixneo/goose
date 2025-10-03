use base64::{engine::general_purpose::STANDARD, Engine as _};
use etcetera::{choose_app_strategy, AppStrategy};
use indoc::formatdoc;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, Content, ErrorCode, ErrorData, Implementation, ResourceContents, Role,
        ServerCapabilities, ServerInfo,
    },
    tool, tool_handler, tool_router, ServerHandler,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

/// Validates that the data parameter is a proper JSON value and not a string
fn validate_data_param(params: &Value, allow_array: bool) -> Result<Value, ErrorData> {
    let data_value = params.get("data").ok_or_else(|| {
        ErrorData::new(
            ErrorCode::INVALID_PARAMS,
            "Missing 'data' parameter".to_string(),
            None,
        )
    })?;

    if data_value.is_string() {
        return Err(ErrorData::new(
            ErrorCode::INVALID_PARAMS,
            "The 'data' parameter must be a JSON object, not a JSON string. Please provide valid JSON without comments.".to_string(),
            None,
        ));
    }

    if allow_array {
        if !data_value.is_object() && !data_value.is_array() {
            return Err(ErrorData::new(
                ErrorCode::INVALID_PARAMS,
                "The 'data' parameter must be a JSON object or array.".to_string(),
                None,
            ));
        }
    } else if !data_value.is_object() {
        return Err(ErrorData::new(
            ErrorCode::INVALID_PARAMS,
            "The 'data' parameter must be a JSON object.".to_string(),
            None,
        ));
    }

    Ok(data_value.clone())
}

/// Sankey node structure
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct SankeyNode {
    /// The name of the node
    pub name: String,
    /// Optional category for the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

/// Sankey link structure
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct SankeyLink {
    /// Source node name
    pub source: String,
    /// Target node name
    pub target: String,
    /// Flow value
    pub value: f64,
}

/// Sankey data structure
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct SankeyData {
    /// Array of nodes
    pub nodes: Vec<SankeyNode>,
    /// Array of links between nodes
    pub links: Vec<SankeyLink>,
}

/// Parameters for render_sankey tool
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct RenderSankeyParams {
    /// The data for the Sankey diagram
    pub data: SankeyData,
}

/// Radar dataset structure
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct RadarDataset {
    /// Label for this dataset
    pub label: String,
    /// Data values for each category
    pub data: Vec<f64>,
}

/// Radar chart data structure
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct RadarData {
    /// Category labels
    pub labels: Vec<String>,
    /// Datasets to compare
    pub datasets: Vec<RadarDataset>,
}

/// Parameters for render_radar tool
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct RenderRadarParams {
    /// The data for the radar chart
    pub data: RadarData,
}

/// Data item for donut/pie charts - can be a number or labeled value
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
#[serde(untagged)]
pub enum DonutDataItem {
    /// Simple numeric value
    Number(f64),
    /// Labeled value with explicit label
    LabeledValue {
        /// Label for this data point
        label: String,
        /// Numeric value
        value: f64,
    },
}

/// Chart type for donut/pie charts
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum DonutChartType {
    /// Doughnut chart (with hole in center)
    Doughnut,
    /// Pie chart (no hole)
    Pie,
}

/// Single donut/pie chart data
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct SingleDonutChart {
    /// Data values - can be numbers or objects with label and value
    pub data: Vec<DonutDataItem>,
    /// Optional chart title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional chart type (doughnut or pie)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub chart_type: Option<DonutChartType>,
    /// Optional labels array (used when data is just numbers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
}

/// Donut chart data wrapper - matches the old schema structure
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
#[serde(untagged)]
pub enum DonutChartData {
    /// Single donut chart
    Single(SingleDonutChart),
    /// Multiple donut charts
    Multiple(Vec<SingleDonutChart>),
}

/// Root structure for donut chart data - matches old schema
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct DonutData {
    /// The chart data (single or multiple charts)
    pub data: DonutChartData,
}

/// Parameters for render_donut tool
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct RenderDonutParams {
    /// The data for the donut/pie chart(s) - wrapped in data property
    #[serde(flatten)]
    pub data: DonutData,
}

/// Treemap node structure
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct TreemapNode {
    /// Name of the node
    pub name: String,
    /// Value for leaf nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// Category for coloring
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Children nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<TreemapNode>>,
}

/// Parameters for render_treemap tool
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct RenderTreemapParams {
    /// The hierarchical data for the treemap
    pub data: TreemapNode,
}

/// Chord diagram data structure
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct ChordData {
    /// Labels for each entity
    pub labels: Vec<String>,
    /// 2D matrix of flows (matrix[i][j] = flow from i to j)
    pub matrix: Vec<Vec<f64>>,
}

/// Parameters for render_chord tool
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct RenderChordParams {
    /// The data for the chord diagram
    pub data: ChordData,
}

/// Map marker structure
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct MapMarker {
    /// Latitude (required)
    pub lat: f64,
    /// Longitude (required)
    pub lng: f64,
    /// Location name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Numeric value for sizing/coloring
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// Description text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Custom popup HTML
    #[serde(skip_serializing_if = "Option::is_none")]
    pub popup: Option<String>,
    /// Custom marker color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Custom marker label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Use default Leaflet icon
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "useDefaultIcon")]
    pub use_default_icon: Option<bool>,
}

/// Map center point
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct MapCenter {
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lng: f64,
}

/// Map data structure
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct MapData {
    /// Array of markers
    pub markers: Vec<MapMarker>,
    /// Optional title for the map
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional subtitle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    /// Optional center point
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<MapCenter>,
    /// Optional initial zoom level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<f64>,
    /// Optional boolean to enable/disable clustering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clustering: Option<bool>,
    /// Optional cluster radius
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clusterRadius")]
    pub cluster_radius: Option<f64>,
    /// Optional boolean to auto-fit map to markers
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "autoFit")]
    pub auto_fit: Option<bool>,
}

/// Parameters for render_map tool
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct RenderMapParams {
    /// The data for the map visualization
    pub data: MapData,
}

/// Chart data point for scatter charts
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct ChartPoint {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

/// Chart dataset structure
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct ChartDataset {
    /// Label for this dataset
    pub label: String,
    /// Data points - can be numbers or x/y points
    pub data: ChartDataValues,
    /// Optional background color for the dataset
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<String>,
    /// Optional border color for the dataset
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "borderColor")]
    pub border_color: Option<String>,
    /// Optional border width for the dataset
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "borderWidth")]
    pub border_width: Option<f64>,
    /// Optional tension for line curves (0 = straight lines, higher = more curved)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tension: Option<f64>,
    /// Optional fill setting for area under the line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<bool>,
}

/// Chart data values - can be simple numbers or x/y points
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
#[serde(untagged)]
pub enum ChartDataValues {
    /// Simple numeric values (for line/bar charts with labels)
    Numbers(Vec<f64>),
    /// X/Y points (for scatter charts or line charts without labels)
    Points(Vec<ChartPoint>),
}

/// Chart type enumeration
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ChartType {
    /// Line chart
    Line,
    /// Scatter chart
    Scatter,
    /// Bar chart
    Bar,
}

/// Chart data structure
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct ChartData {
    /// Chart type
    #[serde(rename = "type")]
    pub chart_type: ChartType,
    /// Datasets to display
    pub datasets: Vec<ChartDataset>,
    /// Optional labels for x-axis (for line/bar charts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Optional chart title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional subtitle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    /// Optional x-axis label
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "xAxisLabel")]
    pub x_axis_label: Option<String>,
    /// Optional y-axis label
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "yAxisLabel")]
    pub y_axis_label: Option<String>,
}

/// Parameters for show_chart tool
#[derive(Debug, Serialize, Deserialize, rmcp::schemars::JsonSchema)]
pub struct ShowChartParams {
    /// The data for the chart
    pub data: ChartData,
}

/// An extension for automatic data visualization and UI generation
#[derive(Clone)]
pub struct AutoVisualiserRouter {
    tool_router: ToolRouter<Self>,
    #[allow(dead_code)]
    cache_dir: PathBuf,
    instructions: String,
}

impl Default for AutoVisualiserRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for AutoVisualiserRouter {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "goose-autovisualiser".to_string(),
                version: env!("CARGO_PKG_VERSION").to_owned(),
                title: None,
                icons: None,
                website_url: None,
            },
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            instructions: Some(self.instructions.clone()),
            ..Default::default()
        }
    }
}

#[tool_router(router = tool_router)]
impl AutoVisualiserRouter {
    pub fn new() -> Self {
        // choose_app_strategy().cache_dir()
        // - macOS/Linux: ~/.cache/goose/autovisualiser/
        // - Windows:     ~\AppData\Local\Block\goose\cache\autovisualiser\
        let cache_dir = choose_app_strategy(crate::APP_STRATEGY.clone())
            .unwrap()
            .cache_dir()
            .join("autovisualiser");

        // Create cache directory if it doesn't exist
        let _ = std::fs::create_dir_all(&cache_dir);

        let instructions = formatdoc! {r#"
            This extension provides tools for automatic data visualization
            Use these tools when you are presenting data to the user which could be complemented by a visual expression
            Choose the most appropriate chart type based on the data you have and can provide
            It is important you match the data format as appropriate with the chart type you have chosen
            The user may specify a type of chart or you can pick one of the most appopriate that you can shape the data to

            ## Available Tools:
            - **render_sankey**: Creates interactive Sankey diagrams from flow data
            - **render_radar**: Creates interactive radar charts for multi-dimensional data comparison
            - **render_donut**: Creates interactive donut/pie charts for categorical data (supports multiple charts)
            - **render_treemap**: Creates interactive treemap visualizations for hierarchical data
            - **render_chord**: Creates interactive chord diagrams for relationship/flow visualization
            - **render_map**: Creates interactive map visualizations with location markers
            - **show_chart**: Creates interactive line, scatter, or bar charts for data visualization
        "#};

        Self {
            tool_router: Self::tool_router(),
            cache_dir,
            instructions,
        }
    }

    /// show a Sankey diagram from flow data
    #[tool(
        name = "render_sankey",
        description = r#"show a Sankey diagram from flow data
The data must contain:
- nodes: Array of objects with 'name' and optional 'category' properties
- links: Array of objects with 'source', 'target', and 'value' properties

Example:
{
  "nodes": [
    {"name": "Source A", "category": "source"},
    {"name": "Target B", "category": "target"}
  ],
  "links": [
    {"source": "Source A", "target": "Target B", "value": 100}
  ]
}"#
    )]
    pub async fn render_sankey(
        &self,
        params: Parameters<RenderSankeyParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let data = validate_data_param(
            &serde_json::to_value(params.0).map_err(|e| {
                ErrorData::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid parameters: {}", e),
                    None,
                )
            })?,
            false,
        )?;

        // Convert the data to JSON string
        let data_json = serde_json::to_string(&data).map_err(|e| {
            ErrorData::new(
                ErrorCode::INVALID_PARAMS,
                format!("Invalid JSON data: {}", e),
                None,
            )
        })?;

        // Load all resources at compile time using include_str!
        const TEMPLATE: &str = include_str!("templates/sankey_template.html");
        const D3_MIN: &str = include_str!("templates/assets/d3.min.js");
        const D3_SANKEY: &str = include_str!("templates/assets/d3.sankey.min.js");

        // Replace all placeholders with actual content
        let html_content = TEMPLATE
            .replace("{{D3_MIN}}", D3_MIN)
            .replace("{{D3_SANKY}}", D3_SANKEY) // Note: keeping the typo to match template
            .replace("{{SANKEY_DATA}}", &data_json);

        // Save to /tmp/vis.html for debugging
        let debug_path = std::path::Path::new("/tmp/vis.html");
        if let Err(e) = std::fs::write(debug_path, &html_content) {
            tracing::warn!("Failed to write debug HTML to /tmp/vis.html: {}", e);
        } else {
            tracing::info!("Debug HTML saved to /tmp/vis.html");
        }

        // Use BlobResourceContents with base64 encoding to avoid JSON string escaping issues
        let html_bytes = html_content.as_bytes();
        let base64_encoded = STANDARD.encode(html_bytes);

        let resource_contents = ResourceContents::BlobResourceContents {
            uri: "ui://sankey/diagram".to_string(),
            mime_type: Some("text/html".to_string()),
            blob: base64_encoded,
            meta: None,
        };

        Ok(CallToolResult::success(vec![Content::resource(
            resource_contents,
        )
        .with_audience(vec![Role::User])]))
    }

    /// show a radar chart (spider chart) for multi-dimensional data comparison
    #[tool(
        name = "render_radar",
        description = r#"show a radar chart (spider chart) for multi-dimensional data comparison

The data must contain:
- labels: Array of strings representing the dimensions/axes
- datasets: Array of dataset objects with 'label' and 'data' properties

Example:
{
  "labels": ["Speed", "Strength", "Endurance", "Agility", "Intelligence"],
  "datasets": [
    {
      "label": "Player 1",
      "data": [85, 70, 90, 75, 80]
    },
    {
      "label": "Player 2",
      "data": [75, 85, 80, 90, 70]
    }
  ]
}"#
    )]
    pub async fn render_radar(
        &self,
        params: Parameters<RenderRadarParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let data = validate_data_param(
            &serde_json::to_value(params.0).map_err(|e| {
                ErrorData::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid parameters: {}", e),
                    None,
                )
            })?,
            false,
        )?;

        // Convert the data to JSON string
        let data_json = serde_json::to_string(&data).map_err(|e| {
            ErrorData::new(
                ErrorCode::INVALID_PARAMS,
                format!("Invalid JSON data: {}", e),
                None,
            )
        })?;

        // Load all resources at compile time using include_str!
        const TEMPLATE: &str = include_str!("templates/radar_template.html");
        const CHART_MIN: &str = include_str!("templates/assets/chart.min.js");

        // Replace all placeholders with actual content
        let html_content = TEMPLATE
            .replace("{{CHART_MIN}}", CHART_MIN)
            .replace("{{RADAR_DATA}}", &data_json);

        // Save to /tmp/radar.html for debugging
        let debug_path = std::path::Path::new("/tmp/radar.html");
        if let Err(e) = std::fs::write(debug_path, &html_content) {
            tracing::warn!("Failed to write debug HTML to /tmp/radar.html: {}", e);
        } else {
            tracing::info!("Debug HTML saved to /tmp/radar.html");
        }

        // Use BlobResourceContents with base64 encoding to avoid JSON string escaping issues
        let html_bytes = html_content.as_bytes();
        let base64_encoded = STANDARD.encode(html_bytes);

        let resource_contents = ResourceContents::BlobResourceContents {
            uri: "ui://radar/chart".to_string(),
            mime_type: Some("text/html".to_string()),
            blob: base64_encoded,
            meta: None,
        };

        Ok(CallToolResult::success(vec![Content::resource(
            resource_contents,
        )
        .with_audience(vec![Role::User])]))
    }

    /// show pie or donut charts for categorical data visualization
    #[tool(
        name = "render_donut",
        description = r#"show pie or donut charts for categorical data visualization
Supports single or multiple charts in a grid layout.

Each chart should contain:
- data: Array of values or objects with 'label' and 'value'
- type: Optional 'doughnut' (default) or 'pie'
- title: Optional chart title
- labels: Optional array of labels (if data is just numbers)

Example single chart:
{
  "title": "Budget",
  "type": "doughnut",
  "data": [
    {"label": "Marketing", "value": 25000},
    {"label": "Development", "value": 35000}
  ]
}

Example multiple charts:
[{
  "title": "Q1 Sales",
  "labels": ["Product A", "Product B"],
  "data": [45000, 38000]
}]"#
    )]
    pub async fn render_donut(
        &self,
        params: Parameters<RenderDonutParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let data = validate_data_param(
            &serde_json::to_value(params.0).map_err(|e| {
                ErrorData::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid parameters: {}", e),
                    None,
                )
            })?,
            true,
        )?; // true because donut accepts arrays

        // Convert the data to JSON string
        let data_json = serde_json::to_string(&data).map_err(|e| {
            ErrorData::new(
                ErrorCode::INVALID_PARAMS,
                format!("Invalid JSON data: {}", e),
                None,
            )
        })?;

        // Load all resources at compile time using include_str!
        const TEMPLATE: &str = include_str!("templates/donut_template.html");
        const CHART_MIN: &str = include_str!("templates/assets/chart.min.js");

        // Replace all placeholders with actual content
        let html_content = TEMPLATE
            .replace("{{CHART_MIN}}", CHART_MIN)
            .replace("{{CHARTS_DATA}}", &data_json);

        // Save to /tmp/donut.html for debugging
        let debug_path = std::path::Path::new("/tmp/donut.html");
        if let Err(e) = std::fs::write(debug_path, &html_content) {
            tracing::warn!("Failed to write debug HTML to /tmp/donut.html: {}", e);
        } else {
            tracing::info!("Debug HTML saved to /tmp/donut.html");
        }

        // Use BlobResourceContents with base64 encoding to avoid JSON string escaping issues
        let html_bytes = html_content.as_bytes();
        let base64_encoded = STANDARD.encode(html_bytes);

        let resource_contents = ResourceContents::BlobResourceContents {
            uri: "ui://donut/chart".to_string(),
            mime_type: Some("text/html".to_string()),
            blob: base64_encoded,
            meta: None,
        };

        Ok(CallToolResult::success(vec![Content::resource(
            resource_contents,
        )
        .with_audience(vec![Role::User])]))
    }

    /// show a treemap visualization for hierarchical data
    #[tool(
        name = "render_treemap",
        description = r#"show a treemap visualization for hierarchical data with proportional area representation as boxes

The data should be a hierarchical structure with:
- name: Name of the node (required)
- value: Numeric value for leaf nodes (optional for parent nodes)
- children: Array of child nodes (optional)
- category: Category for coloring (optional)

Example:
{
  "name": "Root",
  "children": [
    {
      "name": "Group A",
      "children": [
        {"name": "Item 1", "value": 100, "category": "Type1"},
        {"name": "Item 2", "value": 200, "category": "Type2"}
      ]
    },
    {"name": "Item 3", "value": 150, "category": "Type1"}
  ]
}"#
    )]
    pub async fn render_treemap(
        &self,
        params: Parameters<RenderTreemapParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let data = validate_data_param(
            &serde_json::to_value(params.0).map_err(|e| {
                ErrorData::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid parameters: {}", e),
                    None,
                )
            })?,
            false,
        )?;

        // Convert the data to JSON string
        let data_json = serde_json::to_string(&data).map_err(|e| {
            ErrorData::new(
                ErrorCode::INVALID_PARAMS,
                format!("Invalid JSON data: {}", e),
                None,
            )
        })?;

        // Load all resources at compile time using include_str!
        const TEMPLATE: &str = include_str!("templates/treemap_template.html");
        const D3_MIN: &str = include_str!("templates/assets/d3.min.js");

        // Replace all placeholders with actual content
        let html_content = TEMPLATE
            .replace("{{D3_MIN}}", D3_MIN)
            .replace("{{TREEMAP_DATA}}", &data_json);

        // Save to /tmp/treemap.html for debugging
        let debug_path = std::path::Path::new("/tmp/treemap.html");
        if let Err(e) = std::fs::write(debug_path, &html_content) {
            tracing::warn!("Failed to write debug HTML to /tmp/treemap.html: {}", e);
        } else {
            tracing::info!("Debug HTML saved to /tmp/treemap.html");
        }

        // Use BlobResourceContents with base64 encoding to avoid JSON string escaping issues
        let html_bytes = html_content.as_bytes();
        let base64_encoded = STANDARD.encode(html_bytes);

        let resource_contents = ResourceContents::BlobResourceContents {
            uri: "ui://treemap/visualization".to_string(),
            mime_type: Some("text/html".to_string()),
            blob: base64_encoded,
            meta: None,
        };

        Ok(CallToolResult::success(vec![Content::resource(
            resource_contents,
        )
        .with_audience(vec![Role::User])]))
    }

    /// Show a chord diagram visualization for relationships and flows
    #[tool(
        name = "render_chord",
        description = r#"Show a chord diagram visualization for showing relationships and flows between entities.

The data must contain:
- labels: Array of strings representing the entities
- matrix: 2D array of numbers representing flows (matrix[i][j] = flow from i to j)

Example:
{
  "labels": ["North America", "Europe", "Asia", "Africa"],
  "matrix": [
    [0, 15, 25, 8],
    [18, 0, 20, 12],
    [22, 18, 0, 15],
    [5, 10, 18, 0]
  ]
}"#
    )]
    pub async fn render_chord(
        &self,
        params: Parameters<RenderChordParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let data = validate_data_param(
            &serde_json::to_value(params.0).map_err(|e| {
                ErrorData::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid parameters: {}", e),
                    None,
                )
            })?,
            false,
        )?;

        // Convert the data to JSON string
        let data_json = serde_json::to_string(&data).map_err(|e| {
            ErrorData::new(
                ErrorCode::INVALID_PARAMS,
                format!("Invalid JSON data: {}", e),
                None,
            )
        })?;

        // Load all resources at compile time using include_str!
        const TEMPLATE: &str = include_str!("templates/chord_template.html");
        const D3_MIN: &str = include_str!("templates/assets/d3.min.js");

        // Replace all placeholders with actual content
        let html_content = TEMPLATE
            .replace("{{D3_MIN}}", D3_MIN)
            .replace("{{CHORD_DATA}}", &data_json);

        // Save to /tmp/chord.html for debugging
        let debug_path = std::path::Path::new("/tmp/chord.html");
        if let Err(e) = std::fs::write(debug_path, &html_content) {
            tracing::warn!("Failed to write debug HTML to /tmp/chord.html: {}", e);
        } else {
            tracing::info!("Debug HTML saved to /tmp/chord.html");
        }

        // Use BlobResourceContents with base64 encoding to avoid JSON string escaping issues
        let html_bytes = html_content.as_bytes();
        let base64_encoded = STANDARD.encode(html_bytes);

        let resource_contents = ResourceContents::BlobResourceContents {
            uri: "ui://chord/diagram".to_string(),
            mime_type: Some("text/html".to_string()),
            blob: base64_encoded,
            meta: None,
        };

        Ok(CallToolResult::success(vec![Content::resource(
            resource_contents,
        )
        .with_audience(vec![Role::User])]))
    }

    /// show an interactive map visualization with location markers
    #[tool(
        name = "render_map",
        description = r#"show an interactive map visualization with location markers using Leaflet.

The data must contain:
- markers: Array of objects with 'lat', 'lng', and optional properties
- title: Optional title for the map (default: "Interactive Map")
- subtitle: Optional subtitle (default: "Geographic data visualization")
- center: Optional center point {lat, lng} (default: USA center)
- zoom: Optional initial zoom level (default: 4)
- clustering: Optional boolean to enable/disable clustering (default: true)
- autoFit: Optional boolean to auto-fit map to markers (default: true)

Marker properties:
- lat: Latitude (required)
- lng: Longitude (required)
- name: Location name
- value: Numeric value for sizing/coloring
- description: Description text
- popup: Custom popup HTML
- color: Custom marker color
- label: Custom marker label
- useDefaultIcon: Use default Leaflet icon

Example:
{
  "title": "Store Locations",
  "markers": [
    {"lat": 37.7749, "lng": -122.4194, "name": "SF Store", "value": 150000},
    {"lat": 40.7128, "lng": -74.0060, "name": "NYC Store", "value": 200000}
  ]
}"#
    )]
    pub async fn render_map(
        &self,
        params: Parameters<RenderMapParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let data = validate_data_param(
            &serde_json::to_value(params.0).map_err(|e| {
                ErrorData::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid parameters: {}", e),
                    None,
                )
            })?,
            false,
        )?;

        // Extract title and subtitle from data if provided
        let title = data
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Interactive Map");
        let subtitle = data
            .get("subtitle")
            .and_then(|v| v.as_str())
            .unwrap_or("Geographic data visualization");

        // Convert the data to JSON string
        let data_json = serde_json::to_string(&data).map_err(|e| {
            ErrorData::new(
                ErrorCode::INVALID_PARAMS,
                format!("Invalid JSON data: {}", e),
                None,
            )
        })?;

        // Load all resources at compile time using include_str!
        const TEMPLATE: &str = include_str!("templates/map_template.html");
        const LEAFLET_JS: &str = include_str!("templates/assets/leaflet.min.js");
        const LEAFLET_CSS: &str = include_str!("templates/assets/leaflet.min.css");
        const MARKERCLUSTER_JS: &str =
            include_str!("templates/assets/leaflet.markercluster.min.js");

        // Replace all placeholders with actual content
        let html_content = TEMPLATE
            .replace("{{LEAFLET_JS}}", LEAFLET_JS)
            .replace("{{LEAFLET_CSS}}", LEAFLET_CSS)
            .replace("{{MARKERCLUSTER_JS}}", MARKERCLUSTER_JS)
            .replace("{{MAP_DATA}}", &data_json)
            .replace("{{TITLE}}", title)
            .replace("{{SUBTITLE}}", subtitle);

        // Save to /tmp/map.html for debugging
        let debug_path = std::path::Path::new("/tmp/map.html");
        if let Err(e) = std::fs::write(debug_path, &html_content) {
            tracing::warn!("Failed to write debug HTML to /tmp/map.html: {}", e);
        } else {
            tracing::info!("Debug HTML saved to /tmp/map.html");
        }

        // Use BlobResourceContents with base64 encoding to avoid JSON string escaping issues
        let html_bytes = html_content.as_bytes();
        let base64_encoded = STANDARD.encode(html_bytes);

        let resource_contents = ResourceContents::BlobResourceContents {
            uri: "ui://map/visualization".to_string(),
            mime_type: Some("text/html".to_string()),
            blob: base64_encoded,
            meta: None,
        };

        Ok(CallToolResult::success(vec![Content::resource(
            resource_contents,
        )
        .with_audience(vec![Role::User])]))
    }

    /// show interactive line, scatter, or bar charts
    #[tool(
        name = "show_chart",
        description = r#"show interactive line, scatter, or bar charts

Required: type ('line', 'scatter', or 'bar'), datasets array
Optional: labels, title, subtitle, xAxisLabel, yAxisLabel, options

Example:
{
  "type": "line",
  "title": "Monthly Sales",
  "labels": ["Jan", "Feb", "Mar"],
  "datasets": [
    {"label": "Product A", "data": [65, 59, 80]}
  ]
}"#
    )]
    pub async fn show_chart(
        &self,
        params: Parameters<ShowChartParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let data = validate_data_param(
            &serde_json::to_value(params.0).map_err(|e| {
                ErrorData::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid parameters: {}", e),
                    None,
                )
            })?,
            false,
        )?;

        // Convert the data to JSON string
        let data_json = serde_json::to_string(&data).map_err(|e| {
            ErrorData::new(
                ErrorCode::INVALID_PARAMS,
                format!("Invalid JSON data: {}", e),
                None,
            )
        })?;

        // Load all resources at compile time using include_str!
        const TEMPLATE: &str = include_str!("templates/chart_template.html");
        const CHART_MIN: &str = include_str!("templates/assets/chart.min.js");

        // Replace all placeholders with actual content
        let html_content = TEMPLATE
            .replace("{{CHART_MIN}}", CHART_MIN)
            .replace("{{CHART_DATA}}", &data_json);

        // Save to /tmp/chart.html for debugging
        let debug_path = std::path::Path::new("/tmp/chart.html");
        if let Err(e) = std::fs::write(debug_path, &html_content) {
            tracing::warn!("Failed to write debug HTML to /tmp/chart.html: {}", e);
        } else {
            tracing::info!("Debug HTML saved to /tmp/chart.html");
        }

        // Use BlobResourceContents with base64 encoding to avoid JSON string escaping issues
        let html_bytes = html_content.as_bytes();
        let base64_encoded = STANDARD.encode(html_bytes);

        let resource_contents = ResourceContents::BlobResourceContents {
            uri: "ui://chart/interactive".to_string(),
            mime_type: Some("text/html".to_string()),
            blob: base64_encoded,
            meta: None,
        };

        Ok(CallToolResult::success(vec![Content::resource(
            resource_contents,
        )
        .with_audience(vec![Role::User])]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::handler::server::wrapper::Parameters;
    use rmcp::model::RawContent;
    use serde_json::json;

    #[test]
    fn test_validate_data_param_rejects_string() {
        // Test that a string value for data is rejected
        let params = json!({
            "data": "{\"labels\": [\"A\", \"B\"], \"matrix\": [[0, 1], [1, 0]]}"
        });

        let result = validate_data_param(&params, false);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
        assert!(err
            .message
            .contains("must be a JSON object, not a JSON string"));
        assert!(err.message.contains("without comments"));
    }

    #[test]
    fn test_validate_data_param_accepts_object() {
        // Test that a proper object is accepted
        let params = json!({
            "data": {
                "labels": ["A", "B"],
                "matrix": [[0, 1], [1, 0]]
            }
        });

        let result = validate_data_param(&params, false);
        assert!(result.is_ok());

        let data = result.unwrap();
        assert!(data.is_object());
        assert_eq!(data["labels"][0], "A");
    }

    #[test]
    fn test_validate_data_param_rejects_array_when_not_allowed() {
        // Test that an array is rejected when allow_array is false
        let params = json!({
            "data": [
                {"label": "A", "value": 10},
                {"label": "B", "value": 20}
            ]
        });

        let result = validate_data_param(&params, false);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
        assert!(err.message.contains("must be a JSON object"));
    }

    #[test]
    fn test_validate_data_param_accepts_array_when_allowed() {
        // Test that an array is accepted when allow_array is true
        let params = json!({
            "data": [
                {"label": "A", "value": 10},
                {"label": "B", "value": 20}
            ]
        });

        let result = validate_data_param(&params, true);
        assert!(result.is_ok());

        let data = result.unwrap();
        assert!(data.is_array());
        assert_eq!(data[0]["label"], "A");
    }

    #[test]
    fn test_validate_data_param_missing_data() {
        // Test that missing data parameter is rejected
        let params = json!({
            "other": "value"
        });

        let result = validate_data_param(&params, false);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
        assert!(err.message.contains("Missing 'data' parameter"));
    }

    #[test]
    fn test_validate_data_param_rejects_primitive_values() {
        // Test that primitive values (number, boolean) are rejected
        let params_number = json!({
            "data": 42
        });

        let result = validate_data_param(&params_number, false);
        assert!(result.is_err());

        let params_bool = json!({
            "data": true
        });

        let result = validate_data_param(&params_bool, false);
        assert!(result.is_err());

        let params_null = json!({
            "data": null
        });

        let result = validate_data_param(&params_null, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_data_param_with_json_containing_comments_as_string() {
        // Test that JSON with comments passed as a string is rejected
        let params = json!({
            "data": r#"{
                "labels": ["A", "B"],
                "matrix": [
                    [0, 1],  // This is a comment
                    [1, 0]   /* Another comment */
                ]
            }"#
        });

        let result = validate_data_param(&params, false);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
        assert!(err.message.contains("not a JSON string"));
        assert!(err.message.contains("without comments"));
    }

    #[tokio::test]
    async fn test_render_sankey() {
        let router = AutoVisualiserRouter::new();
        let params = Parameters(RenderSankeyParams {
            data: SankeyData {
                nodes: vec![
                    SankeyNode {
                        name: "A".to_string(),
                        category: None,
                    },
                    SankeyNode {
                        name: "B".to_string(),
                        category: None,
                    },
                ],
                links: vec![SankeyLink {
                    source: "A".to_string(),
                    target: "B".to_string(),
                    value: 10.0,
                }],
            },
        });

        let result = router.render_sankey(params).await;
        assert!(result.is_ok());
        let tool_result = result.unwrap();
        assert_eq!(tool_result.content.len(), 1);

        // Check the audience is set to User
        assert!(tool_result.content[0].audience().is_some());
        assert_eq!(
            tool_result.content[0].audience().unwrap(),
            &vec![Role::User]
        );

        // Check it's a resource with HTML content
        // Content is Annotated<RawContent>, access underlying RawContent via *
        if let RawContent::Resource(resource) = &*tool_result.content[0] {
            if let ResourceContents::BlobResourceContents { uri, mime_type, .. } =
                &resource.resource
            {
                assert_eq!(uri, "ui://sankey/diagram");
                assert_eq!(mime_type.as_ref().unwrap(), "text/html");
            } else {
                panic!("Expected BlobResourceContents");
            }
        } else {
            panic!("Expected Resource content");
        }
    }

    #[tokio::test]
    async fn test_render_radar() {
        let router = AutoVisualiserRouter::new();
        let params = Parameters(RenderRadarParams {
            data: RadarData {
                labels: vec![
                    "Speed".to_string(),
                    "Power".to_string(),
                    "Agility".to_string(),
                ],
                datasets: vec![RadarDataset {
                    label: "Player 1".to_string(),
                    data: vec![80.0, 90.0, 85.0],
                }],
            },
        });

        let result = router.render_radar(params).await;
        assert!(result.is_ok());
        let tool_result = result.unwrap();
        assert_eq!(tool_result.content.len(), 1);

        // Check the audience is set to User
        assert!(tool_result.content[0].audience().is_some());
        assert_eq!(
            tool_result.content[0].audience().unwrap(),
            &vec![Role::User]
        );

        // Check it's a resource with HTML content
        // Content is Annotated<RawContent>, access underlying RawContent via *
        if let RawContent::Resource(resource) = &*tool_result.content[0] {
            if let ResourceContents::BlobResourceContents {
                uri,
                mime_type,
                blob,
                ..
            } = &resource.resource
            {
                assert_eq!(uri, "ui://radar/chart");
                assert_eq!(mime_type.as_ref().unwrap(), "text/html");
                assert!(!blob.is_empty(), "HTML content should not be empty");
            } else {
                panic!("Expected BlobResourceContents");
            }
        } else {
            panic!("Expected Resource content");
        }
    }

    #[tokio::test]
    async fn test_render_donut() {
        let router = AutoVisualiserRouter::new();
        let params = Parameters(RenderDonutParams {
            data: DonutData {
                data: DonutChartData::Single(SingleDonutChart {
                    data: vec![
                        DonutDataItem::Number(30.0),
                        DonutDataItem::Number(40.0),
                        DonutDataItem::Number(30.0),
                    ],
                    labels: Some(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
                    title: None,
                    chart_type: None,
                }),
            },
        });

        let result = router.render_donut(params).await;
        assert!(result.is_ok());
        let tool_result = result.unwrap();
        assert_eq!(tool_result.content.len(), 1);

        // Check the audience is set to User
        assert!(tool_result.content[0].audience().is_some());
        assert_eq!(
            tool_result.content[0].audience().unwrap(),
            &vec![Role::User]
        );
    }

    #[tokio::test]
    async fn test_render_treemap() {
        let router = AutoVisualiserRouter::new();
        let params = Parameters(RenderTreemapParams {
            data: TreemapNode {
                name: "root".to_string(),
                value: None,
                category: None,
                children: Some(vec![
                    TreemapNode {
                        name: "A".to_string(),
                        value: Some(100.0),
                        category: Some("Type1".to_string()),
                        children: None,
                    },
                    TreemapNode {
                        name: "B".to_string(),
                        value: Some(200.0),
                        category: Some("Type2".to_string()),
                        children: None,
                    },
                ]),
            },
        });

        let result = router.render_treemap(params).await;
        assert!(result.is_ok());
        let tool_result = result.unwrap();
        assert_eq!(tool_result.content.len(), 1);

        // Check the audience is set to User
        assert!(tool_result.content[0].audience().is_some());
        assert_eq!(
            tool_result.content[0].audience().unwrap(),
            &vec![Role::User]
        );
    }

    #[tokio::test]
    async fn test_render_chord() {
        let router = AutoVisualiserRouter::new();
        let params = Parameters(RenderChordParams {
            data: ChordData {
                labels: vec!["A".to_string(), "B".to_string(), "C".to_string()],
                matrix: vec![
                    vec![0.0, 10.0, 5.0],
                    vec![10.0, 0.0, 15.0],
                    vec![5.0, 15.0, 0.0],
                ],
            },
        });

        let result = router.render_chord(params).await;
        assert!(result.is_ok());
        let tool_result = result.unwrap();
        assert_eq!(tool_result.content.len(), 1);

        // Check the audience is set to User
        assert!(tool_result.content[0].audience().is_some());
        assert_eq!(
            tool_result.content[0].audience().unwrap(),
            &vec![Role::User]
        );
    }

    #[tokio::test]
    async fn test_render_map() {
        let router = AutoVisualiserRouter::new();
        let params = Parameters(RenderMapParams {
            data: MapData {
                markers: vec![MapMarker {
                    lat: 0.0,
                    lng: 0.0,
                    name: Some("Origin".to_string()),
                    value: None,
                    description: None,
                    popup: None,
                    color: None,
                    label: None,
                    use_default_icon: None,
                }],
                title: None,
                subtitle: None,
                center: None,
                zoom: None,
                clustering: None,
                cluster_radius: None,
                auto_fit: None,
            },
        });

        let result = router.render_map(params).await;
        assert!(result.is_ok());
        let tool_result = result.unwrap();
        assert_eq!(tool_result.content.len(), 1);

        // Check the audience is set to User
        assert!(tool_result.content[0].audience().is_some());
        assert_eq!(
            tool_result.content[0].audience().unwrap(),
            &vec![Role::User]
        );
    }

    #[tokio::test]
    async fn test_show_chart() {
        let router = AutoVisualiserRouter::new();
        let params = Parameters(ShowChartParams {
            data: ChartData {
                chart_type: ChartType::Scatter,
                datasets: vec![ChartDataset {
                    label: "Test Data".to_string(),
                    data: ChartDataValues::Points(vec![
                        ChartPoint { x: 1.0, y: 2.0 },
                        ChartPoint { x: 2.0, y: 4.0 },
                    ]),
                    background_color: None,
                    border_color: None,
                    border_width: None,
                    tension: None,
                    fill: None,
                }],
                labels: None,
                title: None,
                subtitle: None,
                x_axis_label: None,
                y_axis_label: None,
            },
        });

        let result = router.show_chart(params).await;
        if let Err(e) = &result {
            eprintln!("Error in test_show_chart: {:?}", e);
        }
        assert!(result.is_ok());
        let tool_result = result.unwrap();
        assert_eq!(tool_result.content.len(), 1);

        // Check the audience is set to User
        assert!(tool_result.content[0].audience().is_some());
        assert_eq!(
            tool_result.content[0].audience().unwrap(),
            &vec![Role::User]
        );
    }
}
