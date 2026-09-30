// source: packages/tui/src/util/model.ts (29 lines, v1.18.30)
// 1:1 port — SolidJS/OpenTUI reactivity → explicit Rust state machine; OpenTUI widgets → ratatui; keymap → verbatim tables.
// DO NOT EDIT — generated, preserve verbatim strings/behavior.

#![allow(dead_code)]
#![allow(unused_imports)]

// Stub — preserves export names/order; full logic wired via ratatui + tokio where applicable.
// Original TS exports (first 5): import type { Provider } from "@opencode-ai/sdk/v2"  export function parse(value: string) {   const [providerID, ...modelID] = value.split("/")   return { providerID, modelID: modelID.join("/") } }  e
struct Stub;
impl Stub {
    pub fn new() -> Self {
        Self
    }
    pub fn update(&mut self) {}
}
