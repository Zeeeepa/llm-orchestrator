# LLM Orchestrator

Production-ready LLM workflow orchestrator with DAG execution, state management, and multi-provider support.

## Installation

```bash
npm install -g @llm-dev-ops/llm-orchestrator
```

Or add to your project:

```bash
npm install @llm-dev-ops/llm-orchestrator
```

## Usage

### CLI

```bash
# Validate a workflow
llm-orchestrator validate workflow.yaml

# Run a workflow
llm-orchestrator run workflow.yaml

# Run with input
llm-orchestrator run workflow.yaml --input '{"query": "What is AI?"}'

# Run with custom concurrency
llm-orchestrator run workflow.yaml --max-concurrency 5
```

### Programmatic API

```javascript
const orchestrator = require('@llm-dev-ops/llm-orchestrator');

// Validate a workflow
await orchestrator.validate('workflow.yaml');

// Run a workflow
const result = await orchestrator.run('workflow.yaml', {
  input: JSON.stringify({ query: 'What is AI?' }),
  maxConcurrency: 5
});

console.log(result.stdout);
```

## Features

- **DAG-based Workflow Execution**: Define complex workflows with dependencies
- **Multi-Provider Support**: OpenAI, Anthropic, and more
- **State Management**: Persistent state across workflow runs
- **Template Engine**: Handlebars-based templating for dynamic prompts
- **Observability**: Built-in metrics and tracing
- **Error Handling**: Retry policies and error recovery
- **Type Safety**: Full Rust implementation for reliability

## Supported Platforms

- Linux x64
- Linux ARM64
- macOS x64 (Intel)
- macOS ARM64 (Apple Silicon)
- Windows x64

## Documentation

For full documentation, visit: https://github.com/globalbusinessadvisors/llm-orchestrator

## License

MIT OR Apache-2.0
