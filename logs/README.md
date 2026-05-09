## `logs/` kernel module

### What is it?

This folder is to manage a minimalistic logging system and a minimalistic kernel error system.

We also declare what system module `logmanager` should implement to manage manage logs.

> **Note:**
>
> This is not a double implementation.
>
> We implement in the microkernel a minimalistic logging and kernel error system but it is used only in case of critical crashes or errors (example: kernel panic, etc.), or anything that should not depend on external modules.
>
> The rest is just declarations so that system module `logmanager` can implement it.

### How it works?

*TODO: Document how the logging and kerrors kernel module works.*
