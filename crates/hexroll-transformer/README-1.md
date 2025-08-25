# hexroll_transformer (AI + Yarn-focused files)

This trimmed crate shows **real OpenAI Dive integration** and **full Yarn Spinner integration**.
Use these files as the source of truth and drop them into your main repo.

- `src/analyzer.rs` — uses **openai_dive v1.2.4** to classify pages and generate Yarn.
- `src/yarn_integration.rs` — utilities to write Yarn scripts to `assets/dialogue/*.yarn` and a Bevy example that compiles and runs those scripts with **bevy_yarnspinner v0.5.0**.

Set `OPENAI_API_KEY` in your environment to use the analyzer.

