; ModuleID = 'main.bc'
source_filename = "main"

define i64 @test(i64 %0, i64 %1) {
test:
  %a = alloca i64
  store i64 %0, i64* %a
  %b = alloca i64
  store i64 %1, i64* %b
  %a1 = load i64, i64* %a
  %b2 = load i64, i64* %b
  %"0" = icmp sge i64 %a1, %b2
  br i1 %"0", label %"03", label %"1"

"03":                                             ; preds = %test
  %a4 = load i64, i64* %a
  ret i64 %a4

"1":                                              ; preds = %"1", %test
  br label %"1"

"2":                                              ; No predecessors!
  %b5 = load i64, i64* %b
  ret i64 %b5
}

define i64 @main() {
main:
  %a = alloca i64
  store i64 10, i64* %a
  %b = alloca i64
  store i64 5, i64* %b
  %a1 = load i64, i64* %a
  %b2 = load i64, i64* %b
  %test = call i64 @test(i64 %a1, i64 %b2)
  ret i64 %test
}
