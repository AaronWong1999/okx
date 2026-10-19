import Foundation
import Vision
import AppKit

// Batch OCR over image files. Prints one JSON object per image:
// {"file": "...", "text": "...", "lines": [{"text":..,"y":..,"h":..,"conf":..}]}
let args = CommandLine.arguments
guard args.count >= 2 else {
    FileHandle.standardError.write("usage: ocr <file-or-dir>...\n".data(using: .utf8)!)
    exit(1)
}

func collect(_ path: String) -> [String] {
    let fm = FileManager.default
    var isDir: ObjCBool = false
    guard fm.fileExists(atPath: path, isDirectory: &isDir) else { return [] }
    if !isDir.boolValue { return [path] }
    let items = (try? fm.contentsOfDirectory(atPath: path)) ?? []
    return items.filter { ["png", "jpg", "jpeg"].contains(($0 as NSString).pathExtension.lowercased()) }
        .sorted()
        .map { (path as NSString).appendingPathComponent($0) }
}

for arg in args.dropFirst() {
    for file in collect(arg) {
        guard let img = NSImage(contentsOfFile: file),
              let cg = img.cgImage(forProposedRect: nil, context: nil, hints: nil) else {
            print("{\"file\": \(jsonQuote(file)), \"error\": \"load failed\"}")
            continue
        }
        let W = Double(cg.width), H = Double(cg.height)
        let request = VNRecognizeTextRequest()
        request.recognitionLevel = .accurate
        request.recognitionLanguages = ["zh-Hans", "en-US"]
        request.usesLanguageCorrection = false
        let handler = VNImageRequestHandler(cgImage: cg, options: [:])
        var lines: [[String: Any]] = []
        do {
            try handler.perform([request])
            for obs in (request.results ?? []) {
                guard let top = obs.topCandidates(1).first else { continue }
                let bb = obs.boundingBox  // normalized, origin bottom-left
                // convert to pixel coords with origin TOP-left
                let yTop = (1.0 - bb.origin.y - bb.size.height) * H
                lines.append([
                    "text": top.string,
                    "conf": Double(round(top.confidence * 1000) / 1000),
                    "x": Int(bb.origin.x * W),
                    "y": Int(yTop),
                    "w": Int(bb.size.width * W),
                    "h": Int(bb.size.height * H),
                ])
            }
        } catch {
            lines = []
        }
        let all = lines.compactMap { $0["text"] as? String }.joined(separator: " | ")
        let obj: [String: Any] = ["file": file, "text": all, "lines": lines]
        if let d = try? JSONSerialization.data(withJSONObject: obj, options: []),
           let s = String(data: d, encoding: .utf8) {
            print(s)
        }
    }
}

func jsonQuote(_ s: String) -> String {
    let d = (try? JSONSerialization.data(withJSONObject: [s], options: [])) ?? Data("[\"\"]".utf8)
    let arr = String(data: d, encoding: .utf8) ?? "[\"\"]"
    return String(arr.dropFirst().dropLast())
}
