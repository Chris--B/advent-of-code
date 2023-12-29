use swift_rs::SwiftLinker;

fn main() {
    SwiftLinker::new("10.15") // Match Package.swift
        .with_package("ApplePlatform", "./ApplePlatform/")
        .link();
}
