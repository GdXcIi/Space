# Space Architecture

## Introduction

### Language

The language used to code Space is Rust.
I chose this language for its memory stability (critical for a microkernel).

### Conventions

#### Structure of a folder

Each folder must have :
- a `mod.rs` file that defines **abstractions** and make the **folder importable**
- an `orchestrator.rs` file that **implements abstractions** defined in `mod.rs` and **handle** all the folder *(like an entry point)*

## 1. Kernel Core

#### 1.1.1 `core/arch/`

This folder contains every subsystems linked to specifics architectures.
This contains:
- IRQ Handler

##### 1.1.1.1 `core/arch/irq_handler/`

IRQ Handler is the subsystem that generalizes irq management in Space.
This is in `arch/` because this is an entry point of irq management for each cpu architecture that are in this folder too.

##### 1.1.1.2 `core/arch/cpu_arch/x86/`

> *TODO*

#### 1.1.2 `core/modloader/`

Module Loader loads modules and sets:
- the runtime
- the sandbox

So this contains:
- `runtime/`
- `sandbox/`

Folder `loader` contains the main code of the loader.

### 1.3 `core/scheds/`

In Space there are two schedulers:
- **CPU Scheduler (`cpu_sched`)**: Decides who uses cpu and when (to simplify)
- **IPC Scheduler (`ipc_sched`)**: Manages IPC queue so that the kernel can treat IPCs in the good order.

Both are linked but they don't have the same utility so they are separated.

##### 1.1.3.1 `core/scheds/cpu_sched/`

> *TODO*

##### 1.1.3.2 `core/scheds/ipc_sched/`

> *TODO*

#### 1.1.4 `core/security/`

This folder is responsable of IPC and Tickets low level management (abstractions, low level logic, etc...)

##### 1.1.4.1 `core/security/ipc/`

> *TODO*

##### 1.1.4.2 `core/security/tickets`

> *TODO*

#### 1.1.5 `core/syscalls/`

> *TODO*

### 1.2 `mm/`

> *TODO*
