# AI SDK

This is a port of [Vercel's AI SDK Core](https://sdk.vercel.ai/docs/ai-sdk-core/overview) to Rust.

Main requirements are:
1. This should be compilable to WebAssembly.
2. The WebAssembly should be runnable in the browser and also on any target platform.
3. The SDK should support many providers.


The main methods are:
| Method | Description |
|--------|-------------|
| generateText | Generates text and tool calls. This function is ideal for non-interactive use cases such as automation tasks where you need to write text (e.g. drafting email or summarizing web pages) and for agents that use tools. |
| streamText | Stream text and tool calls. You can use the streamText function for interactive use cases such as chat bots and content streaming. |
| generateObject | Generates a typed, structured object that matches a Zod schema. You can use this function to force the language model to return structured data, e.g. for information extraction, synthetic data generation, or classification tasks. |
| streamObject | Stream a structured object that matches a Zod schema. You can use this function to stream generated UIs. |
| embedMany | Embed several values using an embedding model. The type of the value is defined by the embedding model. embedMany automatically splits large requests into smaller chunks if the model has a limit on how many embeddings can be generated in a single call. |
| embed | Generate an embedding for a single value using an embedding model. This is ideal for use cases where you need to embed a single value to e.g. retrieve similar items or to use the embedding in a downstream task. |


There are helper methods:
| Method | Description |
|--------|-------------|
| createIdGenerator | Creates a customizable ID generator function. You can configure the alphabet, prefix, separator, and default size of the generated IDs. Example usage: `let generate_id = create_id_generator(IdGeneratorOptions { prefix: "user".to_string(), separator: "_".to_string(), ..Default::default() }); let id = generate_id();` |


# generateText and streamText
## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| model | `LanguageModel` | The language model to use. Example: `openai('gpt-4-turbo')` |
| system | `string` | The system prompt to use that specifies the behavior of the model |
| prompt | `string` | The input prompt to generate the text from |
| messages | `Array<CoreSystemMessage \| CoreUserMessage \| CoreAssistantMessage \| CoreToolMessage> \| Array<UIMessage>` | A list of messages that represent a conversation. Automatically converts UI messages from the useChat hook |