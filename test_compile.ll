; LLVM IR generado por KURA
target triple = "aarch64-apple-darwin"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"

@.str.fmt.i64 = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
declare i32 @printf(i8*, ...)
declare i32 @sprintf(i8*, i8*, ...)
declare i8* @malloc(i64)
declare void @free(i8*)
declare i8* @memcpy(i8*, i8*, i64)
declare i8* @memset(i8*, i32, i64)
declare i64 @strlen(i8*)
declare i8* @strcpy(i8*, i8*)
declare i8* @strcat(i8*, i8*)
declare i8* @kura_array_create(i64, i64)
declare void @kura_array_push(i8*, i8*)
declare i8* @kura_array_get(i8*, i64)
declare i64 @kura_array_len(i8*)
declare void @kura_array_free(i8*)
declare i8* @kura_dict_create(i64)
declare i64 @kura_dict_len(i8*)
declare void @kura_dict_free(i8*)


define i32 @main() {
entry:
  %r0 = alloca i64
  store i64 10, i64* %r0
  %r1 = alloca i64
  store i64 5, i64* %r1
  %r2 = load i64, i64* %r0
  %r3 = load i64, i64* %r1
  %r4 = add i64 %r2, %r3
  %r5 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.fmt.i64, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %r5, i64 %r4)
  ret i32 0
}
