cc_library (
    name = "cable_master",
    srcs = ["cable_master.cpp"],
    hdrs = ["cable_master.h"],
)

cc_binary (
    name = "main",
    srcs = ["main.cpp"],
    deps = [
        ":cable_master",
        "//lib:proconlib",
    ],
)
