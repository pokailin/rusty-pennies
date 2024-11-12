# RustyPennies

### Overview

RustyPennies is a command-line budget tracker built with Rust, designed to help you manage your finances by tracking income and expenses, categorizing transactions, and providing summaries.

### Features
- **Add and Remove Transactions**: Record income and expenses by category.
- **Transaction Listing**: View all transactions or filter by category.
- **Summaries**: Get a breakdown of total income, expenses, and overall balance.
- **Persistence**: All data is saved locally and reloaded each time you run the CLI.

### Usage

#### *Basic Commands*

- **Add a Transaction**:

```bash
rustypennies add --type expense --amount 50 --category food --description "Lunch at restaurant"
```

- **Remove a Transaction**:

```bash
rustypennies remove --id 1
```

#### *Advanced Commands*

- **Summary**

```bash
rustypennies summary
```

- **Filter by Category**

```bash
rustypennies by-category food
```

#### *Help*

Use the --help flag to see available commands:

```bash
rustypennies --help
```
