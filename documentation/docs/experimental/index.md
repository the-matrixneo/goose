---
title: Experimental
hide_title: true
description: Experimental and radically unstable, but lot's of fun.
---

import Card from '@site/src/components/Card';
import styles from '@site/src/components/Card/styles.module.css';

<h1 className={styles.pageTitle}>Experimental</h1>
<p className={styles.pageDescription}>
  Goose is an open source project that is constantly being improved and expanded upon. These experimental features and projects are still in development and may not be fully stable or ready for production use, but they showcase exciting possibilities for the future of AI automation.
</p>

:::note
The list of experimental features may change as Goose development progresses. Some features may be promoted to stable features, while others might be modified or removed. This section will be updated with specific experimental features as they become available.
:::

<div className={styles.categorySection}>
  <h2 className={styles.categoryTitle}>🧪 Experimental Features</h2>
  <div className={styles.cardGrid}>
    <Card 
      title="Ollama Tool Shim"
      description="Enable tool calling capabilities for language models that don't natively support tool calling (like DeepSeek) using an experimental local interpreter model setup."
      link="/docs/experimental/ollama"
    />
    <Card 
      title="Goose Mobile"
      description="An experimental Android automation app that acts as an open agent running on your phone, providing maximal automation of everyday tasks."
      link="/docs/experimental/goose-mobile"
    />
    <Card 
      title="VS Code Extension"
      description="An experimental extension enabling Goose to work within VS Code."
      link="/docs/experimental/vs-code-extension"
    />
    <Card 
      title="Automatic Multi-Model Switching"
      description="Intelligent, context-aware switching between models based on conversation content, complexity, and tool usage patterns."
      link="/docs/guides/multi-model/autopilot"
    />
    <Card 
      title="Using goose in ACP Clients"
      description="Interact with goose natively in ACP-compatible clients like Zed."
      link="/docs/guides/acp-clients"
    />
  </div>
</div>

<div className={styles.categorySection}>
  <h2 className={styles.categoryTitle}>📝 Featured Blog Posts</h2>
  <div className={styles.cardGrid}>
    <Card 
      title="Finetuning Toolshim Models for Tool Calling"
      description="Addressing performance limitations in models without native tool calling support through dedicated toolshim model development."
      link="/blog/2025/04/11/finetuning-toolshim"
    />
    <Card 
      title="AI, But Make It Local With Goose and Ollama"
      description="Learn how to integrate Goose with Ollama for a fully local AI experience, including structured outputs and tool calling capabilities."
      link="/blog/2025/03/14/goose-ollama"
    />
    <Card 
      title="Community-Inspired Benchmarking: The Goose Vibe Check"
      description="See how open source AI models measure up in our first Goose agent benchmark tests, including toolshim performance analysis."
      link="/blog/2025/03/31/goose-benchmark"
    />
  </div>
</div>

<div className={styles.categorySection}>
  <h2 className={styles.categoryTitle}>💬 Feedback & Support</h2>
  <div className={styles.cardGrid}>
    <Card 
      title="GitHub Issues"
      description="Report bugs, request features, or contribute to the development of experimental features."
      link="https://github.com/block/goose/issues"
    />
    <Card 
      title="Discord Community"
      description="Join our community to discuss experimental features, share feedback, and connect with other users."
      link="https://discord.gg/block-opensource"
    />
  </div>
</div>