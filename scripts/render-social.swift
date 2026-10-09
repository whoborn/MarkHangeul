// Regenerate on macOS: swift scripts/render-social.swift site/share-ko-v1.png
// English: swift scripts/render-social.swift site/share-en-v1.png en
import AppKit
let w = 1200, h = 630
let bitmap = NSBitmapImageRep(bitmapDataPlanes:nil,pixelsWide:w,pixelsHigh:h,bitsPerSample:8,samplesPerPixel:4,hasAlpha:true,isPlanar:false,colorSpaceName:.deviceRGB,bytesPerRow:0,bitsPerPixel:0)!
NSGraphicsContext.saveGraphicsState()
NSGraphicsContext.current = NSGraphicsContext(bitmapImageRep:bitmap)
let ctx = NSGraphicsContext.current!.cgContext
ctx.translateBy(x:0,y:CGFloat(h)); ctx.scaleBy(x:1,y:-1)
NSGraphicsContext.current = NSGraphicsContext(cgContext:ctx,flipped:true)
func color(_ r:CGFloat,_ g:CGFloat,_ b:CGFloat)->NSColor { NSColor(srgbRed:r/255,green:g/255,blue:b/255,alpha:1) }
let green=color(23,77,62), muted=color(105,131,116)
func text(_ s:String,_ x:CGFloat,_ y:CGFloat,_ size:CGFloat,_ c:NSColor,_ bold:Bool=false) {
 let f=NSFont(name:bold ? "AppleSDGothicNeo-Bold":"AppleSDGothicNeo-Regular",size:size)!
 (s as NSString).draw(at:NSPoint(x:x,y:y),withAttributes:[.font:f,.foregroundColor:c])
}
color(248,249,243).setFill(); NSBezierPath(rect:NSRect(x:0,y:0,width:w,height:h)).fill()
text("WHOBORN · HANGEUL DAY 2026",60,38,20,muted,true)
text("MarkHangeul 1.0",60,75,32,green,true)
let english=CommandLine.arguments.contains("en")
text(english ? "Draw the sounds of the world in Hangul." : "한글로 그리는 세계의 소리",60,133,english ? 43:52,green,true)
text(english ? "Pitch becomes a letter’s flow. Duration becomes its width." : "소리의 높낮이는 글자의 흐름으로, 길이는 글자의 너비로.",60,202,25,muted)
color(235,241,230).setFill(); NSBezierPath(roundedRect:NSRect(x:55,y:268,width:1090,height:268),xRadius:24,yRadius:24).fill()
let xs:[CGFloat]=[185,460,735,1010]
let labels=english ? ["High · 55","Rising · 35","Dipping · 214","Falling · 51"]:["높게 · 55","올라가며 · 35","꺾이며 · 214","내려가며 · 51"]
for i in 0..<4 {
 let x=xs[i]
 text("마",x-40,290,92,green)
 let p=NSBezierPath();p.lineWidth=3
 let points:[[CGFloat]] = i==0 ? [[x-60,423],[x+60,423]] : i==1 ? [[x-60,450],[x+60,413]] : i==2 ? [[x-60,422],[x-20,450],[x+60,413]] : [[x-60,413],[x+60,450]]
 p.move(to:NSPoint(x:points[0][0],y:points[0][1]));for v in points.dropFirst(){p.line(to:NSPoint(x:v[0],y:v[1]))};muted.setStroke();p.stroke()
 color(206,129,87).setFill();NSBezierPath(ovalIn:NSRect(x:points[0][0]-5,y:points[0][1]-5,width:10,height:10)).fill()
 text(labels[i],x-68,480,23,green,true)
}
text(english ? "Free · Open source · Markdown pronunciation editor" : "무료 · 오픈소스 · Markdown 발음 편집기",60,571,23,green,true)
text("whoborn.github.io/MarkHangeul",790,577,19,muted)
NSGraphicsContext.restoreGraphicsState()
try bitmap.representation(using:.png,properties:[:])!.write(to:URL(fileURLWithPath:CommandLine.arguments[1]))
