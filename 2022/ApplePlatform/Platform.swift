import os

@available(macOS 12, *)
public class Interval {
    let signposter: OSSignposter
    let name: StaticString
    var state: OSSignpostIntervalState?

    init(_ name: StaticString) {
        self.name = name
        signposter = OSSignposter()
    }

    public func begin() {
        state = signposter.beginInterval(name, id: .exclusive)
    }

    public func end() {
        signposter.endInterval(name, state!)
    }
}
