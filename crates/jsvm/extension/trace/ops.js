globalThis.tracing = {
    "trace": (args) => {
        return Deno.core.ops.op_tracing_trace(args);
    },
    "debug": (args) => {
        return Deno.core.ops.op_tracing_debug(args);
    },
    "info": (args) => {
        return Deno.core.ops.op_tracing_info(args);
    },
    "warn": (args) => {
        return Deno.core.ops.op_tracing_warn(args);
    },
    "error": (args) => {
        return Deno.core.ops.op_tracing_error(args);
    },
};
