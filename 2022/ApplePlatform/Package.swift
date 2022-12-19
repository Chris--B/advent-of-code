// swift-tools-version:5.1
import PackageDescription

let package = Package(
    name: "ApplePlatform",
    platforms: [
        .macOS(.v10_15),
    ],
    products: [
        .library(name: "ApplePlatform", type: .static, targets: ["ApplePlatform"]),
    ],
    targets: [
        .target(name: "ApplePlatform", path: "."),
    ]
)
