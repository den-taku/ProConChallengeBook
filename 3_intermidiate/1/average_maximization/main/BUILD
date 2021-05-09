cc_library (
    name = "average_maximization",
    srcs = ["average_maximization.cpp"],
    hdrs = ["average_maximization.h"],
)

cc_binary (
    name = "main",
    srcs = ["main.cpp"],
    deps = [
        ":average_maximization",
        "//lib:proconlib",
    ],
)
