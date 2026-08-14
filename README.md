# Franz BC Dev Helper

This project aims to make developing Business Central extensions easier and more accessible. The goal is to provide a local development environment that manages repositories, containers, dependencies, and other project-related resources in one place.

For non-technical users who rely on AI-assisted development, the application aims to remove much of the usual setup and tooling overhead. Instead of having to learn technologies such as Docker, Git, and related development tools, users can manage these tasks through an easy-to-use desktop application. Alternatively, the application's functionality can be exposed to a preferred AI agent through MCP, allowing the agent to handle parts of the development workflow directly.

At the same time, the project is not intended only for beginners. Experienced developers should have enough configuration options and flexibility to fine-tune the environment and adapt it to their preferred Business Central development workflow.

## Table of Contents

* [Project Status](#project-status)
* [Tech Stack](#tech-stack)
* [Roadmap](#roadmap)
* [Contributing](#contributing)
* [Getting Started](#getting-started)
* [Disclaimer](#disclaimer)

## Project Status

FranzBCDevHelper is currently in very early development. Application behavior, the technology stack, APIs, and internal structures may change significantly between versions.

At the moment, there is no executable release available.

## Tech Stack

FranzBCDevHelper is built as a lightweight desktop application with a web-based frontend and a Rust backend.

* **Tauri 2** – desktop application framework connecting the frontend with the native backend
* **Rust** – backend logic, container management, filesystem access, and other system-level functionality
* **Vue 3** – frontend framework
* **TypeScript** – frontend development
* **Vite** – frontend development and build tooling
* **Tailwind CSS** – styling and UI development
* **SQLite** – local persistence for application and project-related data
* **Bollard** – communication with the Docker API from Rust
* **Tokio & Reqwest** – asynchronous operations and HTTP communication
* **VitePress** – project documentation

The architecture is intentionally centered around a Rust backend so that core functionality can later be reused by the desktop application, CLI, and MCP server rather than being tied exclusively to the graphical interface.

## Roadmap

The initial focus is on implementing **Business Central Docker container management**. Once that foundation is in place, the next step will be **Git repository management**, followed by **project management** to bring the individual components together into a cohesive development experience.

From there, existing functionality will be improved incrementally while additional features are introduced.

The current roadmap, in roughly the planned order, is:

1. **MCP server for AI agents**
   Expose the application's functionality through MCP so AI agents can directly interact with projects, repositories, containers, dependencies, and other development resources.

2. **CLI for technical users and headless environments**
   Provide a command-line interface for developers who prefer working from the terminal and for use on servers or other environments without a graphical interface.

3. **Integrated AI-assisted workflows**
   Enable direct communication with AI agents through predefined workflows, with the goal of providing a one-stop application for AI-assisted Business Central development.

If the project gains enough traction, additional enterprise-focused features may also be pursued. These are currently not listed in any particular order:

1. **Plug-in system** for extending the application with functionality for more specialized use cases.
2. **GitHub, GitLab, and Azure DevOps integrations** for accessing issues, work items, test cases, and related development resources.
3. **Company-wide project management and access control** to simplify project setup and provide employees with ready-to-use development environments.
4. **Remote caching** for Business Central artifacts, container images, and app dependencies to reduce download times across local networks.
5. **Remote container management and advanced networking options** to allow containers to run on other machines while remaining accessible from the local development environment.
6. **Compilation and test execution** to support complete development and validation workflows without requiring Visual Studio Code.

> [!TIP]
> Have an idea, feature request, or disagree with the current priorities? Feel free to share your thoughts in the [discussions](https://github.com/Franz-Mueller/FranzBCDevHelper/discussions/categories/ideas).

## Contributing

There is still quite a lot to build, so code contributions are very welcome, especially on the frontend using Vite.

Contributions do not have to be code, though. Feedback, feature ideas, and Business Central-specific knowledge about development workflows, containers, and common pain points are just as valuable.

If you have an established Business Central development workflow, whether AI-assisted or not, please feel free to share it in the discussions. Understanding how different developers and teams work will help shape the project and ensure that the resulting features are useful in real-world scenarios.

> [!CAUTION]
> Unreviewed AI-generated contributions are not permitted.
>
> Please do not contribute to the Rust backend if you do not understand the fundamentals of the language. I am still learning Rust myself, and I would like this project to be a place where other beginners can gain practical experience as well.
>
> Using AI as a development tool is completely fine, but contributors should understand, review, and be able to explain the code they submit.

## Getting Started

> [!NOTE]
> Coming soon.

## Disclaimer

This is an independent project and is not affiliated with, endorsed by, or sponsored by Microsoft in any way.
