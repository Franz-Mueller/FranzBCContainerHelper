# 🦀 FranzBCContainerHelper

FranzBCContainerHelper is a personal Rust learning project inspired by Microsoft's BC Container Helper.

The main goal of this repository is not to build an immediate production-ready replacement. It exists primarily so I can practice Rust while working on a real piece of software with a practical use case.

Microsoft's BC Container Helper does its job, but it provides much more functionality than I need for my usual workflows. This project explores what a smaller and more focused tool for managing Business Central development containers could look like.

## Project goals

The priorities of this project are:

1. Practicing Rust through a real-world project
2. Understanding the language and standard library instead of hiding everything behind dependencies
3. Building a small and understandable tool for my own Business Central development workflows
4. Creating a project that documents my progress and can serve as part of my portfolio

Producing valuable software is still a goal, but it is currently a secondary one. Learning and experimentation take priority over feature completeness and production readiness.

## Development approach

Where reasonable, the project avoids external crates and relies on Rust's standard library. This is an intentional learning constraint, not a general recommendation for production Rust projects.

Additional crates may still be introduced when implementing the same functionality from scratch would distract from the actual purpose of the project. For example, writing an entire JSON parser would not be a useful use of time for this project.

The architecture and code will evolve as my understanding of Rust improves. Earlier implementations may therefore not follow every established Rust best practice.

## Current status

This project is in an early and experimental stage.

It should currently be treated as:

* a learning project
* a personal development tool
* a portfolio project
* an exploration of a smaller BC container workflow

It should not currently be treated as:

* a production-ready replacement for BC Container Helper
* a stable tool with backward-compatibility guarantees
* a complete solution for every Business Central container workflow

Use it at your own risk.

## Possible future direction

The project may eventually provide tools for tasks such as:

* creating and removing Docker containers
* preparing Business Central container images
* building and managing artifacts
* checking or installing host prerequisites
* exposing functionality through an MCP server so AI agents can use the available tools

These ideas are not commitments. The direction of the project will depend on what is useful to learn and what remains interesting to build.

## ✅ TODO

Issues are not being used for planning yet. The following list contains some of the next improvements I want to keep in mind:

- [x] Implement custom error types
- [ ] Improve artifact download
- [ ] Add a help flag
- [ ] Add a command for checking and installing host prerequisites, such as Docker
- [ ] Improve command output and error reporting
  - [ ] loading bars
- [ ] Add tests for reusable components
- [ ] Group Errors
- [ ] Replace `box<dyn Error>` with more specific handling
- [ ] Parse and enforce string positioning command to ensure order
- [ ] allow non positional flags
- [ ] early exits:
  - [ ] check if docker service is running

## Disclaimer

This project is not affiliated with or supported by Microsoft.

Business Central, Microsoft, and related product names are trademarks of their respective owners.
