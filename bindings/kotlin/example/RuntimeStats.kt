import kotlin.system.measureNanoTime
import kotlinx.coroutines.runBlocking
import uniffi.goose_llm.*

/* --- quick helper --- */
fun buildProviderConfig(host: String, token: String): String =
    """{ "host": "$host", "token": "$token" }"""

suspend fun timeOneCall(
    modelCfg: ModelConfig,
    providerName: String,
    providerCfg: String
): Pair<Double, CompletionResponse> {

    val req = createCompletionRequest(
        providerName,
        providerCfg,
        modelCfg,
        systemPreamble = "You are a helpful assistant.",
        messages = listOf(
            Message(
                Role.USER,
                System.currentTimeMillis() / 1000,
                listOf(MessageContent.Text(TextContent("Tell me a joke")))
            )
        ),
        extensions = emptyList()
    )

    lateinit var resp: CompletionResponse
    val wallNs = measureNanoTime { resp = completion(req) }
    return wallNs / 1_000_000.0 to resp   // → wall-clock in ms
}

/* --- entry point --- */
fun main() = runBlocking {
    /* provider setup */
    val providerName  = "databricks"
    val host  = System.getenv("DATABRICKS_HOST") ?: error("DATABRICKS_HOST not set")
    val token = System.getenv("DATABRICKS_TOKEN") ?: error("DATABRICKS_TOKEN not set")
    val providerCfg   = buildProviderConfig(host, token)

    val modelNames = listOf("goose-claude-4-sonnet", "goose-gpt-4-1")
    val runsPerModel = 3          // tweak as needed

    for (model in modelNames) {
        val cfg = ModelConfig(model, 100_000u, 0.1f, 200)
        var wallSum = 0.0
        var gooseSum = 0.0

        println("=== $model ===")
        repeat(runsPerModel) { i ->
            val (wallMs, resp) = timeOneCall(cfg, providerName, providerCfg)
            val gooseMs = resp.runtimeMetrics.totalTimeSec * 1_000
            val overhead = wallMs - gooseMs

            wallSum += wallMs
            gooseSum += gooseMs

            println("run ${i + 1}: wall = %.1f ms | goose-llm = %.1f ms | overhead = %.1f ms"
                .format(wallMs, gooseMs, overhead))
        }
        println(
            "-- averages: wall = %.1f ms | goose-llm = %.1f ms | overhead = %.1f ms --\n"
                .format(wallSum / runsPerModel, gooseSum / runsPerModel, (wallSum - gooseSum) / runsPerModel)
        )
    }
}
