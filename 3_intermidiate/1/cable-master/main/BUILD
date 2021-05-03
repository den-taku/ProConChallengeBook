cc_library (
    name = "cable-master",
    srcs = ["cable-master.cpp"],
    hdrs = ["cable-master.h"],
)

cc_binary (
    name = "main",
    srcs = ["main.cpp"],
    deps = [
        ":cable-master",
        "//lib:proconlib",
    ],
)
