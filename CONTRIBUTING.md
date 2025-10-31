# Contributing to Amentys

First off, thank you for considering contributing to Amentys\! We welcome any help, whether it's reporting bugs, submitting features, or improving documentation.

To ensure a smooth collaboration, please review this guide.

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](https://www.google.com/search?q=CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior.

## 1. Getting Started

### Prerequisites

  * **Rust:** You must have the latest stable version of Rust installed. We recommend installing it via `rustup`.
  * **Git:** You'll need Git to fork and clone the repository.
  * **Breath** the tools to commit.

![demo](https://github.com/hackia/amentys/blob/main/commit.gif)

### Project Setup

1.  **Fork** the repository on GitHub.
2.  **Clone** your fork to your local machine:

```bash
git clone https://github.com/YOUR-USERNAME/amentys.git
cd amentys
```

4.  **Build** the project. Amentys is a Cargo workspace, so you can build all crates at once:

```bash
cargo build
```

## 2. Our Development Standards

To maintain code quality and consistency, we enforce a few key standards. **Contributions that do not meet these standards will be asked for corrections before merging.**

### Code Style (rustfmt)

All code must be formatted with the standard Rust formatter (`rustfmt`). Before committing, please run:

```bash
cargo fmt --all
```

### Linting (Clippy Pedantic)

We enforce a very strict set of lints using `clippy::pedantic`. Your code **must** compile cleanly with no warnings under this ruleset.

To check your code, run:

```bash
breath health
```

### Commit Messages (Conventional Commits)

We strictly follow the **Conventional Commits** specification. This is not optional, as it helps us automate changelogs and versioning.

Your commit messages **must** follow this format:

```text
<type>(<scopes>) ~ <summary>

    Why changes?
    
        * <details> 
    
    Breaking Changes:
    
        * <breaking changes>
    
    What changes?
    
        * <what details>
    
    Who changes?
    
        <author> ~ <roles>
        
    Benefits:
    
        * <benefits>
    
    Notes:
    
        * <team message>
    
    Resolves
        
        Fixes #<issues>
```

**Allowed types:**

  * **`feat`**: A new feature (e.g., `feat(plan): Add LayerData enum`)
  * **`fix`**: A bug fix (e.g., `fix(capsule): Fix resource calculation`)
  * **`docs`**: Documentation only changes
  * **`style`**: Changes that do not affect the meaning of the code (formatting, etc.)
  * **`refactor`**: A code change that neither fixes a bug nor adds a feature
  * **`perf`**: A code change that improves performance
  * **`test`**: Adding missing tests or correcting existing tests
  * **`build`**: Changes that affect the build system or external dependencies
  * **`ci`**: Changes to our CI configuration files
  * **`chore`**: Other changes that don't modify `src` or `test` files

## 3. Contribution Workflow

1.  **Create a Branch:** Always work on a new branch, never directly on `main`.

```bash
# Get the latest changes
git checkout main
git pull origin main
```

# Create your new branch

```bash
git checkout -b feat/my-awesome-feature
```

2.  **Write Your Code:** Make your changes.

      * Add new tests for any new logic.
      * Update existing tests if you change logic.

3.  **Validate Your Changes:** Before submitting, run all checks:

```bash
breath health
```

4.  **Commit Your Changes:** Use the Conventional Commit format described above.

```bash
git add .
breath commit
```

5.  **Push to Your Fork:**

```bash
git push -u origin feat/my-awesome-feature
```

## 4. Submitting Your Pull Request

  * Go to the `amentys` repository on GitHub and open a new **Pull Request (PR)** from your branch.
  * Write a clear title and description for your PR. If it resolves a specific issue, link it (e.g., `Fixes #123`).
  * A maintainer will review your PR. We may ask for changes. Once approved, your code will be merged.

Thank you for your contribution!
