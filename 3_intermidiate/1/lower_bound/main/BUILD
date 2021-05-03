cc_library (
    name = "lower_bound",
    srcs = ["lower_bound.cpp"],
    hdrs = ["lower_bound.h"],
)

cc_binary (
    name = "main",
    srcs = ["main.cpp"],
    deps = [
        ":lower_bound",
        "//lib:proconlib",
    ],
)
