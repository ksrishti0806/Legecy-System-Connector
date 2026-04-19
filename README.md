# Stellar Legacy Connector

A secure and resilient bridge application that allows older enterprise banking systems to interface seamlessly with the Stellar network. 

## Overview

The **Stellar Legacy Connector** acts as a middleware API that translates legacy bank operations into Stellar network transactions. By providing idempotency controls and reliable retries, it ensures safe processing of high-value transactional data without the risk of double-spending or dropped network connections.

## Features

- **Stellar Network Integration:** Built using the official `java-stellar-sdk` to quickly authorize, submit, and verify transactions.
- **Idempotent Operations:** Leverages Spring Data JPA (compatible with PostgreSQL and H2 databases) to track execution states and prevent duplicate requests.
- **Network Resilience:** Incorporates `spring-retry` with AOP support to automatically recover from transient network failures.
- **Soroban Smart Contracts:** Includes a separate directory (`smart-contract/`) for compiling and deploying WebAssembly (Wasm) Rust smart contracts to Stellar.

## Technology Stack

- **Java 17**
- **Spring Boot 3.2.x** (Web, Validation, Data JPA, AOP)
- **Maven**
- **PostgreSQL / H2 Database**
- **Stellar Java SDK**
- **Rust / Soroban SDK** (for the Smart Contract module)

## Prerequisites

To build and run this application, you need:

- [Java 17 SDK](https://adoptium.net/) or higher
- [Maven 3.6+](https://maven.apache.org/)
- A running instance of PostgreSQL (or rely on the in-memory H2 database for local testing)
- Optional: [Rust toolchain](https://rustup.rs/) and Stellar CLI (for smart contract development)

## Getting Started

### 1. Build the Application

Clone the repository and build the Spring Boot application using Maven:

```bash
mvn clean install
```

### 2. Run the Application

You can start the server locally by running:

```bash
mvn spring-boot:run
```

By default, the REST API will be accessible at `http://localhost:8080`.

## Smart Contract Development

This repository also contains a `smart-contract` directory which houses Rust-based Soroban smart contracts intended to be deployed on the Stellar network. 

For instructions on building, testing, and deploying these contracts, please refer to the [Smart Contract README](smart-contract/README.md).

## Testing

To run the Java test suite, execute:

```bash
mvn clean test
```
